<!--
SPDX-License-Identifier: Apache-2.0
Copyright (c) 2026 Rhumb Protocol Contributors

RWP version: 0.28.0
RWP IDEA Lifecycle Specification — Normative prose specification for the
seven-state IDEA lifecycle and its state machine.

DESIGN PROVENANCE
This specification inherits from AVD-0009 KD-09-02 (Open/Closed Split Table
OPEN row: Artifacts + Schemas + State Machines ship under Apache 2.0) and
from ACS-0038 KD-38.1..7. The seven-state enum, transition graph, and
per-state required-fields metadata are the binding contract of ACS-0038
data-model.yaml#lifecycle_state_machine.

PRECEDENCE RULE (KD-38.2 — frontmatter wins on conflict)
When `status:` (frontmatter) and file location disagree (e.g., status=approved
but file in ideas/captured/), tooling MUST trust frontmatter. Lifecycle-by-
location stays advisory for human navigation; rhumb-validate Level-3 gating
consumes frontmatter only.

CITATION POLICY
The seven-state enum is defined exactly once, in idea.schema.json under the
JSON Pointer #/properties/status (its `enum` array). This specification cites
that location as the single source of truth and does not redeclare the enum
as a normative list. Where a table or diagram below mentions a state name,
the name is rendered for human readability; the authoritative enum lives in
idea.schema.json.
-->

# RWP IDEA Lifecycle Specification

**Schema version**: v0.28.0
**Schema $id**: `https://rhumbprotocol.dev/schemas/v0.28.0/lifecycle.schema.json`
**Status enum source of truth**: [`idea.schema.json#/properties/status`](../schemas/idea.schema.json)
**Lifecycle schema**: [`lifecycle.schema.json`](../schemas/lifecycle.schema.json)
**Component spec**: ACS-0038 (`.meridian/.private/knowledge/components/ACS-0038-rwp-idea-template-and-lifecycle/`)

---

## 1. Overview

The Rhumb Workflow Protocol (RWP) defines a seven-state lifecycle for IDEA artifacts. Every IDEA — from a freshly captured concept to a promoted, parked, or discarded outcome — moves through this lifecycle by transitioning between states, with each transition recorded in the IDEA's frontmatter.

This specification is normative for the state machine, transition graph, per-state required-fields metadata, and precedence rule. It is non-normative for directory layout (per KD-38.7, parked-IDEA organization is implementation-specific) and for runtime field-validator implementation (per KD-38.6, that is rhumb-validate Level-3 territory in ACS-0041).

### 1.1 The seven states

The seven lifecycle states are defined exactly once, by the `enum` array at [`idea.schema.json#/properties/status`](../schemas/idea.schema.json). For human readability, the states are: `captured`, `refining`, `ready`, `approved`, `promoted`, `parked`, `discarded`. The order in the enum is significant for tooling that sorts states for display; it is not significant for state-machine semantics (transitions are governed by the graph, not by enum order).

### 1.2 Validation layering

| Level | Concern | Mechanism |
|-------|---------|-----------|
| Level 1 | IDEA frontmatter shape (presence of unconditional required fields, type/format of each field, enum membership) | [`idea.schema.json`](../schemas/idea.schema.json) |
| Level 2 | Lifecycle state-machine shape (states list, initial state, transition graph, optional metadata) | [`lifecycle.schema.json`](../schemas/lifecycle.schema.json) |
| Level 3 | Cross-reference validation (initial_state ∈ states; transition targets ∈ states; per-state required fields populated when an IDEA is in a given state) | rhumb-validate (ACS-0041) |

The split is deliberate. JSON Schema natively expresses presence and type, and (via `$ref`) per-field enum membership. It does not natively express conditional required-by-state (without `if/then/else`, which KD-38.6 explicitly rejects) or cross-array consistency. Those checks live in Level 3.

---

## 2. Precedence Rule (KD-38.2)

**When the `status:` field in an IDEA's frontmatter and the IDEA's file location disagree, tooling MUST trust frontmatter.**

This rule resolves the ambiguity that arises when an IDEA's lifecycle-by-location organization (e.g., `ideas/captured/`, `ideas/promoted/`) lags behind the frontmatter `status:` field. Frontmatter is updated atomically with the lifecycle transition; file moves are not atomic with edits and may lag during force-push recovery, rebase, or human-driven housekeeping.

