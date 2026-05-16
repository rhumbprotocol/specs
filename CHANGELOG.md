# Changelog

All notable changes to the Rhumb Workflow Protocol are documented in this file.

This changelog follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/) conventions, and the protocol uses [Semantic Versioning](https://semver.org/spec/v2.0.0.html) as described in [PROTOCOL.md](./docs/PROTOCOL.md).

While `MAJOR == 0`, the protocol follows the SemVer pre-1.0 convention: MINOR releases (e.g., `0.25.1` → `0.28.0`) may include breaking changes; PATCH releases within a MINOR series remain backward compatible. See [spec/versioning.format.md](./spec/versioning.format.md#pre-10-stability) for the full rules.

---

## [0.28.0] - 2026-05-06

### Changed - Greenfield RWP Shape

- Canonical plan identifiers now use `MP-NNNN-short-name`, for example `MP-0250-setup-examples`.
- Removed greenfield-incompatible status aliases from the state schema. Current workflow status is `planning | in_progress | paused | completed | failed`; phase status is `pending | in_progress | completed | failed | skipped`.
- Added `VERSION` as the protocol version source of truth and `scripts/sync-version.mjs` to sync the conformance crate, website config, and RWP constants.

---

## [0.25.4] - 2026-05-02

### Changed — Trademark Policy

- `TRADEMARK.md` bumped to document version **1.1.0** (was 1.0.0). Reconciles §3, §4, and §7 with the actual federal-filing state of the marks listed in §2.
  - **§3 Trademark Owner**: entity status now stated as "Delaware C Corporation" (closes OQ-P01-04 from MP-0274 P-01). Postal-address wording preserved verbatim from v1.0.0 ("provided on request for formal legal correspondence"); OQ-P01-03 (postal-address publication) **remains open** pending user disposition on whether to publish a non-residential mail-handling address — see private first-use evidence ledger §New Open Questions OQ-P02-05 for context.
  - **§4 Nature of the Claim**: restructured into §4.1 (filed marks) and §4.2 (common-law marks). Two marks ("Rhumb Workflow Protocol", "RWP") are documented as having pending USPTO Section 1(b) intent-to-use applications (serial numbers 99796677 and 99796729, filed 2026-04-30, IC 009, $350 each, signed by Hans Jones, CEO). Three marks (bare "Rhumb", rhumbprotocol.com brand, Rhumb logo / wordmark / visual identity) remain on common-law ™ posture. The disclaimer in 99796677 ("*Workflow Protocol*" disclaimed apart from the mark) is surfaced in §4.1.
  - **§5 First Public Use**: preserved verbatim. Section 1(b) intent-to-use does not establish first-use-in-commerce; the §5 anchor remains pending until MP-0277 public extraction.
  - **§7 ® Trigger Gates (KD-03b)**: KD-03b preserved verbatim. A scope note is prepended clarifying that the trigger framework is partially superseded for the marks in §4.1 (the pending applications now drive their registration timeline) but remains in force for the marks in §4.2.

### Notes

- Per the policy's own §9 update procedure: this change is recorded in this CHANGELOG entry, the document version is bumped in the `TRADEMARK.md` header (1.0.0 → 1.1.0), and the private first-use evidence ledger has been amended to record the filings (two appended rows, plus resolution of the previously open question OQ-P02-01).
- This is a backward-compatible PATCH increment within the 0.25.x MINOR series. The mark-use rules in §6 are unchanged; the §6.2 conformance gate (`rhumb-validate --all` exit 0) is unchanged.
- The `RWP_VERSION` constant in `conformance/src/lib.rs` is **not** updated by this entry (precedent from [0.25.2]: trademark-only amendments do not bump the protocol version constant).
- Resolves the deferred carry-forward OQ-P05-01 from MP-0274 AUDIT-FINAL (2026-04-29), removing the previously-documented gate on MP-0277 public extraction caused by `TRADEMARK.md` §4 stating "No USPTO federal registration has been filed for any of the Rhumb marks" while filings actually existed.

Plan provenance: in-session reconciliation per user-paste START prompt 2026-05-02 action B (no MP). Direct path per `feedback_no_forcing_path_decisions` allowing user override on path selection.

---

## [0.25.3] - 2026-05-02

### Added — IDEA Artifact (Architecture Pipeline Entry)

- **IDEA Template** (`templates/IDEA.md.template`): Foundation template for capturing architecture ideas — the entry point to the RWP architecture pipeline (Idea → Vision → Component → Plan). 24 frontmatter properties with YAML placeholder syntax. SPDX Apache-2.0 licensed.
- **IDEA Schema** (`spec/schemas/idea.schema.json`): JSON Schema (draft 2020-12) validating IDEA frontmatter. 24 properties, 8 required fields, `additionalProperties: false`. First RWP schema on draft 2020-12 with versioned `$id` URI (`https://rhumbprotocol.dev/schemas/v0.25.3/idea.schema.json`). 19 test fixtures (5 valid + 14 invalid) in `spec/schemas/test-fixtures/idea/`.
- **Lifecycle Schema** (`spec/schemas/lifecycle.schema.json`): JSON Schema (draft 2020-12) validating artifact lifecycle state-machine shape. Reuses the status enum from `idea.schema.json` via `$ref` at 5 sites (single source of truth). 8 test fixtures (1 valid + 7 invalid) in `spec/schemas/test-fixtures/lifecycle/`.
- **Lifecycle Specification** (`spec/lifecycle/idea-lifecycle.spec.md`): Normative specification for the Idea artifact lifecycle — 5 states (captured → approved → promoted / parked / discarded), transition rules, per-state required fields, and three-level validation layering (Level 1: schema, Level 2: cross-field, Level 3: `rhumb-validate` semantic checks).
- **Worked Examples** (`examples/ideas/`): 5 complete Idea artifacts demonstrating each lifecycle state (captured, approved, promoted, parked, discarded) with realistic frontmatter and body content.
- **Anti-Fixtures** (`examples/ideas/anti-fixtures/`): 5 intentionally invalid Idea artifacts demonstrating common failure modes — unquoted timestamps, invalid status values, additional property smuggling, missing per-state fields, and invalid pipeline values. Each includes an `## Expected Validation Result` section documenting the specific error.

### Notes

- **Timestamp quoting (adopter guidance)**: Always quote ISO-8601 timestamp values in YAML frontmatter (e.g., `created: "2026-05-02T15:00:00Z"`). Unquoted timestamps are auto-cast to Date objects by YAML 1.1 parsers and will fail schema validation. See `examples/ideas/anti-fixtures/unquoted-timestamp.md` for a worked example.
- **Forward reference**: The Methodology Guidebook (ACS-0042) will reference these artifacts as the canonical entry point for architecture pipeline adoption guidance.
- `RWP_VERSION` constant in `conformance/src/lib.rs` updated from `0.25.1` to `0.25.3`.
- This is a backward-compatible PATCH increment within the 0.25.x MINOR series per [spec/versioning.format.md](./spec/versioning.format.md#pre-10-stability).

Plan provenance: MP-0288 (AVD-0009 → ACS-0038).

---

## [0.25.2] - 2026-04-29

### Added — Trademark Policy

- `TRADEMARK.md` (new) — common-law ™ trademark policy for "Rhumb"™, "Rhumb Workflow Protocol"™, and "RWP"™ per MP-0274. Defines mark-use conditions, the §6.2 conformance gate (`rhumb-validate --all` exits 0 — forward reference to MP-0276), and the §7 KD-03b trigger gates that would convert the ™ posture to a USPTO ® filing.
- First-use evidentiary record maintained privately by YAKKL, Inc. (request access via `legal@yakkl.com`); not republished publicly.
- README "Trademarks" section added (link to `TRADEMARK.md`; `legal@yakkl.com` contact; cites Delaware C Corp ownership).

### Notes

- `TRADEMARK.md` is the canonical mark-use authority going forward. The interim `TRADEMARK-POLICY.md` cited in the [0.25.1] block remains in place pending disposition at MP-0274 AUDIT-FINAL (open question OQ-P01-01).

Plan provenance: MP-0274 (ACS-0015 Part A).

---

## [0.25.1] - 2026-04-27

Initial published release of the Rhumb Workflow Protocol.

### Added - Core Protocol

- **5 artifact types**: Plan, Intake, Manifest, State, Handoff
- **Lifecycle state machine** with phase and plan states
- **Sub-phase notation** (`P-XX-A/B/C`) for crash resilience
- **Audit checkpoint system** for scheduled and ad-hoc audits
- **Extension mechanism** for domain-specific custom fields
- **Conformance levels**: Minimal, Standard, Advanced
- **Protocol versioning** with SemVer 2.0.0 rules and pre-1.0 stability semantics

### Added - Format Specifications (`spec/`)

- 5 JSON schemas for artifact validation (plan, intake, manifest, state, handoff)
- UUID format specification and generation guidance
- Phase sequence grammar (ABNF)
- Sequence parser specification with reference implementation
- Version embedding format and pre-1.0 stability rules
- Custom field patterns and schema composition guidance
- Conformance-level field-by-field reference
- OpenAPI integration patterns
- Implementation best practices guide

### Added - Foundation Templates (`templates/`)

- 6 core templates: Plan, Intake, Masterplan, State, Dependencies, Manifest
- 6 display/prompt templates: Draft, Commit, Phase-Complete, Handoff-Complete, Start-Prompt, Prompt
- 2 architecture templates: AVD (Architecture Vision Document), ACS (Architecture Component Spec)
- 2 reference templates: Handoff, Phase-Audit
- Sequence configuration template (13 sequence types)
- All templates advisory-only with placeholder syntax

### Added - Integration Adapters (`integrations/`)

- Claude Code CLI adapter (commands, project instructions, manifest)
- OpenAI Codex adapter (skills, rules)
- Google Gemini CLI adapter (commands)
- Claude.ai browser knowledge guide
- ChatGPT browser instructions
- Gemini web browser context document

### Added - Reference Implementations (`util/`)

- TypeScript UUID generator reference
- TypeScript sequence parser reference
- Schema validation test suite

### Added - Documentation (`docs/`)

- Getting Started guide with walkthrough examples
- Extensions guide for custom fields, artifacts, and integrations
- FAQ across multiple categories
- Protocol specification (`PROTOCOL.md`) - the canonical normative reference
- Branching model (`BRANCHING.md`) - git-flow used in this repository
- Repository README with full structure and AI-tools table
- Contribution guidelines (`CONTRIBUTING.md`) with style guide and PR process
- Trademark policy (`TRADEMARK-POLICY.md`)

### Added - Examples (`examples/`)

- Simple feature workflow (2-phase, single developer, REQUIRED-only)
- Multi-phase project workflow (5-phase with sub-phases, multi-team)
- Bug-fix workflow

### Added - Governance (`committee/`)

- Committee charter (`CHARTER.md`)
- Governance document (`GOVERNANCE.md`) - lazy consensus, formal review, AEP process
- Release governance (`RELEASES.md`) - version authority and release process
- Directory scaffolding for meeting minutes and enhancement proposals

### Naming History

This protocol was previously developed internally under the name *Azimuth Workflow Protocol* (AWP). Prior to public release it was renamed to *Rhumb Workflow Protocol* (RWP). The rename retains the nautical-navigation semantic (a rhumb line is a constant-bearing path) while resolving namespace conflicts with unrelated GitHub identifiers using "azimuth" variants.

Internal artifacts using AWP tokens (`awp_uuid`, `awp_version`, `AwpUuid`/`AwpVersion` types, `CLAUDE-AWP.md` and similar filenames, `azimuthprotocol` package paths) were renamed to their RWP equivalents. Schema `$id` URIs were updated to `https://rhumbprotocol.dev/schemas/...`.

---

## Version Policy

RWP uses Semantic Versioning 2.0.0 with one nuance during the pre-1.0 phase. While `MAJOR == 0`:

- **MINOR** (`0.25.1` → `0.28.0`): may include breaking changes
- **PATCH** (`0.25.1` → `0.25.2`): always backward compatible within a MINOR series

After v1.0 ships, standard SemVer rules apply:

- **MAJOR** (`1.0.0` → `2.0.0`): breaking changes
- **MINOR** (`1.0.0` → `1.1.0`): backward-compatible features
- **PATCH** (`1.0.0` → `1.0.1`): backward-compatible fixes

The `rwp_version` field in artifacts indicates which protocol version they conform to. See [spec/versioning.format.md](./spec/versioning.format.md) for the full versioning rules, compatibility matrix, and validation strategy.

---

Rhumb Workflow Protocol™ (RWP™)
https://rhumbprotocol.dev
