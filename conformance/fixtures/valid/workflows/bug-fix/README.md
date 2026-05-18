# Example: Bug Fix - Disable Submit on Click

This example demonstrates RWP for a **single-phase bug fix** - the simplest realistic RWP
workflow. It fixes a double-click double-submission bug in a shared form button component
that was producing duplicate database records.

## What's Included

| File | RWP Artifact Type | Purpose |
|------|-------------------|---------|
| `INTAKE.yaml` | Intake | Captures the bug report: pain points, requirements, constraints, success criteria |
| `PLAN.md` | Plan | Single-phase plan with reproduce → fix → regression test → verify |
| `state.yaml` | State | Tracks execution; shown in completed state |
| `handoffs/HO-MP-0001-disable-submit-P-01-2026-04-25.md` | Handoff | Closure handoff with deliverables, design decisions, sign-off |

## Why This Example

This is the lightest realistic RWP workflow:

- **1 phase** (find → fix → verify), no orchestration needed
- **Single developer**, no team coordination
- **~1.5 hours** of total work
- **No audit** (mechanical verification - test passes, build clean - is sufficient)
- **Minimal custom fields** - only REQUIRED fields populated

It deliberately demonstrates that RWP scales *down* to small work, not just up to large
projects. You don't need multi-phase orchestration for a bug fix; you do still benefit from
the structured artifacts (INTAKE captures the problem, PLAN documents the approach, handoff
records the closure).

## When to Use This Pattern

Single-phase RWP fits when:

- The work has a single root cause and a clear verification gate
- It fits in one focused session (under 2 hours)
- Verification is mechanical: test passes, build succeeds, smoke test confirms
- Rollback is trivial (revert one or two files)
- No handoff to another developer or session is needed

If your bug fix needs investigation across multiple sessions (e.g., reproducing an
intermittent race condition, or coordinating a fix across multiple services), use the
2-phase pattern from `examples/simple-feature/` instead - phase 1 for investigation,
phase 2 for the fix.

## How to Use

1. Read `INTAKE.yaml` to see how to capture a bug report in RWP form (problem statement,
   pain points with measurable impact, requirements with verification criteria)
2. Read `PLAN.md` to see how a single-phase plan decomposes into tasks (reproduce → locate →
   fix → regression test → verify) without overengineering
3. Read `state.yaml` to see execution tracking on the smallest possible plan
4. Read the handoff to see how closure is documented (deliverables, design decisions,
   sign-off, optional follow-up considerations)

## Comparison With Other Examples

| Example | Phases | Audience | When to use |
|---------|--------|----------|-------------|
| `bug-fix/` (this example) | 1 | "I have a defined bug with a clear fix" | Single-cause defects with mechanical verification |
| `simple-feature/` | 2 | "Single-developer feature with implement + polish" | Features that benefit from explicit polish/edge-case phase |
| `multi-phase/` | 5 (with sub-phases) | "Multi-team project with handoffs and audits" | Cross-team coordination, long-running work, formal audit needs |

All three are RWP-compliant. Pick the shape that matches the actual scope of your work - RWP
doesn't prescribe complexity.

## Key Takeaway

RWP scales down to a 4-file workflow for a single-developer bug fix. The discipline of
writing an INTAKE (what's the bug?) and a PLAN (what's the fix?) before opening the editor
is valuable even when the work is small - it forces the kind of clarity that prevents
"quick fixes" from missing edge cases. The artifacts are the durable record of *why* the
change was made, not just *what* changed.

---

Rhumb Workflow Protocol (RWP) v0.29.0 - https://rhumbprotocol.dev
