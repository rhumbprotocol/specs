---
id: IDEA-9901
type: idea
title: "Anti-fixture: Unquoted ISO-8601 timestamp"
status: captured
classification: public
created: 2026-05-01T19:00:00Z
updated: 2026-05-01T19:00:00Z

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
---

# ANTI-FIXTURE: Unquoted ISO-8601 Timestamp

## What This Demonstrates

**Error**: The `created` and `updated` fields contain unquoted ISO-8601 timestamps.

In YAML 1.1 (the default schema used by js-yaml and most YAML parsers), an unquoted
value like `2026-05-01T19:00:00Z` is auto-cast to a native Date object. When the
parsed object is validated against idea.schema.json, the validator sees a Date object
where it expects `type: "string"` — and fails.

## Expected Error

```
Level 1 FAIL — /created: must be string (received object/Date)
Level 1 FAIL — /updated: must be string (received object/Date)
```

The exact error depends on whether the validator receives the raw parsed object
(type mismatch: Date vs string) or a JSON-serialized round-trip (passes, because
JSON.stringify converts Date to string). This anti-fixture demonstrates the RAW
parse path — the most common trap for adopters who parse frontmatter and validate
in the same process without JSON serialization.

## How To Fix

Quote the timestamp values in YAML:

```yaml
created: "2026-05-01T19:00:00Z"
updated: "2026-05-01T19:00:00Z"
```

Or use `yaml.JSON_SCHEMA` when parsing (disables auto-casting), though quoting is
the recommended practice because it makes the intent explicit regardless of parser
configuration.

## Context

This is the #1 mistake adopters make when writing IDEA frontmatter by hand.
YAML's timestamp auto-casting is a well-known footgun documented in the yaml-spec
FAQ. The RWP IDEA template includes quoted timestamps as the canonical pattern.

---
Tags: [anti-fixture, level-1-failure]

---

Produced: "2026-05-01T21:00:00Z"
By: Rhumb Protocol Contributors - https://rhumbprotocol.dev
Copyright: Copyright (c) 2026 Rhumb Protocol Contributors. All Rights Reserved.
