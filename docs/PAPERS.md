# Rhumb Protocol Papers

This document defines the first public paper set for Rhumb Workflow Protocol.
The PDFs are generated distribution artifacts. The maintained sources are this
repository's Markdown specifications, schemas, templates, and paper outlines.

## Paper Set

| Paper | Audience | Purpose | PDF Output |
|---|---|---|---|
| Rhumb Workflow Protocol Executive Brief | Executives, product leaders, technical leads | Explain why the protocol exists, how it reduces lock-in, and how Meridian fits as reference implementation | `executive-brief.pdf` |
| Rhumb Workflow Protocol Implementation Brief | Tool builders, platform engineers, AI workflow implementers | Explain artifacts, conformance levels, lifecycle, and extension rules | `implementation-brief.pdf` |
| Rhumb Workflow Protocol Specification Bundle | Implementers, architects, standards reviewers | Give a guided reading order through the canonical protocol materials | `specification-bundle.pdf` |

## Executive Brief Outline

### Position

Rhumb is the protocol layer, not another project tool. It defines portable
workflow artifacts for AI-assisted delivery while leaving implementation choices
to tools and teams.

### Problem

AI work loses shape when the durable record lives only inside a tool. Context
disappears between sessions, work is hard to audit, and proprietary project
formats create lock-in.

### Adoption

Teams can start with intake, plan, state, and handoff artifacts, then add
schemas, lifecycle, audits, UUIDs, and validators as workflows become more
complex.

### Outcome

The business value is portability and evidence: what was requested, what
changed, what evidence exists, and which tool or agent handled each stage.

## Implementation Brief Outline

### Artifact Contract

RWP centers on `INTAKE.yaml`, `PLAN.md`, `state.yaml`, `HO-*.md`, and
`manifest.yaml`.

### Validation

Conformance levels let implementations declare depth honestly: Minimal,
Standard, or Full.

### Lifecycle

IDEA artifacts move through `captured`, `refining`, `ready`, `approved`,
`promoted`, `parked`, and `discarded`. Frontmatter status is authoritative.

### Extensions

Implementation-specific fields preserve readability and keep custom behavior
visibly namespaced.

## Specification Bundle Outline

### Reading Order

1. `docs/GETTING-STARTED.md`
2. `docs/PROTOCOL.md`
3. `spec/conformance-levels.md`
4. `spec/lifecycle/idea-lifecycle.spec.md`
5. `templates/` and `examples/`

### Architecture Path

The architecture path is `IDEA -> AVD -> ACS -> MP`.

### Portability

Claude Code, Codex, Gemini CLI, browser assistants, and Meridian all consume
Rhumb artifacts. No adapter owns the protocol.

### PDF Policy

PDFs exist for distribution and review. Markdown specs, schemas, templates, and
website content remain the maintained sources.

---

Produced:
  - when: 2026-05-06T14:55:00Z
  - by: YAKKL® Meridian™- https://meridian.yakkl.com
  - copyright: Copyright © 2026 YAKKL Inc. All Rights Reserved.
