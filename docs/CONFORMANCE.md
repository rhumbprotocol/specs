# Rhumb™ Conformance

This document explains what it means to be **Rhumb-compliant**, what
`rhumb-validate` actually checks, how compliance interacts with the
trademark policy, and how to read a non-zero verdict.

It is written for implementers — somebody whose project ingests, emits,
or transforms Rhumb Workflow Protocol™ artifacts and who needs to either
(a) assert compliance publicly or (b) understand where their
implementation stands relative to the spec.

---

## 1. What it means to be Rhumb-compliant

A claim of "Rhumb-compliant" is a public, testable assertion. Under the
trademark policy at `packages/rhumb-protocol/TRADEMARK.md` §6.2, the
claim is lawful **if and only if both** of the following hold at the
moment the claim is made:

1. **Mechanical condition**: `rhumb-validate --all --target <artifacts>`
   exits with status code `0` against the implementation's artifact tree.

2. **Policy condition**: the claimant observes the policy language in
   TRADEMARK.md §6.3 (attribution), §6.4 (product naming), and §6.5
   (forks). Specifically — no implication of YAKKL, Inc. endorsement;
   no use of `rhumb` package-name prefixes outside the official scope;
   no use of the Rhumb marks as the primary identifier of a product
   without a separate written agreement.

Either condition alone is insufficient. A passing validator run by an
implementation that uses the marks as its product name is not
compliant. A policy-observant implementation that fails the validator
is not compliant.

Both conditions are independently verifiable: anyone can run
`rhumb-validate` against the implementation's published artifacts, and
anyone can read the implementation's marketing copy against TRADEMARK.md.

This is the operational definition. There is no other gate — no
internal review, no self-assessment, no third-party seal-of-approval
program.

---

## 2. What `rhumb-validate` checks

The validator runs five independent categories, one per part of the
RWP™ specification. Each category has a documented exit code so CI
scripts can react granularly.

| #  | Category   | Validates                                                                                                                          | Exit |
|---:|------------|------------------------------------------------------------------------------------------------------------------------------------|-----:|
| 1  | Schema     | RWP™ YAML/JSON instances against the canonical JSON Schemas at `spec/schemas/`. Catches structural drift in `PLAN.md` frontmatter, `state.yaml`, `manifest.yaml`, `INTAKE.yaml`, `handoff.yaml`. |  `1` |
| 2  | Template   | Drift-hash check: implementation must ship byte-identical canonical templates after CRLF/BOM/trailing-newline canonicalization. Catches forks of templates that drifted from the spec. |  `2` |
| 3  | Workflow   | Cross-file invariants: `PLAN.md`'s `plan_id` agrees with `state.yaml`'s `plan_id`; the handoff file referenced by `state.yaml.execution.last_handoff` exists; `current_phase` is in the `phases:` map. |  `3` |
| 4  | Adapter    | Integration-adapter shape: `MANIFEST.yaml` carries `integration:` and `components:` blocks; `rwp_version` is a valid version string; required adapter fields are present. |  `4` |
| 5  | Grammar    | Sequence-grammar files (`*.seq`): two-digit phase numbering (`P-01`, not `P-1`), balanced delimiters, sub-phase letters in the `A`–`Z` range, recognized invariant tags. |  `5` |

Multi-category failures roll up to exit code `6`. CLI usage errors map
to `10`; I/O errors to `11`; internal validator errors to `12`. The
full table is in `packages/rhumb-protocol/conformance/README.md`.

The validator is **content-driven**. It does not assume that every file
under `--target` is RWP. It looks for positive signals (a `$schema`
field pointing at one of the canonical RWP schemas, a recognizable
filename in the canonical templates list, a `MANIFEST.yaml` carrying an
`integration:` block, etc.) and silently ignores everything it does not
recognize. An implementation that ships a `Cargo.toml`, a `README.md`,
and a `node_modules/` directory inside the same tree will not see those
files counted as failures.

---

## 3. The mark-use gate

The mark-use gate is the bridge between the Apache 2.0 source license
and the trademark policy. The source code under
`packages/rhumb-protocol/` (including `rhumb-validate` itself) is
freely usable under Apache 2.0 — fork it, modify it, embed it. The
**marks** (`Rhumb`, `RWP`, `Rhumb Workflow Protocol`) are **not**
covered by the source license. Their use is governed by the trademark
policy.

The two-condition rule above (mechanical + policy) is what makes the
gate meaningful. If only the mechanical condition were required, an
implementation could pass the validator, then publish itself as
"RhumbHub by Acme" and trade on the protocol's reputation. If only
the policy condition were required, any implementation could call
itself compliant without engineering substantiation.

The gate is also what makes the validator's behavior **honest**. The
validator refuses to lie about a tree's conformance state — a real
divergence produces a non-zero exit code and a fixture-level message,
even when the divergence has architectural reasons. This is the
property that protects downstream consumers of compliance claims.

