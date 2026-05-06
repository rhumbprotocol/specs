---
id: IDEA-9005
type: idea
title: "Support XML as an alternative artifact format"
status: discarded
classification: public
created: "2026-03-05T16:00:00Z"
updated: "2026-03-12T14:20:00Z"

authors:
  - name: "Jordan Blake"
    role: "engineer"
tags:
  - format
  - interoperability

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

discarded_by: "Hans Jones"
discarded_at: "2026-03-12T14:20:00Z"
discarded_reason: "XML adds significant complexity (schema translation, dual-format validation, tooling parity) for negligible adoption benefit. The target audience overwhelmingly uses Markdown+YAML. Enterprise consumers who require XML can transform at ingestion time using standard XSLT/jq pipelines without protocol-level support."
---

# IDEA-9005: Support XML as an alternative artifact format

## Context

A potential enterprise adopter asked whether RWP artifacts could be authored in
XML instead of Markdown+YAML. Their internal tooling pipeline consumes XML
natively and they would prefer not to add a YAML parser.

## Initial Considerations

- Would require translating all JSON Schemas into XML Schema (XSD)
- Validation tooling would need dual-format support
- Template generation would need XML equivalents
- Lifecycle spec prose would need format-agnostic language

## The "Why"

Enterprise interoperability is valuable, but not at the cost of protocol
complexity that burdens the majority of adopters who use Markdown+YAML.

## Strategic Value

Low. The XML-native enterprise segment is shrinking. Modern enterprise tooling
increasingly consumes JSON/YAML. The effort-to-adoption ratio is unfavorable.

## Key Concepts / Pillars

1. **Single canonical format**: Markdown+YAML remains the single source of truth.
2. **Transform at the boundary**: Consumers needing other formats can transform
   at ingestion; the protocol does not need to support authoring in those formats.

## Target Audience

Enterprise adopters with legacy XML pipelines.

## Proposed Execution (High Level)

- (Not pursued — see discard reason above)

## Open Questions / Unknowns

- (Moot — discarded)

## References

- idea.schema.json (the JSON Schema that would have needed XSD translation)

---
Tags: [idea]

---

Produced: "2026-03-12T14:20:00Z"
By: Rhumb Protocol Contributors - https://rhumbprotocol.dev
Copyright: Copyright (c) 2026 Rhumb Protocol Contributors. All Rights Reserved.
