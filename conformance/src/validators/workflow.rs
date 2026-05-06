// SPDX-License-Identifier: Apache-2.0
//
// Workflow validator (Category 3, MP-0276 P-04).
//
// Runs static cross-file reference validation over a workflow tree (the
// artifact set produced by an RWP-based plan: PLAN.md + state.yaml +
// handoffs/). Per OQ-15.5 (resolved in this phase) the validator does
// NOT execute any implementation against the workflow — it inspects the
// static files only. A `--dynamic` mode for actual execution is deferred
// to a post-1.0 release if demand materializes.
//
// What counts as a "workflow root":
//   A directory containing BOTH `PLAN.md` AND `state.yaml`. The walker
//   recurses into the target tree until it finds such a directory, then
//   stops descending — handoffs/ and any other subdirectories of a
//   workflow root are not re-scanned for further roots. This avoids
//   accidentally treating workflow-internal folders as standalone
//   workflows.
//
// Cross-file invariants (each violation emits one
// `FailureKind::WorkflowBreak` with a distinct details payload):
//   INV-1  PLAN.md frontmatter `plan_id` matches state.yaml top-level
//          `plan_id`. Either-side-missing is also a violation.
//   INV-2  PLAN.md frontmatter `current_phase` matches state.yaml
//          `execution.current_phase`. Either-side-missing while the other
//          is present is a violation; both-absent is silent (INV-1 will
//          have already caught a wholly-blank workflow).
//   INV-3  state.yaml `handoffs.last_handoff` (when present and non-null)
//          refers to a file that exists relative to the workflow root.
//   INV-4  Every entry in state.yaml `handoffs.handoff_files[].file`
//          refers to a file that exists relative to the workflow root.
//   INV-5  Every entry in state.yaml `handoffs.handoff_files[].phase`
//          appears as a defined phase ID under state.yaml `phases:` (or
//          any nested `sub_phases:` map — the namespace flattens).
//
// I/O failures (PLAN.md / state.yaml unreadable) push a
// `FailureKind::Io` for the workflow and short-circuit further checks on
// it. YAML parse failures (PLAN.md frontmatter or state.yaml not valid
// YAML) push a `FailureKind::WorkflowBreak` with the parser's error in
// details and continue with whatever defaults are available — that lets
// a single fixture surface both "bad YAML" and any structural
// invariants the fallback can still evaluate.
//
// Counter discipline (binding for re-implementers):
//   Each workflow root counts as exactly ONE fixture. Multiple invariant
//   violations within a single workflow contribute multiple `Failure`
//   entries to `result.failures` but increment `result.failed` by 1.
//   This matches the user-meaningful unit ("did this workflow validate?")
//   rather than counting violations.
//
// Files outside the discovered workflow set (random JSON, README.md,
// stray Cargo.toml in the target tree) are silently skipped — same
// discipline as the schema and template validators (P-02, P-03 binding).

use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};

use serde::Deserialize;

use crate::validators::walk::walk_dir;
use crate::{Category, CategoryResult, Failure, FailureKind};

// ---------------------------------------------------------------------------
// PLAN.md frontmatter (YAML between the first two `---\n` markers)
// ---------------------------------------------------------------------------

#[derive(Debug, Default, Deserialize)]
struct PlanFrontmatter {
    #[serde(default)]
    plan_id: Option<String>,
    #[serde(default)]
    current_phase: Option<String>,
}

// ---------------------------------------------------------------------------
// state.yaml (top-level)
//
// Only the fields the validator actually reads are typed. Unknown fields
// are silently ignored by serde (no `#[serde(deny_unknown_fields)]`),
// which is intentional — RWP state.yaml will accumulate optional sections
// over time and a strict allowlist would force lock-step versioning.
// ---------------------------------------------------------------------------

#[derive(Debug, Default, Deserialize)]
struct StateYaml {
    #[serde(default)]
    plan_id: Option<String>,
    #[serde(default)]
    execution: Option<Execution>,
    #[serde(default)]
    phases: Option<HashMap<String, PhaseEntry>>,
    #[serde(default)]
    handoffs: Option<Handoffs>,
}

