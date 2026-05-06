// SPDX-License-Identifier: Apache-2.0
//
// Schema validator (Category 1, MP-0276 P-02).
//
// Validates RWP artifact documents against the five canonical JSON
// Schemas (handoff, intake, manifest, plan, state) embedded at build
// time from packages/rhumb-protocol/spec/schemas/.
//
// Per ACS-0015 §6 and OQ-15.6, this validator is fully offline:
// - Schemas are embedded via include_str! (no on-disk dependency at runtime)
// - The jsonschema crate is configured with default-features=false
//   (no resolve-http, no resolve-file)
// - Schemas declare zero $ref so no resolver runs

use std::fs;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

use jsonschema::Validator;
use serde_json::Value;

use crate::validators::walk::walk_dir;
use crate::{Category, CategoryResult, Failure, FailureKind};

// ---------------------------------------------------------------------------
// Embedded schemas
// ---------------------------------------------------------------------------

const HANDOFF_SCHEMA: &str = include_str!("../../../spec/schemas/handoff.schema.json");
const INTAKE_SCHEMA: &str = include_str!("../../../spec/schemas/intake.schema.json");
const MANIFEST_SCHEMA: &str = include_str!("../../../spec/schemas/manifest.schema.json");
const PLAN_SCHEMA: &str = include_str!("../../../spec/schemas/plan.schema.json");
const STATE_SCHEMA: &str = include_str!("../../../spec/schemas/state.schema.json");

const SCHEMA_NAMES: &[&str] = &["handoff", "intake", "manifest", "plan", "state"];

// ---------------------------------------------------------------------------
// Compiled-validator cache
//
// Validator construction is the documented expensive step in jsonschema.
// We cache one Result<Validator, String> per schema in a OnceLock so:
//   - successful compilation runs at most once per schema per process
//   - compilation errors are deterministic (embedded schemas don't change)
//     but still surface through Result rather than panicking
// ---------------------------------------------------------------------------

type ValidatorCache = Result<Validator, String>;

fn validator_for(name: &str) -> Result<&'static Validator, String> {
    let (cell, source) = match name {
        "handoff" => {
            static CELL: OnceLock<ValidatorCache> = OnceLock::new();
            (&CELL, HANDOFF_SCHEMA)
        }
        "intake" => {
            static CELL: OnceLock<ValidatorCache> = OnceLock::new();
            (&CELL, INTAKE_SCHEMA)
        }
        "manifest" => {
            static CELL: OnceLock<ValidatorCache> = OnceLock::new();
            (&CELL, MANIFEST_SCHEMA)
        }
        "plan" => {
            static CELL: OnceLock<ValidatorCache> = OnceLock::new();
            (&CELL, PLAN_SCHEMA)
        }
        "state" => {
            static CELL: OnceLock<ValidatorCache> = OnceLock::new();
            (&CELL, STATE_SCHEMA)
        }
        other => return Err(format!("unknown RWP schema: {other}")),
    };

    let cached = cell.get_or_init(|| compile_one(name, source));
    cached.as_ref().map_err(|s| s.clone())
}

fn compile_one(name: &str, source: &str) -> ValidatorCache {
    let schema: Value = serde_json::from_str(source)
        .map_err(|e| format!("embedded {name} schema parse error: {e}"))?;
    jsonschema::draft7::new(&schema)
        .map_err(|e| format!("embedded {name} schema compile error: {e}"))
}

// ---------------------------------------------------------------------------
// Document discovery + schema-name resolution
// ---------------------------------------------------------------------------

/// Walk `target` recursively and yield every regular file with a `.json`
/// extension. Symlinks are not followed. Hidden directories (starting with
/// `.`) are skipped to avoid descending into VCS or editor metadata.
///
/// Recursion + symlink/hidden-dir handling is delegated to
/// `validators::walk::walk_dir` (factored in P-05 from the duplicated
/// walkers in schema.rs / template.rs / workflow.rs).
fn discover_json_files(target: &Path, out: &mut Vec<PathBuf>) -> std::io::Result<()> {
    walk_dir(
        target,
        &mut |path| {
            if has_json_extension(path) {
                out.push(path.to_path_buf());
            }
        },
        &mut |_| false,
    )
}

fn has_json_extension(path: &Path) -> bool {
    path.extension()
        .and_then(|e| e.to_str())
        .is_some_and(|e| e.eq_ignore_ascii_case("json"))
}

