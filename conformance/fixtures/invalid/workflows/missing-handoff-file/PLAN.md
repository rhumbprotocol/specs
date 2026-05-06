# Rhumb Workflow Protocol: Plan Document

---

plan_id: RWP-MISSINGHO-3333
request_id: null
name: Missing Handoff File (negative fixture)
classification: public
status: processing
phases: 1
current_phase: P-01
rwp_version: "0.25.1"

---

# RWP-MISSINGHO-3333: Missing Handoff File Fixture

state.yaml's `handoffs.handoff_files` lists a path that does not exist on
disk. INV-4 should trip. INV-3 should also trip because `handoffs.last_handoff`
points at the same nonexistent file.