#[derive(Debug, Default, Deserialize)]
struct Execution {
    #[serde(default)]
    current_phase: Option<String>,
}

#[derive(Debug, Default, Deserialize)]
struct PhaseEntry {
    #[serde(default)]
    sub_phases: Option<HashMap<String, PhaseEntry>>,
}

#[derive(Debug, Default, Deserialize)]
struct Handoffs {
    #[serde(default)]
    last_handoff: Option<String>,
    #[serde(default)]
    handoff_files: Option<Vec<HandoffFile>>,
}

#[derive(Debug, Default, Deserialize)]
struct HandoffFile {
    #[serde(default)]
    file: Option<String>,
    #[serde(default)]
    phase: Option<String>,
}

// ---------------------------------------------------------------------------
// Workflow-root discovery
// ---------------------------------------------------------------------------

/// Walk `target` recursively, yielding every directory that contains
/// BOTH `PLAN.md` and `state.yaml`. Once a workflow root is found,
/// descent into its subdirectories stops.
///
/// Symlinks are not followed. Hidden directories (starting with `.`) are
/// skipped — they're typically build artifacts (target/, .git/, etc.) or
/// hidden runtime state that should not be misclassified as workflows.
///
/// Recursion + symlink/hidden-dir handling + stop-on-find is delegated
/// to `validators::walk::walk_dir` (factored in P-05).
fn discover_workflow_roots(target: &Path, out: &mut Vec<PathBuf>) -> std::io::Result<()> {
    walk_dir(
        target,
        &mut |_| {},
        &mut |dir| {
            if has_workflow_files(dir) {
                out.push(dir.to_path_buf());
                true
            } else {
                false
            }
        },
    )
}

fn has_workflow_files(dir: &Path) -> bool {
    dir.join("PLAN.md").is_file() && dir.join("state.yaml").is_file()
}

// ---------------------------------------------------------------------------
// Frontmatter extraction + first-document YAML slicing
//
// A markdown frontmatter block is the YAML content between the first
// line that consists of exactly `---` and the next such line. The opening
// `---` does NOT have to be at byte 0 — the canonical RWP PLAN.md format
// places a level-1 heading and a blank line before the frontmatter (see
// examples/{simple-feature,multi-phase,bug-fix}/PLAN.md).
//
// Both LF and CRLF line terminators are accepted. A leading UTF-8 BOM is
// tolerated.
//
// state.yaml may be a multi-document YAML stream (RWP convention places a
// `Produced: ...` trailer block as a second document after a `---`
// separator). serde_yml's `from_str` rejects multi-document input, so the
// validator slices off the first document before parsing. Documents are
// separated by a line consisting of exactly `---`. Embedded `---` inside
// scalar values is not handled — the RWP convention is to avoid it.
// ---------------------------------------------------------------------------

/// Locate the next line in `text[start..]` whose content (after stripping
/// any trailing `\r`) equals exactly `---`. Returns `(line_start_byte,
/// byte_after_terminating_newline)`. `byte_after_terminating_newline` ==
/// `text.len()` if the line ends the file with no trailing newline.
fn next_dashes_line(text: &str, start: usize) -> Option<(usize, usize)> {
    if start > text.len() {
        return None;
    }
    let mut pos = start;
    loop {
        let line_end = text[pos..]
            .find('\n')
            .map(|i| pos + i)
            .unwrap_or(text.len());
        let raw = &text[pos..line_end];
        if raw.trim_end_matches('\r') == "---" {
            let after = if line_end < text.len() {
                line_end + 1
            } else {
                line_end
            };
            return Some((pos, after));
        }
        if line_end >= text.len() {
            return None;
        }
        pos = line_end + 1;
    }
}

