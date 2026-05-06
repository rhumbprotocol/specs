---
id: IDEA-9905
type: idea
title: "Anti-fixture: Invalid status enum value"
status: archived
classification: public
created: "2026-04-22T13:00:00Z"
updated: "2026-04-22T13:00:00Z"

authors:
  - name: "Test Author"
    role: "pm"
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
---

# ANTI-FIXTURE: Invalid Status Enum Value

## What This Demonstrates

**Error**: The `status` field is set to `archived` — a value that does NOT exist
in the 7-state lifecycle enum. The valid states are:

1. `captured` — initial state, low-friction entry
2. `refining` — being shaped (scope, constraints)
3. `ready` — fully shaped, awaiting approval
4. `approved` — approved for downstream work
5. `promoted` — promoted to downstream artifact (terminal)
6. `parked` — held for later (only non-terminal sideways state)
7. `discarded` — rejected (terminal)

There is no `archived` state. The concept "archived" maps to either `parked`
(temporarily shelved, may re-enter) or `discarded` (permanently rejected). The
lifecycle forces adopters to choose which semantics they mean rather than using
an ambiguous catch-all.

## Expected Error

```
Level 1 FAIL — /status: must be equal to one of the allowed values
  Allowed: "captured", "refining", "ready", "approved", "promoted", "parked", "discarded"
  Received: "archived"
```

## How To Fix

Choose the lifecycle state that matches the intent:

```yaml
# If the IDEA is shelved for later but may return:
status: parked
parked_as: "backlog"
parked_reason: "Deferring to next quarter"

# If the IDEA is permanently rejected:
status: discarded
discarded_by: "Project Lead"
discarded_at: "2026-04-22T13:00:00Z"
discarded_reason: "Superseded by IDEA-0042"
```

## Context

The 7-state enum is intentionally minimal. Each state has distinct semantics and
transition rules (see idea-lifecycle.spec.md Section 3). Common values from other
systems that adopters try to use — `archived`, `closed`, `done`, `cancelled`,
`draft` — all map to one of the 7 states. The lifecycle spec's state machine
(Section 3.1) is the authoritative reference for state semantics.

---
Tags: [anti-fixture, level-1-failure]

---

Produced: "2026-05-01T21:00:00Z"
By: Rhumb Protocol Contributors - https://rhumbprotocol.dev
Copyright: Copyright (c) 2026 Rhumb Protocol Contributors. All Rights Reserved.