---

## 4. Why the canonical reference run currently exits non-zero

The canonical reference run for v0.1.0 — `rhumb-validate --all
--target .meridian/` against YAKKL's internal Meridian artifact tree —
**currently exits `6`** (multi-category failure). This is the honest
v1 baseline, and the project explicitly contemplated and budgeted for
this state in the architecture spec (ACS-0015 §8 R5).

Treating exit-non-zero as the v1 baseline keeps the mark-use gate
load-bearing: an exit-`0` only attaches to artifact trees that
genuinely pass. If the project had silently widened the validator's
exemption door to force exit-`0`, the gate would become ceremonial.

The current divergences fall into the two narrow architectural
exemption categories and one larger reconciliation-in-flight category.

### 4.1 Two narrow architectural exemptions

Both exemptions are **path-narrow**: they are scoped to a specific
file or directory, not a class of artifacts. Both have a documented
follow-up MP candidate that would replace the exemption with a
principled validator detection rule (positive-signal adapter discovery
for the first; an `well-known-archive` walker skip-list for the second).
Until those land, the v1 disposition is documented exemption with full
rationale.

### 4.2 Template entries — reconciliation in flight

The validator's drift report is **data input** for that work, not a
bug list to fix in the conformance MP itself. The 11 entries have
sub-classification recorded in `exemptions.yaml`:

- 4 templates carry `RWP-SHARED-BEGIN`/`...END` markers (multi-region
  materialization in flight per ACS-0014 KD-14.3)
- 7 templates are freeform Meridian extensions ahead of RWP (Meridian
  has features the RWP spec has not yet absorbed)

When the reconciliation work lands, the canonical reference run is
expected to converge to exit `0`. Until then, the documented baseline
is `6`.

### 4.3 What this means for adopters

If you run `rhumb-validate` against your own artifact tree and it
exits `0`, you are passing the same gate that YAKKL's reference run
will eventually pass. The validator does not lower the bar for
YAKKL® — the YAKKL® reference run is held to exactly the same
mechanical condition as any external implementation. That symmetry
is the policy's defense against accusations of self-dealing.

If your run exits non-zero, see §6 for how to interpret and
remediate.

---

## 5. Validator philosophy

The behavior of `rhumb-validate` is governed by four properties. Each
exists for a specific reason; understanding them helps interpret the
output.

### 5.1 Positive identification

The validator only acts on files it recognizes as RWP. Recognition is
based on positive signals — a `$schema` URI matching one of the
canonical RWP schemas, a filename in the canonical templates allowlist,
an `integration:` block in `MANIFEST.yaml`, a `*.seq` extension. Files
without any positive signal are silently skipped, regardless of whether
they live inside `.rwp/`, inside `node_modules/`, or anywhere else
under `--target`.

This is the property that lets you point `rhumb-validate` at a real
project tree without it inventing failures on `Cargo.toml`,
`package.json`, `Dockerfile`, README files, or any other neutral content.

### 5.2 Silent skip, never silent pass

Skipping is for files the validator does not recognize. It is **not**
for files the validator recognizes but cannot fully evaluate — those
are reported as failures with the specific invariant violated. The
distinction matters: a skipped file is "this is not my responsibility";
a failed file is "this is my responsibility and it is wrong."

The validator never converts a recognized-but-broken file into a
silent pass.

### 5.3 Deterministic output

A given `--target` tree (with fixed file contents) and a given
`--category` selection produce a byte-identical report across runs,
modulo `started_at` / `completed_at` / `total_duration_ms`. Validators
do not re-order failures, do not depend on filesystem walk ordering for
their counters, and do not consult the network, the system clock for
content decisions, or any environment variables. CI dashboards can
diff JSON outputs across runs to detect actual drift.

### 5.4 Offline, pure-Rust

The validator has zero network surface, zero async runtime, zero
`iana-time-zone` / `tz-rs` dependencies, and zero `meridian-*` crate
dependencies. The dependency tree is auditable in <30 lines of
`cargo tree` output, and a release build produces a single static
binary suitable for air-gapped CI.

This was a deliberate non-negotiable: a conformance gate that depends
on network reachability or vendor-runtime availability is one that can
be silently weakened by ambient infrastructure changes.

---

## 6. Reading a failure

A non-zero run produces per-category counts plus a `Failures:` block.
Each failure line carries:

- the category (`schema`, `template`, `workflow`, `adapter`, `grammar`)
- the path to the offending fixture or artifact
- the invariant tag (e.g. `INV-1`, `ADP-3`, `GRM-2`)
- a one-line message
- optional indented details (often a position pointer for grammar
  failures, or a JSON Path for schema failures)

Example:

