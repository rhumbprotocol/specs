# Rhumb Implementation Profiles

This document separates the **core protocol contract** from **recommended file-tree
profiles**. The distinction matters because RWP is a portable artifact protocol,
not a mandate that every implementation must copy Meridian's private directory
layout.

## 1. Core Rule

An RWP implementation is conformant because its artifacts preserve the protocol
semantics, not because it stores them in one exact directory tree.

Core RWP defines:

- artifact formats: `INTAKE.yaml`, `PLAN.md`, `state.yaml`, `manifest.yaml`,
  `dependencies.yaml`, handoff documents, IDEA, AVD, ACS, prompts, and audits;
- identifier formats: plans, phases, sub-phases, audits, handoffs, and UUIDs;
- lifecycle semantics: how artifacts move from capture through planning,
  execution, handoff, audit, completion, parking, or discard;
- validation rules: JSON Schemas, sequence grammar, conformance categories, and
  cross-artifact invariants;
- extension rules: namespaced additions that do not break portability.

Core RWP does **not** require:

- `.meridian/` as the root directory;
- `.private/` as a privacy boundary;
- SQLite, a daemon, a desktop app, or a CLI runtime;
- a specific repository layout;
- Meridian-specific lifecycle buckets as the only valid storage model.

That means an external tool may use `rwp/`, `.workflow/`, `.ai-work/`,
database-backed storage, object storage, or another layout if it can emit,
read, validate, and preserve the same RWP artifact contract.

## 2. Why Profiles Exist

Directory layout still matters operationally. People need to find artifacts.
Tools need stable discovery rules. CI needs predictable targets. Therefore RWP
defines **implementation profiles**: named storage recommendations that sit
above the core artifact contract.

Profiles are useful when an implementation wants to say:

- where artifacts are expected to live;
- which directories are durable knowledge versus transient runtime state;
- which files are authoritative and which are generated indexes;
- how validators should discover artifacts without scanning unrelated files;
- how an implementation can interoperate with Meridian without adopting all of
  Meridian.

Profiles are advisory unless a tool explicitly claims support for a profile.

## 3. Core File-Tree Profile

The simplest portable profile keeps artifacts in one plan directory. This is
the right starting point for examples, small teams, and non-Meridian tools.

```text
rwp/
  plans/
    MP-0001-example/
      INTAKE.yaml
      PLAN.md
      MASTERPLAN.yaml
      state.yaml
      manifest.yaml
      dependencies.yaml
      handoffs/
        HO-MP-0001-P-01-2026-05-06.md
      audits/
        AUD-MP-0001-P-03-2026-05-06.md
      prompts/
        START-PROMPT.md
```

This profile is intentionally boring. It demonstrates that the protocol can be
used without Meridian, a database, or a hidden private tree.

## 4. Meridian Reference Profile

Meridian is the reference implementation. Current Meridian code and runtime
state use a split private tree under `.meridian/.private/`:

```text
.meridian/
  MANIFEST.yaml
  MERIDIAN.yaml
  rules/
  .private/
    runtime/
      STATE.yaml
      COMPLETED.yaml
      sequences.yaml
      config.yaml
      locks/
    data/
      meridian.db
    knowledge/
      plans/
        backlog/
        planning/
        processing/
        completed/
        cancelled/
        onhold/
        archived/
      ideas/
      visions/
      components/
      notes/
      captured/
      evidence/
```

The production Meridian path constants live in
`packages/yakkl-meridian-rs/meridian-ops/src/paths.rs`. The important point for
RWP is the split:

| Meridian area | Purpose | RWP meaning |
|---|---|---|
| `.private/knowledge/` | Durable human and AI-authored knowledge | Long-lived protocol artifacts and architecture records |
| `.private/knowledge/plans/<lifecycle>/` | Plan directories organized by lifecycle | RWP plan-level artifacts grouped by execution state |
| `.private/runtime/` | Session/runtime coordination files | Meridian runtime control plane, not the same thing as plan `state.yaml` |
| `.private/data/meridian.db` | SQLite operational index | Meridian implementation detail, not a required RWP artifact |

### 4.1 Plan Directory Shape

Within a Meridian plan directory, RWP-compatible artifacts should be grouped
like this:

```text
.meridian/.private/knowledge/plans/processing/MP-0001-example/
  INTAKE.yaml
  PLAN.md
  MASTERPLAN.yaml
  state.yaml
  manifest.yaml
  dependencies.yaml
  handoffs/
  audits/
  prompts/
  evidence/
```

