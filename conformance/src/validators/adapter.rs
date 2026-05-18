// SPDX-License-Identifier: Apache-2.0
//
// Adapter validator (Category 4, MP-0276 P-05).
//
// Validates the shape of integration adapters under a target tree. An
// adapter is a directory containing a `MANIFEST.yaml` file at its root.
// The walker stops descending once it finds one — the manifest's
// presence claims the directory as an adapter root, so its
// subdirectories (commands/, skills/, knowledge/, etc.) are not
// re-classified as nested adapters. Same stop-on-find pattern as the
// workflow validator (P-04).
//
// What an adapter manifest must contain (six structural invariants):
//   ADP-1  Top-level `integration:` mapping is present.
//   ADP-2  `integration.name` is a non-empty string.
//   ADP-3  `integration.platform` is a non-empty string.
//   ADP-4  `integration.rwp_version` is a non-empty string.
//   ADP-5  `integration.version` is a non-empty string.
//   ADP-6  Top-level `components:` mapping is present (an empty map is
//          accepted — the validator checks shape, not adoption depth).
//
// Each violation emits one `FailureKind::AdapterShapeMismatch` failure
// with a distinct details payload keyed by the ADP-N message prefix.
// External implementers can grep for `ADP-3` to locate the broken
// invariant type.
//
// Counter discipline (binding for re-implementers — mirrors P-04's
// per-workflow rule): each manifest counts as exactly ONE fixture.
// Multiple invariant violations within a single manifest contribute
// multiple `Failure` entries to `result.failures` but increment
// `result.failed` by 1. The user-meaningful unit is "did this adapter
// validate?" rather than "how many invariant types were violated?".
//
// Out of scope (deferred — flagged in handoff, not enforced here):
//   - Cross-reference checks: does `integration.components.commands[i].path`
//     resolve on disk, do referenced templates/schemas exist? These would
//     expand Category 4 toward end-to-end install validation. Defer.
//   - Semver validation on `rwp_version` / `version`. Strings non-empty
//     is the bar. Format checks belong to a future MP if demand
//     materializes.
//   - Adoption-depth signals (e.g., "≥1 component must be defined").
//     A platform that only ships project-instructions and no commands
//     is still a valid adapter shape.
//
// Why YAML and not JSON: the only manifest in the canonical tree
// today (claude-code/MANIFEST.yaml) is YAML, and the spec line
// (ACS-0015 §5 line 433) says "manifest" without picking a format.
// YAML matches what's already shipping, and serde_yml is already in
// the dep tree from P-04 — no new dependency cost.
//
// Files outside the discovered adapter set (random `.md`, `commands/`
// subdirectories of an adapter root, etc.) are silently skipped — the
// same discipline as P-02..P-04. Dirs without a `MANIFEST.yaml` are
// not adapters and contribute neither pass nor fail.

use std::fs;
use std::path::{Path, PathBuf};

use crate::validators::walk::walk_dir;
use crate::{Category, CategoryResult, Failure, FailureKind};

// ---------------------------------------------------------------------------
// Adapter-root discovery
// ---------------------------------------------------------------------------

const MANIFEST_FILENAME: &str = "MANIFEST.yaml";

/// Walk `target` recursively, yielding every directory that contains a
/// `MANIFEST.yaml` file at its root. Once an adapter root is found,
/// descent into its subdirectories stops.
///
/// Recursion + symlink/hidden-dir handling + stop-on-find is delegated
/// to `validators::walk::walk_dir`.
fn discover_adapter_roots(target: &Path, out: &mut Vec<PathBuf>) -> std::io::Result<()> {
    walk_dir(
        target,
        &mut |_| {},
        &mut |dir| {
            if has_manifest(dir) {
                out.push(dir.to_path_buf());
                true
            } else {
                false
            }
        },
    )
}

fn has_manifest(dir: &Path) -> bool {
    dir.join(MANIFEST_FILENAME).is_file()
}

// ---------------------------------------------------------------------------
// Per-adapter validation
// ---------------------------------------------------------------------------