/// Map a candidate document file to one of the RWP schema names.
///
/// Resolution order:
///   1. `$schema` field — if it points at the JSON Schema meta-schema
///      (json-schema.org), the document is itself a JSON Schema, not an
///      RWP instance. Silently skip (return None).
///   2. `$schema` field — if it points at one of the RWP schema URIs,
///      that wins (most explicit, highest fidelity).
///   3. Filename-segment heuristic — split the file stem on `-` and `_`,
///      match the leftmost segment that equals an RWP schema name.
///
/// Returns `None` for files that look like JSON but aren't RWP artifacts
/// (e.g., `package.json`, `tsconfig.json`, `*.schema.json` meta-schemas).
/// These are silently skipped rather than counted as failures.
fn resolve_schema_name(path: &Path, value: &Value) -> Option<&'static str> {
    if let Some(uri) = value.get("$schema").and_then(Value::as_str) {
        if uri.starts_with("http://json-schema.org/")
            || uri.starts_with("https://json-schema.org/")
        {
            return None;
        }
        for name in SCHEMA_NAMES {
            if uri.contains(&format!("/{name}.schema.json")) {
                return Some(name);
            }
        }
    }
    let stem = path.file_stem().and_then(|s| s.to_str())?.to_ascii_lowercase();
    for segment in stem.split(['-', '_']) {
        for name in SCHEMA_NAMES {
            if segment == *name {
                return Some(name);
            }
        }
    }
    None
}

// ---------------------------------------------------------------------------
// Validator entry point
// ---------------------------------------------------------------------------

/// Run schema validation against every RWP artifact under `target`.
///
/// Per ACS-0015 §6 contract:
/// - I/O errors push a `FailureKind::Io` failure and continue to the next
///   file rather than aborting the whole run.
/// - Schema violations push a `FailureKind::SchemaMismatch` with the first
///   error's instance pointer in `details`.
/// - Files whose schema cannot be resolved (no `$schema`, no matching
///   filename keyword) are silently skipped — they are not RWP artifacts.
pub fn run(target: &Path, result: &mut CategoryResult) {
    let mut files = Vec::new();
    if let Err(err) = discover_json_files(target, &mut files) {
        result.failures.push(Failure {
            fixture: target.display().to_string(),
            category: Category::Schema,
            kind: FailureKind::Io,
            message: format!("failed to enumerate target tree: {err}"),
            details: None,
        });
        result.failed = result.failed.saturating_add(1);
        return;
    }

    files.sort();

    for file in files {
        validate_one(&file, result);
    }
}

