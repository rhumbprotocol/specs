# Rhumb Protocol Diagram Guide

This guide defines the public diagram set for Rhumb Workflow Protocol documentation.
The diagrams are explanatory views over the canonical artifacts, schemas, templates,
and specifications in this repository.

## Diagram Set

| Diagram | Purpose | Source Material |
|---|---|---|
| Artifact constellation | Shows the five core files around the RWP contract | `docs/GETTING-STARTED.md`, `docs/PROTOCOL.md`, `templates/`, `spec/schemas/` |
| IDEA lifecycle state machine | Shows concept state transitions from capture through promotion, parking, or discard | `spec/lifecycle/idea-lifecycle.spec.md`, `spec/schemas/idea.schema.json`, `spec/schemas/lifecycle.schema.json` |
| Architecture path | Shows `IDEA -> AVD -> ACS -> MP` as the path from concept to executable work | `templates/IDEA.md.template`, `templates/AVD.md.template`, `templates/ACS.md.template`, `templates/PLAN.md.template` |
| Conformance ladder | Shows Minimal, Standard, and Full adoption depth | `spec/conformance-levels.md`, `conformance/` |
| Implementation profiles | Shows Core File-Tree Profile versus Meridian Reference Profile | `docs/IMPLEMENTATION-PROFILES.md`, Meridian `meridian-ops/src/paths.rs` |
| Portability map | Shows adapters as consumers of the protocol, not owners of it | `integrations/`, `docs/PROTOCOL.md` |

## Artifact Constellation

The artifact constellation places `RWP` at the center and arranges the five
core artifacts clockwise:

| Order | Artifact | Role |
|---:|---|---|
| 01 | `INTAKE.yaml` | Captures problem, constraints, requirements, and success criteria |
| 02 | `PLAN.md` | Defines phases, deliverables, dependencies, risks, and verification |
| 03 | `state.yaml` | Records current execution status, phase progress, and recovery context |
| 04 | `HO-*.md` | Preserves transition context between phases, people, agents, and sessions |
| 05 | `manifest.yaml` | Registers produced artifacts, prompts, handoffs, audits, and outputs |

The order is clockwise so readers can scan the workflow from request capture to
produced evidence.

## IDEA Lifecycle

The IDEA lifecycle diagram uses the state names from
`idea.schema.json#/properties/status`. The forward path is:

```text
captured -> refining -> ready -> approved -> promoted
```

`parked` is a holding state with re-entry to active states. `promoted` and
`discarded` are terminal states. The frontmatter `status` field is authoritative;
file location is only a navigation aid.

## Architecture Path

The architecture path is:

```text
IDEA -> AVD -> ACS -> MP
```

This path lets teams start with a rough concept, promote it into architecture
vision, specify component contracts, and execute work through a managed plan.

## Conformance Ladder

The conformance ladder presents adoption depth without overstating support:

| Level | Meaning |
|---|---|
| Minimal | Plan and state continuity |
| Standard | Intake, manifest, handoff, and sub-phase support |
| Full | Audit, UUID, lifecycle, schema, and validator-backed workflow support |

## Implementation Profiles

The implementation profile diagram must show that directory layout is not core
RWP:

| Profile | Diagram Emphasis |
|---|---|
| Core File-Tree | `rwp/plans/<plan-id>/` with all plan artifacts together |
| Meridian Reference | `.meridian/.private/{runtime,data,knowledge}` split, with plan artifacts under `knowledge/plans/<lifecycle>/` |
| Custom | Any layout that preserves artifact semantics and validation behavior |

The Meridian profile view must label plan-level `state.yaml` separately from
workspace-level `.private/runtime/STATE.yaml`.

## Portability Map

The portability map puts Rhumb at the center and shows adapters around it:

| Adapter | Role |
|---|---|
| Claude Code | CLI adapter and command context |
| OpenAI Codex | Plan skill and safety rules |
| Gemini CLI | RWP planning command |
| Browser AI | Pasteable project knowledge and instructions |
| Meridian | Reference implementation |

Adapters consume Rhumb artifacts. They do not own the protocol.

---

Produced:
  - when: 2026-05-06T14:55:00Z
  - by: YAKKL® Meridian™- https://meridian.yakkl.com
  - copyright: Copyright © 2026 YAKKL Inc. All Rights Reserved.