fn validate_adapter(root: &Path, result: &mut CategoryResult) {
    let adapter_id = root.display().to_string();
    let manifest_path = root.join(MANIFEST_FILENAME);

    let manifest_text = match fs::read_to_string(&manifest_path) {
        Ok(t) => t,
        Err(err) => {
            push_io(result, &adapter_id, &manifest_path, err);
            result.failed = result.failed.saturating_add(1);
            return;
        }
    };

    let mut violations: Vec<Failure> = Vec::new();

    let value: serde_yml::Value = match serde_yml::from_str(&manifest_text) {
        Ok(v) => v,
        Err(err) => {
            violations.push(adapter_break(
                &adapter_id,
                "MANIFEST.yaml is not valid YAML",
                Some(err.to_string()),
            ));
            result.failures.extend(violations);
            result.failed = result.failed.saturating_add(1);
            return;
        }
    };

    let manifest_map = match value.as_mapping() {
        Some(m) => m,
        None => {
            violations.push(adapter_break(
                &adapter_id,
                "MANIFEST.yaml top-level value is not a mapping",
                None,
            ));
            result.failures.extend(violations);
            result.failed = result.failed.saturating_add(1);
            return;
        }
    };

    // ADP-1 + ADP-2..5: integration block + required string fields.
    let integration_value = manifest_map.get("integration");
    let integration_mapping = integration_value.and_then(|v| v.as_mapping());
    match (integration_value, integration_mapping) {
        (None, _) => {
            violations.push(adapter_break(
                &adapter_id,
                "ADP-1 MANIFEST.yaml is missing the top-level 'integration:' block",
                None,
            ));
        }
        (Some(_), None) => {
            violations.push(adapter_break(
                &adapter_id,
                "ADP-1 MANIFEST.yaml 'integration:' is not a mapping",
                None,
            ));
        }
        (Some(_), Some(integration)) => {
            check_required_string(integration, "name", "ADP-2", &adapter_id, &mut violations);
            check_required_string(
                integration,
                "platform",
                "ADP-3",
                &adapter_id,
                &mut violations,
            );
            check_required_string(
                integration,
                "rwp_version",
                "ADP-4",
                &adapter_id,
                &mut violations,
            );
            check_required_string(
                integration,
                "version",
                "ADP-5",
                &adapter_id,
                &mut violations,
            );
        }
    }

    // ADP-6: components block must be present and a mapping (empty OK).
    match manifest_map.get("components") {
        None => {
            violations.push(adapter_break(
                &adapter_id,
                "ADP-6 MANIFEST.yaml is missing the top-level 'components:' block",
                None,
            ));
        }
        Some(v) if v.is_mapping() => {}
        Some(serde_yml::Value::Null) => {
            violations.push(adapter_break(
                &adapter_id,
                "ADP-6 MANIFEST.yaml 'components:' is null (expected a mapping; empty map allowed)",
                None,
            ));
        }
        Some(_) => {
            violations.push(adapter_break(
                &adapter_id,
                "ADP-6 MANIFEST.yaml 'components:' is not a mapping",
                None,
            ));
        }
    }

    if violations.is_empty() {
        result.passed = result.passed.saturating_add(1);
    } else {
        result.failures.extend(violations);
        result.failed = result.failed.saturating_add(1);
    }
}

/// Check a required `integration.<key>` field for presence, string type,
/// and non-empty content. Pushes one violation tagged with `code` if the field
/// is missing, wrong-typed, or empty.
fn check_required_string(
    integration: &serde_yml::Mapping,
    key: &str,
    code: &str,
    adapter_id: &str,
    violations: &mut Vec<Failure>,
) {
    match integration.get(key) {
        None => violations.push(adapter_break(
            adapter_id,
            &format!("{code} MANIFEST.yaml integration.{key} is missing"),
            None,
        )),
        Some(serde_yml::Value::String(s)) if !s.is_empty() => {}
        Some(serde_yml::Value::String(_)) => violations.push(adapter_break(
            adapter_id,
            &format!("{code} MANIFEST.yaml integration.{key} is an empty string"),
            None,
        )),
        Some(_) => violations.push(adapter_break(
            adapter_id,
            &format!("{code} MANIFEST.yaml integration.{key} is not a string"),
            None,
        )),
    }
}

fn adapter_break(adapter_id: &str, message: &str, details: Option<String>) -> Failure {
    Failure {
        fixture: adapter_id.to_string(),
        category: Category::Adapter,
        kind: FailureKind::AdapterShapeMismatch,
        message: message.to_string(),
        details,
    }
}

fn push_io(result: &mut CategoryResult, adapter_id: &str, path: &Path, err: std::io::Error) {
    result.failures.push(Failure {
        fixture: adapter_id.to_string(),
        category: Category::Adapter,
        kind: FailureKind::Io,
        message: format!("read failed: {}", path.display()),
        details: Some(err.to_string()),
    });
}

// ---------------------------------------------------------------------------
// Validator entry point
// ---------------------------------------------------------------------------

