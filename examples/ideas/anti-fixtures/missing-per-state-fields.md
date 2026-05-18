---
id: IDEA-9902
type: idea
title: "Anti-fixture: Missing per-state required fields"
status: approved
classification: public
created: "2026-04-15T10:00:00Z"
updated: "2026-04-28T14:30:00Z"

authors:
  - name: "Test Author"
    role: "engineer"
tags:
  - anti-fixture

parent: null
children: []

pipeline: architecture

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
---

# ANTI-FIXTURE: Missing Per-State Required Fields

## What This Demonstrates

**Error**: This IDEA has `status: approved` but leaves `approved_by`, `approved_at`,
and `approval_policy` as null. The JSON Schema (idea.schema.json) declares these
fields as `type: ["string", "null"]` — meaning null IS a valid value at Level 1.

This anti-fixture **PASSES Level 1 validation** (JSON Schema). It **FAILS Level 3
validation** (rhumb-validate per idea-lifecycle.spec.md Section 5).

This demonstrates the intentional division of labor in RWP validation:
- **Level 1** (JSON Schema): validates structure, types, enums, required fields.
- **Level 3** (rhumb-validate): validates state-dependent constraints — i.e., that
  fields which are conditionally required based on `status` are populated.

## Expected Error

```
Level 1: PASS (null is valid for ["string", "null"] typed fields)
Level 3 FAIL — status=approved requires non-null: approved_by, approved_at, approval_policy
```

## How To Fix

Populate the per-state required fields appropriate to the current status:

```yaml
status: approved
approved_by: "Jane Doe"
approved_at: "2026-04-28T14:30:00Z"
approval_policy: "review"
```

The per-state required-fields contract is documented in idea-lifecycle.spec.md
Section 5. Key principle: JSON Schema handles the "always required" set (id, type,
title, status, classification, created, updated, authors); the lifecycle spec handles
"required-when-in-state-X" constraints.

## Context

This trap catches adopters who change an IDEA's status without also populating the
corresponding metadata. The approval ceremony should be: (1) change status to
approved, (2) populate approved_by, approved_at, and approval_policy in the same
edit. Tooling (rhumb-validate) catches the omission; the JSON Schema alone does not.

---
Tags: [anti-fixture, level-3-failure]

---

Produced: "2026-05-01T21:00:00Z"
By: Rhumb Protocol Contributors - https://rhumbprotocol.dev
Copyright: Copyright (c) 2026 Rhumb Protocol Contributors. All Rights Reserved.
