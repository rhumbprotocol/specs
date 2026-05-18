# Frequently Asked Questions

Common questions about the Rhumb Workflow Protocol™ - adoption, implementation, and compatibility.

> See also: [Getting Started](./GETTING-STARTED.md), [PROTOCOL.md](./PROTOCOL.md), [Extensions Guide](./EXTENSIONS.md)

---

## General

### What is RWP™?

The **Rhumb Workflow Protocol™** (RWP™) is a formal, open-source protocol specification for structured, multi-phase workflow management in AI-assisted environments. It defines standard artifact types (plans, intakes, manifests, state, handoffs), lifecycle rules, and integration patterns. See the [full specification](./PROTOCOL.md) for details.

### Why "Rhumb™"?

In navigation, **rhumb** and **meridian** are the two coordinates needed to locate any point. RWP™ is the open protocol specification; [YAKKL® Meridian™](https://meridian.yakkl.com?utm_source=RWP_faq) is the reference implementation. Together they provide complete workflow navigation - the specification (rhumb) and the tooling (meridian).

### Who created RWP?

RWP was created by [YAKKL, Inc.](https://yakkl.com) and is released under the [Apache License 2.0](../LICENSE). It is an open protocol - contributions from the community are welcome. See [CONTRIBUTING.md](../CONTRIBUTING.md).

### Is RWP free to use?

Yes. RWP is licensed under **Apache 2.0**, which permits commercial and non-commercial use, modification, and distribution. There are no fees, royalties, or usage restrictions.

### What is the relationship between RWP™ and Meridian™?

RWP™ is the **protocol specification** - it defines artifact formats, lifecycle rules, and integration patterns. [YAKKL® Meridian™](https://meridian.yakkl.com?utm_source=RWP_faq2) is the **reference implementation** - a production tool that implements RWP™. The relationship is similar to LSP (specification) and VS Code (implementation), or MCP (specification) and Claude Code (implementation). You can implement RWP™ without using Meridian™.

---

## Adoption

### Do I need special tools to use RWP?

No. RWP artifacts are plain YAML and Markdown files. Any text editor, AI coding assistant, or scripting tool can create and manage them. RWP provides [integration adapters](../integrations/) for popular platforms, but they are conveniences, not requirements.

### What size projects benefit from RWP?

RWP works best for projects that span multiple sessions, involve multiple phases, or require handoffs between people or AI agents. Typical use cases:

- **Multi-day features** - Complex implementations spanning days or weeks
- **Cross-team work** - Projects involving multiple teams or contributors
- **Complex migrations** - Database, infrastructure, or platform migrations
- **Regulated workflows** - Projects requiring audit trails and compliance documentation
- **AI-assisted development** - Structured collaboration between humans and AI tools

For quick, single-session tasks, RWP may be more overhead than it's worth. Use your judgment. However, you see how YAKKL® Meridian™ handles it with additional `Getting Started` guides and it will ask what do you want to do. The Chat option is just like plain ole AI chat but will not a large overhead like the AI Vendors have (but for good reason for their target audience).

### How do I start using RWP?

1. Read the [Getting Started guide](./GETTING-STARTED.md) for a practical walkthrough
2. Copy the relevant [templates](../templates/) into your project
3. Create an INTAKE.yaml describing your requirements
4. Write a PLAN.md decomposing work into phases
5. Track progress with state.yaml and handoff documents

>This process is mainly for tool builders or someone wishing to incorporate the process into their product. For the rest of us, you can take a look at [YAKKL® Meridian™](https://meridian.yakkl.com?utm_source=RWP_faq3).

### Can I adopt RWP incrementally?

Yes. RWP has three [conformance levels](../spec/conformance-levels.md):

- **Basic** - Use REQUIRED fields only (minimal adoption)
- **Standard** - Include RECOMMENDED fields (best practice)
- **Full** - Support extensions and all artifact types (comprehensive)

>Start with Basic conformance (just plans and state tracking) and add more structure as needed.

### Can I use RWP with my existing project management tools?

RWP complements rather than replaces existing tools. RWP artifacts live alongside your code and capture the technical workflow - they don't replace Jira tickets, GitHub issues, or Confluence pages. Many teams use RWP for the AI-assisted development workflow while keeping their existing project management stack.

---

## Implementation

### What are the core artifact types?

RWP defines five standard artifact types:

| Artifact | Purpose | Key Fields |
|----------|---------|------------|
| **Plan** | Defines phases, deliverables, and tasks | title, overview, phases |
| **Intake** | Captures requirements and constraints | pain_points, requirements, success_criteria |
| **Manifest** | Tracks files and deliverables | artifacts, version |
| **State** | Records execution progress | current_phase, status, phase history |
| **Handoff** | Documents transitions between phases | context_summary, from_phase, to_phase |

See [PROTOCOL.md](./PROTOCOL.md) for complete schema definitions.

### What fields are required?

Each artifact type has REQUIRED, RECOMMENDED, and OPTIONAL fields. The [Conformance Levels](../spec/conformance-levels.md) document provides field-by-field tables for every artifact type. At minimum, a valid plan needs: `title`, `overview`, `created_at`, and at least one `phase`.

### How do phases work?

Phases are the primary unit of work in RWP. Each phase has:
- An identifier (e.g., `P-01` or `P-02-A` for sub-phases)
- A title and objective
- Deliverables and tasks
- Verification criteria

Phases follow a lifecycle: `pending` → `in_progress` → `completed` (or `failed` → `recovery`). Sub-phases (P-01-A, P-01-B, P-01-C) break large phases into smaller sessions for crash resilience and drift.

### How do handoffs work?

Handoffs document the transition between phases and sub-phases. They capture what was accomplished, what decisions were made, and what the next phase needs to know. Handoffs are especially valuable for:
- Multi-session work (continuing across hours or days)
- Multi-agent work (different AI agents handling different phases)
- Team transitions (handing work to another person)

### Can I use sub-phases?

Yes. RWP supports sub-phases using a letter suffix: `P-01-A`, `P-01-B`, `P-01-C`. Sub-phases are useful for:
- Breaking long phases into manageable chunks
- Providing crash resilience (if a session ends mid-phase, the sub-phase boundary is a safe restart point)
- Parallel execution (multiple sub-phases can sometimes run concurrently)

The [sequence grammar](../spec/sequence.grammar) defines the full syntax.

### What goes in state.yaml?

The state file tracks execution progress:

```yaml
plan_id: "MP-0042-dark-mode-toggle"
execution:
  status: "in_progress"
  current_phase: "P-02-A"
phases:
  P-01:
    status: "completed"
    completed_at: "2026-03-04T10:00:00Z"
  P-02-A:
    status: "in_progress"
    started_at: "2026-03-04T11:00:00Z"
```

Update it as phases progress. It serves as the single source of truth for where a workflow stands.

---

## Compatibility

### Which AI tools support RWP?

RWP provides integration adapters for:

| Platform | Type | Adapter |
|----------|------|---------|
| Claude Code | CLI | [integrations/claude-code/](../integrations/claude-code/) |
| OpenAI Codex | CLI | [integrations/codex/](../integrations/codex/) |
| Gemini CLI | CLI | [integrations/gemini-cli/](../integrations/gemini-cli/) |
| Claude.ai | Browser | [integrations/claude-ai/](../integrations/claude-ai/) |
| ChatGPT | Browser | [integrations/chatgpt/](../integrations/chatgpt/) |
| Gemini Web | Browser | [integrations/gemini-web/](../integrations/gemini-web/) |

These adapters translate RWP concepts into each platform's native format. You can also create custom integrations - see the [Extensions guide](./EXTENSIONS.md).

### Can I use RWP with multiple AI tools on the same project?

Yes. RWP artifacts are tool-agnostic YAML and Markdown files. You can start a workflow in Claude Code, hand off to a colleague using Codex, and review in ChatGPT. The artifacts are the shared contract - each tool reads and writes the same files.

### Is RWP compatible with existing workflow tools?

RWP artifacts are plain files that coexist with any project structure. They don't conflict with:
- CI/CD configurations (GitHub Actions, GitLab CI, etc.)
- Project management tools (Jira, Linear, Asana, etc.)
- Documentation systems (Confluence, Notion, etc.)
- Version control (Git, SVN, etc.)

### What programming languages does RWP support?

RWP is language-agnostic. The protocol defines document formats (YAML, Markdown, JSON Schema), not runtime APIs. RWP has been used with TypeScript, Rust, Python, Go, and many other languages. The [reference implementations](../spec/reference/) include TypeScript examples for UUID generation and sequence parsing.

### Does RWP require a specific directory structure?

RWP recommends but does not enforce a directory structure. The suggested layout:

```
.rwp/                     # or your preferred directory
├── plans/                # Plan documents
├── intakes/              # Requirements captures
├── state/                # Execution state files
├── handoffs/             # Phase transition documents
└── manifests/            # File/deliverable registries
```

You can adapt this to your project's conventions.

---

## Schemas & Validation

### Are there JSON Schemas for RWP artifacts?

Yes. RWP provides five JSON Schemas in the [spec/schemas/](../spec/schemas/) directory:

- `plan.schema.json` - Plan artifact validation
- `intake.schema.json` - Intake artifact validation
- `manifest.schema.json` - Manifest artifact validation
- `state.schema.json` - State artifact validation
- `handoff.schema.json` - Handoff artifact validation

Use these with any JSON Schema validator (ajv, jsonschema, etc.) to validate your artifacts.

### How do I validate RWP artifacts?

```bash
# Using ajv (JavaScript)
npx ajv validate -s spec/schemas/plan.schema.json -d my-plan.json

# Using jsonschema (Python)
python -m jsonschema -i my-plan.json spec/schemas/plan.schema.json

# Using check-jsonschema
check-jsonschema --schemafile spec/schemas/plan.schema.json my-plan.json

# RWP Validate
rhumb-validate ... # See rhumb-validate documentation in /conformance
```

For YAML artifacts, convert to JSON first or use a YAML-aware validator.

### Can I extend the schemas?

Yes. RWP schemas support extension through:
- **Custom fields** (`x-*` prefix) - See [Custom Fields](../spec/custom-fields.md)
- **Schema composition** (`allOf`, `oneOf`) - See [Schema Composition](../spec/schema-composition.md)
- **Custom artifact types** - See the [Extensions guide](./EXTENSIONS.md)

---

## Contributing

### How do I contribute to RWP?

See [CONTRIBUTING.md](../CONTRIBUTING.md) for guidelines. Contributions are welcome for:
- Protocol specification improvements
- Template additions
- Integration adapters for new platforms
- Documentation improvements
- Bug reports and feature requests

### How are protocol changes managed?

RWP follows [Semantic Versioning 2.0.0](https://semver.org/):
- **Patch** (1.0.x): Documentation fixes, clarifications
- **Minor** (1.x.0): New optional fields, new artifact types, new templates
- **Major** (x.0.0): Breaking changes to required fields or core lifecycle

Changes are discussed through issues and pull requests. The protocol changelog is maintained in [CHANGELOG.md](../CHANGELOG.md).

### Where do I report issues?

File issues at the RWP GitHub repository (https://github.com/rhumbprotocol/issues). Include:
- Which artifact type or spec document is affected
- What behavior you expected vs. what happened
- Your RWP version and tool/platform

---

## Troubleshooting

### My AI tool isn't following RWP structure

Ensure the integration adapter is properly installed:
- **Claude Code**: Copy adapter files to `.claude/` in your project root
- **Codex**: Copy to `.codex/` directory
- **Gemini CLI**: Copy to `.gemini/` directory
- **Browser tools**: Paste the guide into project knowledge or custom instructions

If the tool still deviates, paste the relevant RWP artifact (PLAN.md, state.yaml) into the conversation as context.

### My artifacts fail schema validation

Common causes:
1. **Missing required fields** - Check [Conformance Levels](../spec/conformance-levels.md) for required fields
2. **Wrong field types** - Ensure dates are ISO 8601 strings, not date objects
3. **YAML formatting** - Ensure proper indentation and quoting
4. **Custom fields without prefix** - All custom fields need the `x-` prefix

### Phases are getting too large

Break large phases into sub-phases (P-01-A, P-01-B, P-01-C). Each sub-phase should be completable in 30 minutes or less. This provides:
- Natural checkpoint boundaries
- Crash resilience
- Clearer handoff documents

### State tracking gets out of sync

The state file (state.yaml) is the single source of truth. If it gets out of sync:
1. Read the latest handoff document for ground truth
2. Update state.yaml to match actual progress
3. Continue from the corrected state

RWP™ doesn't enforce automated state updates - it's your responsibility (or your tool's) to keep state current. You can use a DB too.

---

## Further Reading

- [PROTOCOL.md](./PROTOCOL.md) - Full RWP™ specification
- [Getting Started](./GETTING-STARTED.md) - Practical adoption guide
- [Extensions Guide](./EXTENSIONS.md) - Extending RWP™ for your needs
- [Templates](../templates/) - Foundation templates for all artifact types
- [Integration Adapters](../integrations/) - Platform-specific setup guides

---

Rhumb Workflow Protocol™ (RWP™) v0.28.1
https://rhumbprotocol.dev
