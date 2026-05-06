# Contributing to the Rhumb Workflow Protocol

Thank you for your interest in contributing to RWP. This document explains how to participate in the protocol's development.

---

## Overview

The Rhumb Workflow Protocol is an open-source specification licensed under Apache-2.0. Contributions are welcome in several areas:

- **Protocol specification** - Clarifications, corrections, and extensions to PROTOCOL.md
- **Schemas** - Improvements to JSON/YAML schemas in `spec/schemas/`
- **Templates** - New or improved foundation templates in `templates/`
- **Integration adapters** - Platform adapters for AI tools in `integrations/`
- **Documentation** - Guides, examples, and reference materials in `docs/`
- **Reference implementations** - Code examples in `reference/`

---

## Getting Started

### 1. Read the Protocol

Before contributing, familiarize yourself with the core specification:

- [Protocol Specification](./docs/PROTOCOL.md) - Full RWP v0.25.1 spec
- [Getting Started Guide](./docs/GETTING-STARTED.md) - Practical introduction

### 2. Understand the Repository Structure

```
rhumbprotocol/
├── docs/           # Documentation (PROTOCOL.md, GETTING-STARTED.md)
├── spec/           # Format specifications and JSON schemas
├── templates/      # Foundation templates (advisory-only)
├── integrations/   # Platform-specific adapters
├── reference/      # Reference implementations
├── examples/       # Example workflows
└── extensions/     # Third-party extensions
```

### 3. Check Existing Issues

Before starting work, check the issue tracker for:
- Open issues related to your contribution
- Ongoing discussions about the area you want to change
- Any duplicate proposals

---

## Contribution Types

### Specification Changes

Changes to the core protocol (PROTOCOL.md) go through a more rigorous review process:

1. Open an issue describing the proposed change and its rationale
2. Discuss the change with maintainers and the community
3. Submit a pull request with the specification update
4. Include updated schemas if the change affects artifact structure

Specification changes should:
- Maintain backward compatibility where possible
- Include clear rationale for the change
- Update all affected sections (not just one area)
- Follow the existing document style and structure

### Template Contributions

New templates or template improvements are welcome:

- Templates should be advisory-only (suggest, not enforce)
- Use placeholder syntax: `{{PLACEHOLDER_NAME}}`
- Include a header comment explaining the template's purpose
- Follow the naming convention: `NAME.{md|yaml}.template`
- Place in the appropriate subdirectory under `templates/`

### Integration Adapters

Adding support for a new AI platform:

1. Create a directory under `integrations/` named after the platform
2. Include platform-specific configuration files
3. Reference PROTOCOL.md for the underlying protocol details
4. Test the adapter with the target platform
5. Document any platform-specific limitations or considerations

### Documentation

Documentation contributions include:
- Fixing typos or unclear explanations
- Adding examples or use cases
- Improving the Getting Started guide
- Creating tutorials for specific workflows

### Schema Changes

When modifying JSON schemas:
- Validate the schema with a JSON Schema validator
- Include `$id` and `$comment` fields
- Update conformance-levels.md if the change affects conformance
- Add or update tests for schema validation

---

## Style Guide

### Markdown

- Use ATX-style headers (`#`, `##`, `###`)
- Separate sections with `---` horizontal rules
- Use fenced code blocks with language identifiers
- Keep line lengths reasonable (no strict limit, but wrap at ~100 characters for prose)

### YAML

- Use 2-space indentation
- Quote strings that contain special characters
- Include `rwp_version` in all artifact examples
- Use ISO 8601 timestamps (`2026-03-04T10:00:00Z`)

### Language

RWP documentation uses advisory language throughout:

| Use | Avoid |
|-----|-------|
| "Consider..." | "You MUST..." |
| "It is recommended..." | "REQUIRED" |
| "Typically..." | "SHALL" |
| "A common approach is..." | "NEVER" |

This is a deliberate design choice. RWP defines structure, not enforcement.

---

## Pull Request Process

> **Branch model**: see [docs/BRANCHING.md](./docs/BRANCHING.md) for the full
> git-flow used in this repo. Short version: `main` is the released spec
> (do not target it), `develop` is the integration branch (target this), and
> contributor branches use the `feature/*`, `fix/*`, `docs/*`, or `wip/*`
> prefix.

1. **Fork** the repository and create a branch from the latest `develop`
   using one of the prefixes above (e.g., `feature/add-aep-artifact`,
   `fix/intake-required-fields`)
2. **Make** your changes following the style guide above
3. **Test** any schema or code changes - run the validator locally
   (`rhumbproto validate <path>` once shipped; see
   [docs/BRANCHING.md](./docs/BRANCHING.md#validation-rhumbproto-utility))
4. **Submit** a pull request **against `develop`** (never against `main`) with:
   - A clear title describing the change
   - A description explaining the rationale
   - References to any related issues

### Review Criteria

Pull requests are reviewed for:
- Consistency with the existing protocol specification
- Advisory-only language (no enforcement terms)
- Correct cross-references to PROTOCOL.md
- Schema validity (for schema changes)
- Template placeholder correctness (for template changes)

---

## Reporting Issues

When reporting issues, include:
- Which document or file is affected
- What the current behavior/content is
- What you expected or propose instead
- Any relevant context (platform, use case)

---

## Code of Conduct

Contributors are expected to:
- Be respectful and constructive in discussions
- Focus on technical merit in reviews
- Welcome newcomers and help them get started
- Acknowledge different perspectives and use cases

---

## License and Trademarks

By contributing to this repository, you agree that your contributions will be licensed under the [Apache License 2.0](./LICENSE).

The YAKKL trademarks (including "Rhumb Protocol"&trade; and "AI Workflow"&trade;) are governed separately and are NOT licensed under Apache 2.0. See [TRADEMARK-POLICY.md](./TRADEMARK-POLICY.md) for permitted and prohibited uses, and the [NOTICE](./NOTICE) file for the canonical trademark statement.

---

## Questions?

- **Protocol questions**: Open an issue with the `question` label
- **Contribution questions**: Open an issue with the `contributing` label
- **General discussion**: Use the Discussions tab if available

---

Rhumb Workflow Protocol (RWP) v0.25.1
https://rhumbprotocol.dev
