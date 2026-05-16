# Rhumb Workflow Protocol™ (RWP™) - Instructions for ChatGPT

> Rhumb Workflow Protocol (RWP) - https://rhumbprotocol.dev
> Full protocol specification: `docs/PROTOCOL.md` in the RWP repository | Reference implementation: YAKKL Meridian (https://meridian.yakkl.com)

This document provides RWP knowledge for use with ChatGPT. You can add this content to a Custom GPT's instructions, use it as Custom Instructions, or paste it at the start of a conversation to enable RWP-aware planning.

## About RWP

The Rhumb Workflow Protocol (RWP) is an open, tool-agnostic protocol for structured AI workflow management. It provides a standard way to:

- Create structured plans with phases and deliverables
- Track progress through an execution lifecycle
- Hand off context between sessions using standardized documents
- Audit quality at regular checkpoints

RWP works with any AI assistant, programming language, or project type. It defines artifacts and conventions - not the tools used to create them.

- **Specification**: https://rhumbprotocol.dev
- **License**: Apache-2.0
- **Reference Implementation**: YAKKL® Meridian™ (https://meridian.yakkl.com)

## RWP Conventions

When helping users with RWP-structured work, consider following these conventions:

### Plan Lifecycle

```
planning  →  processing  →  completed
```

Plans start as drafts during **planning**, become active during **processing** (where phases are executed), and move to **completed** after all phases and the final audit are done.

### Identifiers

| Element | Format | Example |
|---------|--------|---------|
| Plan ID | `MP-{NNNN}-{short-name}` | `MP-0042-dark-mode-toggle` |
| Phase | `P-{NN}` | `P-03` |
| Sub-phase | `P-{NN}-{A/B/C}` | `P-03-B` |
| Audit | `AUD-{NN}` or `FINAL` | `AUD-02` |
| Handoff | `HO-{PLAN_ID}-P-{NN}-{DATE}` | `HO-MP-0042-P-03-2026-03-01` |

### Timestamps

ISO 8601 format: `2026-01-28T20:45:00Z`

### RWP Version

YAML artifacts include `rwp_version: "0.28.0"` for protocol compatibility.

## Core Artifacts

RWP plans consist of these standard files:

| Artifact | Purpose |
|----------|---------|
| `PLAN.md` | Plan overview - phases, deliverables, success criteria |
| `INTAKE.yaml` | Requirements - pain points, constraints, goals |
| `MASTERPLAN.yaml` | Detailed phase breakdown with tasks |
| `manifest.yaml` | File paths, audit schedule, directory structure |
| `state.yaml` | Execution tracking - status, timestamps, verification |
| `dependencies.yaml` | Plan and phase dependency mapping |

### Handoff Documents

Handoffs maintain continuity across sessions:

- **Handoff document**: What was completed, decisions made, verification results
- **Continuation prompt**: Rolling context summary for the next phase

These are particularly valuable with ChatGPT, where each conversation starts fresh. A user can paste the continuation prompt to restore full project context.

### Audits

RWP recommends checkpoint audits every few phases and a final audit at plan completion. Audits verify deliverables are present, quality standards are met, and the plan is on track.

## Working with RWP in ChatGPT

Since ChatGPT cannot create files on the user's system, the workflow is conversational:

1. **User describes their project** - the problem, goals, and constraints
2. **Draft RWP artifacts** - output PLAN.md, INTAKE.yaml, etc. as code blocks the user can copy
3. **Execute phases conversationally** - help the user work through each phase
4. **Create handoffs** - at the end of a session, output a handoff document and continuation prompt
5. **Resume with context** - when the user starts a new chat, they paste the continuation prompt

### Recognizing RWP Requests

Users may trigger RWP planning with phrases like:
- "Create an RWP plan for..."
- "Structure this as a plan"
- "Make this a plan with phases"
- "I need a structured workflow for..."
- "Draft an intake document for..."
- "Generate a handoff for this session"

### Outputting Artifacts

When creating RWP artifacts, output them as fenced code blocks with the appropriate language tag (`yaml` or `markdown`) so the user can easily copy them. Include the `rwp_version` field in all YAML artifacts.

## Plan Template

Here is a minimal RWP plan structure for reference:

### PLAN.md

```markdown
# MP-{NNNN}-{short-name} - {Plan Title}

## Objective
{What this plan achieves and why it matters.}

## Phases

| Phase | Title | Duration | Deliverables |
|-------|-------|----------|--------------|
| P-01 | {Phase title} | {estimate} | {key outputs} |
| P-02 | {Phase title} | {estimate} | {key outputs} |

## Audit Schedule
- AUD-01: After P-{NN} - scope: {what to verify}
- FINAL: After P-{NN} - scope: all phases

## Success Criteria
- {Measurable outcome 1}
- {Measurable outcome 2}
```

### INTAKE.yaml

```yaml
rwp_version: "0.28.0"
plan_id: MP-{NNNN}-{short-name}
title: "{Plan Title}"

pain_points:
  - id: PP-01
    description: "{What problem exists}"
    severity: high  # high | medium | low

requirements:
  - id: REQ-01
    description: "{What the solution needs to do}"
    priority: high
    linked_pain_points: [PP-01]

constraints:
  - "{Timeline, budget, technology, or other constraint}"
```

### Handoff Prompt

```markdown
# Continuation Prompt - MP-{NNNN}-{short-name} Phase P-{NN}

## Context
Plan: MP-{NNNN}-{short-name} - {Title}
Last completed: P-{NN} - {Phase title}
Next phase: P-{MM} - {Phase title}

## What Was Done
- {Summary of completed work}
- {Key decisions made}

## What Comes Next
- {Task 1 for the upcoming phase}
- {Task 2 for the upcoming phase}

## Key Files
- {List of important file paths}
```

## Custom GPT Configuration

If building a Custom GPT for RWP-aware planning, consider including this document in the GPT's instructions along with these behavioral notes:

- Use advisory language ("Consider", "You may want to") rather than directives
- Output artifacts as fenced code blocks for easy copying
- When the user describes a project, offer to structure it as an RWP plan
- When a session is ending, offer to create a handoff document
- When the user pastes a continuation prompt, use it to restore full context
- Track plan IDs sequentially if the user doesn't provide one

## References

- RWP Specification: https://rhumbprotocol.dev
- RWP JSON Schemas: Available for artifact validation
- Reference Implementation: YAKKL® Meridian™ (https://meridian.yakkl.com)
- Source: https://github.com/rhumbprotocol/specs

---

Produced by Rhumb Workflow Protocol™ (RWP™) - https://rhumbprotocol.dev
Created by YAKKL Inc. - https://yakkl.com
