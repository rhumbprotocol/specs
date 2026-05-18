# Rhumb Workflow Protocol: Plan Document

---

plan_id: MP-4444-unknown-handoff
request_id: null
name: Unknown Handoff Phase (negative fixture)
classification: public
status: processing
phases: 1
current_phase: P-01
rwp_version: "0.28.1"

---

# MP-4444-unknown-handoff: Unknown Handoff Phase Fixture

state.yaml's `handoffs.handoff_files[0].phase` references `P-99` which is not
defined under `phases:`. INV-5 should trip. The handoff file itself exists on
disk so INV-3 + INV-4 are satisfied — INV-5 is exercised in isolation.