| Source of truth | When used | Consumed by |
|-----------------|-----------|-------------|
| Frontmatter `status:` | Always authoritative | rhumb-validate Level-3, all tooling |
| File location (e.g., `ideas/captured/`, `ideas/promoted/`) | Advisory only — for human navigation | Humans; non-validator tooling |

**Implementation requirement**: rhumb-validate Level-3 consumes frontmatter `status:` only. It MUST NOT infer state from path, and MUST NOT report a "lifecycle inconsistency" diagnostic when frontmatter and location disagree. (Implementations MAY emit a low-severity hint distinct from a validation error.)

The precedence rule is non-normative for JSON Schema (which cannot express location-versus-frontmatter conflict resolution). It is normative for this prose specification and for rhumb-validate Level-3 behavior.

---

## 3. State Machine

### 3.1 Diagram

The forward flow `captured → refining → ready → approved → promoted` is the primary trajectory. Three sideways exits exist from each active state: to `parked` (pause), to `discarded` (reject), or terminal at `promoted` (success). The `parked` state alone admits re-entry into active states; both `promoted` and `discarded` are terminal.

```
   ┌──────────────┐    ┌──────────────┐    ┌──────────────┐    ┌──────────────┐    ┌──────────────┐
   │   captured  │───►│   refining   │───►│    ready     │───►│   approved   │───►│   promoted   │
   │  (initial)  │    │              │    │              │    │              │    │  (terminal)  │
   └──────┬───────┘    └──────┬───────┘    └──────┬───────┘    └──────┬───────┘    └──────────────┘
         │                   │                   │                   │
         │ park              │ park              │ park              │
         │                   │                   │                   │
         ▼                   ▼                   ▼                   │
   ┌──────────────────────────────────────────────────────┐           │
   │                       parked                        │           │
   │            (re-entry → refining, ready, approved)   │           │
   └──────┬───────────────────────────────────────────────┘           │
          │                                                           │
          │ discard                                                   │ discard
          │                                                           │
          ▼                                                           ▼
   ┌──────────────────────────────────────────────────────────────────────────────────────────────┐
   │                                       discarded                                             │
   │                                       (terminal)                                            │
   └──────────────────────────────────────────────────────────────────────────────────────────────┘

   discard transitions also exist from: captured, refining, ready (omitted from diagram for clarity).
   re-entry from parked: parked → refining, parked → ready, parked → approved (NOT parked → captured).
```

### 3.2 Initial state

A newly captured IDEA enters the lifecycle in the `captured` state. This is the only valid entry point. Implementations MUST NOT seed an IDEA in any other state.

### 3.3 Terminal states

Two states are terminal — once an IDEA enters them, it has no outgoing transitions:

- **`promoted`**: The IDEA has been promoted into a downstream artifact (AVD, MP, or one-off execution per the `pipeline` field). Downstream work continues in the new artifact; the IDEA itself does not transition further.
- **`discarded`**: The IDEA has been rejected. It is preserved for provenance and audit, but has no path back to any active state.

Terminal does not mean immutable: the `updated`, `discarded_reason`, or other documentation fields may continue to be edited (e.g., to refine the rationale on a discarded IDEA). Terminal means no further state transitions.

### 3.4 Re-entry from `parked`

The `parked` state allows re-entry into three active states: `refining`, `ready`, and `approved`. A `parked` IDEA does NOT re-enter `captured` — once an IDEA has progressed past initial capture, returning to `captured` is not meaningful (the capture event already happened; re-parking and re-activating returns the IDEA to wherever its substantive work sits).

Re-entry from `parked` is the single deviation from a strictly forward-flowing lifecycle. All other transitions move forward (toward `promoted`) or sideways (toward `parked` or `discarded`).

---

## 4. Transitions

