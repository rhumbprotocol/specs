# Rhumb Protocol Papers

This document defines the first public paper set for Rhumb Workflow Protocol™.
The PDFs are distribution artifacts generated from the website paper pages.
The maintained sources remain this repository's Markdown specifications,
schemas, templates, examples, and conformance notes.

## Paper Set

| Paper | Audience | Minimum Substance Bar | PDF Output |
|---|---|---|---|
| Rhumb Workflow Protocol Executive Brief | Executives, product leaders, technical leads | 1-2 polished pages with business rationale, protocol boundaries, reference implementation position, adoption path, and known current-state risk | `executive-brief.pdf` |
| Rhumb Workflow Protocol Implementation Brief | Tool builders, platform engineers, AI workflow implementers | Multi-page technical brief covering artifacts, schemas, profiles, conformance, validator behavior, Meridian compatibility, and drift decisions | `implementation-brief.pdf` |
| Rhumb Workflow Protocol Specification Reading Bundle | Implementers, architects, standards reviewers | Curated reading packet mapping the repository's canonical materials to an implementation sequence | `specification-bundle.pdf` |

## Source Map

| Topic | Canonical Source |
|---|---|
| Core artifact model | `docs/GETTING-STARTED.md`, `docs/PROTOCOL.md`, `templates/`, `spec/schemas/` |
| Implementation profiles | `docs/IMPLEMENTATION-PROFILES.md` |
| Conformance and validator behavior | `docs/CONFORMANCE.md`, `conformance/README.md`, `conformance/src/validators/` |
| Sequence grammar | `spec/sequence.grammar`, `spec/sequence-parser.md`, `util/sequence-parser.ts` |
| IDEA lifecycle | `spec/lifecycle/idea-lifecycle.spec.md`, `spec/schemas/idea.schema.json`, `spec/schemas/lifecycle.schema.json` |
| Architecture path | `templates/IDEA.md.template`, `templates/AVD.md.template`, `templates/ACS.md.template`, `templates/PLAN.md.template` |
| Meridian compatibility | `docs/CONFORMANCE.md`, `docs/IMPLEMENTATION-PROFILES.md`, Meridian |

## Paper 1: Executive Brief

### Required Argument

Rhumb is not another project-management app. It is the neutral protocol layer
for durable AI workflow records. It defines the artifacts and lifecycle that
make AI-assisted delivery auditable and portable across tools.

### Must Cover

- The problem: AI work often disappears into chat transcripts, vendor project
  formats, or tool-specific state.
- The protocol move: plain Markdown/YAML artifacts plus schemas, templates,
  lifecycle semantics, and conformance checks.
- The adoption path: start with intake, plan, state, and handoff; add manifest,
  dependencies, audits, lifecycle, and validator-backed conformance as risk
  increases.
- The reference implementation: Meridian proves the protocol, but Rhumb must
  remain vendor-neutral.
- The honest current-state note: Meridian and Rhumb are still reconciling
  templates and discovery behavior; the public protocol should expose that
  clearly instead of claiming false finality.

### Visuals

- Artifact constellation.
- Adoption ladder from Minimal to Full.
- Meridian as reference implementation orbiting the protocol, not owning it.

## Paper 2: Implementation Brief

### Required Argument

An RWP implementation must preserve artifact semantics and validation behavior.
It does not need to copy Meridian's directory tree unless it claims Meridian
Reference Profile compatibility.

### Must Cover

- Core artifacts: `INTAKE.yaml`, `PLAN.md`, `state.yaml`, `manifest.yaml`,
  `dependencies.yaml`, handoffs, prompts, audits, IDEA, AVD, ACS.
- Distinction between protocol artifacts and implementation runtime state.
- Core File-Tree Profile versus Meridian Reference Profile.
- Plan-level `state.yaml` versus `.meridian/.private/runtime/STATE.yaml`.
- Conformance categories: schema, template, workflow, adapter, grammar.
- Exit codes and why multi-category failure is reported as exit `6`.
- RWP 0.26.0 reconciliation decisions: canonical template names,
  `MP-NNNN-short-name` plan IDs, greenfield status vocabulary, and `A-Z`
  sub-phase language.
- Safe extension mechanisms and namespaced implementation fields.

### Visuals

- Core artifact constellation.
- Meridian profile tree.
- Validator category pipeline.
- State/status reconciliation table.

## Paper 3: Specification Reading Bundle

### Required Argument

The full repository is the spec. A reader needs a path through it, not a pasted
copy of website text.

### Reading Order

1. `docs/GETTING-STARTED.md` - practical adoption and first workflow.
2. `docs/IMPLEMENTATION-PROFILES.md` - storage layout decisions before wiring
   tools.
3. `docs/PROTOCOL.md` - formal protocol contract.
4. `spec/conformance-levels.md` and `docs/CONFORMANCE.md` - claim depth and
   validator behavior.
5. `spec/lifecycle/idea-lifecycle.spec.md` - IDEA lifecycle state machine.
6. `spec/sequence.grammar` and `spec/sequence-parser.md` - phase identifier and
   sequence semantics.
7. `templates/` and `examples/` - implementation fixtures.
8. `integrations/` - adapter-specific context after the core contract is clear.

### Must Cover

- Which documents are normative, which are guides, and which are generated
  distribution surfaces.
- How an implementer should validate a first implementation.
- How to avoid confusing Meridian-specific storage with core protocol rules.
- What remains unresolved or in reconciliation.

---

Produced:
  - when: 2026-05-06T16:25:00Z
  - by: YAKKL® Meridian™- https://meridian.yakkl.com
  - copyright: Copyright © 2026 YAKKL Inc. All Rights Reserved.
