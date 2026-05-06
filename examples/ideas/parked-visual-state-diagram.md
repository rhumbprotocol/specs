---
id: IDEA-9004
type: idea
title: "Interactive visual state diagram for lifecycle documentation"
status: parked
classification: public
created: "2026-03-22T13:00:00Z"
updated: "2026-04-10T09:30:00Z"

authors:
  - name: "Aisha Okafor"
    role: "engineer"
tags:
  - documentation
  - visualization
  - lifecycle

parent: null
children: []

pipeline: null

approved_by: null
approved_at: null
approval_policy: null

promoted_to: null
promoted_at: null
promoted_pipeline: null

parked_as: "roadmap"
parked_reason: "Depends on the lifecycle spec being finalized (MP-0288). Parking until the 7-state enum and transition table are stable; building a visualization against a moving target wastes effort."
parked_until: "2026-06-01T00:00:00Z"

discarded_by: null
discarded_at: null
discarded_reason: null
---

# IDEA-9004: Interactive visual state diagram for lifecycle documentation

## Context

The lifecycle state machine (7 states, ~15 transitions) is currently documented
as an ASCII diagram in `idea-lifecycle.spec.md`. While accurate, ASCII diagrams
are hard to read for visual thinkers and impossible to click-through for
exploration. An interactive SVG or HTML diagram would let adopters hover over
states to see required fields, click transitions to see rules, and zoom into
subgraphs.

## Initial Considerations

- Generate from lifecycle.schema.json so it stays in sync automatically
- Render as static SVG for docs site (no JavaScript dependency for basic view)
- Optional interactive layer (hover tooltips, click-to-expand) via lightweight JS
- Must degrade gracefully to the ASCII version in terminal/plain-text contexts
- Consider Mermaid as the authoring format — widely supported, renderable in
  GitHub, exportable to SVG

## The "Why"

Visual documentation dramatically reduces onboarding time for new adopters.
The lifecycle state machine is the conceptual core of RWP; if adopters don't
internalize it, they'll mis-author IDEAs and fight the validator.

## Strategic Value

A polished interactive diagram signals production-quality documentation. It's
the kind of artifact that gets shared in "check out this protocol" messages.

## Key Concepts / Pillars

1. **Generated, not hand-drawn**: Source of truth remains lifecycle.schema.json;
   the diagram is a view, not a second source.
2. **Progressive disclosure**: Basic SVG first, interactive layer second.

## Target Audience

New adopters reading the documentation site; team leads evaluating RWP.

## Proposed Execution (High Level)

- Write a generator that reads lifecycle.schema.json and emits Mermaid syntax
- Render Mermaid to SVG via `mmdc` CLI
- Embed in docs site with optional JS tooltip layer
- Add to CI so diagram regenerates on schema change

## Open Questions / Unknowns

* [ ] Mermaid vs. D3.js vs. custom SVG generation?
* [ ] Where does the interactive version live — docs site only, or also in-repo?

## References

- idea-lifecycle.spec.md §3 (State Machine)
- lifecycle.schema.json

---
Tags: [idea]

---

Produced: "2026-04-10T09:30:00Z"
By: Rhumb Protocol Contributors - https://rhumbprotocol.dev
Copyright: Copyright (c) 2026 Rhumb Protocol Contributors. All Rights Reserved.
