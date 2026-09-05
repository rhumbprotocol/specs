# rhumb-validate

Conformance test runner for the **Rhumb Workflow Protocol™** (RWP™).

`rhumb-validate` walks an artifact tree (your `.rwp/` directory, an
adapter package, or any RWP implementation under test) and verifies it
against the five-part RWP conformance suite. It is the canonical mechanism
for substantiating a "Rhumb-compliant" claim under the trademark policy
shipped at `packages/rhumb-protocol/TRADEMARK.md`.

License: **Apache 2.0**. Source: `packages/rhumb-protocol/conformance/`.
The crate has zero `meridian-*` dependencies and zero network surface —
read the source, audit the result, contest a finding via PR.

---

## Install

```bash
# From a checkout of the rhumb-protocol repository:
cd packages/rhumb-protocol/conformance
cargo install --path .

# Or build a release binary in place:
cargo build --release
./target/release/rhumb-validate --version
```

Once published, this will also be available as:

```bash
cargo install rhumb-validate
```

(Distribution onto crates.io / npm is gated by **MP-0277**, not part of
the v0.1.0 ship.)

---

## Run

```bash
# .rwp/ - Assuming .rwp/ but it could be anything you made it to be

# Full conformance run — typical CI invocation:
rhumb-validate --all --target .rwp/

# Single category — when iterating on one part of an implementation:
rhumb-validate --category schema --target packages/my-rwp-impl/

# Multiple categories — selectively enabled:
rhumb-validate --category schema --category grammar --target ./artifacts/

# Machine-readable JSON for dashboard ingestion:
rhumb-validate --all --target .rpw/ --format json --output report.json
```

`--target <PATH>` is required for any run that exercises a category. The
walker descends recursively, applies the relevant validators per category,
and reports a per-category pass/fail/skip counter plus an overall verdict.

---

## Interpret

### Exit codes

The exit code is the caller-facing contract. CI scripts switch on it; the
trademark mark-use gate (TRADEMARK.md §6.2) reads it.

| Code | Meaning                                                            |
|-----:|--------------------------------------------------------------------|
| `0`  | All requested categories passed against `--target`.                |
| `1`  | Category 1 (Schema) failure.                                       |
| `2`  | Category 2 (Template) failure.                                     |
| `3`  | Category 3 (Workflow) failure.                                     |
| `4`  | Category 4 (Adapter) failure.                                      |
| `5`  | Category 5 (Grammar) failure.                                      |
| `6`  | Multi-category failure (more than one of `1`..`5` failed).         |
| `10` | CLI usage error (unknown flag, missing `--target`, conflict).      |
| `11` | I/O error (cannot write `--output` file, target unreadable).       |
| `12` | Internal error — file an issue with the `--format json` output.    |

A run that exits `0` means **every fixture and artifact under `--target`
that the validators recognized passed**. Files the validators do not
recognize (non-RWP content, hidden directories, build artifacts) are
silently skipped — `rhumb-validate` is content-driven, not file-extension
driven, and will not invent failures on neutral content.

### Pass / Fail / Skip counters

Each category's row reports three numbers:

| Counter   | Meaning                                                                  |
|-----------|--------------------------------------------------------------------------|
| `Passed`  | Fixtures/artifacts the validator recognized **and** that conform.        |
| `Failed`  | Fixtures/artifacts the validator recognized that do **not** conform.     |
| `Skipped` | Fixtures the validator recognized but cannot fully evaluate (rare).      |

Files the validator does not recognize at all are **not counted**.
This is deliberate: an RWP implementation that ships a `README.md`, a
`Cargo.toml`, or unrelated source code in the same tree should not have
those files inflate any counter.

### Categories

| # | Category   | Validates                                                                                                  |
|--:|------------|------------------------------------------------------------------------------------------------------------|
| 1 | Schema     | RWP YAML/JSON instances (`PLAN.md` frontmatter, `state.yaml`, `manifest.yaml`, etc.) against canonical schemas. |
| 2 | Template   | Drift-hash check on canonical templates — implementation must ship byte-identical templates after canonicalization. |
| 3 | Workflow   | Cross-file invariants (PLAN ↔ state agreement, handoff file existence, current-phase consistency).         |
| 4 | Adapter    | Integration-adapter `MANIFEST.yaml` shape (top-level `integration:` and `components:` blocks, etc.).       |
| 5 | Grammar    | Sequence-grammar files (`*.seq`) — phase numbering, unbalanced delimiters, sub-phase letter range.         |

The validator does not infer which categories apply from the contents of
`--target`. Use `--all` for a full check or `--category <CAT>` to scope
the run.

---

## Library use

