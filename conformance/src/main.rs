// SPDX-License-Identifier: Apache-2.0
//
// rhumb-validate CLI — entry point and exit-code dispatch.
//
// CLI surface tracks ACS-0015 §6 verbatim. Exit-code mapping is the
// caller-facing contract for CI integration; do not reorder without an
// ACS amendment.

use std::fs;
use std::io::Write as _;
use std::path::PathBuf;
use std::process::ExitCode;

use clap::Parser;
use clap::ValueEnum;

use rhumb_validate::{validate, Category, Error, Report, RWP_VERSION};

const AFTER_HELP: &str = "\
USAGE:
  rhumb-validate [OPTIONS] [--all | --category <CAT>...] --target <PATH>

EXIT CODES:
  0    All requested categories passed against --target.
  1    Category 1 (Schema) failure.
  2    Category 2 (Template) failure.
  3    Category 3 (Workflow) failure.
  4    Category 4 (Adapter) failure.
  5    Category 5 (Grammar) failure.
  6    Multi-category failure (more than one of 1..5 failed).
  10   CLI usage error (unknown flag, missing target).
  11   I/O error (target not found, unreadable).
  12   Internal error — file an issue.

EXAMPLES:
  # Full run — typical CI invocation:
  rhumb-validate --all --target .rwp/    # YAKKL Meridian uses .rwp/ directory structure to hold many RWP™ artifacts

  # Single category — developer iterating on one area:
  rhumb-validate --category schema --target packages/my-impl/

  # Machine-readable output for dashboard ingestion:
  rhumb-validate --all --target .rwp/ --format json --output report.json
";

#[derive(Debug, Clone, Copy, ValueEnum)]
#[value(rename_all = "lowercase")]
enum CategoryArg {
    Schema,
    Template,
    Workflow,
    Adapter,
    Grammar,
}

impl From<CategoryArg> for Category {
    fn from(c: CategoryArg) -> Self {
        match c {
            CategoryArg::Schema => Category::Schema,
            CategoryArg::Template => Category::Template,
            CategoryArg::Workflow => Category::Workflow,
            CategoryArg::Adapter => Category::Adapter,
            CategoryArg::Grammar => Category::Grammar,
        }
    }
}

#[derive(Debug, Clone, Copy, ValueEnum)]
#[value(rename_all = "lowercase")]
enum FormatArg {
    Text,
    Json,
}

#[derive(Parser, Debug)]
#[command(
    name = "rhumb-validate",
    version,
    about = "RWP™™ conformance test runner",
    long_about = "rhumb-validate runs the Rhumb Workflow Protocol™ conformance suite \
                  against a target artifact tree and reports pass/fail per category.",
    after_long_help = AFTER_HELP,
    after_help = AFTER_HELP,
    disable_version_flag = true
)]
struct Cli {
    /// Run all five categories (default if no --category given).
    #[arg(long, conflicts_with = "category")]
    all: bool,

    /// Run a specific category; may repeat. CAT in {schema,template,workflow,adapter,grammar}.
    #[arg(long = "category", value_name = "CAT", num_args = 1)]
    category: Vec<CategoryArg>,

    /// Output format.
    #[arg(long, value_name = "FMT", default_value = "text")]
    format: FormatArg,

    /// Write report to FILE instead of stdout.
    #[arg(long, value_name = "FILE")]
    output: Option<PathBuf>,

    /// Override fixtures directory (default: embedded at build time).
    #[arg(long, value_name = "DIR")]
    fixtures: Option<PathBuf>,

    /// Artifact directory to validate (e.g. .meridian/, packages/foo/artifacts/).
    #[arg(long, value_name = "PATH")]
    target: Option<PathBuf>,

    /// Print rhumb-validate version and the RWP protocol version it was built against.
    #[arg(long)]
    version: bool,
}