| From | Allowed targets | Description |
|------|-----------------|-------------|
| `captured` | `refining`, `parked`, `discarded` | Initial state. Forward to `refining` when work begins; sideways to `parked` for later; to `discarded` if rejected at capture. |
| `refining` | `ready`, `parked`, `discarded` | The IDEA is being shaped — scope, problem statement, constraints. Forward to `ready` when refinement is complete; sideways to `parked` or `discarded`. |
| `ready` | `approved`, `parked`, `discarded` | The IDEA is fully shaped and awaits approval. Forward to `approved` on the approval ceremony (per `approval_policy`); sideways to `parked` or `discarded`. |
| `approved` | `promoted`, `discarded` | The IDEA has been approved for downstream work. Forward to `promoted` once the downstream artifact (AVD, MP, or one-off) is created; to `discarded` only as a late reversal (rare). Note: `approved` does NOT transition to `parked`; the only sideways exit from `approved` is `discarded`. |
| `promoted` | (terminal) | The IDEA has been promoted. No further transitions. |
| `parked` | `refining`, `ready`, `approved`, `discarded` | The IDEA is held. May re-enter any active state (forward in the lifecycle), or move sideways to `discarded`. May not re-enter `captured`. |
| `discarded` | (terminal) | The IDEA has been rejected. No further transitions. |

The transition graph is encoded in lifecycle.schema.json under `transitions`. Cross-array consistency (every transition source is a valid state, every transition target is a valid state) is enforced by Level-3 rhumb-validate, with enum-membership checks performed at Level 2 via the schema's `$ref` to the canonical state enum.

---

## 5. Per-State Required Fields (KD-38.6)

The IDEA frontmatter declares lifecycle metadata fields (e.g., `approved_by`, `promoted_to`, `parked_reason`, `discarded_reason`) as **sparse-optional** in [`idea.schema.json`](../schemas/idea.schema.json). They are not in the schema's unconditional `required[]` array because JSON Schema's `required[]` cannot conditionally require a field based on the value of another field, and KD-38.6 explicitly rejects `if/then/else` for this purpose (uneven 2020-12 tooling support, plus the validation responsibility belongs at a higher abstraction level).

Instead, this specification declares the per-state required-fields contract. A Level-3 validator (rhumb-validate, ACS-0041) enforces the contract by reading the IDEA's frontmatter `status:` field and checking that the corresponding required fields below are populated (non-null and non-empty).

| State | Required frontmatter fields | Notes |
|-------|----------------------------|-------|
| `captured` | (none) | Capture is intentionally low-friction. The IDEA's `id`, `title`, `status`, `classification`, `created`, `updated`, `authors` are required by Level 1 (idea.schema.json's unconditional `required[]`); no additional fields are required by Level 3 in this state. |
| `refining` | (none) | Same as `captured`. |
| `ready` | (none) | Same as `captured`. |
| `approved` | `approved_by`, `approved_at`, `approval_policy` | Approval ceremony attribution. `approval_policy` records whether the approval was solo, reviewed, or implementation-specific. |
| `promoted` | `promoted_to`, `promoted_at`, `promoted_pipeline` | Downstream artifact identifier (e.g., AVD-0123, MP-0456-example-plan), promotion timestamp, and the pipeline path actually taken. `promoted_pipeline` uses the same enum as `pipeline`. |
| `parked` | `parked_as`, `parked_reason` | The free-string `parked_as` label (e.g., `roadmap`, `backlog`, or implementation-specific) and the reason. `parked_until` is optional even when parked — null means open-ended parking. |
| `discarded` | `discarded_by`, `discarded_at`, `discarded_reason` | Discarder attribution and rationale. The `discarded_reason` field is load-bearing — discard without rationale is not a valid state. |

This per-state metadata is also encoded as data in `lifecycle.schema.json` under the optional `required_fields_per_state` property. The schema validates the *shape* of that metadata; this prose spec is the *meaning*.

### 5.1 Field-lifetime semantics (open)

ACS-0038 does not normatively define whether lifecycle metadata fields are cumulative (i.e., populated values persist as provenance through subsequent state transitions) or current-state-only (i.e., values are cleared when the IDEA leaves the state that populated them). Implementations MAY treat populated lifecycle fields as carry-forward provenance, and the P-03 fixture [`valid-status-promoted.json`](../schemas/test-fixtures/idea/valid-status-promoted.json) demonstrates that interpretation — but this is non-normative. A future revision of ACS-0038 may settle the question; until then, both interpretations are conformant.

---

## 6. Examples

The five valid IDEA fixtures published in P-03 each demonstrate a distinct lifecycle state. They are normative examples for IDEA-instantiation tooling and for adopters writing their first IDEAs.

