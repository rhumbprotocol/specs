// SPDX-License-Identifier: Apache-2.0
//
// Grammar validator (Category 5, MP-0276 P-06).
//
// Validates that sequence-id strings under a target tree conform to the
// RWP phase-sequence grammar (packages/rhumb-protocol/spec/sequence.grammar).
// The grammar is small (5 productions) and the canonical TypeScript
// reference parser lives at packages/rhumb-protocol/util/sequence-parser.ts;
// this module is a faithful Rust port of that recursive-descent parser
// rather than a generic ABNF runtime.
//
// Why no ABNF parser crate: the grammar has 5 productions and zero
// dynamic vocabulary. A generic ABNF runtime (e.g., `abnf` crate)
// would add a transitive dep tree (typically nom + regex_automata)
// for code we'd write by hand anyway. Per OQ-15.6 the dep closure
// is held to its minimum; per KD-15.8 zero meridian-* deps. P-06
// adds zero new crate dependencies — Cargo.toml is unchanged.
//
// What an RWP sequence string is (essence of the ABNF):
//   sequence     = or-expr
//   or-expr      = and-expr *( "," and-expr )
//   and-expr     = primary  *( "+" primary  )
//   primary      = phase-id / "(" or-expr ")" / "[" or-expr "]"
//   phase-id     = "P-" 2DIGIT [ "-" 1ALPHA ]
//
// Example sequences:
//   "P-01"                        single phase
//   "P-01, P-02"                  sequential (OR)
//   "P-01 + P-02"                 parallel  (AND)
//   "P-01, (P-02 + P-03), P-04"   mixed with grouping
//   "[P-01-A, P-01-B, P-01-C]"    sub-phase grouping
//
// Five grammar invariants are checked, each emitting a distinct
// FailureKind::GrammarViolation entry tagged with a GRM-N prefix:
//   GRM-1  Invalid character at position N (lexer-level: anything that
//          isn't part of the grammar's vocabulary).
//   GRM-2  Invalid phase-id format (a "P-" prefix was found but the
//          following two digits / optional "-ALPHA" sub-phase did not
//          match P-NN[-X] with X uppercase A-Z).
//   GRM-3  Parser error: unbalanced delimiter, missing operand after
//          comma/plus, unexpected token, trailing content after a
//          complete sequence.
//   GRM-4  Empty sequence (zero tokens — a .seq file with only
//          whitespace and comments).
//   GRM-5  Reserved (room to split GRM-3's parse-error class later
//          without renumbering).
//
// External implementers can grep CI logs for `GRM-3` to locate
// parser-level errors without scanning all GrammarViolation entries.
//
// Sub-phase letter range:
//   sequence.grammar, the TypeScript reference parser, and this
//   validator now converge on uppercase A-Z only via the pattern
//   `P-NN[-X]`, where X is one uppercase letter. Older schema/prose
//   references to A-C were illustrative drift, not the intended
//   validator behavior.
//
// Discovery:
//   The walker (validators::walk::walk_dir) yields every regular
//   file under target. Files with the `.seq` extension are parsed
//   as one sequence per file. Files without that extension are
//   silently skipped (binding #1 from P-02). One file = one fixture
//   = one pass-or-fail entry (per-file counter discipline).
//
//   `.seq` files may contain blank lines and `;` line-comments
//   (matching the grammar's own comment syntax §x20-7E after a
//   semicolon). Exactly one non-comment, non-blank token sequence
//   is expected per file. Zero non-comment content trips GRM-4
//   (empty sequence); more than one trips GRM-3 (trailing content
//   after a valid sequence).
//
// Out of scope (deferred — flagged in handoff, not enforced here):
//   - Sequence-string validation INSIDE YAML files (e.g., the
//     `sequence:` field of MASTERPLAN.yaml's plans block). Those
//     live inside structured artifacts that Categories 1-3 already
//     cover at the schema/template level. Wiring grammar validation
//     into YAML field extraction would be a Category-6 problem;
//     P-06 only validates standalone .seq files.
//   - ABNF grammar drift: this validator does not parse
//     sequence.grammar itself. The Rust parser is the implementation
//     of the grammar spec; if the spec is amended, the parser must
//     be amended in lockstep. A drift-hash anchor on
//     sequence.grammar (mirroring template.rs's binding #6) would
//     catch silent grammar edits — deferred to a future MP because
//     the grammar is small enough that a parallel review during
//     spec amendment is currently sufficient.