fn main() -> ExitCode {
    // Parse arguments manually so we can map clap's exit semantics to the
    // ACS-0015 §6 exit-code table (--help / --version → 0, other parse
    // errors → 10).
    let cli = match Cli::try_parse() {
        Ok(cli) => cli,
        Err(err) => {
            use clap::error::ErrorKind;
            let kind = err.kind();
            if matches!(kind, ErrorKind::DisplayHelp | ErrorKind::DisplayVersion) {
                let _ = err.print();
                return ExitCode::from(0);
            }
            let _ = err.print();
            return ExitCode::from(10);
        }
    };

    if cli.version {
        println!(
            "rhumb-validate {} (RWP {})",
            env!("CARGO_PKG_VERSION"),
            RWP_VERSION
        );
        return ExitCode::from(0);
    }

    let target = match cli.target.as_ref() {
        Some(t) => t.clone(),
        None => {
            eprintln!(
                "rhumb-validate: error: --target <PATH> is required (use --help for usage)"
            );
            return ExitCode::from(10);
        }
    };

    let categories: Vec<Category> = if cli.all || cli.category.is_empty() {
        Category::ALL.to_vec()
    } else {
        cli.category.iter().copied().map(Category::from).collect()
    };

    let report = match validate(&target, &categories) {
        Ok(report) => report,
        Err(err) => {
            eprintln!("rhumb-validate: error: {err}");
            return error_exit_code(&err);
        }
    };

    if let Err(err) = emit_report(&report, cli.format, cli.output.as_deref()) {
        eprintln!("rhumb-validate: error writing output: {err}");
        return ExitCode::from(11);
    }

    ExitCode::from(report_exit_code(&report))
}

fn emit_report(
    report: &Report,
    format: FormatArg,
    output: Option<&std::path::Path>,
) -> std::io::Result<()> {
    let rendered = match format {
        FormatArg::Json => match serde_json::to_string_pretty(report) {
            Ok(s) => s,
            Err(e) => return Err(std::io::Error::other(e)),
        },
        FormatArg::Text => render_text(report),
    };

    match output {
        Some(path) => fs::write(path, rendered),
        None => {
            let mut stdout = std::io::stdout().lock();
            stdout.write_all(rendered.as_bytes())?;
            if !rendered.ends_with('\n') {
                stdout.write_all(b"\n")?;
            }
            Ok(())
        }
    }
}

fn render_text(report: &Report) -> String {
    let mut out = String::new();
    out.push_str(&format!(
        "rhumb-validate {} (RWP {})\n",
        report.rhumb_validate_version, report.rwp_version
    ));
    out.push_str(&format!("target: {}\n", report.target_path));
    out.push_str(&format!(
        "started: {}    completed: {}    duration: {} ms\n",
        report.started_at, report.completed_at, report.total_duration_ms
    ));
    out.push('\n');

    if report.categories.is_empty() {
        out.push_str("(no categories executed — placeholder run)\n");
    } else {
        out.push_str("Category    Passed  Failed  Skipped  Duration(ms)\n");
        out.push_str("--------    ------  ------  -------  ------------\n");
        for cat in &report.categories {
            out.push_str(&format!(
                "{:<11} {:>6}  {:>6}  {:>7}  {:>12}\n",
                cat.category.as_str(),
                cat.passed,
                cat.failed,
                cat.skipped,
                cat.duration_ms,
            ));
        }
        let any_failures = report.categories.iter().any(|c| !c.failures.is_empty());
        if any_failures {
            out.push_str("\nFailures:\n");
            for cat in &report.categories {
                for f in &cat.failures {
                    out.push_str(&format!(
                        "  [{}] {} — {}\n",
                        cat.category.as_str(),
                        f.fixture,
                        f.message
                    ));
                    if let Some(details) = &f.details {
                        for line in details.lines() {
                            out.push_str(&format!("      {line}\n"));
                        }
                    }
                }
            }
        }
    }

    out.push('\n');
    out.push_str(if report.overall_passed {
        "RESULT: PASS\n"
    } else {
        "RESULT: FAIL\n"
    });
    out
}

