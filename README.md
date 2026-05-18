# Rhumb Workflow Protocol™ (RWP™)

A structured protocol for AI-assisted workflow management.

---

## Overview

The **Rhumb Workflow Protocol™** (RWP™) is a formal, open-source protocol specification designed to enable structured, multi-phase AI Workflow™ Management in AI-assisted environments. RWP combines clear artifact definitions, lifecycle state machines, and formalized integration points to enable consistent workflow execution across tools, platforms, and AI providers.

Created by [YAKKL, Inc.](https://yakkl.com) - the Rhumb Workflow Protocol™ is a complementary, open counterpart to [YAKKL® Meridian™](https://meridian.yakkl.com), which serves as the reference implementation.

**Protocol URL**: https://rhumbprotocol.dev

---

## Quick Start

Get started with RWP in 3 steps:

1. **Capture requirements** - Create an `INTAKE.yaml` describing pain points, requirements, and constraints
2. **Write a plan** - Decompose work into phases in a `PLAN.md` with deliverables and tasks
3. **Track execution** - Update `state.yaml` as phases progress, writing handoff documents between sessions

For a complete walkthrough with examples, see the **[Getting Started Guide](./docs/GETTING-STARTED.md)**.

### Using RWP with AI Tools

RWP provides integration adapters for major AI platforms:

| Platform | Adapter | Setup |
|----------|---------|-------|
| Claude Code | [claude-code/](./integrations/claude-code/) | Copy to `.claude/` directory |
| OpenAI Codex | [codex/](./integrations/codex/) | Copy to `.codex/` directory |
| Gemini CLI | [gemini-cli/](./integrations/gemini-cli/) | Copy to `.gemini/` directory |
| Claude.ai | [claude-ai/](./integrations/claude-ai/) | Paste into project knowledge |
| ChatGPT | [chatgpt/](./integrations/chatgpt/) | Paste into custom instructions |
| Gemini Web | [gemini-web/](./integrations/gemini-web/) | Paste into conversation context |

---

## Documentation

- **[Getting Started](./docs/GETTING-STARTED.md)** - Practical guide to adopting RWP
- **[Protocol Specification](./docs/PROTOCOL.md)** - Full RWP specification (2000+ lines)
- **[Implementation Profiles](./docs/IMPLEMENTATION-PROFILES.md)** - Core RWP versus file-tree profiles, including the Meridian reference profile
- **[Extensions Guide](./docs/EXTENSIONS.md)** - Extending RWP with custom fields, artifacts, and integrations
- **[FAQ](./docs/FAQ.md)** - Frequently asked questions about adoption, implementation, and compatibility
- **[Diagram Guide](./docs/DIAGRAMS.md)** - Public diagram set for artifacts, lifecycle, architecture path, conformance, and portability
- **[Papers](./docs/PAPERS.md)** - Executive, implementation, and specification-bundle paper outlines
- **[Format Specifications](./spec/)** - 11 spec documents + 7 JSON schemas
- **[Templates](./templates/)** - 18 foundation templates for RWP-compliant workflows
- **[Integration Guides](./integrations/)** - 9 adapters across 6 platforms
- **[Governance](./committee/)** - Committee charter, decision-making process, and release governance
- **[Contributing](./CONTRIBUTING.md)** - How to contribute to RWP
- **[Changelog](./CHANGELOG.md)** - Version history
- **[Examples](./examples/)** - Complete example workflows demonstrating RWP in practice
  - [Ideas](./examples/ideas/) - Idea artifact worked examples (5 lifecycle states) and anti-fixtures
  - [Bug Fix](./examples/bug-fix/) - 1-phase single-developer bug fix (lightest RWP usage)
  - [Simple Feature](./examples/simple-feature/) - 2-phase, single developer (REQUIRED-only approach)
  - [Multi-Phase Project](./examples/multi-phase/) - 5-phase with sub-phases, multi-team (REQUIRED + RECOMMENDED)

### Repository Structure

```
rhumbprotocol/
├── docs/
│   ├── PROTOCOL.md                    # Full RWP specification
│   ├── GETTING-STARTED.md             # Practical adoption guide
│   ├── IMPLEMENTATION-PROFILES.md     # Core profile + Meridian reference profile
│   ├── EXTENSIONS.md                  # Extension patterns & custom artifacts
│   └── FAQ.md                         # Frequently asked questions
├── spec/
│   ├── schemas/                       # 7 JSON schemas (plan, intake, manifest, state, handoff, idea, lifecycle)
│   ├── uuid.format.md                 # UUID format specification
│   ├── uuid-generation.md             # UUID generation guidance
│   ├── versioning.format.md           # Version embedding format
│   ├── sequence.grammar               # Phase sequence ABNF grammar
│   ├── sequence-parser.md             # Sequence parser specification
│   ├── implementation-guide.md        # Implementation best practices
│   ├── conformance-levels.md          # Conformance level guidance
│   ├── custom-fields.md               # Custom field extension patterns
│   ├── schema-composition.md          # Schema composition & inheritance
│   ├── openapi-integration.md         # OpenAPI integration patterns
│   └── lifecycle/
│       └── idea-lifecycle.spec.md     # Idea artifact lifecycle state machine
├── templates/
│   ├── PLAN.md.template               # Master plan document
│   ├── INTAKE.yaml.template           # Requirements capture
│   ├── MASTERPLAN.yaml.template       # Execution masterplan
│   ├── PLAN-STATE.yaml.template       # Runtime execution state
│   ├── DEPENDENCIES.yaml.template     # Dependency tracking
│   ├── MANIFEST-PLAN.yaml.template    # File/deliverable registry
│   ├── IDEA.md.template               # Idea artifact (architecture pipeline entry)
│   ├── sequences.yaml.template        # Sequence configuration
│   ├── architecture/                  # AVD + ACS templates
│   ├── display/                       # 6 display/prompt templates
│   └── reference/                     # Handoff + audit templates
├── integrations/
│   ├── claude-code/                   # Claude Code CLI adapter
│   ├── codex/                         # OpenAI Codex adapter
│   ├── gemini-cli/                    # Google Gemini CLI adapter
│   ├── claude-ai/                     # Claude.ai browser guide
│   ├── chatgpt/                       # ChatGPT browser instructions
│   └── gemini-web/                    # Gemini browser context
├── examples/
│   ├── ideas/                         # Idea artifact worked examples + anti-fixtures
│   ├── simple-feature/                # 2-phase dark mode toggle (minimal RWP)
│   │   ├── INTAKE.yaml
│   │   ├── PLAN.md
│   │   ├── state.yaml
│   │   └── handoffs/
│   ├── multi-phase/                   # 5-phase rate limiter (full RWP)
│   │   ├── INTAKE.yaml
│   │   ├── PLAN.md
│   │   ├── state.yaml
│   │   └── handoffs/
│   └── bug-fix/                       # 1-phase form-button double-submission fix (lightest RWP)
│       ├── INTAKE.yaml
│       ├── PLAN.md
│       ├── state.yaml
│       └── handoffs/
├── committee/
│   ├── CHARTER.md                     # Committee charter and membership
│   ├── GOVERNANCE.md                  # Decision-making and proposal process
│   ├── RELEASES.md                    # Release governance and versioning
│   ├── minutes/                       # Published meeting notes
│   └── proposals/                     # RWP Enhancement Proposals (AEPs)
├── rules/                             # Custom validation rules (extensible)
├── schemas/                           # Additional schemas (extensible)
├── util/                              # Reference implementations
│   ├── uuid-generator.ts              # UUID generation reference
│   ├── uuid-generator.test.ts         # UUID tests
│   ├── sequence-parser.ts             # Sequence parser reference
│   └── sequence-parser.test.ts        # Parser tests
├── LICENSE                            # Apache-2.0
├── NOTICE                             # License attribution inclusion
├── TRADEMARK.md                       # Trademark policies
├── CHANGELOG.md                       # Version history
├── CONTRIBUTING.md                    # Contribution guidelines
└── README.md                          # This file
```

---

## Features

- **Formal Specification**: Comprehensive, versioned protocol documentation
- **Flexible Integration**: Adapters for Claude Code, Codex, Gemini CLI, and web platforms
- **Foundation Templates**: Advisory-only templates for rapid workflow setup
- **Crypto UUID Generation**: Deterministic, verifiable artifact identifiers
- **Schema Validation**: JSON/YAML schemas for all artifact types
- **Lifecycle State Machines**: Formal state machines with per-artifact transition rules
- **Governance**: Open committee structure and contribution guidelines

---

## Created By

[YAKKL, Inc.](https://yakkl.com)

## Reference Implementation

[YAKKL® Meridian™](https://meridian.yakkl.com) - The official reference implementation of the Rhumb Workflow Protocol™.

---

## License

Licensed under the **Apache License 2.0**. See [LICENSE](./LICENSE) for details.

---

## Contributing

We welcome contributions to the Rhumb Workflow Protocol™. See [CONTRIBUTING.md](./CONTRIBUTING.md) for guidelines on:

- Protocol specification changes
- Template contributions
- Integration adapters for new platforms
- Documentation improvements

## Changelog

See [CHANGELOG.md](./CHANGELOG.md) for version history.

---

## Acknowledgments

The Rhumb Workflow Protocol™ draws inspiration from established open standards including:
- **LSP** (Language Server Protocol) - Microsoft's standardized interface for editor tools
- **MCP** (Model Context Protocol) - Anthropic's protocol for structured tool integration
- **OpenAPI** - Industry standard for API specification

These protocols demonstrate the value of formal, open specifications in enabling ecosystem interoperability. RWP extends these principles to AI workflow management.

---

## Trademarks

"Rhumb"™, "Rhumb Workflow Protocol"™, and "RWP"™ are trademarks of YAKKL, Inc. The marks are claimed under U.S. common law; no federal registration is asserted in this document. Use of the marks is governed by [TRADEMARK.md](./TRADEMARK.md). Apache 2.0 §6 trademark exclusion applies — the License does not grant trademark rights.

For mark-use enquiries, written-agreement requests, or infringement reports, contact `legal@yakkl.com`.

---

**Rhumb Workflow Protocol™ (RWP™)**

## Version Source of Truth

The protocol version is stored in [`VERSION`](./VERSION). For breaking protocol-shape changes during the pre-1.0 period, bump MINOR, for example `0.25.4` to `0.28.1`. PATCH is only for backward-compatible corrections.

After changing `VERSION`, run:

```bash
node scripts/sync-version.mjs
```

