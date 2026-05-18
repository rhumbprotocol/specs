# Rhumb Workflow Protocol: Plan Document

---

plan_id: MP-2222-current-phase-mismatch
request_id: null
name: Current Phase Mismatch (negative fixture)
classification: public
status: processing
phases: 2
current_phase: P-01
rwp_version: "0.28.1"

---

# MP-2222-current-phase-mismatch: Current Phase Mismatch Fixture

PLAN.md frontmatter says `current_phase: P-01`. state.yaml says
`execution.current_phase: P-02`. Both phases exist in `phases:` so
INV-5 is satisfied — only INV-2 should trip.