fn extract_frontmatter(text: &str) -> Option<&str> {
    let body = text.strip_prefix('\u{feff}').unwrap_or(text);
    let (_, yaml_start) = next_dashes_line(body, 0)?;
    let (yaml_end_line_start, _) = next_dashes_line(body, yaml_start)?;
    let mut end = yaml_end_line_start;
    if end > yaml_start && body.as_bytes()[end - 1] == b'\n' {
        end -= 1;
        if end > yaml_start && body.as_bytes()[end - 1] == b'\r' {
            end -= 1;
        }
    }
    Some(&body[yaml_start..end])
}

/// Return the first YAML document of a possibly multi-document stream.
/// A leading `---\n` start-of-stream marker is preserved (serde_yml
/// accepts it). The function searches for the NEXT `---` line — that's a
/// document separator — and returns everything before it.
fn first_yaml_document(text: &str) -> &str {
    let body = text.strip_prefix('\u{feff}').unwrap_or(text);
    // Skip past a leading start-of-stream `---` so it isn't mistaken for a
    // document separator.
    let scan_start = if body.starts_with("---\n") {
        4
    } else if body.starts_with("---\r\n") {
        5
    } else {
        0
    };
    let Some((sep_line_start, _)) = next_dashes_line(body, scan_start) else {
        return body;
    };
    let mut end = sep_line_start;
    if end > 0 && body.as_bytes()[end - 1] == b'\n' {
        end -= 1;
        if end > 0 && body.as_bytes()[end - 1] == b'\r' {
            end -= 1;
        }
    }
    &body[..end]
}

// ---------------------------------------------------------------------------
// Phase-namespace flattening
// ---------------------------------------------------------------------------

fn collect_phase_ids(phases: &HashMap<String, PhaseEntry>, out: &mut HashSet<String>) {
    for (id, entry) in phases {
        out.insert(id.clone());
        if let Some(sub) = &entry.sub_phases {
            collect_phase_ids(sub, out);
        }
    }
}

// ---------------------------------------------------------------------------
// Per-workflow validation
// ---------------------------------------------------------------------------

