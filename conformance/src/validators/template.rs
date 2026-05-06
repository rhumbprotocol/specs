// SPDX-License-Identifier: Apache-2.0
//
// Template validator (Category 2, MP-0276 P-03).
//
// Computes a SHA-256 over a canonicalized form of each canonical RWP
// template and compares it to the canonicalized hash of every matching
// file under the target tree. Any difference is reported as
// `FailureKind::TemplateRenderDiff`.
//
// OQ-15.4 resolution (locked in this phase): drift-hash / identity-render.
// ACS-0015 §5 says "renders each template with known inputs, hash-checks"
// and the fixtures/valid/templates/ corpus is described as "expected-output
// hashes for each of 17 templates" — exactly one hash per template, not
// per (template × input set). Since AVD-0004 / ACS-0015 do not define a
// placeholder grammar or a canonical input dictionary, "render with known
// inputs" is interpreted as the identity render: the canonical template is
// hashed as-is (after canonicalization) and compared to the candidate file
// hashed in the same way. Real placeholder substitution is deferred to a
// future MP if and when a grammar is specified.
//
// Canonicalization (binding for any re-implementation):
//   1. Strip UTF-8 BOM if present.
//   2. Replace every CRLF with LF.
//   3. Replace any standalone CR (legacy Mac) with LF.
//   4. Ensure exactly one trailing LF.
// Per-line trailing-whitespace stripping is DELIBERATELY NOT applied:
// some Markdown flavors treat trailing two-space sequences as a hard line
// break, and altering that semantic in canonicalization would let real
// drift slip through. The four canonicalization steps above cover the
// common platform-induced (CRLF) and editor-induced (BOM, missing trailing
// newline) noise without rewriting meaningful whitespace.
//
// Per ACS-0015 §6 contract:
// - I/O errors push a `FailureKind::Io` failure and continue.
// - Hash mismatches push a `FailureKind::TemplateRenderDiff` with the
//   expected and actual hashes in `details`.
// - Files whose name does not match any canonical RWP template are
//   silently skipped (per P-02 binding decision: non-target files are
//   invisible to the validator, they don't increment any counter).

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

use sha2::{Digest, Sha256};

use crate::validators::walk::walk_dir;
use crate::{Category, CategoryResult, Failure, FailureKind};

// ---------------------------------------------------------------------------
// Embedded canonical templates
//
// 17 templates per ACS-0015 §3 and §5 corpus layout. Identified by full
// filename (not extension) because four of the seventeen do not carry the
// `.template` suffix (`ACS-TEMPLATE.md`, `AVD-TEMPLATE.md`,
// `HANDOFF-TEMPLATE.md`, `PHASE-AUDIT.md`). Filename-allowlist matching is
// explicit and survives future templates being added without changing the
// discovery logic.
// ---------------------------------------------------------------------------

const TEMPLATES: &[(&str, &str)] = &[
    // Top-level templates (9)
    (
        "DEPENDENCIES.yaml.template",
        include_str!("../../../templates/DEPENDENCIES.yaml.template"),
    ),
    (
        "INTAKE.yaml.template",
        include_str!("../../../templates/INTAKE.yaml.template"),
    ),
    (
        "MANIFEST-PLAN.yaml.template",
        include_str!("../../../templates/MANIFEST-PLAN.yaml.template"),
    ),
    (
        "MASTERPLAN.yaml.template",
        include_str!("../../../templates/MASTERPLAN.yaml.template"),
    ),
    (
        "PLAN-STATE.yaml.template",
        include_str!("../../../templates/PLAN-STATE.yaml.template"),
    ),
    (
        "PLAN.md.template",
        include_str!("../../../templates/PLAN.md.template"),
    ),
    (
        "PROMPT.md.template",
        include_str!("../../../templates/PROMPT.md.template"),
    ),
    (
        "sequences.yaml.template",
        include_str!("../../../templates/sequences.yaml.template"),
    ),
    (
        "START-PROMPT.md.template",
        include_str!("../../../templates/START-PROMPT.md.template"),
    ),
    // architecture/ (2)
    (
        "ACS-TEMPLATE.md",
        include_str!("../../../templates/architecture/ACS-TEMPLATE.md"),
    ),
    (
        "AVD-TEMPLATE.md",
        include_str!("../../../templates/architecture/AVD-TEMPLATE.md"),
    ),
    // display/ (4)
    (
        "HANDOFF-COMPLETE-DISPLAY.md.template",
        include_str!("../../../templates/display/HANDOFF-COMPLETE-DISPLAY.md.template"),
    ),
    (
        "PHASE-COMPLETE-DISPLAY.md.template",
        include_str!("../../../templates/display/PHASE-COMPLETE-DISPLAY.md.template"),
    ),
    (
        "PLAN-COMMIT-DISPLAY.md.template",
        include_str!("../../../templates/display/PLAN-COMMIT-DISPLAY.md.template"),
    ),
    (
        "PLAN-DRAFT-DISPLAY.md.template",
        include_str!("../../../templates/display/PLAN-DRAFT-DISPLAY.md.template"),
    ),
    // reference/ (2)
    (
        "HANDOFF-TEMPLATE.md",
        include_str!("../../../templates/reference/HANDOFF-TEMPLATE.md"),
    ),
    (
        "PHASE-AUDIT.md",
        include_str!("../../../templates/reference/PHASE-AUDIT.md"),
    ),
];

