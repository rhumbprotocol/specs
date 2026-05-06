# RWP Reference Utilities

This directory contains reference implementations and utilities that
accompany the Rhumb Workflow Protocol specification. They serve two
audiences:

1. **Implementors** - code that demonstrates correct parsing and generation
   for spec-defined formats, useful as a porting reference.
2. **Contributors and downstream users** - tooling to validate that artifacts
   conform to the spec.

All code in this directory is licensed under Apache-2.0, the same license as
the rest of the protocol.

---

## What ships today

| File | Purpose | Tested |
|------|---------|--------|
| [sequence-parser.ts](./sequence-parser.ts) | Reference parser for the phase-sequence grammar defined in [spec/sequence.grammar](../spec/sequence.grammar). Implements the full ABNF (logical AND/OR, phase groups, sub-phases). | [sequence-parser.test.ts](./sequence-parser.test.ts) |
| [uuid-generator.ts](./uuid-generator.ts) | Reference generator for RWP UUID v7 artifacts per [spec/uuid.format.md](../spec/uuid.format.md) and [spec/uuid-generation.md](../spec/uuid-generation.md). | [uuid-generator.test.ts](./uuid-generator.test.ts) |

These are not packaged for distribution yet. They are reference code intended
to be read alongside the spec.

---

## Planned: `rhumbproto` validator CLI

The validator is a planned CLI utility that will validate RWP artifacts
against the published schemas and conformance levels. It is not yet
implemented; this section captures the design intent so contributors and
implementors can follow along.

### Design intent

```
util/
├── README.md                       (this file)
├── sequence-parser.ts              (existing)
├── sequence-parser.test.ts         (existing)
├── uuid-generator.ts               (existing)
├── uuid-generator.test.ts          (existing)
└── cli/                            (planned)
    ├── rhumbproto.ts               (CLI entry point)
    ├── validators/
    │   ├── schema-meta.ts          (validates spec/schemas/* against the
    │   │                            JSON Schema metaschema)
    │   ├── template-conformance.ts (validates each template against its
    │   │                            corresponding schema)
    │   ├── artifact.ts             (validates user PLAN/INTAKE/state/manifest
    │   │                            files against the published schemas)
    │   └── conformance-level.ts    (runs conformance-levels.md checks)
    └── README.md                   (CLI usage docs)
```

### Intended commands

```bash
# Validate one file against its inferred schema
rhumbproto validate <path>

# Validate everything in a directory
rhumbproto validate <directory>

# Validate the spec repo's own schemas and templates
rhumbproto validate --spec-self

# Run conformance-level checks at a given level
rhumbproto conform <directory> --level <1|2|3>

# Print version and supported schemas
rhumbproto info
```

### Distribution plan

| Channel | Package name | When |
|---------|--------------|------|
| npm | `@rhumbprotocol/validator` | After the CLI surface stabilizes |
| GitHub Action | `rhumbprotocol/validate-action` | Co-released with the npm package, for CI use |
| Rust port | `rhumbproto` (crates.io) | Optional, only if non-Node implementations request it |

A GitHub Action wrapping the validator means contributors and downstream
consumers can drop one workflow step into their CI to validate every PR
against the latest published schemas.

### Status

Design only. Tracked as part of the v1.0 release readiness work. See
[docs/BRANCHING.md](../docs/BRANCHING.md#validation-rhumbproto-utility) for
how the validator integrates with the branching model and PR review.

---

## Contributing

The same rules in [CONTRIBUTING.md](../CONTRIBUTING.md) and
[docs/BRANCHING.md](../docs/BRANCHING.md) apply to this directory.

For reference-implementation code specifically:

- Code must compile and pass its tests on Node.js 20 LTS or newer.
- New utilities require a spec section that defines the format the utility
  consumes or produces. Reference implementations follow the spec; they do
  not extend it.
- Tests should cover the spec's stated examples plus negative cases the
  spec explicitly disallows.
- Avoid runtime dependencies where the standard library suffices. The
  utilities in this directory are reference code - the bar for adding a
  third-party dependency is high.

---

Rhumb Workflow Protocol (RWP)
https://rhumbprotocol.dev
