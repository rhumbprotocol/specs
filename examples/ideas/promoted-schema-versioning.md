---
id: IDEA-9003
type: idea
title: "Schema versioning strategy for breaking changes"
status: promoted
classification: public
created: "2026-02-01T08:00:00Z"
updated: "2026-04-25T16:45:00Z"

authors:
  - name: "Hans Jones"
    role: "architect"
tags:
  - schemas
  - versioning
  - governance

parent: null
children:
  - "AVD-0009"

pipeline: architecture

approved_by: "Hans Jones"
approved_at: "2026-03-15T10:30:00Z"
approval_policy: "solo"

promoted_to: "AVD-0009"
promoted_at: "2026-04-25T16:45:00Z"
promoted_pipeline: architecture

parked_as: null
parked_reason: null
parked_until: null

discarded_by: null
discarded_at: null
discarded_reason: null
---

# IDEA-9003: Schema versioning strategy for breaking changes

## Context

RWP publishes JSON Schemas at versioned URLs (e.g.,
`https://rhumbprotocol.dev/schemas/v0.27.0/idea.schema.json`). As the protocol
evolves, breaking changes (new required fields, removed properties, enum value
changes) need a clear versioning contract so that existing artifacts remain
valid against the schema version they were authored under, while new artifacts
adopt the latest schema.

## Initial Considerations

- Semantic versioning at the protocol level (MAJOR.MINOR.PATCH) already in use
- Each schema carries its own `$id` with the protocol version embedded
- Backward-incompatible changes bump MINOR (pre-1.0) or MAJOR (post-1.0)
- Validation tools must accept a `--schema-version` flag or auto-detect from
  the artifact's `$id` reference
- Migration guides published per breaking change

## The "Why"

Without a clear versioning contract, adopters cannot upgrade safely. Broken
validation after an upgrade erodes trust faster than any feature builds it.

## Strategic Value

A predictable upgrade path is the difference between a protocol that accumulates
adopters over years and one that peaks at early-adopter enthusiasm then stalls.

## Key Concepts / Pillars

1. **Version-pinned validation**: An artifact authored against v0.27.0 is always
   valid against v0.27.0, regardless of what v0.27.0 introduces.
2. **Forward-compatibility window**: Non-breaking additions (new optional fields)
   do not require a version bump from the artifact author's perspective.
3. **Migration tooling**: `rhumb migrate` command that rewrites frontmatter for
   the new version and reports what changed.

## Target Audience

Protocol maintainers, adopter organizations managing long-lived artifact repos.

## Resolved Decisions

* [x] **SemVer policy** — **MINOR for breaking pre-1.0.** Aligns with Rust/npm
  ecosystem conventions.
* [x] **Schema URL includes version** — **Yes.** Enables parallel schema hosting.
* [x] **No multi-version validation in a single run** — **Correct.** One version
  per invocation; CI can matrix across versions.

## Path Selection

Architecture path selected. The versioning strategy affects every schema, every
validator, and every migration tool — this is cross-cutting infrastructure that
warrants AVD-level design and decomposition into multiple ACSs.

## References

- ACS-0040 (Schema Infrastructure specification)
- idea.schema.json `$id` field

---
Tags: [idea]

---

Produced: "2026-04-25T16:45:00Z"
By: Rhumb Protocol Contributors - https://rhumbprotocol.dev
Copyright: Copyright (c) 2026 Rhumb Protocol Contributors. All Rights Reserved.