fn validate_workflow(root: &Path, result: &mut CategoryResult) {
    let workflow_id = root.display().to_string();
    let plan_md_path = root.join("PLAN.md");
    let state_yaml_path = root.join("state.yaml");

    let plan_md_text = match fs::read_to_string(&plan_md_path) {
        Ok(t) => t,
        Err(err) => {
            push_io(result, &workflow_id, &plan_md_path, err);
            result.failed = result.failed.saturating_add(1);
            return;
        }
    };
    let state_yaml_text = match fs::read_to_string(&state_yaml_path) {
        Ok(t) => t,
        Err(err) => {
            push_io(result, &workflow_id, &state_yaml_path, err);
            result.failed = result.failed.saturating_add(1);
            return;
        }
    };

    let mut violations: Vec<Failure> = Vec::new();

    let plan: PlanFrontmatter = match extract_frontmatter(&plan_md_text) {
        Some(fm) => match serde_yml::from_str::<PlanFrontmatter>(fm) {
            Ok(p) => p,
            Err(err) => {
                violations.push(workflow_break(
                    &workflow_id,
                    "PLAN.md frontmatter is not valid YAML",
                    Some(err.to_string()),
                ));
                PlanFrontmatter::default()
            }
        },
        None => {
            violations.push(workflow_break(
                &workflow_id,
                "PLAN.md is missing the YAML frontmatter block (expected '---' delimited block at top of file)",
                None,
            ));
            PlanFrontmatter::default()
        }
    };

    let state_yaml_first_doc = first_yaml_document(&state_yaml_text);
    let state: StateYaml = match serde_yml::from_str::<StateYaml>(state_yaml_first_doc) {
        Ok(s) => s,
        Err(err) => {
            violations.push(workflow_break(
                &workflow_id,
                "state.yaml is not valid YAML",
                Some(err.to_string()),
            ));
            StateYaml::default()
        }
    };

    // INV-1: plan_id agreement.
    match (plan.plan_id.as_deref(), state.plan_id.as_deref()) {
        (Some(a), Some(b)) if a == b => {}
        (Some(a), Some(b)) => {
            violations.push(workflow_break(
                &workflow_id,
                "INV-1 plan_id mismatch between PLAN.md and state.yaml",
                Some(format!("plan_md_plan_id={a} state_yaml_plan_id={b}")),
            ));
        }
        (Some(_), None) => {
            violations.push(workflow_break(
                &workflow_id,
                "INV-1 state.yaml is missing top-level plan_id",
                None,
            ));
        }
        (None, Some(_)) => {
            violations.push(workflow_break(
                &workflow_id,
                "INV-1 PLAN.md frontmatter is missing plan_id",
                None,
            ));
        }
        (None, None) => {
            violations.push(workflow_break(
                &workflow_id,
                "INV-1 plan_id is missing from BOTH PLAN.md frontmatter and state.yaml",
                None,
            ));
        }
    }

    // INV-2: current_phase agreement.
    let plan_cur = plan.current_phase.as_deref();
    let state_cur = state
        .execution
        .as_ref()
        .and_then(|e| e.current_phase.as_deref());
    match (plan_cur, state_cur) {
        (Some(a), Some(b)) if a == b => {}
        (Some(a), Some(b)) => {
            violations.push(workflow_break(
                &workflow_id,
                "INV-2 current_phase mismatch between PLAN.md and state.yaml",
                Some(format!(
                    "plan_md_current_phase={a} state_yaml_current_phase={b}"
                )),
            ));
        }
        (Some(_), None) => {
            violations.push(workflow_break(
                &workflow_id,
                "INV-2 state.yaml execution.current_phase is missing while PLAN.md frontmatter has current_phase",
                None,
            ));
        }
        (None, Some(_)) => {
            violations.push(workflow_break(
                &workflow_id,
                "INV-2 PLAN.md frontmatter current_phase is missing while state.yaml has execution.current_phase",
                None,
            ));
        }
        (None, None) => {
            // Both absent — INV-1 already covers wholly-blank workflows;
            // partially-built workflows in the no-current_phase state are
            // legitimate.
        }
    }

    let handoffs = state.handoffs.unwrap_or_default();
    let phases_set = {
        let mut set: HashSet<String> = HashSet::new();
        if let Some(phases) = &state.phases {
            collect_phase_ids(phases, &mut set);
        }
        set
    };

    // INV-3: last_handoff exists on disk.
    if let Some(last) = handoffs.last_handoff.as_deref() {
        let resolved = root.join(last);
        if !resolved.is_file() {
            violations.push(workflow_break(
                &workflow_id,
                "INV-3 state.yaml handoffs.last_handoff references a file that does not exist",
                Some(format!(
                    "last_handoff={last} resolved_path={}",
                    resolved.display()
                )),
            ));
        }
    }

    // INV-4 + INV-5: handoff_files entries — file exists + phase defined.
    if let Some(files) = &handoffs.handoff_files {
        for entry in files {
            if let Some(file) = entry.file.as_deref() {
                let resolved = root.join(file);
                if !resolved.is_file() {
                    violations.push(workflow_break(
                        &workflow_id,
                        "INV-4 state.yaml handoffs.handoff_files entry references a file that does not exist",
                        Some(format!(
                            "file={file} resolved_path={}",
                            resolved.display()
                        )),
                    ));
                }
            }
            if let Some(phase) = entry.phase.as_deref() {
                if !phases_set.contains(phase) {
                    violations.push(workflow_break(
                        &workflow_id,
                        "INV-5 state.yaml handoffs.handoff_files entry references a phase not defined under phases:",
                        Some(format!("phase={phase}")),
                    ));
                }
            }
        }
    }

    if violations.is_empty() {
        result.passed = result.passed.saturating_add(1);
    } else {
        result.failures.extend(violations);
        result.failed = result.failed.saturating_add(1);
    }
}

fn workflow_break(workflow_id: &str, message: &str, details: Option<String>) -> Failure {
    Failure {
        fixture: workflow_id.to_string(),
        category: Category::Workflow,
        kind: FailureKind::WorkflowBreak,
        message: message.to_string(),
        details,
    }
}

