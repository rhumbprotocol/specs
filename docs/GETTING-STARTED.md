# Getting Started with RWP

A practical guide to adopting the Rhumb Workflow Protocol for structured AI workflow management.

---

## What is RWP?

The **Rhumb Workflow Protocol** (RWP) is a formal, open-source protocol for structuring multi-phase projects that involve AI and human collaboration. It provides:

- **Artifact definitions** - Standard documents (plans, intakes, manifests, state, handoffs) that describe and track work
- **Lifecycle management** - A state machine governing how phases progress and recover from failure
- **Platform adapters** - Integration guides for Claude Code, Codex, Gemini CLI, and browser-based AI tools

RWP is protocol-level - it defines *what* artifacts look like and *how* they relate, not *how* your tool implements them internally.

For the full specification, see [PROTOCOL.md](./PROTOCOL.md).

---

## Prerequisites

RWP is language- and platform-agnostic. To use it, you need:

1. A text editor or AI coding assistant (Claude Code, Codex, Gemini CLI, etc.)
2. Familiarity with YAML and Markdown
3. A project that benefits from structured phasing (multi-day features, cross-team work, complex migrations)

No build tools, package managers, or runtime dependencies are needed to adopt the protocol.

---

## Core Concepts in 5 Minutes

### Artifacts

RWP defines 5 core artifact types:

| Artifact | Format | Purpose |
|----------|--------|---------|
| **Plan** | Markdown | Master workflow document - phases, deliverables, tasks |
| **Intake** | YAML | Requirements capture - pain points, constraints, success criteria |
| **Manifest** | YAML | File/deliverable registry - what was produced |
| **State** | YAML | Runtime execution state - phase progress, timestamps |
| **Handoff** | Markdown | Phase transition document - context for the next agent/session |

### Lifecycle

Plans move through states:

```
created → in_progress → completed
                ↘ paused → in_progress (resume)
                ↘ failed → in_progress (retry)
```

Phases within a plan follow the same pattern. Sub-phases (P-01-A, P-01-B, P-01-C) provide crash resilience - if a 30-minute sub-phase fails, only that sub-phase restarts.

### Sequences

Phase identifiers follow a structured format:

- `P-01` - Phase 1
- `P-01-A` - Sub-phase A of phase 1
- `AUD-01` - Audit checkpoint 1
- `HO-MP-0001-P-01-A` - Handoff for plan MP-0001, sub-phase P-01-A

For the full grammar, see [spec/sequence.grammar](../spec/sequence.grammar).

---

## Your First RWP Workflow

### Step 1: Capture Requirements

Create an intake document describing what you want to build:

```yaml
# INTAKE.yaml
rwp_version: "0.25.1"
id: "intake-my-feature"
title: "Add user authentication"
created: "2026-03-01T10:00:00Z"
status: draft

pain_points:
  - "Users cannot securely access their accounts"
  - "No session management exists"

requirements:
  - id: REQ-001
    description: "Implement login flow with email/password"
    priority: high
  - id: REQ-002
    description: "Add session tokens with configurable expiry"
    priority: high

constraints:
  - "Use existing database schema"
  - "Complete within 2 weeks"

success_criteria:
  - "Users can log in and maintain sessions"
  - "Sessions expire after configured timeout"
```

### Step 2: Create a Plan

Write a plan that decomposes the work into phases:

```markdown
# Plan: Add User Authentication

## Objective
Implement secure user authentication with session management.

## Phases

### P-01: Database & Models
- Add user table with password hash column
- Create session token table
- Write migration scripts

### P-02: Authentication Logic
- Implement password hashing (bcrypt)
- Create login/logout endpoints
- Add session token generation

### P-03: Integration & Testing
- Wire up middleware for protected routes
- Write unit and integration tests
- Document API endpoints
```

### Step 3: Track State

As you execute each phase, update the state artifact:

```yaml
# state.yaml
rwp_version: "0.25.1"
plan_id: "MP-0001"
execution:
  status: in_progress
  current_phase: P-02
phases:
  P-01:
    status: completed
    started_at: "2026-03-01T10:00:00Z"
    completed_at: "2026-03-01T14:00:00Z"
  P-02:
    status: in_progress
    started_at: "2026-03-01T14:30:00Z"
```

### Step 4: Write Handoffs

When transitioning between phases (or sessions), create a handoff document:

