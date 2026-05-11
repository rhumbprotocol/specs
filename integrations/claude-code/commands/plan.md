# RWP Plan - Conversational Plan Generation

Generate structured plans using the Rhumb Workflow Protocol (RWP). Plans follow the RWP lifecycle: planning, processing, completed.

> This command uses the Rhumb Workflow Protocol. See https://rhumbprotocol.dev for the full specification.
> Protocol spec: [PROTOCOL.md](../../../docs/PROTOCOL.md) | Artifact types: [Artifact Types & Schemas](../../../docs/PROTOCOL.md#artifact-types--schemas) | Templates: [Foundation Templates](../../../docs/PROTOCOL.md#foundation-templates)

## Usage

```bash
/plan                              # Draft plan from conversation context
/plan "Title"                      # Draft with specific title
/plan review                       # Analyze conversation, suggest approach
/plan commit                       # Finalize draft and create plan files
/plan --sub-phases                 # Use sub-phases (P-XX-A/B/C) for resilience
```

## Arguments

- `$ARGUMENTS` - Optional: title, subcommand (review/commit), or flags

## How It Works

### Three Modes

| Mode | What Happens | How to Enter |
|------|--------------|--------------|
| **Explore** | Free discussion, problem discovery | Default - talk normally |
| **Draft** | AI proposes structured plan | `/plan` or "draft a plan" |
| **Commit** | Create files, assign ID | `/plan commit` or "create it" |

### Flow

```
[Explore] User describes problem, discusses options
    |
    v
[Draft]   "/plan" -> AI shows structured proposal following RWP format
    |
    v
[Commit]  "looks good" or "/plan commit" -> Create RWP plan artifacts
```

## Process

### 1. Context Extraction

When `/plan` is invoked, analyze the conversation to extract:
- **Problem/Need**: What issue is being addressed
- **Technical Context**: Files, technologies, previous attempts
- **Constraints**: Backward compatibility, performance, security
- **Scope**: What's in and out of scope

### 2. Draft Generation

Show the draft using the RWP plan draft display format:

```
======================================================================
  PLAN DRAFT (Not yet created)
======================================================================

Title: <extracted or provided title>
Type:  MP (<Public/Confidential>)
ID:    Will be assigned on commit

----------------------------------------------------------------------
CONTEXT (from our conversation)
----------------------------------------------------------------------
- <key point 1 from conversation>
- <key point 2 from conversation>

----------------------------------------------------------------------
OBJECTIVE
----------------------------------------------------------------------
<Clear, measurable objective derived from conversation>

----------------------------------------------------------------------
PHASES
----------------------------------------------------------------------
P-01: <Phase Name>
  - <Task 1>
  - <Task 2>
  - Deliverables: <what this phase produces>

P-02: <Phase Name>
  - <Task 1>
  - Deliverables: <what this phase produces>

----------------------------------------------------------------------
CONSTRAINTS
----------------------------------------------------------------------
- <constraint from conversation>
- <implied constraint>

----------------------------------------------------------------------
FILES LIKELY AFFECTED
----------------------------------------------------------------------
- <file/component mentioned>
- <related files inferred>

======================================================================
  Say "create it" to commit, or suggest changes
======================================================================
```

### 3. Iteration

After showing draft, the user can refine with natural language:
- "Add a phase for regression tests"
- "The objective should focus on X specifically"
- "Add constraint: backwards compatible"
- "Remove P-02, combine with P-01"

### 4. Commit

When the user says "create it", "looks good", or `/plan commit`:

#### 4.1 Create plan directory structure following RWP conventions:

```
{plan_directory}/MP-{NNNN}-{short-name}-{short-name}-{short-name}-{short-name}-{short-name}-{short-name}-{short-name}-{short-name}-{short-name}-{short-name}-{short-name}/
  PLAN.md                              # RWP Plan template
  INTAKE.yaml                          # RWP Intake template
  MASTERPLAN.yaml                      # RWP Masterplan template
  manifest.yaml                        # RWP Manifest template
  state.yaml                           # RWP Plan-State template
  dependencies.yaml                    # RWP Dependencies template
  assets/                              # Phase assets
  audits/                              # Audit documents
  handoffs/                            # Handoff documents and prompts
    HO-{PLAN_ID}-START-P-01-PROMPT.md  # Initial phase prompt
```

#### 4.2 RWP Templates

Consider using RWP templates from the `templates/` directory:

| File | RWP Template |
|------|-------------|
| PLAN.md | `PLAN.md.template` |
| INTAKE.yaml | `INTAKE.yaml.template` |
| MASTERPLAN.yaml | `MASTERPLAN.yaml.template` |
| manifest.yaml | `MANIFEST-PLAN.yaml.template` |
| state.yaml | `PLAN-STATE.yaml.template` |
| dependencies.yaml | `DEPENDENCIES.yaml.template` |
| Start prompt | `START-PROMPT.md.template` |
| Phase prompts | `PROMPT.md.template` |

#### 4.3 Audit Schedule

RWP recommends checkpoint audits at regular intervals:
- Consider audit checkpoints every 3rd phase (P-03, P-06, P-09...)
- A final audit at plan completion helps verify all deliverables

#### 4.4 Report Success

After creating all files, display a summary of what was created and suggest next steps.

## Sub-Phase Pattern

Sub-phases break large phases into smaller units for crash resilience:

```
P-XX-Y where:
  XX = Logical phase number (01-99)
  Y  = Sub-phase letter (A, B, C...)

Examples:
  P-01-A -> First sub-phase of logical phase 1
  P-01-B -> Second sub-phase of logical phase 1
  P-02-A -> First sub-phase of logical phase 2
```

## Handoff Naming

Within same logical phase: `HO-{PLAN_ID}-P-01-A-TO-P-01-B-PROMPT.md`
Crossing logical phases: `HO-{PLAN_ID}-P-01-C-TO-P-02-A-PROMPT.md`

## References

- RWP Specification: https://rhumbprotocol.dev
- Reference Implementation: YAKKL Meridian - https://meridian.yakkl.com
- RWP Templates: `templates/` directory in the RWP package
- RWP Schemas: `spec/schemas/` directory for JSON schema validation

---

Produced by Rhumb Workflow Protocol (RWP) - https://rhumbprotocol.dev
Created by YAKKL Inc. - https://yakkl.com
