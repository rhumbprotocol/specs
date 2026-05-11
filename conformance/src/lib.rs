// SPDX-License-Identifier: Apache-2.0
//
// rhumb-validate — public library surface.
//
// Type definitions track ACS-0015 §5 Part B verbatim. Validator
// implementations land in P-02..P-06; this file is the stable contract
// every later phase wires into.

use std::path::Path;
use std::time::Instant;

use time::format_description::well_known::Rfc3339;
use time::OffsetDateTime;

pub mod validators;

/// RWP protocol version this build of `rhumb-validate` was compiled against.
/// Tracked separately from the crate version so the report distinguishes
/// "tool drift" from "spec drift" (AVD-0004 §5 — Versioning & Compatibility).
pub const RWP_VERSION: &str = "0.26.0";

/// Top-level category enumeration. Matches AVD-0004 §5 five-part breakdown
/// and the CLI exit-code map in ACS-0015 §6.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Category {
    Schema,
    Template,
    Workflow,
    Adapter,
    Grammar,
}

impl Category {
    /// Every category, in canonical order. P-02..P-06 implement them in this order.
    pub const ALL: [Category; 5] = [
        Category::Schema,
        Category::Template,
        Category::Workflow,
        Category::Adapter,
        Category::Grammar,
    ];

    pub fn as_str(&self) -> &'static str {
        match self {
            Category::Schema => "schema",
            Category::Template => "template",
            Category::Workflow => "workflow",
            Category::Adapter => "adapter",
            Category::Grammar => "grammar",
        }
    }
}

impl std::fmt::Display for Category {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::str::FromStr for Category {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "schema" => Ok(Category::Schema),
            "template" => Ok(Category::Template),
            "workflow" => Ok(Category::Workflow),
            "adapter" => Ok(Category::Adapter),
            "grammar" => Ok(Category::Grammar),
            other => Err(format!(
                "unknown category '{other}'; expected one of: schema, template, workflow, adapter, grammar"
            )),
        }
    }
}

/// Result of a single validator run.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CategoryResult {
    pub category: Category,
    pub passed: u32,
    pub failed: u32,
    pub skipped: u32,
    pub failures: Vec<Failure>,
    pub duration_ms: u64,
}

impl CategoryResult {
    pub fn empty(category: Category) -> Self {
        Self {
            category,
            passed: 0,
            failed: 0,
            skipped: 0,
            failures: Vec::new(),
            duration_ms: 0,
        }
    }
}

/// One failure, with enough context for a human or CI to act on it.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Failure {
    pub fixture: String,
    pub category: Category,
    pub kind: FailureKind,
    pub message: String,
    pub details: Option<String>,
}

/// Classification of failures. Maps to CLI exit codes (see §6).
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum FailureKind {
    SchemaMismatch,
    TemplateRenderDiff,
    WorkflowBreak,
    AdapterShapeMismatch,
    GrammarViolation,
    Io,
    Internal,
}

/// Aggregate report — this is what `--format json` outputs.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Report {
    pub rhumb_validate_version: String,
    pub rwp_version: String,
    pub started_at: String,
    pub completed_at: String,
    pub target_path: String,
    pub categories: Vec<CategoryResult>,
    pub overall_passed: bool,
    pub total_duration_ms: u64,
}

/// Crate error type. Surfaced to the CLI for exit-code dispatch (§6).
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("I/O error at {path}: {source}")]
    Io {
        path: String,
        #[source]
        source: std::io::Error,
    },

    #[error("fixture parse error in {fixture}: {message}")]
    FixtureParse { fixture: String, message: String },

    #[error("validator internal error: {0}")]
    Internal(String),
}

