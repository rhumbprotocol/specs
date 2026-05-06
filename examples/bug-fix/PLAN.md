# Rhumb Workflow Protocol: Plan Document

---

plan_id: RWP-0001
request_id: null
name: Disable Submit on Click - Fix Double-Submission
classification: public
status: completed
created: 2026-04-25T09:45:00Z
author: Developer
phases: 1
current_phase: P-01
started: 2026-04-25T09:45:00Z
completed: 2026-04-25T11:30:00Z
parent: null
rwp_version: "0.25.1"
dependencies: []
packages:
  - frontend-app

---

# RWP-0001: Disable Submit on Click - Fix Double-Submission

---

## Executive Summary

Single-phase bug fix. The shared `SubmitButton` component does not disable on click, allowing
fast double-clicks to fire two network requests and produce duplicate records server-side. Fix
the component to disable on first click, show a spinner, and re-enable on response. Add a
regression test asserting exactly one network request per double-click.

This is the simplest realistic RWP workflow: one phase, single developer, no audit, minimal
artifacts. It demonstrates how RWP handles the "find → fix → verify" rhythm of a bug fix
without the overhead of multi-phase orchestration.

---

## Problem Statement

The shared `src/components/forms/SubmitButton` component handles its `onClick` synchronously
without disabling itself between the click and the server response. Users who tap the button
twice quickly (often inadvertently, e.g. mobile double-tap or impatient retry) produce two
network requests for the same submission. Server-side, this manifests as duplicate database
records - confirmed by a 30-day audit showing 47 duplicate contact-form rows from
same-IP+user-agent submissions within a 200ms window.

The fix is small (one component file plus a test), but the bug is high-impact because the
`SubmitButton` component is shared across all forms in the application.

---

## Phase Breakdown

### P-01: Fix and Verify

**Depends On**: None
**Estimated Duration**: 90 minutes
**Actual Duration**: 105 minutes

**Objective**: Modify the shared `SubmitButton` component to disable on click, show a visual
in-flight indicator, and re-enable on server response. Add a regression test.

**Tasks**:

1. **Reproduce** - write a failing test that double-clicks the button and asserts two
   network requests fire (proves the bug exists in the current codebase)
2. **Locate** - confirm the bug is in `SubmitButton.svelte` (not in the form parents); read
   the current implementation
3. **Fix** - add `disabled` state bound to an `isSubmitting` reactive variable; show a
   spinner while submitting; re-enable on response (success or error)
4. **Convert the failing test to a passing regression test** - same double-click setup, but
   now assert exactly one network request
5. **Verify across consumer forms** - run the existing test suites for `ContactForm` and
   `OrderForm` to confirm no integration breakage
6. **Manual smoke test** - open the app, double-click submit on the contact form, verify
   only one confirmation email arrives

**Files to Create/Modify**:

| File | Action | Description |
|------|--------|-------------|
| `src/components/forms/SubmitButton/SubmitButton.svelte` | Modify | Add `isSubmitting` state, `disabled` binding, spinner slot |
| `src/components/forms/SubmitButton/SubmitButton.test.ts` | Modify | Add regression test for double-click → single-request invariant |
| `src/components/forms/SubmitButton/Spinner.svelte` | Create | Minimal inline spinner (no new dependency) |

**Verification**:
```bash
# Regression test passes
pnpm test src/components/forms/SubmitButton/SubmitButton.test.ts

# Full form suite passes (no integration breakage)
pnpm test src/components/forms/

# Build clean
pnpm build
```

**Expected Results**:
- Regression test passes (double-click produces exactly 1 network request)
- Spinner appears within 100ms of click
- Button re-enables on server response
- ContactForm and OrderForm test suites still pass
- Build succeeds with 0 errors

---

## Phase Dependency Graph

```
P-01 (Fix and Verify) - single phase, no dependencies
```

---

## Dependencies

| Dependency | Type | Status | Notes |
|------------|------|--------|-------|
| Svelte 5 | Package | Met | Already in project |
| vitest | Package | Met | Test framework already configured |

---

## Files Reference

### New Files

| File | Purpose |
|------|---------|
| `src/components/forms/SubmitButton/Spinner.svelte` | Inline spinner shown during in-flight submission |

### Modified Files

| File | Changes |
|------|---------|
| `src/components/forms/SubmitButton/SubmitButton.svelte` | Add `isSubmitting` state; `disabled` binding; spinner integration |
| `src/components/forms/SubmitButton/SubmitButton.test.ts` | Add double-click regression test |

---

## Success Criteria

1. Double-click on SubmitButton produces exactly one network request (regression test asserts)
2. Spinner appears within 100ms of click
3. Button re-enables on server response (success or error)
4. ContactForm and OrderForm test suites pass without modification
5. Build succeeds with zero errors
6. Manual smoke test confirms only one confirmation email per submission

---

## Risk Assessment

| Risk | Likelihood | Impact | Mitigation |
|------|------------|--------|------------|
| `disabled` attribute breaks existing form keyboard navigation | Low | Medium | Test confirms `Tab` still skips disabled-and-submitting button correctly |
| Async response handler error causes button to stay disabled forever | Medium | High | Use `try/finally` so `isSubmitting=false` is always set; add a 30-second safety timeout |
| Spinner CSS conflicts with form layout | Low | Low | Spinner is absolutely positioned within button; sized to button height |

---

## Rollback Plan

If this fix needs to be reverted:

1. Revert `src/components/forms/SubmitButton/SubmitButton.svelte`
2. Delete `src/components/forms/SubmitButton/Spinner.svelte`
3. Remove the regression test from `SubmitButton.test.ts`

The bug returns but no other code depends on the new behavior.

---

## Why This Is a Single-Phase Plan

This bug has a single root cause, a small fix, and a clear verification gate. There is no
sequencing or coordination concern that would justify decomposition. RWP single-phase plans
are appropriate when:

- The work fits in one focused session (<2 hours)
- There is no handoff to another developer or session
- Verification is mechanical (test passes / build clean), not multi-step
- Rollback is trivial

For comparison: see `examples/simple-feature/` (2 phases - implement + polish) and
`examples/multi-phase/` (5 phases with sub-phases - multi-team coordination). All three are
RWP-compliant; they differ in *how much workflow scaffolding the work actually warrants*.

---

Produced: 2026-04-25T11:30:00Z
By: Rhumb Protocol Contributors - https://rhumbprotocol.dev