/// Run static adapter shape validation against every adapter root
/// discovered under `target`.
pub fn run(target: &Path, result: &mut CategoryResult) {
    let mut roots = Vec::new();
    if let Err(err) = discover_adapter_roots(target, &mut roots) {
        result.failures.push(Failure {
            fixture: target.display().to_string(),
            category: Category::Adapter,
            kind: FailureKind::Io,
            message: format!("failed to enumerate target tree: {err}"),
            details: None,
        });
        result.failed = result.failed.saturating_add(1);
        return;
    }
    roots.sort();
    for root in &roots {
        validate_adapter(root, result);
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    type TestResult = Result<(), Box<dyn std::error::Error>>;

    fn fixtures_root() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("fixtures")
    }

    fn integrations_root() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../integrations")
    }

    fn run_against(target: &Path) -> CategoryResult {
        let mut result = CategoryResult::empty(Category::Adapter);
        super::run(target, &mut result);
        result
    }

    fn write_manifest(dir: &Path, body: &str) -> std::io::Result<()> {
        fs::create_dir_all(dir)?;
        fs::write(dir.join(MANIFEST_FILENAME), body)
    }

    fn temp_path(tag: &str) -> PathBuf {
        std::env::temp_dir().join(format!(
            "rhumb-validate-adapter-{}-{}-{}",
            tag,
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_nanos())
                .unwrap_or(0)
        ))
    }

    const MINIMAL_VALID_MANIFEST: &str = "\
integration:
  name: \"Test Adapter\"
  platform: \"test-platform\"
  rwp_version: \"0.29.0\"
  version: \"0.1.0\"