```markdown
# Handoff: P-01 → P-02

## What Was Done
- Created users table with bcrypt password_hash column
- Created sessions table with token, user_id, expires_at
- Migration scripts tested and verified

## What's Next
- P-02 starts with the authentication logic
- The user model is at src/models/user.ts
- Password hashing should use the bcrypt library already in package.json

## Open Questions
- Should failed login attempts be rate-limited? (deferred to P-03)
```

---

## Using RWP with AI Tools

RWP integrates with major AI coding assistants. Each platform has its own adapter:

### Claude Code

Copy the integration files from `integrations/claude-code/` into your project's `.claude/` directory. The adapter provides a `/plan` command for creating RWP-compliant plans.

See: [integrations/claude-code/CLAUDE-RWP.md](../integrations/claude-code/CLAUDE-RWP.md)

### OpenAI Codex

Copy the integration files from `integrations/codex/` into your project's `.codex/` directory. The adapter provides plan skills and RWP safety rules.

See: [integrations/codex/skills/plan/SKILL.md](../integrations/codex/skills/plan/SKILL.md)

### Google Gemini CLI

Copy the integration files from `integrations/gemini-cli/` into your project's `.gemini/` directory. The adapter provides an RWP plan command.

See: [integrations/gemini-cli/commands/rwp/rwp-plan.md](../integrations/gemini-cli/commands/rwp/rwp-plan.md)

### Browser-Based AI (Claude.ai, ChatGPT, Gemini)

For browser-based tools, paste the relevant context document into your conversation or project knowledge:

- Claude.ai: [integrations/claude-ai/knowledge/RWP-GUIDE.md](../integrations/claude-ai/knowledge/RWP-GUIDE.md)
- ChatGPT: [integrations/chatgpt/RWP-INSTRUCTIONS.md](../integrations/chatgpt/RWP-INSTRUCTIONS.md)
- Gemini: [integrations/gemini-web/RWP-CONTEXT.md](../integrations/gemini-web/RWP-CONTEXT.md)

---

## Templates

RWP provides 17 foundation templates across 4 categories:

| Category | Templates | Purpose |
|----------|-----------|---------|
| **Core** | Plan, Intake, Masterplan, State, Dependencies, Manifest | Workflow structure and tracking |
| **Display** | Draft, Commit, Phase-Complete, Handoff-Complete, Start-Prompt, Prompt | Conversation output formatting |
| **Architecture** | AVD, ACS | Architecture vision and component specs |
| **Reference** | Handoff, Phase-Audit | Phase transitions and quality checks |

All templates are advisory-only - they suggest structure and best practices without enforcing any particular tool behavior.

Browse templates: [templates/](../templates/)

---

## JSON Schemas

RWP provides JSON schemas for validating YAML artifacts:

- `spec/schemas/plan.schema.json` - Plan document validation
- `spec/schemas/intake.schema.json` - Intake document validation
- `spec/schemas/manifest.schema.json` - Manifest validation
- `spec/schemas/state.schema.json` - State tracking validation
- `spec/schemas/handoff.schema.json` - Handoff document validation

These schemas define the expected structure and can be used with any JSON Schema validator.

---

## Conformance Levels

RWP defines three conformance levels to accommodate different adoption depths:

| Level | What It Means |
|-------|---------------|
| **Minimal** | Uses Plan + State artifacts with basic lifecycle |
| **Standard** | Adds Intake, Manifest, Handoff artifacts and sub-phase support |
| **Full** | Includes audit checkpoints, UUID generation, version embedding |

Start with Minimal and adopt more as your workflows grow in complexity.

For details, see [spec/conformance-levels.md](../spec/conformance-levels.md).

---

## Reference Implementation

[YAKKL Meridian](https://meridian.yakkl.com) is the official reference implementation of RWP. It provides a full CLI and desktop application implementing the protocol with additional features like minification, budget enforcement, and multi-provider AI integration.

---

## Next Steps

- Read the full [Protocol Specification](./PROTOCOL.md) for detailed artifact definitions
- Browse [templates/](../templates/) for ready-to-use workflow templates
- Check [spec/](../spec/) for JSON schemas and format specifications
- See [CONTRIBUTING.md](../CONTRIBUTING.md) for how to contribute to RWP

---

Rhumb Workflow Protocol (RWP) v0.25.1
https://rhumbprotocol.dev
