# RWP™ Reference Utilities

This directory contains reference implementations and utilities that
accompany the Rhumb Workflow Protocol™ specification. They serve two
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
### Distribution plan

| Channel | Package name | When |
|---------|--------------|------|
| Rust | `rhumb` (crates.io) | Optional, only if implementations want it |

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

- Code must compile and pass its tests on Node.js 20 LTS or newer if Typescript.
- Or, If Code is in Rust it must compile and pass its tests in Rust which means the latest Rust - Cargo
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