| Fixture | State | Demonstrates |
|---------|-------|--------------|
| [`valid-baseline.json`](../schemas/test-fixtures/idea/valid-baseline.json) | `captured` | Minimum-viable IDEA: required fields plus minimal optional set. The shape every IDEA starts in. |
| [`valid-status-approved.json`](../schemas/test-fixtures/idea/valid-status-approved.json) | `approved` | `approved_by`, `approved_at`, `approval_policy` populated per Section 5. |
| [`valid-status-promoted.json`](../schemas/test-fixtures/idea/valid-status-promoted.json) | `promoted` | `promoted_to`, `promoted_at`, `promoted_pipeline` populated; `approved_*` fields carried forward (Section 5.1). |
| [`valid-status-parked.json`](../schemas/test-fixtures/idea/valid-status-parked.json) | `parked` | `parked_as` (set to the free-string label `roadmap`), `parked_reason`, and `parked_until` (ISO-8601). |
| [`valid-status-discarded.json`](../schemas/test-fixtures/idea/valid-status-discarded.json) | `discarded` | `discarded_by`, `discarded_at`, `discarded_reason` populated. Terminal. |

Each fixture validates against [`idea.schema.json`](../schemas/idea.schema.json) at Level 1. Each fixture's lifecycle metadata satisfies the per-state required-fields contract in Section 5 at Level 3.

---

## 7. Implementer Notes

### 7.1 Initial seeding

When a tool creates a new IDEA from the [IDEA template](../../templates/IDEA.md.template), it MUST set `status: captured` and populate the unconditional required fields per [`idea.schema.json#/required`](../schemas/idea.schema.json) (`id`, `type`, `title`, `status`, `classification`, `created`, `updated`, `authors`). All other fields are sparse and SHOULD be omitted at capture time.

### 7.2 State transition recording

A state transition is recorded by:

1. Updating the `status:` field in frontmatter to the new state.
2. Bumping the `updated:` timestamp to the current ISO-8601 time.
3. Populating the per-state required fields per Section 5 of this spec.
4. (Optional, advisory) Moving the IDEA file to the corresponding directory if the implementation organizes IDEAs lifecycle-by-location.

The frontmatter update (steps 1-3) is the authoritative transition record per KD-38.2. Step 4 is advisory and may lag.

### 7.3 Conformance levels

An RWP implementation MAY conform at Level 1 only (idea.schema.json), Level 2 (idea.schema.json + lifecycle.schema.json), or Level 3 (rhumb-validate or equivalent). This specification is normative for all three levels with respect to the data shape and semantics; the runtime enforcement is the implementation's responsibility.

### 7.4 ISO-8601 timestamp policy

All lifecycle timestamp fields (`created`, `updated`, `approved_at`, `promoted_at`, `parked_until`, `discarded_at`) MUST be ISO-8601 strings (RFC 3339 profile). Legacy quarter-shorthand (e.g., `2026-Q3`) is not accepted. Implementations using YAML 1.1 parsers MUST quote timestamp values in frontmatter to prevent automatic Date typecasting; an unquoted ISO-8601 value is parsed as a JavaScript Date in js-yaml and other YAML 1.1 implementations, which fails the schema's `type: "string"` constraint.

---

## 8. References

- [`idea.schema.json`](../schemas/idea.schema.json) — IDEA frontmatter shape; canonical source for the seven-state enum.
- [`lifecycle.schema.json`](../schemas/lifecycle.schema.json) — lifecycle state-machine shape; consumes the seven-state enum via `$ref`.
- [IDEA template](../../templates/IDEA.md.template) — paste-and-go template for new IDEAs.
- ACS-0038 — RWP IDEA Template + Lifecycle component specification (`.meridian/.private/knowledge/components/ACS-0038-rwp-idea-template-and-lifecycle/`).
- AVD-0009 — Rhumb Workflow Protocol architecture vision (`.meridian/.private/knowledge/visions/AVD-0009-rwp/`).

---

Produced:
  - when: 2026-05-01T19:55:00Z
  - by: Rhumb Protocol™ Contributors - https://rhumbprotocol.dev
  - copyright: Copyright © 2026 Rhumb Protocol Contributors. All Rights Reserved.
