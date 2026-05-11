# Rhumb Workflow Protocol (RWP) - Guide for Claude.ai

> Rhumb Workflow Protocol (RWP) - https://rhumbprotocol.dev

This document provides RWP knowledge for use in Claude.ai Projects. Add this file to a Project's knowledge base so Claude understands RWP conventions when helping you plan and manage structured work.

## What is RWP?

The Rhumb Workflow Protocol (RWP) is an open protocol for structured AI workflow management. It defines a standard way to create plans, track progress through phases, hand off context between sessions, and audit quality at checkpoints.

RWP is tool-agnostic and platform-independent. It works with any AI assistant, any programming language, and any project type. The protocol focuses on the artifacts and lifecycle - not the tools used to create them.

- **Specification**: https://rhumbprotocol.dev
- **Protocol Spec (full)**: See `docs/PROTOCOL.md` in the RWP repository for the complete specification, including artifact schemas, lifecycle state machine, conformance levels, and foundation templates
- **License**: Apache-2.0
- **Reference Implementation**: YAKKL Meridian (https://meridian.yakkl.com)

## RWP Plan Lifecycle

Plans move through three stages:

```
planning  →  processing  →  completed
                 |
                 v
           (audits at checkpoints)
```

- **Planning**: Requirements gathered, phases designed, plan drafted and reviewed
- **Processing**: Phases executed sequentially, handoffs created between sessions
- **Completed**: All phases done, final audit passed

## Core Artifacts

RWP plans produce a standard set of files. When working in a browser chat, you can ask the assistant to draft these artifacts for you to save locally.

| Artifact | Format | Purpose |
|----------|--------|---------|
| `PLAN.md` | Markdown | Plan overview with phases, deliverables, and success criteria |
| `INTAKE.yaml` | YAML | Requirements capture - pain points, constraints, goals |
| `MASTERPLAN.yaml` | YAML | Detailed phase breakdown with tasks and deliverables |
| `manifest.yaml` | YAML | Pre-computed file paths, audit schedule, directory structure |
| `state.yaml` | YAML | Execution tracking - phase status, timestamps, verification |
| `dependencies.yaml` | YAML | Plan and phase dependency mapping |

### Handoff Documents

RWP uses handoff documents to maintain continuity when work spans multiple sessions:

- **Handoff** (`HO-{PLAN_ID}-P-{NN}-{DATE}.md`): Records what was completed, decisions made, verification results, and open items
- **Prompt** (`HO-{PLAN_ID}-P-{NN}-TO-P-{MM}-PROMPT.md`): Provides rolling context for the next phase, including history summary and specific tasks

Handoffs are especially valuable in browser chat environments where conversation context resets between sessions. Starting a new session with the handoff prompt gives the assistant full continuity.

### Audit Checkpoints

RWP recommends audits at regular intervals (e.g., every 3rd phase) and a final audit at plan completion. Audits verify:

- All deliverables are present and correct
- Quality standards are met
- No regressions from earlier phases
- The plan is on track

## Plan IDs and Conventions

### Identifiers

| Element | Format | Example |
|---------|--------|---------|
| Plan ID | `MP-{NNNN}-{short-name}-{short-name}-{short-name}-{short-name}-{short-name}-{short-name}-{short-name}-{short-name}-{short-name}-{short-name}` | `MP-0042-dark-mode-toggle` |
| Phase | `P-{NN}` | `P-03` |
| Sub-phase | `P-{NN}-{A/B/C}` | `P-03-B` |
| Audit | `AUD-{NN}` or `FINAL` | `AUD-02` |

### Timestamps

RWP uses ISO 8601 format for all timestamps: `2026-01-28T20:45:00Z`

### RWP Version

All YAML artifacts include an `rwp_version` field for protocol compatibility tracking:

```yaml
rwp_version: "0.26.0"
```

## How to Use RWP in Claude.ai

Since browser-based chat cannot create files directly, the workflow is conversational:

1. **Describe your project or task** - explain what you want to accomplish
2. **Ask for an RWP plan** - say something like "Create an RWP plan for this" or "Structure this as an RWP plan"
3. **Review the draft** - the assistant will output plan artifacts you can review
4. **Copy artifacts to your project** - save the PLAN.md, INTAKE.yaml, and other files locally
5. **Execute phases** - work through phases, asking the assistant for help with each
6. **Use handoffs for continuity** - when starting a new session, paste the handoff prompt to restore context

### Example Prompts

- "Create an RWP plan for building a REST API with authentication"
- "Draft an INTAKE.yaml for migrating our database to PostgreSQL"
- "I'm starting phase P-02-A of plan MP-0015-example-project. Here's the handoff prompt: [paste prompt]"
- "Generate an audit checklist for phases P-01 through P-03"

## RWP Plan Structure Example

Here's a minimal plan structure for reference:

```yaml
# INTAKE.yaml
rwp_version: "0.26.0"
plan_id: MP-0001-example-plan
title: "Example Project Plan"

pain_points:
  - id: PP-01
    description: "Current process is manual and error-prone"
    severity: high

requirements:
  - id: REQ-01
    description: "Automate the deployment pipeline"
    priority: high
    linked_pain_points: [PP-01]

constraints:
  - "Timeline: 2 weeks"
  - "Budget: Use existing infrastructure"
```

```markdown
# PLAN.md

## MP-0001-example-plan - Example Project Plan

### Objective
Automate the deployment pipeline to reduce manual errors.

### Phases
- **P-01**: Infrastructure setup (2 days)
- **P-02**: Pipeline implementation (5 days)
- **P-03**: Testing and documentation (3 days)

### Audit Schedule
- AUD-01 after P-02
- FINAL after P-03
```

## Directory Structure

RWP plans are typically stored in a plans directory within your project:

```
.plans/
  planning/           # Plans being drafted
  processing/         # Plans being executed
    MP-0001-example-plan/
      PLAN.md
      INTAKE.yaml
      MASTERPLAN.yaml
      manifest.yaml
      state.yaml
      dependencies.yaml
      handoffs/
      audits/
  completed/          # Finished plans
```

The exact directory structure is flexible - RWP defines the artifacts, not where you put them.

## Templates

RWP provides foundation templates for all artifact types. Templates use advisory language ("Consider", "You may want to") rather than enforcement directives. You can ask the assistant to output any template for you to customize.

Available templates:
- Plan document (PLAN.md)
- Requirements intake (INTAKE.yaml)
- Phase breakdown (MASTERPLAN.yaml)
- Execution state (PLAN-STATE.yaml)
- Dependencies (DEPENDENCIES.yaml)
- File manifest (MANIFEST-PLAN.yaml)
- Architecture Vision Document (AVD)
- Architecture Component Spec (ACS)
- Handoff document
- Phase audit checklist

## References

- RWP Specification: https://rhumbprotocol.dev
- RWP JSON Schemas: Available for validation of YAML/JSON artifacts
- Reference Implementation: YAKKL Meridian (https://meridian.yakkl.com)
- Source: https://github.com/rhumbprotocol/specs

---

Produced by Rhumb Workflow Protocol (RWP) - https://rhumbprotocol.dev
Created by YAKKL Inc. - https://yakkl.com