components: {}
";

    // -----------------------------------------------------------------
    // Discovery tests
    // -----------------------------------------------------------------

    #[test]
    fn run_returns_empty_on_nonexistent_target() {
        let r = run_against(Path::new(
            "/this/path/should/not/exist/rhumb-validate-empty",
        ));
        assert_eq!(r.passed, 0);
        assert_eq!(r.failed, 0);
        assert_eq!(r.skipped, 0);
        assert!(r.failures.is_empty());
    }

    #[test]
    fn run_skips_directories_without_manifest_silently() -> TestResult {
        let tmp = temp_path("no-manifest");
        let _ = fs::remove_dir_all(&tmp);
        fs::create_dir_all(tmp.join("not-an-adapter"))?;
        fs::write(tmp.join("not-an-adapter/README.md"), b"x")?;

        let r = run_against(&tmp);
        assert_eq!(r.passed, 0);
        assert_eq!(r.failed, 0);
        assert!(r.failures.is_empty());

        let _ = fs::remove_dir_all(&tmp);
        Ok(())
    }

    #[test]
    fn run_treats_manifest_dir_as_root_and_does_not_recurse() -> TestResult {
        let tmp = temp_path("stop-descent");
        let _ = fs::remove_dir_all(&tmp);
        // Outer adapter at tmp/outer; nested subdir tmp/outer/inner with
        // its OWN MANIFEST.yaml. The outer's manifest claims the dir, so
        // the inner one must NOT be discovered.
        write_manifest(&tmp.join("outer"), MINIMAL_VALID_MANIFEST)?;
        write_manifest(&tmp.join("outer/inner"), MINIMAL_VALID_MANIFEST)?;

        let r = run_against(&tmp);
        assert_eq!(r.passed, 1, "expected only the outer adapter");
        assert_eq!(r.failed, 0);
        assert!(r.failures.is_empty());

        let _ = fs::remove_dir_all(&tmp);
        Ok(())
    }

    #[test]
    fn run_finds_multiple_sibling_adapters() -> TestResult {
        let tmp = temp_path("siblings");
        let _ = fs::remove_dir_all(&tmp);
        write_manifest(&tmp.join("a"), MINIMAL_VALID_MANIFEST)?;
        write_manifest(&tmp.join("b"), MINIMAL_VALID_MANIFEST)?;
        write_manifest(&tmp.join("c"), MINIMAL_VALID_MANIFEST)?;

        let r = run_against(&tmp);
        assert_eq!(r.passed, 3);
        assert_eq!(r.failed, 0);

        let _ = fs::remove_dir_all(&tmp);
        Ok(())
    }

    // -----------------------------------------------------------------
    // Invariant tests (ADP-1..ADP-6)
    // -----------------------------------------------------------------

    #[test]
    fn adp1_missing_integration_block() -> TestResult {
        let tmp = temp_path("adp1");
        let _ = fs::remove_dir_all(&tmp);
        write_manifest(
            &tmp.join("a"),
            "components: {}\nother: stuff\n",
        )?;

        let r = run_against(&tmp);
        assert_eq!(r.failed, 1);
        assert_eq!(r.passed, 0);
        assert!(
            r.failures.iter().any(|f| f.message.contains("ADP-1")),
            "expected ADP-1 violation; got {:?}",
            r.failures
        );

        let _ = fs::remove_dir_all(&tmp);
        Ok(())
    }

    #[test]
    fn adp1_integration_not_a_mapping() -> TestResult {
        let tmp = temp_path("adp1-not-map");
        let _ = fs::remove_dir_all(&tmp);
        write_manifest(
            &tmp.join("a"),
            "integration: \"oops\"\ncomponents: {}\n",
        )?;

        let r = run_against(&tmp);
        assert_eq!(r.failed, 1);
        assert!(
            r.failures
                .iter()
                .any(|f| f.message.contains("ADP-1") && f.message.contains("not a mapping")),
            "expected ADP-1 not-a-mapping violation; got {:?}",
            r.failures
        );

        let _ = fs::remove_dir_all(&tmp);
        Ok(())
    }

    #[test]
    fn adp2_missing_name() -> TestResult {
        let tmp = temp_path("adp2");
        let _ = fs::remove_dir_all(&tmp);
        write_manifest(
            &tmp.join("a"),
            "\
integration:
  platform: p
  rwp_version: 1.0.0
  version: 0.1.0
components: {}
",
        )?;

        let r = run_against(&tmp);
        assert_eq!(r.failed, 1);
        assert!(
            r.failures
                .iter()
                .any(|f| f.message.contains("ADP-2") && f.message.contains("name")),
            "expected ADP-2 missing-name violation; got {:?}",
            r.failures
        );

        let _ = fs::remove_dir_all(&tmp);
        Ok(())
    }

    #[test]
    fn adp3_empty_platform() -> TestResult {
        let tmp = temp_path("adp3");
        let _ = fs::remove_dir_all(&tmp);
        write_manifest(
            &tmp.join("a"),
            "\
integration:
  name: A
  platform: \"\"
  rwp_version: 1.0.0
  version: 0.1.0
components: {}
",
        )?;

        let r = run_against(&tmp);
        assert_eq!(r.failed, 1);
        assert!(
            r.failures
                .iter()
                .any(|f| f.message.contains("ADP-3") && f.message.contains("empty")),
            "expected ADP-3 empty-platform violation; got {:?}",
            r.failures
        );

        let _ = fs::remove_dir_all(&tmp);
        Ok(())
    }

    #[test]
    fn adp4_rwp_version_wrong_type() -> TestResult {
        let tmp = temp_path("adp4");
        let _ = fs::remove_dir_all(&tmp);
        write_manifest(
            &tmp.join("a"),
            "\
integration:
  name: A
  platform: p
  rwp_version: 1.0
  version: 0.1.0
components: {}
",
        )?;

        let r = run_against(&tmp);
        assert_eq!(r.failed, 1);
        assert!(
            r.failures
                .iter()
                .any(|f| f.message.contains("ADP-4") && f.message.contains("not a string")),
            "expected ADP-4 not-a-string violation; got {:?}",
            r.failures
        );

        let _ = fs::remove_dir_all(&tmp);
        Ok(())
    }

    #[test]
    fn adp5_missing_version() -> TestResult {
        let tmp = temp_path("adp5");
        let _ = fs::remove_dir_all(&tmp);
        write_manifest(
            &tmp.join("a"),
            "\
integration:
  name: A
  platform: p
  rwp_version: 1.0.0
components: {}
",
        )?;

        let r = run_against(&tmp);
        assert_eq!(r.failed, 1);
        assert!(
            r.failures
                .iter()
                .any(|f| f.message.contains("ADP-5") && f.message.contains("version")),
            "expected ADP-5 missing-version violation; got {:?}",
            r.failures
        );

        let _ = fs::remove_dir_all(&tmp);
        Ok(())
    }

    #[test]
    fn adp6_missing_components() -> TestResult {
        let tmp = temp_path("adp6");
        let _ = fs::remove_dir_all(&tmp);
        write_manifest(
            &tmp.join("a"),
            "\
integration:
  name: A
  platform: p
  rwp_version: 1.0.0
  version: 0.1.0
",
        )?;

        let r = run_against(&tmp);
        assert_eq!(r.failed, 1);
        assert!(
            r.failures
                .iter()
                .any(|f| f.message.contains("ADP-6") && f.message.contains("missing")),
            "expected ADP-6 missing-components violation; got {:?}",
            r.failures
        );

        let _ = fs::remove_dir_all(&tmp);
        Ok(())
    }

    #[test]
    fn adp6_components_is_list_not_map() -> TestResult {
        let tmp = temp_path("adp6-list");
        let _ = fs::remove_dir_all(&tmp);
        write_manifest(
            &tmp.join("a"),
            "\
integration:
  name: A
  platform: p
  rwp_version: 1.0.0
  version: 0.1.0
components:
  - one
  - two
",
        )?;

        let r = run_against(&tmp);
        assert_eq!(r.failed, 1);
        assert!(
            r.failures
                .iter()
                .any(|f| f.message.contains("ADP-6") && f.message.contains("not a mapping")),
            "expected ADP-6 not-a-mapping violation; got {:?}",
            r.failures
        );

        let _ = fs::remove_dir_all(&tmp);
        Ok(())
    }

    // -----------------------------------------------------------------
    // Counter discipline + multiple-violations
    // -----------------------------------------------------------------

    #[test]
    fn multiple_violations_in_one_manifest_increment_failed_by_one() -> TestResult {
        let tmp = temp_path("multi-violate");
        let _ = fs::remove_dir_all(&tmp);
        // No name, no platform, no rwp_version, no version, no components.
        write_manifest(&tmp.join("a"), "integration: {}\n")?;

        let r = run_against(&tmp);
        assert_eq!(r.failed, 1, "one adapter, one fail-count");
        assert!(
            r.failures.len() >= 5,
            "expected ≥5 distinct invariant entries (ADP-2..6); got {}",
            r.failures.len()
        );

        let _ = fs::remove_dir_all(&tmp);
        Ok(())
    }

    // -----------------------------------------------------------------
    // YAML parse errors
    // -----------------------------------------------------------------

    #[test]
    fn yaml_parse_error_emits_adapter_shape_mismatch() -> TestResult {
        let tmp = temp_path("bad-yaml");
        let _ = fs::remove_dir_all(&tmp);
        // Genuinely invalid YAML — unbalanced bracket.
        write_manifest(&tmp.join("a"), "integration: [\n")?;

        let r = run_against(&tmp);
        assert_eq!(r.failed, 1);
        assert!(
            r.failures
                .iter()
                .any(|f| f.kind == FailureKind::AdapterShapeMismatch),
            "expected AdapterShapeMismatch; got {:?}",
            r.failures
        );

        let _ = fs::remove_dir_all(&tmp);
        Ok(())
    }

    // -----------------------------------------------------------------
    // Fixture corpus + canonical-self-validation
    // -----------------------------------------------------------------

    #[test]
    fn run_passes_canonical_valid_fixtures() {
        let corpus = fixtures_root().join("valid/adapters");
        if !corpus.is_dir() {
            // Corpus not yet authored when this test was first written;
            // it lands in the same phase. Skip with a clear message.
            return;
        }
        let r = run_against(&corpus);
        assert_eq!(
            r.failed, 0,
            "valid adapter fixtures unexpectedly failed: {:?}",
            r.failures
        );
        assert!(
            r.passed >= 3,
            "expected ≥3 passing adapter fixtures (ACS-0015 §9 contract); got {}",
            r.passed
        );
    }

    #[test]
    fn run_fails_canonical_invalid_fixtures() {
        let corpus = fixtures_root().join("invalid/adapters");
        if !corpus.is_dir() {
            return;
        }
        let r = run_against(&corpus);
        assert!(
            r.failed >= 3,
            "expected ≥3 failing adapter fixtures; got {}",
            r.failed
        );
    }

    /// Self-validation: walking the canonical integrations/ tree should
    /// find at least the claude-code adapter (the only one with a
    /// MANIFEST.yaml today) and validate its shape successfully.
    /// Other platforms in integrations/ ship ad-hoc instructions but no
    /// canonical manifest yet — they are silently skipped per the
    /// validator's discipline.
    #[test]
    fn run_passes_canonical_integrations_tree() {
        let corpus = integrations_root();
        if !corpus.is_dir() {
            return;
        }
        let r = run_against(&corpus);
        assert_eq!(
            r.failed, 0,
            "canonical integrations tree should validate; failures: {:?}",
            r.failures
        );
        assert!(
            r.passed >= 1,
            "expected ≥1 passing adapter from integrations/; got {} (failures: {:?})",
            r.passed,
            r.failures
        );
    }
}