fn push_io(result: &mut CategoryResult, workflow_id: &str, path: &Path, err: std::io::Error) {
    result.failures.push(Failure {
        fixture: workflow_id.to_string(),
        category: Category::Workflow,
        kind: FailureKind::Io,
        message: format!("read failed: {}", path.display()),
        details: Some(err.to_string()),
    });
}

// ---------------------------------------------------------------------------
// Validator entry point
// ---------------------------------------------------------------------------

/// Run static workflow validation against every workflow root discovered
/// under `target`.
pub fn run(target: &Path, result: &mut CategoryResult) {
    let mut roots = Vec::new();
    if let Err(err) = discover_workflow_roots(target, &mut roots) {
        result.failures.push(Failure {
            fixture: target.display().to_string(),
            category: Category::Workflow,
            kind: FailureKind::Io,
            message: format!("failed to enumerate target tree: {err}"),
            details: None,
        });
        result.failed = result.failed.saturating_add(1);
        return;
    }
    roots.sort();
    for root in &roots {
        validate_workflow(root, result);
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

    fn examples_root() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../examples")
    }

    #[test]
    fn extract_frontmatter_strips_bom() {
        let with_bom = "\u{feff}---\nplan_id: X\n---\nbody\n";
        assert_eq!(extract_frontmatter(with_bom), Some("plan_id: X"));
    }

    #[test]
    fn extract_frontmatter_handles_lf() {
        let lf = "---\nplan_id: X\ncurrent_phase: P-01\n---\nbody\n";
        assert_eq!(
            extract_frontmatter(lf),
            Some("plan_id: X\ncurrent_phase: P-01")
        );
    }

    #[test]
    fn extract_frontmatter_handles_crlf() {
        let crlf = "---\r\nplan_id: X\r\ncurrent_phase: P-01\r\n---\r\nbody\r\n";
        assert_eq!(
            extract_frontmatter(crlf),
            Some("plan_id: X\r\ncurrent_phase: P-01")
        );
    }

    #[test]
    fn extract_frontmatter_returns_none_when_missing() {
        assert_eq!(extract_frontmatter("no frontmatter here\n"), None);
        assert_eq!(extract_frontmatter("---\nopen but never closed\n"), None);
    }

    #[test]
    fn extract_frontmatter_handles_leading_heading() -> TestResult {
        // RWP canonical PLAN.md format: a level-1 heading and blank line
        // precede the frontmatter delimiter.
        let canonical = "# Rhumb Workflow Protocol: Plan Document\n\n---\n\nplan_id: RWP-0042\ncurrent_phase: P-02\n\n---\n\n# RWP-0042: Body\n";
        let fm = extract_frontmatter(canonical).ok_or("expected frontmatter")?;
        let parsed: PlanFrontmatter = serde_yml::from_str(fm)?;
        assert_eq!(parsed.plan_id.as_deref(), Some("RWP-0042"));
        assert_eq!(parsed.current_phase.as_deref(), Some("P-02"));
        Ok(())
    }

    #[test]
    fn first_yaml_document_returns_full_text_when_single_doc() {
        let single = "plan_id: A\nexecution:\n  current_phase: P-01\n";
        assert_eq!(first_yaml_document(single), single);
    }

    #[test]
    fn first_yaml_document_strips_trailer_after_dashes() {
        // RWP canonical state.yaml format: a `Produced:` trailer follows
        // a `---` document separator.
        let multi = "plan_id: A\npercentage_complete: 100\n---\nProduced: 2026-03-01\nBy: contrib\n";
        let first = first_yaml_document(multi);
        assert_eq!(first, "plan_id: A\npercentage_complete: 100");
    }

    #[test]
    fn first_yaml_document_preserves_leading_start_of_stream() {
        // A leading `---` start-of-stream marker is preserved so a
        // following `---` is correctly identified as a separator (not
        // mistaken for the file-opening marker).
        let with_open = "---\nplan_id: A\n---\nProduced: foo\n";
        let first = first_yaml_document(with_open);
        assert_eq!(first, "---\nplan_id: A");
    }

    #[test]
    fn collect_phase_ids_flattens_sub_phases() {
        let mut top = HashMap::new();
        top.insert(
            "P-01".to_string(),
            PhaseEntry {
                sub_phases: None,
            },
        );
        let mut sub = HashMap::new();
        sub.insert(
            "P-02-A".to_string(),
            PhaseEntry { sub_phases: None },
        );
        sub.insert(
            "P-02-B".to_string(),
            PhaseEntry { sub_phases: None },
        );
        top.insert(
            "P-02".to_string(),
            PhaseEntry {
                sub_phases: Some(sub),
            },
        );
        let mut out = HashSet::new();
        collect_phase_ids(&top, &mut out);
        assert!(out.contains("P-01"));
        assert!(out.contains("P-02"));
        assert!(out.contains("P-02-A"));
        assert!(out.contains("P-02-B"));
        assert_eq!(out.len(), 4);
    }

    #[test]
    fn discover_workflow_roots_stops_at_root() -> TestResult {
        let tmp = std::env::temp_dir().join("rhumb-validate-workflow-discovery-test");
        let _ = fs::remove_dir_all(&tmp);
        fs::create_dir_all(tmp.join("workflow-a/handoffs"))?;
        fs::write(tmp.join("workflow-a/PLAN.md"), b"---\nplan_id: A\n---\nx\n")?;
        fs::write(tmp.join("workflow-a/state.yaml"), b"plan_id: A\n")?;
        // A nested PLAN.md/state.yaml inside handoffs/ must NOT be picked up
        // as a separate workflow once workflow-a has been identified.
        fs::write(
            tmp.join("workflow-a/handoffs/PLAN.md"),
            b"---\nplan_id: A2\n---\n",
        )?;
        fs::write(tmp.join("workflow-a/handoffs/state.yaml"), b"plan_id: A2\n")?;
        // A sibling workflow under a different child dir should still be
        // discovered via the parent's recursion.
        fs::create_dir_all(tmp.join("nested/workflow-b"))?;
        fs::write(
            tmp.join("nested/workflow-b/PLAN.md"),
            b"---\nplan_id: B\n---\n",
        )?;
        fs::write(tmp.join("nested/workflow-b/state.yaml"), b"plan_id: B\n")?;

        let mut roots = Vec::new();
        discover_workflow_roots(&tmp, &mut roots)?;
        roots.sort();
        assert_eq!(roots.len(), 2);
        assert!(roots.iter().any(|p| p.ends_with("workflow-a")));
        assert!(roots.iter().any(|p| p.ends_with("workflow-b")));
        // Confirm we did NOT descend into workflow-a/handoffs.
        assert!(!roots.iter().any(|p| p.ends_with("handoffs")));

        let _ = fs::remove_dir_all(&tmp);
        Ok(())
    }

    #[test]
    fn discover_workflow_roots_skips_hidden_dirs() -> TestResult {
        let tmp = std::env::temp_dir().join("rhumb-validate-workflow-hidden-test");
        let _ = fs::remove_dir_all(&tmp);
        fs::create_dir_all(tmp.join(".git/workflow-x"))?;
        fs::write(
            tmp.join(".git/workflow-x/PLAN.md"),
            b"---\nplan_id: X\n---\n",
        )?;
        fs::write(tmp.join(".git/workflow-x/state.yaml"), b"plan_id: X\n")?;

        let mut roots = Vec::new();
        discover_workflow_roots(&tmp, &mut roots)?;
        assert!(roots.is_empty(), "hidden dir should not be scanned");

        let _ = fs::remove_dir_all(&tmp);
        Ok(())
    }

    #[test]
    fn run_passes_canonical_examples_in_place() -> TestResult {
        let dir = examples_root();
        if !dir.exists() {
            return Err(format!("canonical examples missing: {}", dir.display()).into());
        }
        let mut result = CategoryResult::empty(Category::Workflow);
        run(&dir, &mut result);
        assert_eq!(
            result.failed, 0,
            "canonical examples failed self-validation: {:?}",
            result.failures
        );
        assert_eq!(
            result.passed, 3,
            "expected 3 canonical workflows, got passed={}",
            result.passed
        );
        Ok(())
    }

    #[test]
    fn run_passes_valid_fixtures() -> TestResult {
        let dir = fixtures_root().join("valid").join("workflows");
        if !dir.exists() {
            return Err(format!("fixture dir missing: {}", dir.display()).into());
        }
        let mut result = CategoryResult::empty(Category::Workflow);
        run(&dir, &mut result);
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
    fn run_rejects_invalid_fixtures() -> TestResult {
        let dir = fixtures_root().join("invalid").join("workflows");
        if !dir.exists() {
            return Err(format!("fixture dir missing: {}", dir.display()).into());
        }
        let mut result = CategoryResult::empty(Category::Workflow);
        run(&dir, &mut result);
        assert_eq!(
            result.passed, 0,
            "invalid fixtures unexpectedly passed: passed={}",
            result.passed
        );
        assert!(
            result.failed >= 3,
            "expected at least 3 invalid fixtures, got failed={}",
            result.failed
        );
        // Every emitted Failure must be a WorkflowBreak (not Io / not Internal).
        for failure in &result.failures {
            assert_eq!(
                failure.kind,
                FailureKind::WorkflowBreak,
                "non-workflow-break failure surfaced from fixtures: {failure:?}"
            );
        }
        Ok(())
    }

    #[test]
    fn run_skips_non_workflow_files_silently() -> TestResult {
        let tmp = std::env::temp_dir().join("rhumb-validate-workflow-skip-test");
        let _ = fs::remove_dir_all(&tmp);
        fs::create_dir_all(&tmp)?;
        fs::write(tmp.join("README.md"), b"# Not a workflow\n")?;
        fs::write(tmp.join("Cargo.toml"), b"[package]\nname = 'x'\n")?;
        // A directory with only PLAN.md (no state.yaml) is NOT a workflow.
        fs::create_dir_all(tmp.join("not-a-workflow"))?;
        fs::write(
            tmp.join("not-a-workflow/PLAN.md"),
            b"---\nplan_id: Y\n---\n",
        )?;

        let mut result = CategoryResult::empty(Category::Workflow);
        run(&tmp, &mut result);
        assert_eq!(result.passed, 0);
        assert_eq!(result.failed, 0);
        assert!(result.failures.is_empty());

        let _ = fs::remove_dir_all(&tmp);
        Ok(())
    }

    #[test]
    fn run_handles_missing_target_gracefully() {
        let mut result = CategoryResult::empty(Category::Workflow);
        let nowhere = PathBuf::from("/does/not/exist/anywhere/rhumb-validate-workflow");
        run(&nowhere, &mut result);
        assert_eq!(result.passed, 0);
        assert_eq!(result.failed, 0);
        assert!(result.failures.is_empty());
    }

    #[test]
    fn invariant_1_plan_id_mismatch_emits_break() -> TestResult {
        let tmp = std::env::temp_dir().join("rhumb-validate-workflow-inv1-test");
        let _ = fs::remove_dir_all(&tmp);
        fs::create_dir_all(&tmp)?;
        fs::write(
            tmp.join("PLAN.md"),
            b"---\nplan_id: AAA\ncurrent_phase: P-01\n---\nbody\n",
        )?;
        fs::write(
            tmp.join("state.yaml"),
            b"plan_id: BBB\nexecution:\n  current_phase: P-01\nphases:\n  P-01: {}\n",
        )?;

        let mut result = CategoryResult::empty(Category::Workflow);
        run(&tmp, &mut result);
        assert_eq!(result.passed, 0);
        assert_eq!(result.failed, 1);
        assert!(result.failures.iter().any(|f| f.message.starts_with("INV-1")));
        let _ = fs::remove_dir_all(&tmp);
        Ok(())
    }

    #[test]
    fn invariant_2_current_phase_mismatch_emits_break() -> TestResult {
        let tmp = std::env::temp_dir().join("rhumb-validate-workflow-inv2-test");
        let _ = fs::remove_dir_all(&tmp);
        fs::create_dir_all(&tmp)?;
        fs::write(
            tmp.join("PLAN.md"),
            b"---\nplan_id: XX\ncurrent_phase: P-02\n---\n",
        )?;
        fs::write(
            tmp.join("state.yaml"),
            b"plan_id: XX\nexecution:\n  current_phase: P-99\nphases:\n  P-02: {}\n  P-99: {}\n",
        )?;

        let mut result = CategoryResult::empty(Category::Workflow);
        run(&tmp, &mut result);
        assert_eq!(result.passed, 0);
        assert_eq!(result.failed, 1);
        assert!(result.failures.iter().any(|f| f.message.starts_with("INV-2")));
        let _ = fs::remove_dir_all(&tmp);
        Ok(())
    }

    #[test]
    fn invariant_4_missing_handoff_file_emits_break() -> TestResult {
        let tmp = std::env::temp_dir().join("rhumb-validate-workflow-inv4-test");
        let _ = fs::remove_dir_all(&tmp);
        fs::create_dir_all(&tmp)?;
        fs::write(
            tmp.join("PLAN.md"),
            b"---\nplan_id: ZZ\ncurrent_phase: P-01\n---\n",
        )?;
        fs::write(
            tmp.join("state.yaml"),
            b"plan_id: ZZ\nexecution:\n  current_phase: P-01\nphases:\n  P-01: {}\nhandoffs:\n  handoff_files:\n    - file: handoffs/missing.md\n      phase: P-01\n",
        )?;

        let mut result = CategoryResult::empty(Category::Workflow);
        run(&tmp, &mut result);
        assert_eq!(result.passed, 0);
        assert_eq!(result.failed, 1);
        assert!(result.failures.iter().any(|f| f.message.starts_with("INV-4")));
        let _ = fs::remove_dir_all(&tmp);
        Ok(())
    }

    #[test]
    fn invariant_5_unknown_handoff_phase_emits_break() -> TestResult {
        let tmp = std::env::temp_dir().join("rhumb-validate-workflow-inv5-test");
        let _ = fs::remove_dir_all(&tmp);
        fs::create_dir_all(tmp.join("handoffs"))?;
        fs::write(
            tmp.join("PLAN.md"),
            b"---\nplan_id: WW\ncurrent_phase: P-01\n---\n",
        )?;
        fs::write(
            tmp.join("handoffs/HO-WW-P-01.md"),
            b"# handoff body\n",
        )?;
        fs::write(
            tmp.join("state.yaml"),
            b"plan_id: WW\nexecution:\n  current_phase: P-01\nphases:\n  P-01: {}\nhandoffs:\n  handoff_files:\n    - file: handoffs/HO-WW-P-01.md\n      phase: P-99\n",
        )?;

        let mut result = CategoryResult::empty(Category::Workflow);
        run(&tmp, &mut result);
        assert_eq!(result.passed, 0);
        assert_eq!(result.failed, 1);
        assert!(result.failures.iter().any(|f| f.message.starts_with("INV-5")));
        let _ = fs::remove_dir_all(&tmp);
        Ok(())
    }

    #[test]
    fn workflow_with_invalid_state_yaml_emits_workflow_break() -> TestResult {
        let tmp = std::env::temp_dir().join("rhumb-validate-workflow-bad-yaml-test");
        let _ = fs::remove_dir_all(&tmp);
        fs::create_dir_all(&tmp)?;
        fs::write(tmp.join("PLAN.md"), b"---\nplan_id: Q\n---\n")?;
        // Two `: ` on the same key triggers a YAML parse error.
        fs::write(tmp.join("state.yaml"), b"plan_id: : : not yaml\n")?;

        let mut result = CategoryResult::empty(Category::Workflow);
        run(&tmp, &mut result);
        assert_eq!(result.passed, 0);
        assert_eq!(result.failed, 1);
        assert!(result
            .failures
            .iter()
            .any(|f| f.kind == FailureKind::WorkflowBreak
                && f.message.contains("state.yaml is not valid YAML")));
        let _ = fs::remove_dir_all(&tmp);
        Ok(())
    }
}