`state.yaml` here is a **plan execution artifact**. It records the state of one
plan or workflow.

`.meridian/.private/runtime/STATE.yaml` is a **Meridian runtime/session index**.
It may summarize active work, claims, session state, and workspace-level
runtime coordination. It is related to RWP execution, but it is not the same
artifact as plan-level `state.yaml`.

This distinction should be explicit in every Meridian-facing RWP document.
Otherwise readers will confuse protocol state with implementation runtime
state.

### 4.2 Project-Aware Meridian Layout

Meridian also has project-aware path helpers for a newer layout:

```text
.meridian/.private/projects/<project-slug>/
  knowledge/
    plans/
    ideas/
    visions/
    components/
    notes/
  architecture/
  runtime/
```

This does not change core RWP. It means Meridian may use either a legacy
single-project profile or a project-aware profile while still preserving the
same RWP artifacts inside the appropriate knowledge subtree.

## 5. Compatibility Guidance

Use this decision table:

| Question | Protocol answer |
|---|---|
| Must all adopters use `.meridian/.private/knowledge/plans/`? | No. That is Meridian's reference profile, not core RWP. |
| Should Rhumb document the Meridian tree? | Yes. Meridian is the reference implementation and its current layout must be legible. |
| Can a tool store artifacts somewhere else? | Yes, if it preserves artifact formats, identifiers, lifecycle semantics, and validation behavior. |
| Can a tool keep runtime indexes in a database? | Yes. Databases are implementation details if the tool can still emit or validate protocol artifacts. |
| Should Rhumb copy every Meridian internal field? | No. Vendor-specific behavior belongs in a profile or namespaced extension, not core. |
| Should Meridian adopt hardened Rhumb changes? | Yes, when the protocol change is deliberate and compatible with the reference implementation path. |

## 6. Current Reconciliation Findings

The repository previously showed real drift that must not be hidden. RWP 0.26.0
resolves the protocol-side pieces and leaves remaining implementation-profile
work explicit:

1. Rhumb templates are consumed directly by Meridian CLI RWP template loading
   and validation code, so template/schema changes are not cosmetic.
2. Rhumb sequence parsing, schemas, and conformance fixtures now accept
   uppercase sub-phase letters `A-Z`; older `A-C` references are obsolete.
3. RWP status vocabulary is now greenfield and explicit: execution uses
   `planning | in_progress | paused | completed | failed`; phases use
   `pending | in_progress | completed | failed | skipped`.
4. Canonical template names are RWP names (`AVD.md.template`,
   `ACS.md.template`, `HANDOFF.yaml.template`, `AUDIT.md.template`, etc.),
   not the old Meridian-shaped names.
5. Meridian's current private tree uses `.private/knowledge/plans/...`, not the
   older `.private/plans` shorthand. Any Rhumb documentation that recommends a
   Meridian layout should use the current `knowledge` split.

## 7. Recommended Sync Strategy

Do not make Rhumb "whatever Meridian happens to do today." That would turn a
protocol into a product export format.

Do this instead:

1. Keep **Core RWP** vendor-neutral: artifacts, lifecycle, identifiers,
   validation, conformance, and extension rules.
2. Publish **RWP Core File-Tree Profile** for generic examples and new
   adopters.
3. Publish **Meridian Reference Profile** for teams that want the reference
   implementation layout and for validators that need profile-aware discovery.
4. Move reusable Meridian concepts into Rhumb only when they are protocol-level
   concepts, not Meridian runtime mechanics.
5. After a Rhumb schema/template change hardens, update Meridian's bundled and
   filesystem-loaded RWP templates/validators intentionally.

This keeps Rhumb stable enough for external implementers while letting Meridian
continue to serve as the proving ground.

## 8. Profile Conformance Language

Use precise claims:

- "RWP Core conformant" means the implementation preserves core artifact and
  validation semantics.
- "RWP Core File-Tree Profile compatible" means the implementation uses or can
  emit the generic `rwp/plans/...` layout.
- "RWP Meridian Reference Profile compatible" means the implementation can read
  or emit the current `.meridian/.private/{runtime,data,knowledge}` layout
  without confusing runtime `STATE.yaml` with plan-level `state.yaml`.
- "Rhumb-compliant" remains a public claim governed by the validator and
  trademark policy described in `docs/CONFORMANCE.md`.

---

Produced:
  - when: 2026-05-06T16:20:00Z
  - by: YAKKL® Meridian™- https://meridian.yakkl.com
  - copyright: Copyright © 2026 YAKKL Inc. All Rights Reserved.
