# Rhumb Workflow Protocol: Plan Document

---

plan_id: RWP-PLAN-9999
request_id: null
name: Plan ID Mismatch (negative fixture)
classification: public
status: processing
phases: 1
current_phase: P-01
rwp_version: "0.25.1"

---

# RWP-PLAN-9999: Plan ID Mismatch Fixture

This fixture intentionally has `plan_id: RWP-PLAN-9999` in PLAN.md while
state.yaml has `plan_id: RWP-STATE-1111`. The workflow validator's INV-1
should detect this disagreement and emit a `WorkflowBreak` failure.
