---
id: IDEA-9903
type: idea
title: "Anti-fixture: Invalid pipeline enum value"
status: approved
classification: public
created: "2026-04-10T08:00:00Z"
updated: "2026-04-25T16:45:00Z"

authors:
  - name: "Test Author"
    role: "architect"
tags:
  - anti-fixture

parent: null
children: []

pipeline: standard

approved_by: "Review Board"
approved_at: "2026-04-25T16:45:00Z"
approval_policy: "review"

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

# ANTI-FIXTURE: Invalid Pipeline Enum Value

## What This Demonstrates

**Error**: The `pipeline` field is set to `standard` — a value that does NOT exist
in the schema's enum. The valid values are: `direct`, `architecture`, `one_off`,
or `null` (not yet routed).

The value `standard` was explicitly rejected during the schema design process
(KD-38.5). The term was considered too generic and conflatable with "default behavior"
across different implementations. The chosen terms encode the actual path shape:
- `architecture`: IDEA → AVD → ACS → MP (the architecture path)
- `direct`: IDEA → MP (the direct path, skipping architecture artifacts)
- `one_off`: IDEA → execution (single-shot work, no MP)

## Expected Error

```
Level 1 FAIL — /pipeline: must be equal to one of the allowed values
  Allowed: "direct", "architecture", "one_off", null
  Received: "standard"
```

## How To Fix

Replace `pipeline: standard` with one of the valid enum values:

```yaml
pipeline: architecture    # For work needing AVD → ACS → MP rigor
pipeline: direct          # For directly-plannable work (IDEA → MP)
pipeline: one_off         # For single-shot execution (no plan artifact)
pipeline: null            # Not yet routed (default for captured/refining IDEAs)
```

When ambiguous, the recommended posture is `architecture` (default to the rigorous
path). The pipeline field is populated at approval-time, not capture-time.

## Context

This is a KD-38.5 anti-regression fixture. The value `standard` was considered and
rejected during the ACS-0038 design process because it carries no information about
what the path actually does. Adopters familiar with other workflow systems may
default to naming a path "standard" — this fixture catches that habit.

---
Tags: [anti-fixture, level-1-failure]

---

Produced: "2026-05-01T21:00:00Z"
By: Rhumb Protocol Contributors - https://rhumbprotocol.dev
Copyright: Copyright (c) 2026 Rhumb Protocol Contributors. All Rights Reserved.