// ---------------------------------------------------------------------------
// Canonical-hash cache
//
// One [u8; 32] per template, computed once on first use. Stored inside a
// OnceLock<HashMap<&'static str, [u8; 32]>> to mirror the per-schema
// OnceLock<Result<Validator, _>> pattern from validators/schema.rs.
// ---------------------------------------------------------------------------

type CanonicalHashes = HashMap<&'static str, [u8; 32]>;

fn canonical_hashes() -> &'static CanonicalHashes {
    static CELL: OnceLock<CanonicalHashes> = OnceLock::new();
    CELL.get_or_init(|| {
        let mut map = HashMap::with_capacity(TEMPLATES.len());
        for (name, source) in TEMPLATES {
            map.insert(*name, hash_canonical(source.as_bytes()));
        }
        map
    })
}

// ---------------------------------------------------------------------------
// Canonicalization + hashing
// ---------------------------------------------------------------------------

/// Apply the four canonicalization steps and return the SHA-256 digest.
fn hash_canonical(bytes: &[u8]) -> [u8; 32] {
    let canon = canonicalize(bytes);
    let mut hasher = Sha256::new();
    hasher.update(&canon);
    hasher.finalize().into()
}

/// Strip UTF-8 BOM, normalize CRLF and lone CR to LF, ensure exactly one
/// trailing LF.
fn canonicalize(bytes: &[u8]) -> Vec<u8> {
    // 1. BOM strip.
    let body: &[u8] = if bytes.starts_with(b"\xEF\xBB\xBF") {
        &bytes[3..]
    } else {
        bytes
    };

    // 2 + 3. Line-ending normalization in one pass. Capacity hint: input
    // length is the upper bound (we only ever drop bytes, never add until
    // the trailing-newline step below).
    let mut out = Vec::with_capacity(body.len() + 1);
    let mut i = 0;
    while i < body.len() {
        let b = body[i];
        if b == b'\r' {
            // CRLF → LF, lone CR → LF
            out.push(b'\n');
            if i + 1 < body.len() && body[i + 1] == b'\n' {
                i += 2;
            } else {
                i += 1;
            }
            continue;
        }
        out.push(b);
        i += 1;
    }

    // 4. Ensure exactly one trailing LF. Strip any run of trailing LFs
    // first so an input ending in "\n\n\n" canonicalizes to "...\n".
    while out.last() == Some(&b'\n') {
        out.pop();
    }
    out.push(b'\n');

    out
}

fn hex_lower(digest: &[u8; 32]) -> String {
    let mut s = String::with_capacity(64);
    for byte in digest {
        s.push_str(&format!("{byte:02x}"));
    }
    s
}

// ---------------------------------------------------------------------------
// Document discovery + canonical-name resolution
// ---------------------------------------------------------------------------

/// Walk `target` recursively and yield every regular file whose final path
/// component matches a canonical RWP template name. Symlinks are not
/// followed. Hidden directories (starting with `.`) are skipped.
///
/// Recursion + symlink/hidden-dir handling is delegated to
/// `validators::walk::walk_dir` (factored in P-05 from the duplicated
/// walkers in schema.rs / template.rs / workflow.rs).
fn discover_template_files(
    target: &Path,
    out: &mut Vec<(PathBuf, &'static str)>,
) -> std::io::Result<()> {
    walk_dir(
        target,
        &mut |path| {
            if let Some(canonical) = canonical_name_for(path) {
                out.push((path.to_path_buf(), canonical));
            }
        },
        &mut |_| false,
    )
}

/// Return the matching canonical template key (one of the 17 names) for a
/// candidate path, by full-filename match. Returns `None` for any file
/// whose name is not in the allowlist — these are silently skipped.
fn canonical_name_for(path: &Path) -> Option<&'static str> {
    let file_name = path.file_name().and_then(|n| n.to_str())?;
    for (name, _) in TEMPLATES {
        if file_name == *name {
            return Some(*name);
        }
    }
    None
}

// ---------------------------------------------------------------------------
// Validator entry point
// ---------------------------------------------------------------------------

