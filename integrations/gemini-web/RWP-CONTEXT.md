# Rhumb Workflow Protocol™ (RWP™) - Context for Gemini

> Rhumb Workflow Protocol (RWP) - https://rhumbprotocol.dev
> Full protocol specification: `docs/PROTOCOL.md` in the RWP repository | Reference implementation: YAKKL Meridian (https://meridian.yakkl.com)

This document provides RWP context for use in Google Gemini web sessions. Paste this at the start of a conversation or add it as a Gem's instructions to enable RWP-aware structured planning.

## RWP Overview

The Rhumb Workflow Protocol (RWP) is an open protocol for structured AI workflow management. It provides standardized artifacts, lifecycle conventions, and continuity mechanisms for planning and executing complex work with AI assistance.

RWP is:
- **Open**: Apache-2.0 licensed, tool-agnostic, platform-independent
- **Structured**: Plans have phases, deliverables, audits, and handoffs
- **Portable**: Works with any AI assistant, language, or project type
- **Continuity-focused**: Handoff documents bridge sessions and tools

**Specification**: https://rhumbprotocol.dev
**Reference Implementation**: YAKKL® Meridian™ (https://meridian.yakkl.com)

## Plan Lifecycle

RWP plans follow a three-stage lifecycle:

```
planning  →  processing  →  completed
```

During **processing**, work progresses through phases. Audits occur at scheduled checkpoints to verify quality and completeness.

## Conventions

### Identifiers

| Element | Format | Example |
|---------|--------|---------|
| Plan ID | `MP-{NNNN}-{short-name}` | `MP-0042-dark-mode-toggle` |
| Phase | `P-{NN}` | `P-03` |
| Sub-phase | `P-{NN}-{A/B/C}` | `P-03-B` |
| Audit | `AUD-{NN}` or `FINAL` | `AUD-02` |
| Handoff | `HO-{PLAN_ID}-P-{NN}-{DATE}` | `HO-MP-0042-P-03-2026-03-01` |

### Timestamps

ISO 8601: `2026-01-28T20:45:00Z`

### Version Field

All YAML artifacts include: `rwp_version: "0.29.0"`

## Artifacts

RWP plans produce a standard set of documents:

| Artifact | Format | Purpose |
|----------|--------|---------|
| `PLAN.md` | Markdown | Plan overview with phases and success criteria |
| `INTAKE.yaml` | YAML | Requirements capture - pain points, constraints |
| `MASTERPLAN.yaml` | YAML | Detailed phase and task breakdown |
| `manifest.yaml` | YAML | File paths and audit schedule |
| `state.yaml` | YAML | Execution tracking - phase status, timestamps |
| `dependencies.yaml` | YAML | Plan and phase dependencies |
| Handoff documents | YAML/MD | Session continuity and context transfer |
| Audit reports | Markdown | Quality verification at checkpoints |

## Using RWP in Gemini Web

Gemini web sessions are conversational, so the RWP workflow adapts accordingly:

### Creating a Plan

When a user describes a project or task, you can help structure it as an RWP plan:

1. **Capture requirements** - ask clarifying questions, then draft an INTAKE.yaml
2. **Design phases** - break the work into logical phases with clear deliverables
3. **Output artifacts** - provide PLAN.md, INTAKE.yaml, and MASTERPLAN.yaml as code blocks
4. **The user saves locally** - they copy the artifacts into their project directory

### Executing Phases

Help the user work through phases one at a time:

1. Review what the current phase requires
2. Provide guidance, code, or analysis for the phase tasks
3. Track completed tasks and verification results
4. At the end of the phase, create a handoff document

### Maintaining Continuity

Handoff documents are the key mechanism for continuity:

- **At session end**: Output a handoff document summarizing completed work and a continuation prompt for the next phase
- **At session start**: When the user pastes a continuation prompt, use it to understand the full project context and pick up where the previous session left off

### Recognizing RWP Requests

Users may ask for RWP-style planning with:
- "Create an RWP plan for..."
- "Structure this as a plan with phases"
- "Draft an intake document"
- "Generate a handoff for this session"
- "I'm continuing plan MP-NNNN, here's the context: [prompt]"

## Quick Reference Templates

### Minimal Plan

```markdown
# MP-{NNNN}-{short-name} - {Title}

## Objective
{What this plan achieves.}

## Phases
- **P-01**: {Title} - {deliverables}
- **P-02**: {Title} - {deliverables}
- **P-03**: {Title} - {deliverables}

## Audits
- AUD-01 after P-02
- FINAL after P-03
```

### Requirements Intake

```yaml
rwp_version: "0.29.0"
plan_id: MP-{NNNN}-{short-name}
title: "{Title}"

pain_points:
  - id: PP-01
    description: "{Problem description}"
    severity: high

requirements:
  - id: REQ-01
    description: "{What needs to happen}"
    priority: high
    linked_pain_points: [PP-01]

constraints:
  - "{Constraint description}"
```

### Session Handoff

```markdown
# Handoff - MP-{NNNN}-{short-name} Phase P-{NN}

## Completed
- {Summary of completed work}

## Decisions
- {Key decisions made during this phase}

## Verification
- {What was tested or checked}

## Next Phase: P-{MM} - {Title}
- {Task 1}
- {Task 2}
```

## Gem Configuration

If creating a Gemini Gem for RWP planning, consider these behavioral guidelines:

- Use advisory language throughout ("Consider", "You may want to")
- Output artifacts as fenced code blocks with language tags
- When a user describes a project, offer to create an RWP plan
- When a session is wrapping up, offer to generate a handoff
- When a user provides a continuation prompt, acknowledge the context and continue from where the previous session ended

## References

- RWP Specification: https://rhumbprotocol.dev
- RWP JSON Schemas: Available for artifact validation
- Reference Implementation: YAKKL® Meridian™ (https://meridian.yakkl.com)
- Source: https://github.com/rhumbprotocol/specs

---

Produced by Rhumb Workflow Protocol™ (RWP™) - https://rhumbprotocol.dev
Created by YAKKL Inc. - https://yakkl.com