fn validate_one(file: &Path, result: &mut CategoryResult) {
    let fixture = file.display().to_string();

    let bytes = match fs::read(file) {
        Ok(b) => b,
        Err(err) => {
            result.failures.push(Failure {
                fixture,
                category: Category::Schema,
                kind: FailureKind::Io,
                message: format!("read failed: {err}"),
                details: None,
            });
            result.failed = result.failed.saturating_add(1);
            return;
        }
    };

    let mut value: Value = match serde_json::from_slice(&bytes) {
        Ok(v) => v,
        Err(err) => {
            result.failures.push(Failure {
                fixture,
                category: Category::Schema,
                kind: FailureKind::SchemaMismatch,
                message: format!("not valid JSON: {err}"),
                details: None,
            });
            result.failed = result.failed.saturating_add(1);
            return;
        }
    };

    let schema_name = match resolve_schema_name(file, &value) {
        Some(n) => n,
        None => {
            // Not an RWP artifact — silently skip without counting.
            return;
        }
    };

    let validator = match validator_for(schema_name) {
        Ok(v) => v,
        Err(err) => {
            result.failures.push(Failure {
                fixture,
                category: Category::Schema,
                kind: FailureKind::Internal,
                message: format!("validator construction failed for {schema_name}: {err}"),
                details: None,
            });
            result.failed = result.failed.saturating_add(1);
            return;
        }
    };

    // Strip `$schema` before validation. The RWP schemas all declare
    // additionalProperties: false at root, but `$schema` is a universal
    // JSON-Schema-aware convention used by IDEs and tooling. Documents
    // are allowed to carry it for completion hints without conflicting
    // with the schema's strict-property rule.
    if let Some(obj) = value.as_object_mut() {
        obj.remove("$schema");
    }

    if validator.is_valid(&value) {
        result.passed = result.passed.saturating_add(1);
        return;
    }

    let first_error = validator.iter_errors(&value).next();
    let (message, details) = match first_error {
        Some(err) => (
            format!("does not conform to {schema_name} schema: {err}"),
            Some(format!(
                "instance_path={} schema_path={}",
                err.instance_path, err.schema_path
            )),
        ),
        None => (
            format!("does not conform to {schema_name} schema (no error details available)"),
            None,
        ),
    };

    result.failures.push(Failure {
        fixture,
        category: Category::Schema,
        kind: FailureKind::SchemaMismatch,
        message,
        details,
    });
    result.failed = result.failed.saturating_add(1);
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    type TestResult = Result<(), Box<dyn std::error::Error>>;

    fn fixtures_root() -> PathBuf {
        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        PathBuf::from(manifest_dir).join("fixtures")
    }

    #[test]
    fn embedded_schemas_compile() -> TestResult {
        for name in SCHEMA_NAMES {
            let v = validator_for(name)?;
            // Trivial: the compiled validator should reject a non-object
            // for every RWP schema (all five are root `type: object`).
            assert!(
                !v.is_valid(&serde_json::json!("string")),
                "{name} validator unexpectedly accepted a string at root"
            );
        }
        Ok(())
    }

    #[test]
    fn schema_name_resolved_from_filename() {
        let cases = [
            ("valid-plan.json", Some("plan")),
            ("valid-intake.json", Some("intake")),
            ("invalid-manifest-missing-plan-id.json", Some("manifest")),
            ("invalid-state-bad-status.json", Some("state")),
            ("invalid-handoff-no-metadata.json", Some("handoff")),
            ("package.json", None),
            ("tsconfig.json", None),
        ];
        for (filename, expected) in cases {
            let path = PathBuf::from(filename);
            let value = serde_json::Value::Object(serde_json::Map::new());
            let got = resolve_schema_name(&path, &value);
            assert_eq!(got, expected, "filename: {filename}");
        }
    }

    #[test]
    fn schema_name_resolved_from_dollar_schema_field() {
        let path = PathBuf::from("anything.json");
        let value = serde_json::json!({
            "$schema": "https://rhumbprotocol.dev/schemas/plan.schema.json"
        });
        assert_eq!(resolve_schema_name(&path, &value), Some("plan"));
    }

    #[test]
    fn json_schema_meta_documents_are_silently_skipped() {
        // Documents that ARE JSON Schemas (their $schema points at the
        // JSON Schema meta-schema, not at one of the five RWP schemas)
        // must not be classified as RWP instances. Without this guard
        // the filename-stem heuristic would match e.g. `state-update`
        // → "state" and force-validate the schema document against the
        // state instance schema, producing an "additionalProperties not
        // allowed" failure cascade.
        let cases = [
            // draft 2020-12, no fragment
            "https://json-schema.org/draft/2020-12/schema",
            // draft-07 with fragment
            "http://json-schema.org/draft-07/schema#",
            // draft-04
            "http://json-schema.org/draft-04/schema#",
        ];
        for uri in cases {
            let path = PathBuf::from("state-update.schema.json");
            let value = serde_json::json!({"$schema": uri});
            assert_eq!(
                resolve_schema_name(&path, &value),
                None,
                "expected $schema={uri} on state-update.schema.json to be silently skipped",
            );
        }
    }

    #[test]
    fn schema_run_passes_valid_fixtures() -> TestResult {
        let valid_dir = fixtures_root().join("valid").join("schemas");
        if !valid_dir.exists() {
            // Phase ordering: tests run after T-02-05 lands fixtures.
            // If fixtures haven't been created yet, treat as a setup error.
            return Err(format!(
                "fixture directory missing: {}",
                valid_dir.display()
            )
            .into());
        }
        let mut result = CategoryResult::empty(Category::Schema);
        run(&valid_dir, &mut result);
        assert_eq!(
            result.failed, 0,
            "valid fixtures produced failures: {:?}",
            result.failures
        );
        assert!(
            result.passed >= 5,
            "expected at least 5 valid fixtures, got passed={}",
            result.passed
        );
        Ok(())
    }

    #[test]
    fn schema_run_rejects_invalid_fixtures() -> TestResult {
        let invalid_dir = fixtures_root().join("invalid").join("schemas");
        if !invalid_dir.exists() {
            return Err(format!(
                "fixture directory missing: {}",
                invalid_dir.display()
            )
            .into());
        }
        let mut result = CategoryResult::empty(Category::Schema);
        run(&invalid_dir, &mut result);
        assert_eq!(
            result.passed, 0,
            "invalid fixtures unexpectedly passed: passed={}",
            result.passed
        );
        assert!(
            result.failed >= 5,
            "expected at least 5 invalid fixtures to fail, got failed={}",
            result.failed
        );
        for failure in &result.failures {
            assert_eq!(
                failure.kind,
                FailureKind::SchemaMismatch,
                "non-schema-mismatch failure: {failure:?}"
            );
        }
        Ok(())
    }

    #[test]
    fn schema_run_skips_non_rwp_json_silently() -> TestResult {
        // A package.json-shaped file shouldn't count toward passed or failed.
        let tmp = std::env::temp_dir().join("rhumb-validate-skip-test");
        let _ = fs::remove_dir_all(&tmp);
        fs::create_dir_all(&tmp)?;
        let pkg = tmp.join("package.json");
        fs::write(&pkg, br#"{"name":"x","version":"1.0.0"}"#)?;

        let mut result = CategoryResult::empty(Category::Schema);
        run(&tmp, &mut result);
        assert_eq!(result.passed, 0);
        assert_eq!(result.failed, 0);
        assert!(result.failures.is_empty());

        let _ = fs::remove_dir_all(&tmp);
        Ok(())
    }

    #[test]
    fn schema_run_handles_missing_target_gracefully() {
        let mut result = CategoryResult::empty(Category::Schema);
        let nowhere = PathBuf::from("/does/not/exist/anywhere/rhumb-validate");
        run(&nowhere, &mut result);
        // discover_json_files short-circuits on !exists() with Ok(()), so
        // no I/O failure is reported and no documents validated.
        assert_eq!(result.passed, 0);
        assert_eq!(result.failed, 0);
        assert!(result.failures.is_empty());
    }
}