/// Run template drift-hash validation against every matching file under
/// `target`.
pub fn run(target: &Path, result: &mut CategoryResult) {
    let mut files = Vec::new();
    if let Err(err) = discover_template_files(target, &mut files) {
        result.failures.push(Failure {
            fixture: target.display().to_string(),
            category: Category::Template,
            kind: FailureKind::Io,
            message: format!("failed to enumerate target tree: {err}"),
            details: None,
        });
        result.failed = result.failed.saturating_add(1);
        return;
    }

    files.sort_by(|a, b| a.0.cmp(&b.0));

    let canonicals = canonical_hashes();

    for (file, canonical_name) in files {
        validate_one(&file, canonical_name, canonicals, result);
    }
}

fn validate_one(
    file: &Path,
    canonical_name: &'static str,
    canonicals: &CanonicalHashes,
    result: &mut CategoryResult,
) {
    let fixture = file.display().to_string();

    let bytes = match fs::read(file) {
        Ok(b) => b,
        Err(err) => {
            result.failures.push(Failure {
                fixture,
                category: Category::Template,
                kind: FailureKind::Io,
                message: format!("read failed: {err}"),
                details: None,
            });
            result.failed = result.failed.saturating_add(1);
            return;
        }
    };

    let actual = hash_canonical(&bytes);

    let expected = match canonicals.get(canonical_name) {
        Some(h) => h,
        None => {
            // Unreachable in practice: canonical_name comes from the
            // TEMPLATES allowlist, and canonicals is built from the same
            // list. Guard with FailureKind::Internal rather than panic.
            result.failures.push(Failure {
                fixture,
                category: Category::Template,
                kind: FailureKind::Internal,
                message: format!(
                    "no embedded canonical hash for template '{canonical_name}'"
                ),
                details: None,
            });
            result.failed = result.failed.saturating_add(1);
            return;
        }
    };

    if &actual == expected {
        result.passed = result.passed.saturating_add(1);
        return;
    }

    result.failures.push(Failure {
        fixture,
        category: Category::Template,
        kind: FailureKind::TemplateRenderDiff,
        message: format!(
            "template '{canonical_name}' has drifted from the canonical RWP version"
        ),
        details: Some(format!(
            "expected_sha256={} actual_sha256={}",
            hex_lower(expected),
            hex_lower(&actual)
        )),
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
    fn embedded_template_count_is_seventeen() {
        // ACS-0015 §3 line 114 + §5 corpus layout: 17 canonical templates.
        // Locks the count so anyone adding/removing a template has to
        // update the spec and the validator together.
        assert_eq!(TEMPLATES.len(), 17);
    }

    #[test]
    fn embedded_template_names_are_unique() {
        let mut seen = std::collections::HashSet::new();
        for (name, _) in TEMPLATES {
            assert!(seen.insert(*name), "duplicate template name in TEMPLATES: {name}");
        }
    }

    #[test]
    fn canonical_hashes_populates_for_every_template() {
        let h = canonical_hashes();
        assert_eq!(h.len(), TEMPLATES.len());
        for (name, _) in TEMPLATES {
            assert!(h.contains_key(*name), "missing canonical hash for {name}");
        }
    }

    #[test]
    fn canonicalize_strips_utf8_bom() {
        let with_bom = b"\xEF\xBB\xBFhello\n";
        let without = b"hello\n";
        assert_eq!(hash_canonical(with_bom), hash_canonical(without));
    }

    #[test]
    fn canonicalize_normalizes_crlf_to_lf() {
        let crlf = b"line one\r\nline two\r\n";
        let lf = b"line one\nline two\n";
        assert_eq!(hash_canonical(crlf), hash_canonical(lf));
    }

    #[test]
    fn canonicalize_normalizes_lone_cr_to_lf() {
        let cr = b"line one\rline two\r";
        let lf = b"line one\nline two\n";
        assert_eq!(hash_canonical(cr), hash_canonical(lf));
    }

    #[test]
    fn canonicalize_collapses_trailing_newlines_to_one() {
        let many = b"hello\n\n\n\n";
        let one = b"hello\n";
        assert_eq!(hash_canonical(many), hash_canonical(one));
    }

    #[test]
    fn canonicalize_appends_missing_trailing_newline() {
        let no_nl = b"hello";
        let one_nl = b"hello\n";
        assert_eq!(hash_canonical(no_nl), hash_canonical(one_nl));
    }

    #[test]
    fn canonicalize_preserves_internal_whitespace_and_trailing_spaces() {
        // Trailing spaces on a line are NOT stripped. This matters for
        // some Markdown flavors where 'two trailing spaces' is a hard
        // line break.
        let with = b"line  \nbody\n";
        let without = b"line\nbody\n";
        assert_ne!(hash_canonical(with), hash_canonical(without));
    }

    #[test]
    fn canonical_name_for_matches_full_filename_only() {
        assert_eq!(
            canonical_name_for(Path::new("PLAN.md.template")),
            Some("PLAN.md.template"),
        );
        assert_eq!(
            canonical_name_for(Path::new("/tmp/foo/ACS-TEMPLATE.md")),
            Some("ACS-TEMPLATE.md"),
        );
        // Substring of an allowlisted name must NOT match.
        assert_eq!(canonical_name_for(Path::new("PLAN.md")), None);
        assert_eq!(canonical_name_for(Path::new("MY-PLAN.md.template")), None);
        // Random unrelated files are silently skipped.
        assert_eq!(canonical_name_for(Path::new("README.md")), None);
        assert_eq!(canonical_name_for(Path::new("Cargo.toml")), None);
    }

    #[test]
    fn template_run_passes_valid_fixtures() -> TestResult {
        let valid_dir = fixtures_root().join("valid").join("templates");
        if !valid_dir.exists() {
            return Err(format!(
                "fixture directory missing: {}",
                valid_dir.display()
            )
            .into());
        }
        let mut result = CategoryResult::empty(Category::Template);
        run(&valid_dir, &mut result);
        assert_eq!(
            result.failed, 0,
            "valid fixtures produced failures: {:?}",
            result.failures
        );
        assert!(
            result.passed >= 3,
            "expected at least 3 valid fixtures, got passed={}",
            result.passed
        );
        Ok(())
    }

    #[test]
    fn template_run_rejects_invalid_fixtures() -> TestResult {
        let invalid_dir = fixtures_root().join("invalid").join("templates");
        if !invalid_dir.exists() {
            return Err(format!(
                "fixture directory missing: {}",
                invalid_dir.display()
            )
            .into());
        }
        let mut result = CategoryResult::empty(Category::Template);
        run(&invalid_dir, &mut result);
        assert_eq!(
            result.passed, 0,
            "invalid fixtures unexpectedly passed: passed={}",
            result.passed
        );
        assert!(
            result.failed >= 3,
            "expected at least 3 invalid fixtures to fail, got failed={}",
            result.failed
        );
        for failure in &result.failures {
            assert_eq!(
                failure.kind,
                FailureKind::TemplateRenderDiff,
                "non-render-diff failure: {failure:?}"
            );
            // details should carry both expected and actual hashes.
            let details = failure.details.as_deref().unwrap_or_default();
            assert!(
                details.contains("expected_sha256=") && details.contains("actual_sha256="),
                "missing hash details: {details}"
            );
        }
        Ok(())
    }

    #[test]
    fn template_run_skips_non_template_files_silently() -> TestResult {
        // A README.md or Cargo.toml in the target tree must not increment
        // any counter. Mirrors the schema validator's silent-skip semantic
        // for non-RWP JSON files (P-02 binding).
        let tmp = std::env::temp_dir().join("rhumb-validate-template-skip-test");
        let _ = fs::remove_dir_all(&tmp);
        fs::create_dir_all(&tmp)?;
        fs::write(tmp.join("README.md"), b"# Not a template\n")?;
        fs::write(tmp.join("Cargo.toml"), b"[package]\nname = 'x'\n")?;

        let mut result = CategoryResult::empty(Category::Template);
        run(&tmp, &mut result);
        assert_eq!(result.passed, 0);
        assert_eq!(result.failed, 0);
        assert!(result.failures.is_empty());

        let _ = fs::remove_dir_all(&tmp);
        Ok(())
    }

    #[test]
    fn template_run_handles_missing_target_gracefully() {
        let mut result = CategoryResult::empty(Category::Template);
        let nowhere = PathBuf::from("/does/not/exist/anywhere/rhumb-validate");
        run(&nowhere, &mut result);
        assert_eq!(result.passed, 0);
        assert_eq!(result.failed, 0);
        assert!(result.failures.is_empty());
    }

    #[test]
    fn template_run_passes_canonical_in_place() -> TestResult {
        // The actual canonical templates checked into the repo must hash
        // to themselves under our canonicalization. Walking the canonical
        // directory should produce passed=17 and failed=0.
        let canonical_dir =
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../templates");
        if !canonical_dir.exists() {
            return Err(format!(
                "canonical templates dir missing: {}",
                canonical_dir.display()
            )
            .into());
        }
        let mut result = CategoryResult::empty(Category::Template);
        run(&canonical_dir, &mut result);
        assert_eq!(
            result.failed, 0,
            "canonical templates failed self-validation: {:?}",
            result.failures
        );
        assert_eq!(
            result.passed, 17,
            "expected 17 templates discovered + passed under canonical tree, got {}",
            result.passed
        );
        Ok(())
    }
}