use std::fs;
use std::path::{Path, PathBuf};

use crate::validators::walk::walk_dir;
use crate::{Category, CategoryResult, Failure, FailureKind};

// ---------------------------------------------------------------------------
// Sequence-file discovery
// ---------------------------------------------------------------------------

const SEQ_EXTENSION: &str = "seq";

fn discover_seq_files(target: &Path, out: &mut Vec<PathBuf>) -> std::io::Result<()> {
    walk_dir(
        target,
        &mut |path| {
            if path
                .extension()
                .and_then(|e| e.to_str())
                .is_some_and(|e| e == SEQ_EXTENSION)
            {
                out.push(path.to_path_buf());
            }
        },
        &mut |_| false,
    )
}

// ---------------------------------------------------------------------------
// Tokenizer
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq)]
enum TokenKind {
    PhaseId(String),
    Comma,
    Plus,
    LParen,
    RParen,
    LBracket,
    RBracket,
}

#[derive(Debug, Clone)]
struct Token {
    kind: TokenKind,
    position: usize,
}

#[derive(Debug, Clone)]
struct LexError {
    code: &'static str, // GRM-1 or GRM-2
    message: String,
    position: usize,
}

/// Tokenize `input`, stripping `;` line-comments and whitespace.
/// Mirrors util/sequence-parser.ts Tokenizer with the addition of
/// comment skipping (the TS parser was written for inline strings
/// passed at runtime; this validator reads `.seq` files where
/// the grammar's comment syntax is allowed).
fn tokenize(input: &str) -> Result<Vec<Token>, LexError> {
    let bytes = input.as_bytes();
    let mut pos = 0;
    let mut tokens = Vec::new();

    while pos < bytes.len() {
        let c = bytes[pos];

        // Whitespace (per ABNF: space, tab, CR, LF).
        if c == b' ' || c == b'\t' || c == b'\r' || c == b'\n' {
            pos += 1;
            continue;
        }

        // Line comment: `;` to end of line.
        if c == b';' {
            while pos < bytes.len() && bytes[pos] != b'\n' {
                pos += 1;
            }
            continue;
        }

        // Phase id: P- 2DIGIT [ - 1ALPHA ]
        if c == b'P' && pos + 1 < bytes.len() && bytes[pos + 1] == b'-' {
            let start = pos;
            pos += 2; // consume "P-"

            // Need exactly two ASCII digits.
            if pos >= bytes.len() || !bytes[pos].is_ascii_digit() {
                return Err(LexError {
                    code: "GRM-2",
                    message: "expected digit after 'P-'".to_string(),
                    position: start,
                });
            }
            let d1 = bytes[pos];
            pos += 1;
            if pos >= bytes.len() || !bytes[pos].is_ascii_digit() {
                return Err(LexError {
                    code: "GRM-2",
                    message: "expected two-digit phase number after 'P-'".to_string(),
                    position: start,
                });
            }
            let d2 = bytes[pos];
            pos += 1;

            // Reject P-NND where another digit follows (e.g., P-100).
            if pos < bytes.len() && bytes[pos].is_ascii_digit() {
                return Err(LexError {
                    code: "GRM-2",
                    message: "phase number must be exactly two digits".to_string(),
                    position: start,
                });
            }

            let mut value = format!("P-{}{}", d1 as char, d2 as char);

            // Optional sub-phase: -[A-Z]
            // Lookahead: only consume the dash if a sub-phase letter follows.
            // Otherwise the dash belongs to a sibling phase-id.
            if pos < bytes.len()
                && bytes[pos] == b'-'
                && pos + 1 < bytes.len()
                && bytes[pos + 1].is_ascii_alphabetic()
            {
                let sub = bytes[pos + 1];
                if !sub.is_ascii_uppercase() {
                    return Err(LexError {
                        code: "GRM-2",
                        message: "sub-phase letter must be uppercase A-Z".to_string(),
                        position: pos + 1,
                    });
                }
                pos += 2;
                value.push('-');
                value.push(sub as char);

                // Reject extra alpha chars (e.g., P-01-AB).
                if pos < bytes.len() && bytes[pos].is_ascii_alphabetic() {
                    return Err(LexError {
                        code: "GRM-2",
                        message: "sub-phase must be exactly one letter".to_string(),
                        position: pos,
                    });
                }
            }

            tokens.push(Token {
                kind: TokenKind::PhaseId(value),
                position: start,
            });
            continue;
        }

        // Single-character punctuation.
        let kind = match c {
            b',' => Some(TokenKind::Comma),
            b'+' => Some(TokenKind::Plus),
            b'(' => Some(TokenKind::LParen),
            b')' => Some(TokenKind::RParen),
            b'[' => Some(TokenKind::LBracket),
            b']' => Some(TokenKind::RBracket),
            _ => None,
        };
        if let Some(kind) = kind {
            tokens.push(Token { kind, position: pos });
            pos += 1;
            continue;
        }

        return Err(LexError {
            code: "GRM-1",
            message: format!("invalid character {:?}", c as char),
            position: pos,
        });
    }

    Ok(tokens)
}

