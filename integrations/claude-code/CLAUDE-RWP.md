# RWP-Enabled Project Instructions

> Rhumb Workflow Protocol™ (RWP™) - https://rhumbprotocol.dev
> Protocol spec: [PROTOCOL.md](../../docs/PROTOCOL.md) | Reference implementation: [YAKKL Meridian](https://meridian.yakkl.com)

This file provides project-level instructions for AI assistants working in an RWP-enabled workspace. Copy this file to your project root and rename to `CLAUDE.md` (or append to an existing `CLAUDE.md`).

## Rhumb Workflow Protocol™

This project uses the Rhumb Workflow Protocol™ (RWP™) for structured AI workflow management.

**Key principle**: Significant changes benefit from a plan created and approved before code changes begin.

**Quick triggers**:
- `/plan` or "make this a plan" - Creates an RWP™ plan draft
- `/plan commit` - Finalizes the plan and creates artifacts
- `/plan review` - Analyzes conversation without drafting

## RWP™ Plan Lifecycle

Plans follow a structured lifecycle:

```
planning -> processing -> completed
                |
                v
           (audits at checkpoints)
```

### Artifacts

RWP™ plans produce these artifacts in the plan directory:

| File | Purpose |
|------|---------|
| `PLAN.md` | Plan definition with phases and deliverables |
| `INTAKE.yaml` | Requirements, pain points, constraints |
| `MASTERPLAN.yaml` | Detailed phase breakdown with tasks |
| `manifest.yaml` | Pre-computed file paths and audit schedule |
| `state.yaml` | Execution tracking and phase status |
| `dependencies.yaml` | Plan and phase dependencies |
| `handoffs/` | Phase handoff documents and continuation prompts |
| `audits/` | Checkpoint and final audit documents |

### Handoff Documents

RWP™ uses handoff documents to maintain continuity between sessions:
- **Handoff**: Records what was completed, decisions made, and verification results
- **Prompt**: Provides rolling context for the next phase or sub-phase
- **Naming**: `HO-{PLAN_ID}-P-{NN}-{DATE}.yaml` for handoffs, `HO-{PLAN_ID}-P-{NN}-TO-P-{MM}-PROMPT.md` for prompts

### Audit Checkpoints

RWP™ recommends audits at regular intervals (e.g., every 3rd phase) and a final audit at plan completion. Audits verify deliverables, check quality, and identify issues.

## RWP™ Templates

RWP™ provides foundation templates for all artifact types. Templates use advisory language ("Consider", "You may want to") rather than enforcement language. Templates are located in the RWP™ package under `templates/`.

### Template Types

| Template | Purpose |
|----------|---------|
| `PLAN.md.template` | Plan document structure |
| `INTAKE.yaml.template` | Requirements capture |
| `MASTERPLAN.yaml.template` | Phase and task breakdown |
| `MANIFEST-PLAN.yaml.template` | File path manifest |
| `PLAN-STATE.yaml.template` | Execution state tracking |
| `DEPENDENCIES.yaml.template` | Dependency mapping |
| `START-PROMPT.md.template` | Initial phase prompt |
| `PROMPT.md.template` | Continuation phase prompts |

## RWP Conventions

### Timestamps
- RWP uses ISO 8601 format for all timestamps (e.g., `2026-01-28T20:45:00Z`)

### Plan IDs
- Format: `MP-{NNNN}-{short-name}-{short-name}-{short-name}-{short-name}-{short-name}-{short-name}` where NNNN is a zero-padded sequence number
- Example: `MP-0042-dark-mode-toggle`, `MP-0235-rhumb-workflow-protocol`

### Phase IDs
- Traditional: `P-01`, `P-02`, `P-03`
- Sub-phases: `P-01-A`, `P-01-B`, `P-01-C` (for crash resilience)

### RWP Version
- RWP artifacts include an `rwp_version` field for protocol compatibility
- Current version: `0.31.0`

## References

- RWP™ Specification: https://rhumbprotocol.dev
- RWP™ JSON Schemas: Available in `spec/schemas/` for validation
- RWP™ Reference Implementation: YAKKL® Meridian™ (https://meridian.yakkl.com)

---

Produced by Rhumb Workflow Protocol™ (RWP™) - https://rhumbprotocol.dev
Created by YAKKL Inc. - https://yakkl.com
