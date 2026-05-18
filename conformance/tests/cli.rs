// SPDX-License-Identifier: Apache-2.0
//
// CLI integration tests — MP-0276 P-08, ACS-0015 §6 (exit-code table) and
// §9 P-08 (test surface).
//
// Each test spawns the real `rhumb-validate` binary via assert_cmd and
// asserts on (a) exit code and (b) stdout/stderr content. The exit codes
// are caller contract — any change here must come with an ACS-0015 §6
// amendment + corresponding TRADEMARK.md §6.2 review (mark-use gate).
//
// Coverage map (one test per cell at minimum; some cells have multiple):
//
//   ┌────────────────────────────────────┬────────────────────────────┐
//   │ Exit code                          │ Trigger                    │
//   ├────────────────────────────────────┼────────────────────────────┤
//   │  0  All passed                     │ fixtures/valid/ + variants │
//   │  1  Schema failure                 │ fixtures/invalid/schemas/  │
//   │  2  Template failure               │ fixtures/invalid/templates │
//   │  3  Workflow failure               │ fixtures/invalid/workflows │
//   │  4  Adapter failure                │ fixtures/invalid/adapters/ │
//   │  5  Grammar failure                │ fixtures/invalid/grammar/  │
//   │  6  Multi-category failure         │ --target fixtures/invalid/ │
//   │ 10  CLI usage error                │ missing --target;          │
//   │                                    │ unknown flag               │
//   │ 11  Output write error             │ --output to bad path       │
//   │ 12  Internal error                 │ unit-tested in main.rs     │
//   │                                    │ (not reachable from CLI    │
//   │                                    │ surface in v1)             │
//   └────────────────────────────────────┴────────────────────────────┘

use assert_cmd::Command;
use predicates::str::contains;

const BIN: &str = "rhumb-validate";

fn fixtures(rel: &str) -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("fixtures")
        .join(rel)
}

// ------------------------------------------------------------------
// Exit 0 — passing runs
// ------------------------------------------------------------------

#[test]
fn exit_0_all_against_valid_corpus() {
    Command::cargo_bin(BIN)
        .unwrap()
        .arg("--all")
        .arg("--target")
        .arg(fixtures("valid"))
        .assert()
        .success()
        .stdout(contains("RESULT: PASS"));
}

#[test]
fn exit_0_single_category_against_valid_schemas() {
    Command::cargo_bin(BIN)
        .unwrap()
        .args(["--category", "schema", "--target"])
        .arg(fixtures("valid/schemas"))
        .assert()
        .success();
}

#[test]
fn exit_0_two_categories_against_valid_corpus() {
    Command::cargo_bin(BIN)
        .unwrap()
        .args(["--category", "schema", "--category", "grammar", "--target"])
        .arg(fixtures("valid"))
        .assert()
        .success();
}

#[test]
fn exit_0_version_flag() {
    Command::cargo_bin(BIN)
        .unwrap()
        .arg("--version")
        .assert()
        .success()
        .stdout(contains("rhumb-validate"))
        .stdout(contains("RWP "));
}

#[test]
fn exit_0_help_flag() {
    Command::cargo_bin(BIN)
        .unwrap()
        .arg("--help")
        .assert()
        .success()
        .stdout(contains("EXIT CODES:"))
        .stdout(contains("rhumb-validate"));
}

#[test]
fn exit_0_format_json_emits_well_formed_report() {
    let assertion = Command::cargo_bin(BIN)
        .unwrap()
        .args(["--all", "--format", "json", "--target"])
        .arg(fixtures("valid"))
        .assert()
        .success();
    let stdout = String::from_utf8(assertion.get_output().stdout.clone()).unwrap();
    let v: serde_json::Value =
        serde_json::from_str(&stdout).expect("JSON output must be well-formed");
    assert_eq!(v["overall_passed"], serde_json::Value::Bool(true));
    let started = v["started_at"].as_str().expect("started_at present");
    assert!(
        !started.starts_with("1970-"),
        "placeholder timestamp leaked into JSON output"
    );
    assert!(started.ends_with('Z'), "started_at must be UTC ISO-8601");
}

#[test]
fn exit_0_output_file_writes_report() {
    let dir = tempdir();
    let report = dir.join("report.json");
    Command::cargo_bin(BIN)
        .unwrap()
        .args(["--all", "--format", "json", "--output"])
        .arg(&report)
        .arg("--target")
        .arg(fixtures("valid"))
        .assert()
        .success();
    let body = std::fs::read_to_string(&report).expect("report.json written");
    let v: serde_json::Value = serde_json::from_str(&body).unwrap();
    assert_eq!(v["overall_passed"], serde_json::Value::Bool(true));
}

#[test]
fn exit_0_format_text_default() {
    Command::cargo_bin(BIN)
        .unwrap()
        .args(["--all", "--target"])
        .arg(fixtures("valid"))
        .assert()
        .success()
        .stdout(contains("Category"))
        .stdout(contains("RESULT: PASS"));
}