/// Run the validator. Primary library entry point (ACS-0015 §6).
///
/// # Parameters
/// - `target`: filesystem path to the artifact tree under test
/// - `categories`: which categories to run. Empty slice → run all.
///
/// # Returns
/// - `Ok(Report)` with full results regardless of pass/fail status.
/// - `Err(Error::Io)` only for I/O failures that prevent the run from starting.
///
/// # Determinism
/// For a fixed `target` tree content and a fixed `categories` argument,
/// this function MUST produce the same `Report` (modulo `started_at` /
/// `completed_at` / `total_duration_ms`) across runs.
///
/// # Phase status
/// MP-0276 P-01 ships a placeholder that runs no validators and reports
/// `overall_passed: true` with an empty `categories` vector. P-02..P-06
/// wire each validator into the dispatch loop below.
pub fn validate(target: &Path, categories: &[Category]) -> Result<Report, Error> {
    let selected: &[Category] = if categories.is_empty() {
        &Category::ALL
    } else {
        categories
    };

    let started_wall = OffsetDateTime::now_utc();
    let started_mono = Instant::now();

    let mut category_results = Vec::with_capacity(selected.len());
    for category in selected {
        if let Some(run) = validators::validator(*category) {
            let mut result = CategoryResult::empty(*category);
            let cat_start = Instant::now();
            run(target, &mut result);
            result.duration_ms =
                u64::try_from(cat_start.elapsed().as_millis()).unwrap_or(u64::MAX);
            category_results.push(result);
        }
    }

    let overall_passed = category_results.iter().all(|c| c.failed == 0);
    let completed_wall = OffsetDateTime::now_utc();
    let total_duration_ms = u64::try_from(started_mono.elapsed().as_millis()).unwrap_or(u64::MAX);

    Ok(Report {
        rhumb_validate_version: env!("CARGO_PKG_VERSION").to_string(),
        rwp_version: RWP_VERSION.to_string(),
        started_at: format_iso8601(started_wall),
        completed_at: format_iso8601(completed_wall),
        target_path: target.display().to_string(),
        categories: category_results,
        overall_passed,
        total_duration_ms,
    })
}

/// Format an RFC-3339 / ISO-8601 UTC timestamp. Falls back to the sentinel
/// `0001-01-01T00:00:00Z` if the formatter rejects the value (in practice
/// unreachable for `OffsetDateTime::now_utc()`, but the fallback keeps
/// `validate()` infallible-on-time per the public contract).
fn format_iso8601(t: OffsetDateTime) -> String {
    t.format(&Rfc3339)
        .unwrap_or_else(|_| "0001-01-01T00:00:00Z".to_string())
}

/// Load the built-in test corpus embedded at build time.
///
/// Returns the canonical `fixtures/` tree (every file under
/// `packages/rhumb-protocol/conformance/fixtures/`, both `valid/` and
/// `invalid/`) serialized in a small self-describing record format.
/// Library callers parse the blob to recover individual fixtures without
/// depending on the on-disk corpus.
///
/// # Format
///
/// All integers little-endian, unsigned:
///
/// ```text
/// 4 bytes  magic        = b"RVF\0"
/// 4 bytes  version      = 1            (u32)
/// 4 bytes  entry_count                (u32)
///
/// per entry (entry_count times):
///   4 bytes  path_len                 (u32)
///   N bytes  path  (UTF-8, '/'-separated, relative to fixtures/)
///   8 bytes  content_len              (u64)
///   M bytes  content
/// ```
///
/// Entries are sorted lexicographically by path; the blob is reproducible
/// byte-for-byte from a fixed `fixtures/` tree.
///
/// # Phase status
///
/// MP-0276 P-08 wired this against the canonical fixture corpus at build
/// time (see `build.rs`). Earlier phases (P-01..P-07) shipped an empty
/// stub.
pub fn embedded_fixtures() -> &'static [u8] {
    include_bytes!(concat!(env!("OUT_DIR"), "/embedded_fixtures.bin"))
}

/// Magic bytes at offset 0 of [`embedded_fixtures()`]. Exposed so library
/// callers can sanity-check the blob before parsing.
pub const EMBEDDED_FIXTURES_MAGIC: [u8; 4] = *b"RVF\0";

/// Format version stamped into [`embedded_fixtures()`] at bytes 4..8 (LE u32).
pub const EMBEDDED_FIXTURES_FORMAT_VERSION: u32 = 1;

#[cfg(test)]
mod tests {
    use super::*;

    type TestResult = Result<(), Box<dyn std::error::Error>>;

