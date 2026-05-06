---
id: IDEA-9001
type: idea
title: "Improve CLI error messages with actionable suggestions"
status: captured
classification: public
created: "2026-04-20T09:15:00Z"
updated: "2026-04-20T09:15:00Z"

authors:
  - name: "Priya Sharma"
    role: "engineer"
tags:
  - developer-experience
  - cli

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

# IDEA-9001: Improve CLI error messages with actionable suggestions

## Context

Users running `rhumb validate` on malformed artifacts receive cryptic JSON Schema
error paths (e.g., `#/properties/status/enum`) without guidance on how to fix the
issue. Support channels show repeated questions about the same three errors.

## Initial Considerations

- Error messages should include a "did you mean?" suggestion when an enum value
  is close to a valid option (e.g., `standrad` → `direct`)
- Reference the relevant spec section in each error message
- Keep the error formatter decoupled from the validator so it can evolve
  independently

## The "Why"

First-time adopters abandon the protocol at the validation step if errors are
opaque. Actionable messages reduce time-to-first-valid-artifact from hours to
minutes.

## Strategic Value

Developer experience is a force multiplier for adoption. Protocols with good
error messages get recommended; protocols with bad ones get forked.

## Key Concepts / Pillars

1. **Actionable over accurate**: A suggestion that's right 90% of the time is
   more useful than a perfectly precise but unhelpful error path.
2. **Layered detail**: Short message first, `--verbose` for full schema path.

## Target Audience

New adopters authoring their first IDEA or plan artifact using the CLI.

## Proposed Execution (High Level)

- Audit the top 10 validation errors from support channels
- Design a message format (short line + suggestion + spec reference)
- Implement the formatter as a pluggable layer over ajv output

## Open Questions / Unknowns

* [ ] Should suggestions be localized or English-only for v1?
* [ ] Is there a performance budget for fuzzy-matching enum values?

## References

- ACS-0041 (rhumb-validate specification)

---
Tags: [idea]

---

Produced: "2026-04-20T09:15:00Z"
By: Rhumb Protocol Contributors - https://rhumbprotocol.dev
Copyright: Copyright (c) 2026 Rhumb Protocol Contributors. All Rights Reserved.