/// Map a `Report` to the §6 exit-code table.
fn report_exit_code(report: &Report) -> u8 {
    let failed: Vec<Category> = report
        .categories
        .iter()
        .filter(|c| c.failed > 0)
        .map(|c| c.category)
        .collect();

    match failed.len() {
        0 => 0,
        1 => category_exit_code(failed[0]),
        _ => 6,
    }
}

fn category_exit_code(c: Category) -> u8 {
    match c {
        Category::Schema => 1,
        Category::Template => 2,
        Category::Workflow => 3,
        Category::Adapter => 4,
        Category::Grammar => 5,
    }
}

fn error_exit_code(err: &Error) -> ExitCode {
    ExitCode::from(error_exit_raw(err))
}

fn error_exit_raw(err: &Error) -> u8 {
    match err {
        Error::Io { .. } => 11,
        Error::FixtureParse { .. } | Error::Internal(_) => 12,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rhumb_validate::{CategoryResult, Failure, FailureKind};

    fn base_report() -> Report {
        Report {
            rhumb_validate_version: "0.1.0".to_string(),
            rwp_version: RWP_VERSION.to_string(),
            started_at: "1970-01-01T00:00:00Z".to_string(),
            completed_at: "1970-01-01T00:00:00Z".to_string(),
            target_path: "/tmp/x".to_string(),
            categories: Vec::new(),
            overall_passed: true,
            total_duration_ms: 0,
        }
    }

    fn failing_category(category: Category) -> CategoryResult {
        CategoryResult {
            category,
            passed: 0,
            failed: 1,
            skipped: 0,
            failures: vec![Failure {
                fixture: "synthetic".to_string(),
                category,
                kind: FailureKind::Internal,
                message: "synthetic failure".to_string(),
                details: None,
            }],
            duration_ms: 0,
        }
    }

    #[test]
    fn exit_code_zero_when_all_pass() {
        assert_eq!(report_exit_code(&base_report()), 0);
    }

    #[test]
    fn exit_code_matches_single_category_failure() {
        let cases: [(Category, u8); 5] = [
            (Category::Schema, 1),
            (Category::Template, 2),
            (Category::Workflow, 3),
            (Category::Adapter, 4),
            (Category::Grammar, 5),
        ];
        for (cat, expected) in cases {
            let mut report = base_report();
            report.categories = vec![failing_category(cat)];
            report.overall_passed = false;
            assert_eq!(report_exit_code(&report), expected, "category {cat}");
        }
    }

    #[test]
    fn exit_code_six_for_multi_category_failure() {
        let mut report = base_report();
        report.categories = vec![
            failing_category(Category::Schema),
            failing_category(Category::Grammar),
        ];
        report.overall_passed = false;
        assert_eq!(report_exit_code(&report), 6);
    }

    #[test]
    fn error_exit_code_maps_io_to_eleven() {
        let err = Error::Io {
            path: "/x".to_string(),
            source: std::io::Error::new(std::io::ErrorKind::NotFound, "not found"),
        };
        assert_eq!(error_exit_raw(&err), 11);
    }

    #[test]
    fn error_exit_code_maps_internal_to_twelve() {
        let err = Error::Internal("boom".to_string());
        assert_eq!(error_exit_raw(&err), 12);
    }

    #[test]
    fn error_exit_code_maps_fixture_parse_to_twelve() {
        let err = Error::FixtureParse {
            fixture: "x".to_string(),
            message: "y".to_string(),
        };
        assert_eq!(error_exit_raw(&err), 12);
    }

    #[test]
    fn render_text_includes_pass_marker_when_passing() {
        let out = render_text(&base_report());
        assert!(out.contains("RESULT: PASS"));
        assert!(out.contains("RWP "));
    }

    #[test]
    fn render_text_includes_fail_marker_when_failing() {
        let mut report = base_report();
        report.categories = vec![failing_category(Category::Schema)];
        report.overall_passed = false;
        let out = render_text(&report);
        assert!(out.contains("RESULT: FAIL"));
        assert!(out.contains("synthetic failure"));
    }
}
