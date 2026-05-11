# Rhumb Workflow Protocol: Plan Document

---

plan_id: MP-9999-plan-side
request_id: null
name: Plan ID Mismatch (negative fixture)
classification: public
status: processing
phases: 1
current_phase: P-01
rwp_version: "0.26.0"

---

# MP-9999-plan-side: Plan ID Mismatch Fixture

This fixture intentionally has `plan_id: MP-9999-plan-side` in PLAN.md while
state.yaml has `plan_id: MP-1111-state-side`. The workflow validator's INV-1
should detect this disagreement and emit a `WorkflowBreak` failure.