// ---------------------------------------------------------------------------
// Parser (recursive descent: or-expr > and-expr > primary)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
struct ParseError {
    code: &'static str, // GRM-3 or GRM-4
    message: String,
    position: usize,
}

struct Parser<'a> {
    tokens: &'a [Token],
    pos: usize,
    input_len: usize,
}

impl<'a> Parser<'a> {
    fn new(tokens: &'a [Token], input_len: usize) -> Self {
        Self {
            tokens,
            pos: 0,
            input_len,
        }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    fn advance(&mut self) {
        self.pos += 1;
    }

    fn position_or_eof(&self) -> usize {
        self.peek().map(|t| t.position).unwrap_or(self.input_len)
    }

    fn parse_sequence(&mut self) -> Result<(), ParseError> {
        if self.tokens.is_empty() {
            return Err(ParseError {
                code: "GRM-4",
                message: "empty sequence (no tokens)".to_string(),
                position: 0,
            });
        }
        self.parse_or_expr()?;
        if self.peek().is_some() {
            return Err(ParseError {
                code: "GRM-3",
                message: "trailing content after end of sequence".to_string(),
                position: self.position_or_eof(),
            });
        }
        Ok(())
    }

    fn parse_or_expr(&mut self) -> Result<(), ParseError> {
        self.parse_and_expr()?;
        while matches!(self.peek().map(|t| &t.kind), Some(TokenKind::Comma)) {
            self.advance();
            self.parse_and_expr().map_err(|e| ParseError {
                code: "GRM-3",
                message: format!("missing operand after ',': {}", e.message),
                position: e.position,
            })?;
        }
        Ok(())
    }

    fn parse_and_expr(&mut self) -> Result<(), ParseError> {
        self.parse_primary()?;
        while matches!(self.peek().map(|t| &t.kind), Some(TokenKind::Plus)) {
            self.advance();
            self.parse_primary().map_err(|e| ParseError {
                code: "GRM-3",
                message: format!("missing operand after '+': {}", e.message),
                position: e.position,
            })?;
        }
        Ok(())
    }

    fn parse_primary(&mut self) -> Result<(), ParseError> {
        let Some(tok) = self.peek().cloned() else {
            return Err(ParseError {
                code: "GRM-3",
                message: "expected phase-id, '(', or '['".to_string(),
                position: self.input_len,
            });
        };
        match tok.kind {
            TokenKind::PhaseId(_) => {
                self.advance();
                Ok(())
            }
            TokenKind::LParen => {
                self.advance();
                self.parse_or_expr()?;
                match self.peek().map(|t| &t.kind) {
                    Some(TokenKind::RParen) => {
                        self.advance();
                        Ok(())
                    }
                    _ => Err(ParseError {
                        code: "GRM-3",
                        message: "unbalanced '(' — expected matching ')'".to_string(),
                        position: tok.position,
                    }),
                }
            }
            TokenKind::LBracket => {
                self.advance();
                self.parse_or_expr()?;
                match self.peek().map(|t| &t.kind) {
                    Some(TokenKind::RBracket) => {
                        self.advance();
                        Ok(())
                    }
                    _ => Err(ParseError {
                        code: "GRM-3",
                        message: "unbalanced '[' — expected matching ']'".to_string(),
                        position: tok.position,
                    }),
                }
            }
            _ => Err(ParseError {
                code: "GRM-3",
                message: format!(
                    "unexpected token at position {}: expected phase-id, '(', or '['",
                    tok.position
                ),
                position: tok.position,
            }),
        }
    }
}

// ---------------------------------------------------------------------------
// Per-file validation
// ---------------------------------------------------------------------------

fn validate_seq_file(path: &Path, result: &mut CategoryResult) {
    let fixture_id = path.display().to_string();

    let text = match fs::read_to_string(path) {
        Ok(t) => t,
        Err(err) => {
            result.failures.push(Failure {
                fixture: fixture_id,
                category: Category::Grammar,
                kind: FailureKind::Io,
                message: format!("read failed: {}", path.display()),
                details: Some(err.to_string()),
            });
            result.failed = result.failed.saturating_add(1);
            return;
        }
    };

    match parse_seq_text(&text) {
        Ok(()) => {
            result.passed = result.passed.saturating_add(1);
        }
        Err(failure) => {
            result.failures.push(Failure {
                fixture: fixture_id,
                category: Category::Grammar,
                kind: FailureKind::GrammarViolation,
                message: failure.message,
                details: failure.details,
            });
            result.failed = result.failed.saturating_add(1);
        }
    }
}

struct GrammarFailure {
    message: String,
    details: Option<String>,
}

fn parse_seq_text(text: &str) -> Result<(), GrammarFailure> {
    match tokenize(text) {
        Err(lex_err) => Err(GrammarFailure {
            message: format!("{} {} at position {}", lex_err.code, lex_err.message, lex_err.position),
            details: Some(format_position_pointer(text, lex_err.position)),
        }),
        Ok(tokens) => {
            let mut parser = Parser::new(&tokens, text.len());
            match parser.parse_sequence() {
                Ok(()) => Ok(()),
                Err(parse_err) => Err(GrammarFailure {
                    message: format!(
                        "{} {} (position {})",
                        parse_err.code, parse_err.message, parse_err.position
                    ),
                    details: Some(format_position_pointer(text, parse_err.position)),
                }),
            }
        }
    }
}

/// Render the line containing `position` plus a caret pointing at the
/// column. Used as the `details` payload for grammar failures so adopters
/// can see where in the input the validator gave up. Position is a byte
/// offset; line/column are 1-indexed in the rendered prefix.
fn format_position_pointer(text: &str, position: usize) -> String {
    let bytes = text.as_bytes();
    let clamped = position.min(bytes.len());

    // Find the start of the line containing `clamped` (byte offset of
    // the first byte after the previous newline, or 0).
    let line_start = bytes[..clamped]
        .iter()
        .rposition(|&b| b == b'\n')
        .map(|i| i + 1)
        .unwrap_or(0);

    // Find the end of the line containing `clamped`.
    let line_end = bytes[clamped..]
        .iter()
        .position(|&b| b == b'\n')
        .map(|i| clamped + i)
        .unwrap_or(bytes.len());

    // Line number = number of newlines before `line_start` + 1.
    let line_no = bytes[..line_start].iter().filter(|&&b| b == b'\n').count() + 1;
    let column = clamped - line_start + 1;

    let line_text = std::str::from_utf8(&bytes[line_start..line_end]).unwrap_or("<non-utf8>");
    let caret_col = clamped - line_start;
    let pointer = " ".repeat(caret_col) + "^";
    format!("line {line_no}, column {column}:\n{line_text}\n{pointer}")
}

// ---------------------------------------------------------------------------
// Validator entry point
// ---------------------------------------------------------------------------

pub fn run(target: &Path, result: &mut CategoryResult) {
    let mut files = Vec::new();
    if let Err(err) = discover_seq_files(target, &mut files) {
        result.failures.push(Failure {
            fixture: target.display().to_string(),
            category: Category::Grammar,
            kind: FailureKind::Io,
            message: format!("failed to enumerate target tree: {err}"),
            details: None,
        });
        result.failed = result.failed.saturating_add(1);
        return;
    }
    files.sort();
    for path in &files {
        validate_seq_file(path, result);
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

    fn run_against(target: &Path) -> CategoryResult {
        let mut result = CategoryResult::empty(Category::Grammar);
        super::run(target, &mut result);
        result
    }

    fn temp_path(tag: &str) -> PathBuf {
        std::env::temp_dir().join(format!(
            "rhumb-validate-grammar-{}-{}-{}",
            tag,
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_nanos())
                .unwrap_or(0)
        ))
    }

    fn write_seq(dir: &Path, name: &str, body: &str) -> std::io::Result<PathBuf> {
        fs::create_dir_all(dir)?;
        let p = dir.join(format!("{name}.seq"));
        fs::write(&p, body)?;
        Ok(p)
    }

    // -----------------------------------------------------------------
    // Direct parser tests (no FS)
    // -----------------------------------------------------------------

    #[test]
    fn parse_accepts_single_phase() {
        assert!(parse_seq_text("P-01").is_ok());
    }

    #[test]
    fn parse_accepts_sub_phase() {
        assert!(parse_seq_text("P-01-A").is_ok());
        assert!(parse_seq_text("P-99-Z").is_ok());
    }

    #[test]
    fn parse_accepts_or_chain() {
        assert!(parse_seq_text("P-01, P-02, P-03").is_ok());
    }

    #[test]
    fn parse_accepts_and_chain() {
        assert!(parse_seq_text("P-01 + P-02 + P-03").is_ok());
    }

    #[test]
    fn parse_accepts_mixed_with_grouping() {
        assert!(parse_seq_text("P-01, (P-02 + P-03), P-04").is_ok());
        assert!(parse_seq_text("[P-01-A, P-01-B, P-01-C]").is_ok());
        assert!(parse_seq_text("P-01 + [P-02-A, P-02-B]").is_ok());
    }

    #[test]
    fn parse_accepts_whitespace_and_comments() {
        let text = "; comment line\n\nP-01,\n  P-02\n; trailing\n";
        assert!(parse_seq_text(text).is_ok());
    }

    #[test]
    fn parse_rejects_single_digit_phase() {
        let err = parse_seq_text("P-1").expect_err("must reject P-1");
        assert!(err.message.contains("GRM-2"), "got: {}", err.message);
    }

    #[test]
    fn parse_rejects_triple_digit_phase() {
        let err = parse_seq_text("P-100").expect_err("must reject P-100");
        assert!(err.message.contains("GRM-2"), "got: {}", err.message);
    }

    #[test]
    fn parse_rejects_lowercase_prefix() {
        let err = parse_seq_text("p-01").expect_err("must reject p-01");
        assert!(err.message.contains("GRM-1"), "got: {}", err.message);
    }

    #[test]
    fn parse_rejects_lowercase_sub_phase() {
        let err = parse_seq_text("P-01-a").expect_err("must reject lowercase sub-phase");
        assert!(err.message.contains("GRM-2"), "got: {}", err.message);
    }

    #[test]
    fn parse_rejects_dangling_comma() {
        let err = parse_seq_text("P-01,").expect_err("must reject trailing comma");
        assert!(err.message.contains("GRM-3"), "got: {}", err.message);
    }

    #[test]
    fn parse_rejects_dangling_plus() {
        let err = parse_seq_text("P-01 +").expect_err("must reject trailing +");
        assert!(err.message.contains("GRM-3"), "got: {}", err.message);
    }

    #[test]
    fn parse_rejects_unbalanced_paren() {
        let err = parse_seq_text("(P-01, P-02").expect_err("must reject unbalanced (");
        assert!(err.message.contains("GRM-3"), "got: {}", err.message);
    }

    #[test]
    fn parse_rejects_unbalanced_bracket() {
        let err = parse_seq_text("[P-01, P-02").expect_err("must reject unbalanced [");
        assert!(err.message.contains("GRM-3"), "got: {}", err.message);
    }

    #[test]
    fn parse_rejects_missing_separator() {
        let err = parse_seq_text("P-01 P-02").expect_err("must reject space-only separator");
        assert!(err.message.contains("GRM-3"), "got: {}", err.message);
    }

    #[test]
    fn parse_rejects_empty_sequence() {
        let err = parse_seq_text("").expect_err("must reject empty");
        assert!(err.message.contains("GRM-4"), "got: {}", err.message);
    }

    #[test]
    fn parse_rejects_only_comments_and_whitespace() {
        let err = parse_seq_text("; just a comment\n\n; another\n")
            .expect_err("must reject all-comments");
        assert!(err.message.contains("GRM-4"), "got: {}", err.message);
    }

    #[test]
    fn parse_rejects_invalid_char() {
        let err = parse_seq_text("P-01 & P-02").expect_err("must reject &");
        assert!(err.message.contains("GRM-1"), "got: {}", err.message);
    }

    #[test]
    fn parse_rejects_extra_sub_phase_letters() {
        let err = parse_seq_text("P-01-AB").expect_err("must reject multi-letter sub-phase");
        assert!(err.message.contains("GRM-2"), "got: {}", err.message);
    }

    // -----------------------------------------------------------------
    // Discovery + run() tests
    // -----------------------------------------------------------------

    #[test]
    fn run_returns_empty_on_nonexistent_target() {
        let r = run_against(Path::new(
            "/this/path/should/not/exist/rhumb-validate-grammar-empty",
        ));
        assert_eq!(r.passed, 0);
        assert_eq!(r.failed, 0);
        assert_eq!(r.skipped, 0);
        assert!(r.failures.is_empty());
    }

    #[test]
    fn run_silently_skips_non_seq_files() -> TestResult {
        let tmp = temp_path("skip-non-seq");
        let _ = fs::remove_dir_all(&tmp);
        fs::create_dir_all(&tmp)?;
        fs::write(tmp.join("README.md"), b"# nothing here\n")?;
        fs::write(tmp.join("data.json"), b"{}")?;
        fs::write(tmp.join("notes.txt"), b"ignored\n")?;

        let r = run_against(&tmp);
        assert_eq!(r.passed, 0);
        assert_eq!(r.failed, 0);
        assert!(r.failures.is_empty());

        let _ = fs::remove_dir_all(&tmp);
        Ok(())
    }

    #[test]
    fn run_passes_valid_seq_file() -> TestResult {
        let tmp = temp_path("valid-one");
        let _ = fs::remove_dir_all(&tmp);
        write_seq(&tmp, "case", "P-01, P-02 + P-03\n")?;

        let r = run_against(&tmp);
        assert_eq!(r.passed, 1);
        assert_eq!(r.failed, 0);

        let _ = fs::remove_dir_all(&tmp);
        Ok(())
    }

    #[test]
    fn run_fails_invalid_seq_file_with_grm_prefix() -> TestResult {
        let tmp = temp_path("invalid-one");
        let _ = fs::remove_dir_all(&tmp);
        write_seq(&tmp, "case", "P-1\n")?;

        let r = run_against(&tmp);
        assert_eq!(r.passed, 0);
        assert_eq!(r.failed, 1);
        assert!(
            r.failures
                .iter()
                .any(|f| f.kind == FailureKind::GrammarViolation
                    && f.message.contains("GRM-2")),
            "expected GRM-2; got {:?}",
            r.failures
        );

        let _ = fs::remove_dir_all(&tmp);
        Ok(())
    }

    #[test]
    fn run_processes_multiple_seq_files() -> TestResult {
        let tmp = temp_path("multi");
        let _ = fs::remove_dir_all(&tmp);
        write_seq(&tmp, "good-1", "P-01\n")?;
        write_seq(&tmp, "good-2", "P-01 + P-02\n")?;
        write_seq(&tmp, "bad-1", "P-1\n")?;
        write_seq(&tmp, "bad-2", "P-01,\n")?;

        let r = run_against(&tmp);
        assert_eq!(r.passed, 2);
        assert_eq!(r.failed, 2);

        let _ = fs::remove_dir_all(&tmp);
        Ok(())
    }

    #[test]
    fn run_recurses_into_subdirectories() -> TestResult {
        let tmp = temp_path("recurse");
        let _ = fs::remove_dir_all(&tmp);
        write_seq(&tmp.join("a"), "case", "P-01\n")?;
        write_seq(&tmp.join("b/c"), "case", "P-02\n")?;

        let r = run_against(&tmp);
        assert_eq!(r.passed, 2);
        assert_eq!(r.failed, 0);

        let _ = fs::remove_dir_all(&tmp);
        Ok(())
    }

    // -----------------------------------------------------------------
    // Fixture corpus
    // -----------------------------------------------------------------

    #[test]
    fn run_passes_canonical_valid_fixtures() {
        let corpus = fixtures_root().join("valid/grammar");
        if !corpus.is_dir() {
            return;
        }
        let r = run_against(&corpus);
        assert_eq!(
            r.failed, 0,
            "valid grammar fixtures unexpectedly failed: {:?}",
            r.failures
        );
        assert!(
            r.passed >= 3,
            "expected ≥3 passing grammar fixtures (ACS-0015 §9 contract); got {}",
            r.passed
        );
    }

    #[test]
    fn run_fails_canonical_invalid_fixtures() {
        let corpus = fixtures_root().join("invalid/grammar");
        if !corpus.is_dir() {
            return;
        }
        let r = run_against(&corpus);
        assert!(
            r.failed >= 3,
            "expected ≥3 failing grammar fixtures; got {}",
            r.failed
        );
        for failure in &r.failures {
            assert_eq!(
                failure.kind,
                FailureKind::GrammarViolation,
                "every invalid fixture must produce GrammarViolation; got {:?}",
                failure
            );
            assert!(
                failure.message.starts_with("GRM-"),
                "every grammar failure message must start with GRM-N prefix; got '{}'",
                failure.message
            );
        }
    }
}
