---
name: rwp-plan
description: "RWP™ plan skill for conversational plan generation using the Rhumb Workflow Protocol™."
---

> Rhumb Workflow Protocol™ (RWP™) - https://rhumbprotocol.dev
> Protocol spec: [PROTOCOL.md](../../../../docs/PROTOCOL.md) | Templates: [Foundation Templates](../../../../docs/PROTOCOL.md#foundation-templates)
> This skill enables structured plan generation following RWP conventions.

# RWP Plan - Conversational Plan Generation

Generate structured plans from natural conversation using the Rhumb Workflow Protocol (RWP). The conversation provides the context - no need to re-enter information.

## Usage

```
/rwp-plan                          # Draft plan from conversation context
/rwp-plan "Title"                  # Draft with specific title
/rwp-plan review                   # Analyze conversation, suggest approach
/rwp-plan commit                   # Finalize draft and create plan files
/rwp-plan --sub-phases             # Use sub-phases (P-XX-A/B/C) for resilience
```

## Arguments

- `$ARGUMENTS` - Optional: title, subcommand (review/commit), or flags

## How It Works

### Three Modes

| Mode | What Happens | How to Enter |
|------|--------------|--------------|
| **Explore** | Free discussion, problem discovery | Default - talk normally |
| **Draft** | AI proposes structured plan | `/rwp-plan` or "draft a plan" |
| **Commit** | Create files, assign ID | `/rwp-plan commit` or "create it" |

## Process

### 1. Context Extraction

When the skill is invoked, analyze the conversation to extract:

**Problem/Need:**
- What issue is being addressed?
- What's the current vs desired state?

**Technical Context:**
- Files/components mentioned
- Technologies involved
- Previous attempts and outcomes

**Constraints:**
- Backward compatibility requirements
- Performance or security considerations
- Scope boundaries

### 2. Draft Generation

Display the draft using the RWP plan draft format. Include: title, context from conversation, objective, phases with tasks and deliverables, constraints, and files likely affected.

### 3. Iteration

After showing the draft, allow natural language refinement:
- "Add a phase for regression tests"
- "The objective should focus on X"
- "Add constraint: backward compatible"
- "Remove P-02, combine with P-01"

### 4. Commit

When the user approves, create the RWP plan directory with all artifacts:

#### RWP Plan Artifacts

| File | RWP Template | Purpose |
|------|-------------|---------|
| `PLAN.md` | `PLAN.md.template` | Plan definition with phases |
| `INTAKE.yaml` | `INTAKE.yaml.template` | Requirements and pain points |
| `MASTERPLAN.yaml` | `MASTERPLAN.yaml.template` | Detailed phase breakdown |
| `manifest.yaml` | `MANIFEST-PLAN.yaml.template` | File path manifest |
| `state.yaml` | `PLAN-STATE.yaml.template` | Execution tracking |
| `dependencies.yaml` | `DEPENDENCIES.yaml.template` | Dependency mapping |

#### Directory Structure

```
{plan_directory}/MP-{NNNN}-{short-name}/
  PLAN.md
  INTAKE.yaml
  MASTERPLAN.yaml
  manifest.yaml
  state.yaml
  dependencies.yaml
  assets/
  audits/
  handoffs/
    HO-{PLAN_ID}-START-P-01-PROMPT.md
```

### 5. Post-Commit

After creating all files:
- Display a summary of created artifacts
- Suggest reviewing the PLAN.md
- Indicate the plan is ready for execution

## RWP Conventions

### Plan IDs
- Format: `MP-{NNNN}-{short-name}` (zero-padded sequence number)

### Phase IDs
- Traditional: `P-01`, `P-02`, `P-03`
- Sub-phases: `P-01-A`, `P-01-B`, `P-01-C`

### Timestamps
- ISO 8601 format: `YYYY-MM-DDTHH:MM:SSZ`

### RWP Version
- Include `rwp_version: "0.31.0"` in YAML artifacts

### Audit Checkpoints
- Consider audits every 3rd phase (P-03, P-06, P-09...)
- A final audit at plan completion verifies all deliverables

### Handoff Documents
- **Handoff**: Records completed work, decisions, verification
- **Prompt**: Provides rolling context for the next phase
- **Naming**: `HO-{PLAN_ID}-P-{NN}-{DATE}.yaml` for handoffs

## Plan Lifecycle

```
planning -> processing -> completed
```

Plans start in `planning` status. When execution begins, they move to `processing`. After all phases complete and the final audit passes, the plan moves to `completed`.

## Sub-Phase Pattern

Sub-phases break large phases into smaller units (~30 min each) for crash resilience:

```
P-XX-Y where:
  XX = Logical phase number (01-99)
  Y  = Sub-phase letter (A, B, C...)
```

## References

- RWP Specification: https://rhumbprotocol.dev
- RWP Templates: `templates/` directory in the RWP package
- RWP JSON Schemas: `spec/schemas/` for validation
- Reference Implementation: YAKKL® Meridian™ (https://meridian.yakkl.com)

---

Produced by Rhumb Workflow Protocol™ (RWP™) - https://rhumbprotocol.dev
Created by YAKKL Inc. - https://yakkl.com
