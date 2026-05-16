# Rhumb Workflow Protocol: Plan Document

---

plan_id: MP-3333-missing-handoff
request_id: null
name: Missing Handoff File (negative fixture)
classification: public
status: processing
phases: 1
current_phase: P-01
rwp_version: "0.28.0"

---

# MP-3333-missing-handoff: Missing Handoff File Fixture

state.yaml's `handoffs.handoff_files` lists a path that does not exist on
disk. INV-4 should trip. INV-3 should also trip because `handoffs.last_handoff`
points at the same nonexistent file.
