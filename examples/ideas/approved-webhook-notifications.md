---
id: IDEA-9002
type: idea
title: "Webhook notifications for lifecycle state transitions"
status: approved
classification: public
created: "2026-03-10T14:30:00Z"
updated: "2026-04-18T11:00:00Z"

authors:
  - name: "Carlos Rivera"
    role: "architect"
  - name: "Mei Lin"
    role: "engineer"
tags:
  - integrations
  - automation
  - lifecycle

parent: null
children: []

pipeline: architecture

approved_by: "Carlos Rivera"
approved_at: "2026-04-18T11:00:00Z"
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

# IDEA-9002: Webhook notifications for lifecycle state transitions

## Context

Teams using RWP in CI/CD pipelines need to react to lifecycle events (e.g.,
trigger a build when an IDEA reaches `approved`, notify a Slack channel when
one is `discarded`). Currently there is no standard mechanism for external
systems to subscribe to state transitions.

## Initial Considerations

- Webhook payload should be a JSON envelope containing the artifact frontmatter
  plus transition metadata (from_state, to_state, timestamp, actor)
- Delivery guarantees: at-least-once with exponential backoff
- Registration is per-repository, not global; keeps blast radius small
- Must not require the webhook consumer to understand the full schema — just the
  envelope shape

## The "Why"

Workflow automation is table-stakes for enterprise adoption. Without it,
organizations wire up ad-hoc file watchers that break on every spec bump.

## Strategic Value

Positions the protocol as an integration-ready platform rather than a
documentation format. Enterprise buyers evaluate integration surface before
content features.

## Key Concepts / Pillars

1. **Event envelope**: Standardized shape (`{ event, artifact, transition, timestamp }`)
   that any consumer can parse without deep RWP knowledge.
2. **Opt-in subscription**: No webhooks fire unless explicitly registered.
3. **Idempotency keys**: Every event carries a unique ID so consumers can
   deduplicate retries.

## Target Audience

Platform teams integrating RWP lifecycle events into CI/CD, ChatOps, and
internal tooling.

## Proposed Execution (High Level)

- Define the event envelope schema (new spec artifact)
- Define a `.rhumb/webhooks.yaml` registration format
- Implement dispatch in `rhumb-validate` post-transition hook
- Document retry semantics and failure modes

## Open Questions / Unknowns

* [ ] Should webhook payloads include the full Markdown body or frontmatter only?
* [ ] How to handle secrets for HMAC-signed payloads without leaking into the repo?

## Resolved Decisions

* [x] **Pipeline path** — **architecture.** The envelope schema, registration
  format, and dispatch logic warrant AVD-level design before implementation.
* [x] **Approval policy** — **review.** Two authors agreed on scope and path.

## References

- idea-lifecycle.spec.md §7.2 (State transition recording)

---
Tags: [idea]

---

Produced: "2026-04-18T11:00:00Z"
By: Rhumb Protocol Contributors - https://rhumbprotocol.dev
Copyright: Copyright (c) 2026 Rhumb Protocol Contributors. All Rights Reserved.