`rhumb-validate` ships both a binary and a library crate:

```rust
use rhumb_validate::{validate, Category};
use std::path::Path;

let report = validate(Path::new(".yakkl/"), &[])?;
if !report.overall_passed {
    // walk report.categories[*].failures for details
}
```

The crate also embeds the canonical fixture set at build time so
library callers can self-test without depending on the on-disk
`fixtures/` tree:

```rust
let blob: &'static [u8] = rhumb_validate::embedded_fixtures();
// Format documented in the embedded_fixtures() docstring.
// 4-byte magic "RVF\0" + 4-byte version (1) + 4-byte LE entry_count,
// followed by per-entry (LE u32 path_len, UTF-8 path, LE u64 content_len, content).
```

The format constants `EMBEDDED_FIXTURES_MAGIC` and
`EMBEDDED_FIXTURES_FORMAT_VERSION` are public so callers can sanity-check
before parsing.

---

## Output formats

### Text (default)

```text
rhumb-validate 0.1.0 (RWP 0.31.0)
target: .yakkl/
started: 2026-05-01T12:34:56.789012Z    completed: 2026-05-01T12:34:57.012345Z    duration: 223 ms

Category    Passed  Failed  Skipped  Duration(ms)
--------    ------  ------  -------  ------------
schema           5       0        0             4
template         4       0        0             1
workflow         3       0        0             7
adapter          3       0        0             1
grammar          6       0        0             2

RESULT: PASS
```

When failures occur, a `Failures:` block lists each one with the full
per-fixture message and the validator's invariant tag (`INV-N`, `ADP-N`,
`GRM-N`).

### JSON

`--format json` emits a machine-readable report with the same content.
Top-level fields: `rhumb_validate_version`, `rwp_version`, `started_at`,
`completed_at`, `target_path`, `categories[]`, `overall_passed`,
`total_duration_ms`. Use `--output FILE` to write directly to disk.

```json
{
  "rhumb_validate_version": "0.1.0",
  "rwp_version": "0.31.0",
  "started_at": "2026-05-01T12:34:56.789012Z",
  "completed_at": "2026-05-01T12:34:57.012345Z",
  "target_path": ".yakkl/",
  "categories": [
    {
      "category": "schema",
      "passed": 5, "failed": 0, "skipped": 0,
      "failures": [],
      "duration_ms": 4
    }
  ],
  "overall_passed": true,
  "total_duration_ms": 223
}
```

The `overall_passed` boolean is the canonical machine-readable signal
for mark-use auditing — a CI pipeline can fail a release on
`overall_passed == false` without parsing any other field.

---

## What "compliant" means

A claim of "Rhumb-compliant" is lawful under TRADEMARK.md §6 if and
only if **both** of the following hold:

1. `rhumb-validate --all --target <your-artifacts>` exits `0`.
2. The claimant observes the policy language in TRADEMARK.md §6.3–§6.5
   (no implication of YAKKL endorsement, no `rhumb` package-name
   prefixes, no logo use, no version-omission).

For the full narrative — what the validator checks, why both conditions
are required, and how mark-use claims can be machine-audited — see
[`../docs/CONFORMANCE.md`](../docs/CONFORMANCE.md).

---

## Reporting issues

| Issue type                         | Where to report                                                       |
|------------------------------------|-----------------------------------------------------------------------|
| Validator false positive           | Open an issue at <https://github.com/rhumbprotocol/specs>             |
| Validator false negative           | Same — include the artifact contents and the missed check.            |
| Fixture set gap                    | Pull request against `packages/rhumb-protocol/conformance/fixtures/`. |
| Trademark / mark-use question      | See TRADEMARK.md §9 for the policy contact.                           |

---

## Development

```bash
cd packages/rhumb-protocol/conformance

cargo build
cargo test                                 # 133 tests across lib + main + cli
cargo clippy --all-targets -- -D warnings  # zero warnings tolerated

# Run validator against the canonical fixture set:
cargo run -- --all --target fixtures/valid/

# Run validator against itself (Target 0 — Meridian artifact tree):
cargo run -- --all --target ../../../.yakkl/
```

The `fixtures/` tree is the canonical fixture set. Every category has paired
`valid/` and `invalid/` subdirectories — a positive case for each
validator's happy path and at least one negative case per documented
invariant. Adding fixtures is the most direct way to extend coverage.

---

## License

Apache License 2.0. See `../LICENSE`.

---

Produced:
  - by: YAKKL® Meridian™— https://meridian.yakkl.com
  - for: the Rhumb Workflow Protocol™ (RWP™) — https://rhumbprotocol.dev
  - copyright: Copyright © 2026 YAKKL Inc. All Rights Reserved.