// ------------------------------------------------------------------
// Exit 1..5 — single-category failures
// ------------------------------------------------------------------

#[test]
fn exit_1_schema_failure() {
    Command::cargo_bin(BIN)
        .unwrap()
        .args(["--category", "schema", "--target"])
        .arg(fixtures("invalid/schemas"))
        .assert()
        .code(1)
        .stdout(contains("RESULT: FAIL"));
}

#[test]
fn exit_2_template_failure() {
    Command::cargo_bin(BIN)
        .unwrap()
        .args(["--category", "template", "--target"])
        .arg(fixtures("invalid/templates"))
        .assert()
        .code(2)
        .stdout(contains("RESULT: FAIL"));
}

#[test]
fn exit_3_workflow_failure() {
    Command::cargo_bin(BIN)
        .unwrap()
        .args(["--category", "workflow", "--target"])
        .arg(fixtures("invalid/workflows"))
        .assert()
        .code(3)
        .stdout(contains("RESULT: FAIL"));
}

#[test]
fn exit_4_adapter_failure() {
    Command::cargo_bin(BIN)
        .unwrap()
        .args(["--category", "adapter", "--target"])
        .arg(fixtures("invalid/adapters"))
        .assert()
        .code(4)
        .stdout(contains("RESULT: FAIL"));
}

#[test]
fn exit_5_grammar_failure() {
    Command::cargo_bin(BIN)
        .unwrap()
        .args(["--category", "grammar", "--target"])
        .arg(fixtures("invalid/grammar"))
        .assert()
        .code(5)
        .stdout(contains("RESULT: FAIL"));
}

// ------------------------------------------------------------------
// Exit 6 — multi-category failure
// ------------------------------------------------------------------

#[test]
fn exit_6_all_against_invalid_tree_multi_category() {
    Command::cargo_bin(BIN)
        .unwrap()
        .args(["--all", "--target"])
        .arg(fixtures("invalid"))
        .assert()
        .code(6)
        .stdout(contains("RESULT: FAIL"));
}

// ------------------------------------------------------------------
// Exit 10 — CLI usage error
// ------------------------------------------------------------------

#[test]
fn exit_10_missing_target() {
    Command::cargo_bin(BIN)
        .unwrap()
        .arg("--all")
        .assert()
        .code(10)
        .stderr(contains("--target"));
}

#[test]
fn exit_10_unknown_flag() {
    Command::cargo_bin(BIN)
        .unwrap()
        .args(["--this-flag-does-not-exist", "--target", "/tmp"])
        .assert()
        .code(10);
}

#[test]
fn exit_10_unknown_category_value() {
    Command::cargo_bin(BIN)
        .unwrap()
        .args(["--category", "nonsense", "--target", "/tmp"])
        .assert()
        .code(10);
}

#[test]
fn exit_10_all_and_category_conflict() {
    // clap declares `--all` conflicts with `--category` — surface as a
    // usage error, not a silent override.
    Command::cargo_bin(BIN)
        .unwrap()
        .args(["--all", "--category", "schema", "--target", "/tmp"])
        .assert()
        .code(10);
}

// ------------------------------------------------------------------
// Exit 11 — output write failure
// ------------------------------------------------------------------

#[test]
fn exit_11_output_to_unwritable_directory_fails() {
    // Writing to a path whose parent directory does not exist surfaces
    // as ExitCode::from(11) per main.rs:167. This is the documented I/O
    // error path reachable from the CLI surface.
    Command::cargo_bin(BIN)
        .unwrap()
        .args([
            "--all",
            "--format",
            "json",
            "--output",
            "/this/parent/path/does/not/exist/report.json",
            "--target",
        ])
        .arg(fixtures("valid"))
        .assert()
        .code(11)
        .stderr(contains("error writing output"));
}

// ------------------------------------------------------------------
// Neutrality — non-RWP tree exits 0 (R10 regression)
// ------------------------------------------------------------------

#[test]
fn exit_0_neutrality_against_empty_tree() {
    let dir = tempdir();
    Command::cargo_bin(BIN)
        .unwrap()
        .args(["--all", "--target"])
        .arg(&dir)
        .assert()
        .success()
        .stdout(contains("RESULT: PASS"));
}

// ------------------------------------------------------------------
// helpers
// ------------------------------------------------------------------

/// Minimal tempdir without pulling the `tempfile` crate. We don't need
/// the cleanup-on-drop guarantees of `tempfile` for these tests — each
/// run gets its own dir, OS cleans up `target/tmp/` on cargo clean, and
/// using PID + nanos avoids cross-test collisions on parallel cargo test.
fn tempdir() -> std::path::PathBuf {
    use std::time::{SystemTime, UNIX_EPOCH};
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    let pid = std::process::id();
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("target")
        .join("tmp")
        .join(format!("cli-test-{pid}-{nanos}"));
    std::fs::create_dir_all(&path).expect("create tempdir");
    path
}
