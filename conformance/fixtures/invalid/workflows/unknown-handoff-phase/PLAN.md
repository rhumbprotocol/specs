# Rhumb Workflow Protocol: Plan Document

---

plan_id: RWP-UNKHO-4444
request_id: null
name: Unknown Handoff Phase (negative fixture)
classification: public
status: processing
phases: 1
current_phase: P-01
rwp_version: "0.25.1"

---

# RWP-UNKHO-4444: Unknown Handoff Phase Fixture

state.yaml's `handoffs.handoff_files[0].phase` references `P-99` which is not
defined under `phases:`. INV-5 should trip. The handoff file itself exists on
disk so INV-3 + INV-4 are satisfied — INV-5 is exercised in isolation.