    // Use a known-nonexistent path so wired validators (Schema in P-02,
    // others in P-03..P-06) walk an empty tree and emit deterministic
    // zero-counts. cargo test runs with cwd = package directory which
    // contains target/ build artifacts and other JSON files we don't
    // want to validate as a side effect of testing the lib surface.
    fn empty_target() -> &'static Path {
        Path::new("/does/not/exist/rhumb-validate-empty-test-tree")
    }

    #[test]
    fn validate_placeholder_returns_well_formed_report() -> TestResult {
        let target = empty_target();
        let report = validate(target, &[])?;

        assert_eq!(report.rhumb_validate_version, env!("CARGO_PKG_VERSION"));
        assert_eq!(report.rwp_version, RWP_VERSION);
        assert!(report.overall_passed);
        // All five categories are wired (P-02 Schema, P-03 Template,
        // P-04 Workflow, P-05 Adapter, P-06 Grammar).
        assert_eq!(report.categories.len(), 5);
        assert_eq!(report.categories[0].category, Category::Schema);
        assert_eq!(report.categories[1].category, Category::Template);
        assert_eq!(report.categories[2].category, Category::Workflow);
        assert_eq!(report.categories[3].category, Category::Adapter);
        assert_eq!(report.categories[4].category, Category::Grammar);
        for c in &report.categories {
            assert_eq!(c.passed, 0);
            assert_eq!(c.failed, 0);
        }
        assert_eq!(report.target_path, target.display().to_string());
        // total_duration_ms is real wall-clock since P-08; an empty-target
        // run is sub-millisecond in practice but not strictly zero. The
        // contract is "monotonic, non-negative, fits in u64" — that's
        // already the type guarantee.
        let _ = report.total_duration_ms;
        // Real ISO-8601 timestamps since P-08 (RFC-3339 UTC, e.g.
        // "2026-05-01T12:00:00.123456789Z"). The placeholder sentinel
        // "1970-01-01T00:00:00Z" is gone; assert the new shape.
        assert!(report.started_at.ends_with('Z'), "started_at must be UTC");
        assert!(report.completed_at.ends_with('Z'), "completed_at must be UTC");
        assert!(
            !report.started_at.starts_with("1970-01-01"),
            "placeholder timestamp leaked into started_at"
        );
        assert!(
            !report.completed_at.starts_with("1970-01-01"),
            "placeholder timestamp leaked into completed_at"
        );
        Ok(())
    }

    #[test]
    fn validate_with_subset_runs_only_requested_categories() -> TestResult {
        // With all 5 categories wired in P-06, a subset returns exactly
        // the categories asked for, in the order requested. Replaces
        // the P-05 "subset hits unwired Grammar → silently skipped"
        // test — that semantic no longer applies.
        let report = validate(
            empty_target(),
            &[Category::Adapter, Category::Grammar],
        )?;

        assert!(report.overall_passed);
        assert_eq!(report.categories.len(), 2);
        assert_eq!(report.categories[0].category, Category::Adapter);
        assert_eq!(report.categories[1].category, Category::Grammar);
        Ok(())
    }

    #[test]
    fn validate_with_single_category_returns_one_row() -> TestResult {
        let report = validate(empty_target(), &[Category::Grammar])?;
        assert!(report.overall_passed);
        assert_eq!(report.categories.len(), 1);
        assert_eq!(report.categories[0].category, Category::Grammar);
        Ok(())
    }

    #[test]
    fn report_round_trips_through_serde() -> TestResult {
        let report = validate(empty_target(), &[])?;
        let json = serde_json::to_string(&report)?;
        let restored: Report = serde_json::from_str(&json)?;
        assert_eq!(restored.rwp_version, RWP_VERSION);
        assert!(restored.overall_passed);
        assert_eq!(restored.categories.len(), 5);
        Ok(())
    }

    #[test]
    fn category_round_trips_through_str() -> TestResult {
        for cat in Category::ALL {
            let s = cat.as_str();
            let parsed: Category = s.parse()?;
            assert_eq!(parsed, cat);
        }
        Ok(())
    }

    #[test]
    fn category_from_str_rejects_unknown() {
        let Err(message) = "nonsense".parse::<Category>() else {
            panic!("expected error for unknown category");
        };
        assert!(message.contains("nonsense"));
    }

    #[test]
    fn embedded_fixtures_blob_starts_with_magic_and_version() {
        let blob = embedded_fixtures();
        assert!(
            blob.len() >= 12,
            "blob too short: {} bytes (need at least 12 for header)",
            blob.len()
        );
        assert_eq!(
            blob[0..4],
            EMBEDDED_FIXTURES_MAGIC,
            "magic mismatch — expected RVF\\0"
        );
        let version = u32::from_le_bytes(blob[4..8].try_into().unwrap());
        assert_eq!(version, EMBEDDED_FIXTURES_FORMAT_VERSION);
    }

    #[test]
    fn embedded_fixtures_blob_round_trips_canonical_corpus() {
        let blob = embedded_fixtures();
        let entries = parse_embedded_fixtures(blob).expect("parse embedded_fixtures blob");
        // The canonical fixture corpus ships >= 21 valid + several invalid
        // fixture files. Lower bound is a smoke check; the exact count
        // varies as new fixtures land in future phases.
        assert!(
            entries.len() >= 25,
            "fewer than 25 fixtures embedded ({}) — corpus regression?",
            entries.len()
        );
        // Sorted lexicographically per build.rs invariant.
        for pair in entries.windows(2) {
            assert!(pair[0].0 < pair[1].0, "entries not sorted");
        }
        // Spot-check: a known valid schema fixture is present and non-empty.
        let some_schema = entries
            .iter()
            .find(|(p, _)| p.starts_with("valid/schemas/") && p.ends_with(".json"));
        assert!(
            some_schema.is_some(),
            "no valid/schemas/*.json fixture in embedded corpus"
        );
        assert!(!some_schema.unwrap().1.is_empty());
    }

    /// Test-only parser for the embedded_fixtures blob format. Asserts on
    /// truncation rather than returning Result-style errors — appropriate
    /// for `#[cfg(test)]` only; the lib never parses the blob at runtime
    /// since callers own that contract per the docstring format spec.
    fn parse_embedded_fixtures(blob: &[u8]) -> Result<Vec<(String, &[u8])>, String> {
        if blob.len() < 12 {
            return Err(format!("blob too short: {}", blob.len()));
        }
        if blob[0..4] != EMBEDDED_FIXTURES_MAGIC {
            return Err("magic mismatch".to_string());
        }
        let version = u32::from_le_bytes(blob[4..8].try_into().unwrap());
        if version != EMBEDDED_FIXTURES_FORMAT_VERSION {
            return Err(format!("unknown version {version}"));
        }
        let entry_count = u32::from_le_bytes(blob[8..12].try_into().unwrap()) as usize;
        let mut out = Vec::with_capacity(entry_count);
        let mut offset = 12usize;
        for _ in 0..entry_count {
            if blob.len() < offset + 4 {
                return Err("truncated path_len".to_string());
            }
            let path_len =
                u32::from_le_bytes(blob[offset..offset + 4].try_into().unwrap()) as usize;
            offset += 4;
            if blob.len() < offset + path_len {
                return Err("truncated path bytes".to_string());
            }
            let path = std::str::from_utf8(&blob[offset..offset + path_len])
                .map_err(|e| format!("path utf-8: {e}"))?
                .to_string();
            offset += path_len;
            if blob.len() < offset + 8 {
                return Err("truncated content_len".to_string());
            }
            let content_len =
                u64::from_le_bytes(blob[offset..offset + 8].try_into().unwrap()) as usize;
            offset += 8;
            if blob.len() < offset + content_len {
                return Err("truncated content bytes".to_string());
            }
            let content = &blob[offset..offset + content_len];
            offset += content_len;
            out.push((path, content));
        }
        if offset != blob.len() {
            return Err(format!(
                "blob has {} trailing bytes after entries",
                blob.len() - offset
            ));
        }
        Ok(out)
    }
}