```text
Failures:
  [grammar] /path/to/artifacts/sequences/onboarding.seq — GRM-3 unbalanced '(' — expected matching ')' (position 123)
      line 3, column 1:
      (P-01, P-02
      ^
  [workflow] /path/to/artifacts/MP-0042-dark-mode-toggle/state.yaml — INV-1 PLAN.md frontmatter is missing plan_id
```

### Remediation playbook

| Category   | Most common fix                                                                                                                                            |
|------------|------------------------------------------------------------------------------------------------------------------------------------------------------------|
| Schema     | Run `--format json`, find the failing fixture, compare its frontmatter against `spec/schemas/<schema-name>.schema.json`. Fix the YAML to match the schema. |
| Template   | Compare the implementation's template against `templates/<TEMPLATE-NAME>.template`. The drift is byte-level after canonicalization (LF endings, exactly one trailing newline). |
| Workflow   | Most failures are agreement violations between paired files — fix `state.yaml` to match `PLAN.md`, or add the missing handoff file the state references.   |
| Adapter    | Add the missing `integration:` or `components:` block to `MANIFEST.yaml`. The adapter validator is structural — once the shape is right, it passes.       |
| Grammar    | The position pointer is a line:column with a caret. Read the offending character; fix the syntax violation it indicates.                                  |

If you believe a failure is a validator false positive (the artifact is
correct but the validator is wrong), open an issue with the artifact
contents inline. The validator's source code is auditable and
contestable; conformance is a public conversation, not a sealed verdict.

---

## 7. Versioning and compatibility

### 7.1 Two version numbers

The validator's report distinguishes two versions:

- `rhumb_validate_version` — the validator's own crate version, e.g. `0.1.0`.
- `rwp_version` — the protocol version this build of the validator was
  compiled against, e.g. `0.28.1`.

A change to `rwp_version` means the protocol itself moved; a change to
`rhumb_validate_version` means only the tool moved (e.g., a bug fix
that does not change the spec).

Both versions appear in the JSON output and the text-format header
line, so downstream dashboards can diff conformance runs across both
axes.

### 7.2 Backward compatibility commitments

For the v0.x line, the project commits to:

- The exit-code table (codes `0`–`12`) is **frozen**. New exit codes,
  if any, will be added at `13`+ and only when an entirely new failure
  axis appears.
- The JSON output's top-level fields are **additive-only**. New fields
  may appear; existing fields will not be renamed or removed within
  a v0.x release.
- The five-category model is the **public surface**. A sixth category
  would land via a separate ACS amendment with public review.
- Validator relaxations (over-strict rules being narrowed) **may**
  happen mid-version. Validator strictenings (a previously-passing
  artifact starts failing) require a minor-version bump.

The v1.0 line will lock the schema set and template set against
backward-incompatible change. v0.x explicitly does not.

### 7.3 Source of truth

The canonical specs live under `packages/rhumb-protocol/spec/`. Every
`spec/` change must be accompanied by either a fixture corpus update
or a validator update — the conformance suite is the spec's executable
form. A spec change without a corresponding suite change is a defect
in the change.

---

## 8. Reporting and contributing

| Issue type                                  | Where                                                                  |
|---------------------------------------------|------------------------------------------------------------------------|
| Validator false positive / negative         | <https://github.com/rhumbprotocol/specs> issue tracker                 |
| Fixture corpus gap                          | PR against `packages/rhumb-protocol/conformance/fixtures/`             |
| Spec ambiguity                              | Issue tracker, with a worked example showing the ambiguity             |
| Trademark / mark-use question               | TRADEMARK.md §10 (policy contact)                                      |
| Vulnerability in `rhumb-validate`           | See SECURITY.md (when published) for the disclosure channel            |

Contributing fixtures is the most direct way to extend coverage. Every
fixture is a test case; the validator iterates `fixtures/` and reports
on each. A pull request that adds one valid and one invalid fixture
per category invariant strengthens the suite for every implementer.

---

## 9. Summary

- **Compliance is two conditions**: validator exits `0` *and* policy
  observance. Both required, neither sufficient.
- **The validator is honest**: it will not paper over real divergence
  to make exit codes look good.
- **The canonical reference run currently exits `6`**: this is
  expected v1 baseline; documented in `exemptions.yaml`; will converge
  as the in-flight reconciliation work lands.
- **The validator is content-driven**: it silently skips non-RWP
  files; it does not invent failures.
- **Both source and verdict are auditable**: read the code, contest a
  finding, submit a fixture.

The mark-use gate is not adversarial. It exists to ensure that anyone
who reads a "Rhumb-compliant" claim can independently verify it, and
that the claim means the same thing in every context it appears.

---

Produced:
  - by: YAKKL® Meridian™— https://meridian.yakkl.com
  - for: the Rhumb Workflow Protocol™ (RWP™) — https://rhumbprotocol.dev
  - copyright: Copyright © 2026 YAKKL® Inc. All Rights Reserved.
