# RWP Plan Creation

Instructions for creating structured plans using the Rhumb Workflow Protocol (RWP).

> Rhumb Workflow Protocol (RWP) - https://rhumbprotocol.dev
> Protocol spec: [PROTOCOL.md](../../../../docs/PROTOCOL.md) | Templates: [Foundation Templates](../../../../docs/PROTOCOL.md#foundation-templates)

## Trigger

User says: "Create a plan", "New plan", "RWP plan", "Plan for...", or invokes `/rwp-plan`.

## Instructions for Gemini

When the user wants to create a plan:

1.  **Understand Requirements**:
    *   Ask clarifying questions if the objective is vague.
    *   Determine the scope, goals, and necessary phases.

2.  **Get Next Plan ID**:
    *   Read the project's sequence configuration (e.g., `sequences.yaml`) to find the next available `MP-NNNN` ID.
    *   If no sequence file exists, ask the user for a starting ID.

3.  **Create Plan Directory**:
    *   Location: `{plan_directory}/MP-NNNN-short-name/`
    *   Create subdirectories: `handoffs/`, `audits/`, `assets/`.

4.  **Generate RWP Artifacts**:

    Consider using RWP templates from the `templates/` directory:

    *   **PLAN.md**: Use `PLAN.md.template`. Fill in frontmatter with plan ID, title, status, classification, timestamps, and phases.
    *   **INTAKE.yaml**: Use `INTAKE.yaml.template`. Capture pain points, requirements, and constraints from the conversation.
    *   **MASTERPLAN.yaml**: Use `MASTERPLAN.yaml.template`. Break down phases into tasks with estimated durations.
    *   **manifest.yaml**: Use `MANIFEST-PLAN.yaml.template`. Pre-calculate expected file paths for handoffs, prompts, and audits.
    *   **state.yaml**: Use `PLAN-STATE.yaml.template`. Initialize execution status as `planning`.
    *   **dependencies.yaml**: Use `DEPENDENCIES.yaml.template`. Map phase dependencies and any external blockers.

5.  **Create Start Prompt**:
    *   Generate `handoffs/HO-MP-NNNN-START-P-01-PROMPT.md` using `START-PROMPT.md.template`.
    *   Include plan overview, P-01 tasks, and a completion checklist.

6.  **Confirm**:
    *   Show the user the plan structure with phases and deliverables.
    *   Ask for approval before finalizing.

7.  **Activate**:
    *   On approval, update `state.yaml` status to `in_progress`.
    *   Update the project's state tracking file if one exists.

## RWP Plan Lifecycle

```
planning -> processing -> completed
```

## RWP Conventions

### Plan IDs
- Format: `MP-{NNNN}-{short-name}-{short-name}-{short-name}-{short-name}-{short-name}-{short-name}-{short-name}-{short-name}-{short-name}-{short-name}-{short-name}-{short-name}-{short-name}-{short-name}-{short-name}` with zero-padded sequence numbers

### Phase IDs
- Traditional: `P-01`, `P-02`, `P-03`
- Sub-phases: `P-01-A`, `P-01-B`, `P-01-C` (for crash resilience)

### Timestamps
- ISO 8601 format: `YYYY-MM-DDTHH:MM:SSZ`

### Handoff Documents
- **Handoff**: `HO-{PLAN_ID}-P-{NN}-{DATE}.md` - Records completed work
- **Prompt**: `HO-{PLAN_ID}-P-{NN}-TO-P-{MM}-PROMPT.md` - Continuation context

### Audit Checkpoints
- Consider scheduling audits every 3rd phase (P-03, P-06, P-09...)
- A final audit at plan completion helps verify all deliverables

### RWP Version
- Include `rwp_version: "0.26.0"` in YAML artifacts for protocol compatibility

## RWP Templates Reference

| Template | Purpose |
|----------|---------|
| `PLAN.md.template` | Plan document structure |
| `INTAKE.yaml.template` | Requirements capture |
| `MASTERPLAN.yaml.template` | Phase and task breakdown |
| `MANIFEST-PLAN.yaml.template` | File path manifest |
| `PLAN-STATE.yaml.template` | Execution state tracking |
| `DEPENDENCIES.yaml.template` | Dependency mapping |
| `START-PROMPT.md.template` | Initial phase prompt |
| `PROMPT.md.template` | Continuation prompts |

## Phase Execution

After plan creation, each phase follows this pattern:

1. Read the phase prompt (handoff document from previous phase)
2. Execute the phase tasks
3. Create a handoff document recording what was completed
4. Create a continuation prompt for the next phase
5. Update state.yaml with phase completion

## References

- RWP Specification: https://rhumbprotocol.dev
- RWP JSON Schemas: `spec/schemas/` for validation
- Reference Implementation: YAKKL Meridian (https://meridian.yakkl.com)

---

Produced by Rhumb Workflow Protocol (RWP) - https://rhumbprotocol.dev
Created by YAKKL Inc. - https://yakkl.com
