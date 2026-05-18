---
id: IDEA-9904
type: idea
title: "Anti-fixture: Additional property rejected by closed schema"
status: captured
classification: public
created: "2026-04-18T11:30:00Z"
updated: "2026-04-18T11:30:00Z"

authors:
  - name: "Test Author"
    role: "engineer"
tags:
  - anti-fixture

parent: null
children: []

pipeline: null

approved_by: null
approved_at: null
approval_policy: null

promoted_to: null
promoted_at: null
promoted_pipeline: null

parked_as: null
parked_reason: null
parked_until: null

discarded_by: null
discarded_at: null
discarded_reason: null

meridian_version: "0.61.0"
---

# ANTI-FIXTURE: Additional Property (Closed Schema Violation)

## What This Demonstrates

**Error**: The frontmatter includes `meridian_version: "0.61.0"` — a field that
does NOT exist in idea.schema.json's 24-property set. The schema is declared with
`additionalProperties: false`, meaning any field not explicitly defined is rejected.

This catches adopters who try to extend the schema ad-hoc by adding implementation-
specific fields to their IDEA frontmatter. The RWP IDEA schema is intentionally
closed: its 24 properties are the complete set. Extension happens at the spec level
(schema revision), not at the document level (field addition).

## Expected Error

```
Level 1 FAIL — must NOT have additional properties
  Additional property: "meridian_version"
```

## How To Fix

Remove the additional property from the frontmatter:

```yaml
# WRONG — schema rejects unknown fields:
meridian_version: "0.61.0"

# RIGHT — only the 24 defined properties are permitted.
# Implementation metadata belongs in your tooling configuration,
# not in the IDEA artifact itself.
```

If your implementation needs to track additional metadata per-IDEA, use a sidecar
file (e.g., `.meta.yaml` alongside the IDEA) or a registry. Do not extend the IDEA
frontmatter — it is a protocol-level contract, not an application-level store.

## Context

This is a KD-38.3 anti-leak fixture. The closed schema prevents implementation-
specific fields from contaminating the portable artifact format. An IDEA authored
in one RWP implementation must be readable by any other implementation without
encountering unknown fields that break parsing or validation.

---
Tags: [anti-fixture, level-1-failure]

---

Produced: "2026-05-01T21:00:00Z"
By: Rhumb Protocol Contributors - https://rhumbprotocol.dev
Copyright: Copyright (c) 2026 Rhumb Protocol Contributors. All Rights Reserved.
