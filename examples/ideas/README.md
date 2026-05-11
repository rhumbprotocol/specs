<!--
SPDX-License-Identifier: Apache-2.0
Copyright (c) 2026 Rhumb Protocol Contributors

RWP version: 0.26.0
-->

# IDEA Examples

Worked examples of [IDEA](../../templates/IDEA.md.template) artifacts in various lifecycle states. Each file is a complete, self-contained IDEA with YAML frontmatter that validates against [`idea.schema.json`](../../spec/schemas/idea.schema.json).

For the lifecycle state machine, transition rules, and per-state required fields, see [`idea-lifecycle.spec.md`](../../spec/lifecycle/idea-lifecycle.spec.md).

---

## Positive Examples

Valid IDEAs demonstrating correct frontmatter for each lifecycle state. All pass Level 1 validation (JSON Schema) and satisfy the Level 3 per-state required-fields contract.

| File | State | ID | Description |
|------|-------|----|-------------|
| [`captured-cli-error-messages.md`](captured-cli-error-messages.md) | `captured` | IDEA-9001 | Minimum-viable IDEA at initial capture. No lifecycle metadata populated. |
| [`approved-webhook-notifications.md`](approved-webhook-notifications.md) | `approved` | IDEA-9002 | Approval ceremony complete: `approved_by`, `approved_at`, `approval_policy` populated. |
| [`promoted-schema-versioning.md`](promoted-schema-versioning.md) | `promoted` | IDEA-9003 | Promoted into AVD-0009. `promoted_to`, `promoted_at`, `promoted_pipeline` populated; approval fields carried forward. |
| [`parked-visual-state-diagram.md`](parked-visual-state-diagram.md) | `parked` | IDEA-9004 | Deferred with `parked_as`, `parked_reason`, and `parked_until`. |
| [`discarded-xml-artifact-format.md`](discarded-xml-artifact-format.md) | `discarded` | IDEA-9005 | Rejected with `discarded_by`, `discarded_at`, `discarded_reason`. Terminal state. |

---

## Anti-Fixtures

Invalid IDEAs demonstrating common authoring errors. Each file includes the incorrect frontmatter, an explanation of the error, the expected validation message, and a corrected version. Use these to understand what the schema rejects and why.

| File | Level 1 | Error | Lesson |
|------|---------|-------|--------|
| [`unquoted-timestamp.md`](anti-fixtures/unquoted-timestamp.md) | FAIL | `/created`: must be string (type) | YAML 1.1 auto-casts unquoted ISO-8601 values to Date objects. Always quote timestamps. |
| [`missing-per-state-fields.md`](anti-fixtures/missing-per-state-fields.md) | PASS | (Level 3 FAIL) | JSON Schema alone cannot enforce per-state required fields. A Level 3 validator catches this. |
| [`invalid-pipeline-value.md`](anti-fixtures/invalid-pipeline-value.md) | FAIL | `/pipeline`: must be equal to allowed values (enum) | `pipeline: standard` is not valid; use `direct`, `architecture`, or `one_off`. |
| [`additional-property-smuggling.md`](anti-fixtures/additional-property-smuggling.md) | FAIL | must NOT have additional properties | `additionalProperties: false` rejects any field not declared in the schema. |
| [`invalid-status-value.md`](anti-fixtures/invalid-status-value.md) | FAIL | `/status`: must be equal to allowed values (enum) | `status: archived` is not in the 7-state lifecycle enum. |

---

## Validation

To validate examples against the schema, use [rhumb-validate](../../README.md) (ACS-0041) or run a Level 1 check manually:

```bash
# Requires: ajv (8.x+), ajv-formats, js-yaml
# 1. Extract YAML frontmatter from the Markdown file.
# 2. Parse with js-yaml using { schema: JSON_SCHEMA } to avoid Date auto-cast.
# 3. Validate against spec/schemas/idea.schema.json using ajv (draft 2020-12, strict).
#
# Anti-fixtures: unquoted-timestamp.md must be parsed WITHOUT JSON_SCHEMA
# (default YAML 1.1 schema) to trigger the Date auto-cast it demonstrates.
```

## References

- [IDEA Template](../../templates/IDEA.md.template) — paste-and-go template for new IDEAs
- [`idea.schema.json`](../../spec/schemas/idea.schema.json) — Level 1 validation schema
- [`lifecycle.schema.json`](../../spec/schemas/lifecycle.schema.json) — Level 2 state-machine schema
- [`idea-lifecycle.spec.md`](../../spec/lifecycle/idea-lifecycle.spec.md) — Normative lifecycle specification

---

Produced:
  - when: 2026-05-02T14:15:00Z
  - by: Rhumb Protocol™ Contributors - https://rhumbprotocol.dev
  - copyright: Copyright © 2026 Rhumb Protocol Contributors. All Rights Reserved.
