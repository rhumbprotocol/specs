# Example: Multi-Phase Project - API Rate Limiter Service

This example demonstrates RWP for a medium-sized project with 5 phases (including
sub-phases), multi-team coordination, and full best-practice artifacts. It shows the
**REQUIRED + RECOMMENDED** approach - comprehensive RWP usage suitable for production
projects.

## What's Included

| File | RWP Artifact Type | Purpose |
|------|-------------------|---------|
| `INTAKE.yaml` | Intake | Detailed problem capture with pain points and constraints |
| `PLAN.md` | Plan | 5-phase plan with sub-phases, dependency graph, risk assessment |
| `state.yaml` | State | Execution tracking after P-02-A completion |
| `handoffs/HO-RWP-0087-P-01-2026-02-15.md` | Handoff | P-01 completion handoff |
| `handoffs/HO-RWP-0087-P-02-A-2026-02-15.md` | Handoff | P-02-A sub-phase handoff |

## Why This Example

This demonstrates RWP at production scale:
- **5 phases** with sub-phases (P-02 split into A/B/C for crash resilience)
- **Multi-team**: Backend team owns P-01-P-03, Platform team owns P-04
- **Audit checkpoint** between P-03 and P-04
- **Risk assessment** and rollback plan
- **Custom fields** for team-specific tracking
- **Dependency graph** showing parallel work opportunities

## How to Use

1. Read `INTAKE.yaml` - note how pain points map to requirements
2. Read `PLAN.md` - note the sub-phase pattern in P-02 and the audit between P-03/P-04
3. Read `state.yaml` - shows mid-execution tracking with deliverables list
4. Read the handoffs - note how context rolls forward between sessions

## Key Takeaway

For larger projects, RWP's sub-phases provide crash resilience (~30 min units), handoffs
maintain continuity across sessions and team members, and audits catch issues before
they compound. The overhead pays for itself in reduced rework.

---

Rhumb Workflow Protocol (RWP) v0.25.1 - https://rhumbprotocol.dev
