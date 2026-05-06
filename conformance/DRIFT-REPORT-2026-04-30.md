---
id: DRIFT-REPORT-2026-04-30
type: drift_audit_report
title: "Meridian ↔ Rhumb Drift Audit — 2026-04-30"
status: draft
classification: confidential
created: 2026-04-30T20:53:35Z
updated: 2026-04-30T20:53:35Z
updated_by: "MP-0275 P-01 drift-audit.zsh"
update_summary: "Initial emission from .meridian/scripts/drift-audit.zsh"
plan_id: MP-0275
phase: P-01
acs_ref: ACS-0014
avd_ref: AVD-0004
rwp_root: "packages/rhumb-protocol/templates"
meridian_root: ".meridian/templates"
counts:
  rwp_total: 17
  meridian_total: 190
  duplicate_identical: 0
  duplicate_divergent: 8
  rwp_only: 9
  meridian_only: 182
  layout_variant_basenames: 3
---

# Drift Audit Report — 2026-04-30

## 1. Summary

| Bucket                    | Count                       |
|---------------------------|-----------------------------|
| RWP tree total            | 17                |
| Meridian tree total       | 190                |
| `duplicate_identical`     | 0              |
| `duplicate_divergent`     | 8              |
| `rwp_only`                | 9               |
| `meridian_only`           | 182          |
| Layout-variant basenames  | 3         |

**Trees compared**:
- RWP: `packages/rhumb-protocol/templates`
- Meridian: `.meridian/templates`

**Method**: Strict relative-path comparison. SHA-256 hashes computed for
each regular file (excluding `.DS_Store` / `.realm` noise). Layout-variant
basenames are emitted separately (same filename, different relative paths
across trees) so refactor planning can decide whether to harmonize layout
or treat as legitimate non-overlap.


## 2. `duplicate_identical` — byte-identical in both trees

_None._

## 3. `duplicate_divergent` — copy in both trees, content differs

**Each entry below requires a per-file resolution call in P-02 / P-03.**
Default direction-of-truth is RWP (KD-14.8); overrides must be explicit.

| Relative Path | RWP SHA-256 | Meridian SHA-256 | Recommended Resolution |
|---------------|-------------|-------------------|------------------------|
| `DEPENDENCIES.yaml.template` | `f5ec0ddd4b31` | `e10045c496f8` | _author narrative below_ |
| `INTAKE.yaml.template` | `193cfcdee6ac` | `91fbf06783d4` | _author narrative below_ |
| `MANIFEST-PLAN.yaml.template` | `2c69c2a78eb2` | `775a54e81d96` | _author narrative below_ |
| `MASTERPLAN.yaml.template` | `6ae081905f04` | `e943d983dd26` | _author narrative below_ |
| `PLAN-STATE.yaml.template` | `eedd2f69eee9` | `e206a7387ce4` | _author narrative below_ |
| `PLAN.md.template` | `c208a95257af` | `1ac0e9088ae3` | _author narrative below_ |
| `PROMPT.md.template` | `2e3441f870b7` | `9264722ddfd3` | _author narrative below_ |
| `START-PROMPT.md.template` | `389cbe7b59c5` | `5f912fc1bed7` | _author narrative below_ |

### 3.1 Canonical divergence pattern

All 8 divergent files share **one consistent pattern**: the Meridian copy
is a *Meridian-augmented overlay* of the RWP base template. Each Meridian
copy adds:

1. A `meridian_version: "{{MERIDIAN_VERSION}}"` substitution marker as
   the first frontmatter line.
2. Meridian-specific rule cross-references (e.g., RULE-63, RULE-87,
   AVD-0005 KD-05.5 enum pins).
3. Additional guidance prose tuned to Meridian's lifecycle and CLI flow.
4. In two cases (`PLAN.md.template`, `PROMPT.md.template`), substantial
   expansion — Meridian's `PROMPT.md.template` is ~535 lines vs RWP's
   ~44, indicating the files are not a simple overlay relationship but a
   "shared skeleton, divergent expansion" relationship.

**Strategic implication for P-02 (KD-14.3 lock)**: a viable consumption
mechanism likely needs to support a **two-layer materialization**:
- **Layer 0**: RWP base template (consumed verbatim from
  `packages/rhumb-protocol/templates/`).
- **Layer 1**: Meridian overlay that injects `{{MERIDIAN_VERSION}}`
  substitution + Meridian-internal commentary on top of the base.

Two-layer materialization is one candidate; alternatives include
"adopt-and-extend" (Meridian fork stays under `.meridian/templates/`,
imports from RWP at refactor-time, drift gate enforces alignment of
shared regions) or "absorb-into-RWP" (push Meridian expansions upstream
into RWP, then consume verbatim — but this couples RWP velocity to
Meridian feature work, which may violate AVD-0004 Open-Protocol-Strategy
intent).

**P-01 disposition**: surface the pattern; let P-02 lock the mechanism.

### 3.2 Per-file divergence narratives

_The 8 narrative blocks below were emitted by drift-audit.zsh with
diff-context already inlined. The Direction-of-Truth and Recommended-
Resolution lines now cross-reference §3.1 (canonical pattern), §3.2
override table, and §7 (consumer counts). P-02 readers should treat
§3.1 + §3.2 + §7 as the load-bearing inputs; the per-file blocks are the
diff archive._

**Per-file overrides applied at this report's emission**:

| File                              | Direction-of-Truth      | Override Reason                                                                                                  |
|-----------------------------------|--------------------------|------------------------------------------------------------------------------------------------------------------|
| `DEPENDENCIES.yaml.template`      | RWP (default)           | Meridian's RULE-63/87 references are Meridian-internal; overlay candidate.                                       |
| `INTAKE.yaml.template`            | RWP (default)           | Meridian copy refers to "MDDF Intake" — legacy naming pre-Rhumb; RWP is canonical post-MP-0272.                  |
| `MANIFEST-PLAN.yaml.template`     | RWP (default)           | Pure overlay candidate.                                                                                          |
| `MASTERPLAN.yaml.template`        | RWP (default)           | Pure overlay candidate.                                                                                          |
| `PLAN-STATE.yaml.template`        | RWP (default)           | Pure overlay candidate.                                                                                          |
| `PLAN.md.template`                | **investigate** — likely two-layer | Meridian frontmatter pins `MpStatus` enum (Rust-internal); cannot live in RWP base. Overlay must carry it.       |
| `PROMPT.md.template`              | **investigate** — split | 535-line vs 44-line gap. Likely Meridian carries the workflow narrative; RWP carries protocol-level shape only.  |
| `START-PROMPT.md.template`        | **investigate** — split | Same shape as PROMPT.md.template.                                                                                |

P-02 must convert the three "investigate" entries into firm dispositions
before P-03 deletion of any Meridian copy.


#### `DEPENDENCIES.yaml.template` — duplicate_divergent

- **RWP path**: `packages/rhumb-protocol/templates/DEPENDENCIES.yaml.template`
- **Meridian path**: `.meridian/templates/DEPENDENCIES.yaml.template`
- **RWP SHA-256**: `f5ec0ddd4b31f0feb897645df8db92d2f129ac750ccf838e68c8c357fe48be2c`
- **Meridian SHA-256**: `e10045c496f81f1831f944f22a26d3e0daa01c5ac6ab99bf6e5292d8de5951af`
- **Divergence summary**: Meridian-augmented overlay of RWP base — see §3.1 canonical pattern. Diff inlined below.
- **Direction of truth**: RWP (default per KD-14.8) — override iff explicit reason.
- **Recommended resolution**: Default per §3.1 (Meridian consumes RWP base + applies overlay); see §3.2 override table for per-file investigate flags. Final disposition locked in P-02 (KD-14.3); refactor in P-03.
- **Resolution owner**: MP-0275 P-03.
- **Consumers of Meridian copy**: See §7 Consumer Grep Sweep table for per-template counts and notable consumers. No template in this set has zero consumers; P-03 refactor MUST preserve the consumed path.

```diff
--- .meridian/templates/DEPENDENCIES.yaml.template	2026-04-25 17:59:06
+++ packages/rhumb-protocol/templates/DEPENDENCIES.yaml.template	2026-04-29 16:04:24
@@ -1,60 +1,54 @@
-meridian_version: "{{MERIDIAN_VERSION}}"
-# Dependencies Template v2.1 (Updated for sub-phase support)
+# Rhumb Workflow Protocol: Dependencies Template v1.0
 # Copy to {plan-dir}/dependencies.yaml and fill in
 #
-# REQUIRED for (RULE-63):
-# - Any plan with status: blocked
-# - Plans that block other plans
-# - Plans with non-linear phase dependencies
-# - Plans with cross-plan phase dependencies (RULE-87)
+# Recommended for:
+# - Any plan that is dependent on other plans
+# - Plans that have non-linear phase dependencies
+# - Plans with cross-plan phase dependencies
 #
-# SUB-PHASE SUPPORT:
-# - Default: Use P-XX-A, P-XX-B, P-XX-C format
-# - Traditional: Use P-01, P-02 format (set sub_phases: false in PLAN.md)
-# - Sub-phases can run in parallel when files don't conflict
-#
-# OPTIONAL but recommended for all plans
+# Optional but recommended for all plans
 
-plan_id: {PLAN_ID}                              # Canonical identity in PLAN.md frontmatter
-# request_id and title: see PLAN.md frontmatter (single source of truth)
+plan_id: {PLAN_ID}  # e.g., RWP-0001
+request_id: {REQUEST_ID}
+title: "{PLAN_TITLE}"
+rwp_version: "0.25.1"
 created: "{TIMESTAMP_ISO8601}"
 updated: "{TIMESTAMP_ISO8601}"
 
 # ============================================================================
-# BLOCKING DEPENDENCIES (HARD BLOCK) - RULE-64, RULE-87
+# BLOCKING DEPENDENCIES (Hard Block)
 # ============================================================================
-# Plans/phases that must complete BEFORE this plan can proceed.
-# Validation script will BLOCK execution if unmet.
+# Plans/phases that should complete BEFORE this plan can proceed.
 #
 # Types:
-#   - plan_completion: Entire plan must complete
-#   - phase_completion: Specific phase must complete (RULE-87)
+#   - plan_completion: Entire plan should complete
+#   - phase_completion: Specific phase should complete
 
 blocked_by:
   # Example 1: Block on entire plan
-  # - id: MP-0064
+  # - id: RWP-0064
   #   type: plan_completion
   #   required_phase: null
-  #   reason: "Extension beta must complete before mobile bindings"
+  #   reason: "Extension beta should complete before mobile bindings"
 
   # Example 2: Block on specific phase (cross-plan coordination)
-  # - id: MP-0178
+  # - id: RWP-0178
   #   type: phase_completion
   #   required_phase: P-02
-  #   reason: "Need IPC protocol from MP-0178 P-02 for our P-04"
-  #   affects_our_phases: [P-04, P-05]  # Which of our phases are blocked
+  #   reason: "Need protocol specification from RWP-0178 P-02 for our P-04"
+  #   affects_our_phases: [P-04, P-05]
   []
 
 # ============================================================================
 # PLANS THIS BLOCKS (Informational)
 # ============================================================================
 # Plans that depend on THIS plan completing.
-# Used for dependency graph visualization, not enforcement.
+# Used for dependency graph visualization.
 
 blocks:
   # Example:
-  # - MP-0068
-  # - MP-0070
+  # - RWP-0068
+  # - RWP-0070
   []
 
 # ============================================================================
@@ -65,7 +59,7 @@
 
 depends_on:
   # Example:
-  # - id: MP-0111
+  # - id: RWP-0111
   #   status: completed
   #   reason: "Established validation patterns"
   []
@@ -92,7 +86,7 @@
   # Parallel logical phases (different tracks can run simultaneously):
   P-02-A:
     depends_on: [P-01-C]
-    parallel_with: [P-03-A]  # Can run parallel with P-03 track
+    parallel_with: [P-03-A]
     title: "{Phase 2 Sub-A Title}"
   P-02-B:
     depends_on: [P-02-A]
@@ -100,7 +94,7 @@
 
   P-03-A:
     depends_on: [P-01-C]
-    parallel_with: [P-02-A]  # Can run parallel with P-02 track
+    parallel_with: [P-02-A]
     title: "{Phase 3 Sub-A Title}"
   P-03-B:
     depends_on: [P-03-A]
@@ -108,70 +102,48 @@
 
   # Merge sub-phase (waits for parallel tracks):
   P-04-A:
-    depends_on: [P-02-B, P-03-B]  # Waits for both parallel tracks
+    depends_on: [P-02-B, P-03-B]
     title: "{Phase 4 Merge Sub-A Title}"
 
```

#### `INTAKE.yaml.template` — duplicate_divergent

- **RWP path**: `packages/rhumb-protocol/templates/INTAKE.yaml.template`
- **Meridian path**: `.meridian/templates/INTAKE.yaml.template`
- **RWP SHA-256**: `193cfcdee6ac9e6c0593189dd47262504beea36953e389ebe2d4356fb733fbeb`
- **Meridian SHA-256**: `91fbf06783d4f8ef7e6eb2fb8ff64ca445eb43a6a943266e426bec497ac68b62`
- **Divergence summary**: Meridian-augmented overlay of RWP base — see §3.1 canonical pattern. Diff inlined below.
- **Direction of truth**: RWP (default per KD-14.8) — override iff explicit reason.
- **Recommended resolution**: Default per §3.1 (Meridian consumes RWP base + applies overlay); see §3.2 override table for per-file investigate flags. Final disposition locked in P-02 (KD-14.3); refactor in P-03.
- **Resolution owner**: MP-0275 P-03.
- **Consumers of Meridian copy**: See §7 Consumer Grep Sweep table for per-template counts and notable consumers. No template in this set has zero consumers; P-03 refactor MUST preserve the consumed path.

```diff
--- .meridian/templates/INTAKE.yaml.template	2026-04-25 17:59:06
+++ packages/rhumb-protocol/templates/INTAKE.yaml.template	2026-04-29 16:04:24
@@ -1,5 +1,4 @@
-meridian_version: "{{MERIDIAN_VERSION}}"
-# MDDF Intake Template
+# Rhumb Workflow Protocol: Intake Template
 # Layer 0: Captures WHAT needs to be built
 #
 # The intake document captures the problem space:
@@ -11,9 +10,7 @@
 # Usage:
 #   1. Copy this template to your plan directory as INTAKE.yaml
 #   2. Replace placeholders with actual content
-#   3. Run: mddf intake --validate INTAKE.yaml
-#
-# Reference: packages/yakkl-meridian-orchestrator/src/types/intake.ts
+#   3. Consider validating with your workflow system
 
 # Unique intake identifier (auto-generated or manual)
 id: INT-{NNNN}
@@ -30,6 +27,9 @@
 # Classification level
 classification: public  # public | confidential
 
+# RWP Version
+rwp_version: "0.25.1"
+
 # =============================================================================
 # PAIN POINTS
 # =============================================================================
@@ -140,21 +140,39 @@
   target_location: "{packages/package-name/}"
 
 # =============================================================================
+# CUSTOM FIELDS
+# =============================================================================
+# RWP supports arbitrary custom fields for domain-specific needs.
+# The following pattern is commonly used:
+#
+# custom_fields:
+#   domain: "{Domain name - e.g., security, performance, ui}"
+#   related_initiatives: []
+#   stakeholder_groups:
+#     - group: "{Group name}"
+#       contact: "{Person or team}"
+#   timeline:
+#     target_start: "{ISO 8601 date}"
+#     target_completion: "{ISO 8601 date}"
+
+custom_fields: {}
+
+# =============================================================================
 # METADATA
 # =============================================================================
 # Additional tracking information
 
 metadata:
-  request_id: REQ-NNNNN | null  # Origin request that spawned this plan (null if bypassed)
-  original_plan: MP-NNNN  # Plan ID this intake belongs to
+  request_id: REQ-NNNNN | null  # Origin request that spawned this plan
+  original_plan: RWP-NNNN  # Plan ID this intake belongs to
   classification: public  # public | confidential
-  parent_mp: null  # Parent plan if this is a sub-plan
+  parent_plan: null  # Parent plan if this is a sub-plan
   sub_plans: []  # List of sub-plan IDs
   dependencies_met: []  # Plan IDs that have been completed
   dependencies_unmet: []  # Plan IDs that must complete first
 
-# @strip
-# Produced: {ISO 8601}  
-# By: YAKKL® Meridian™ - https://meridian.yakkl.com  
-# Copyright: Copyright © 2026 YAKKL Inc. All Rights Reserved.
-# /@strip
+---
+
+Produced: {{TIMESTAMP}}
+By: Rhumb Protocol™ Contributors - https://rhumbprotocol.dev
+Copyright: Copyright © 2026 Rhumb Protocol Contributors. All Rights Reserved.
```

#### `MANIFEST-PLAN.yaml.template` — duplicate_divergent

- **RWP path**: `packages/rhumb-protocol/templates/MANIFEST-PLAN.yaml.template`
- **Meridian path**: `.meridian/templates/MANIFEST-PLAN.yaml.template`
- **RWP SHA-256**: `2c69c2a78eb2ac0e4d6f2552bd31c44f03d97dcde6abe864bf511df538ad0039`
- **Meridian SHA-256**: `775a54e81d96f897263edb4b965e6b0d4f83df6b137d288ff59b28b815a221b9`
- **Divergence summary**: Meridian-augmented overlay of RWP base — see §3.1 canonical pattern. Diff inlined below.
- **Direction of truth**: RWP (default per KD-14.8) — override iff explicit reason.
- **Recommended resolution**: Default per §3.1 (Meridian consumes RWP base + applies overlay); see §3.2 override table for per-file investigate flags. Final disposition locked in P-02 (KD-14.3); refactor in P-03.
- **Resolution owner**: MP-0275 P-03.
- **Consumers of Meridian copy**: See §7 Consumer Grep Sweep table for per-template counts and notable consumers. No template in this set has zero consumers; P-03 refactor MUST preserve the consumed path.

```diff
--- .meridian/templates/MANIFEST-PLAN.yaml.template	2026-04-25 17:59:06
+++ packages/rhumb-protocol/templates/MANIFEST-PLAN.yaml.template	2026-04-29 16:04:24
@@ -1,127 +1,233 @@
-meridian_version: "{{MERIDIAN_VERSION}}"
-# Plan Manifest Template (YAML)
+# Rhumb Workflow Protocol: Plan Manifest Template
+# Tracks files, deliverables, and audit schedule for a plan
 #
-# PURPOSE: Generate a `manifest.yaml` file when a plan is created/approved.
-# LOCATION: Place in plan directory as `manifest.yaml`
+# This manifest serves as the source of truth for:
+# - What files are created/modified by each phase
+# - What deliverables are produced
+# - When audits should occur
+# - Phase handoff tracking
 #
-# USAGE:
-#   1. Copy this template to your plan directory as manifest.yaml
-#   2. Replace all {PLACEHOLDER} values
-#   3. Generate entries for ALL phases upfront
-#   4. Save as manifest.yaml in the plan directory
-#
-# PLACEHOLDERS:
-#   {PLAN_ID}            - e.g., MP-0111
-#   {REQUEST_ID}         - e.g., REQ-00005 (null if bypassed)
-#   {PLAN_TITLE}         - e.g., Meridian Rule Enforcement
-#   {SHORT_NAME}         - e.g., meridian-rule-enforcement (kebab-case)
-#   {TOTAL_PHASES}       - e.g., 9
-#   {CLASSIFICATION}     - public | confidential | top_secret
-#   {CREATED_TIMESTAMP}  - ISO 8601 timestamp
-#   {START_PHASE_ID}     - e.g., P-01 or P-01-A
-#   {DATE}               - e.g., 2026-02-12 (for filenames)
-#   {NN}                 - Two-digit phase number (01, 02, 03)
-#   {PHASE_NN_TITLE}     - Phase title text
-#   {MODE}               - planning | processing | completed
-#
-# VALIDATION:
-#   A valid manifest must have:
-#   1. All required top-level fields (plan_id, request_id, title, total_phases, classification, created, path)
-#   2. files.start_prompt defined
-#   3. One handoffs entry per phase
-#   4. One prompts entry per phase transition (total_phases - 1)
-#   5. audits entries for every 3rd phase + FINAL
-#   6. phases entries with titles and audit_required flags
-#
-# AUDIT CALCULATION:
-#   Checkpoint audits at: P-03, P-06, P-09, P-12, P-15, P-18...
-#     Formula: phase_number % 3 == 0
-#   FINAL audit at: P-{N} (last phase)
-#     Always required regardless of phase number
+# Usage:
+#   1. Create a copy of this template as manifest.yaml in your plan directory
+#   2. List files created/modified in each phase section
+#   3. Document deliverable locations
+#   4. Define audit checkpoints
 
-# =============================================================================
-# PLAN IDENTIFICATION
-# =============================================================================
+# Plan identification
+plan_id: RWP-NNNN
+request_id: REQ-NNNNN | null
+title: "{Plan Title}"
+rwp_version: "0.25.1"
+created: "{ISO 8601}"
 
-plan_id: {PLAN_ID}                              # Canonical identity in PLAN.md frontmatter
-# request_id and title: see PLAN.md frontmatter (single source of truth)
-total_phases: {TOTAL_PHASES}
-classification: {CLASSIFICATION}
-created: {CREATED_TIMESTAMP}
-updated: {CREATED_TIMESTAMP}
+# ============================================================================
+# PLAN-LEVEL METADATA
+# ============================================================================
+plan_metadata:
+  total_phases: NN  # Number of phases in this plan
+  total_deliverables: NN
+  target_completion_date: "{ISO 8601 date}"
+  owner: "{Person or team}"
+  reviewers: []  # People who will review this plan
 
-# Plan location - {MODE} is 'planning', 'processing', 'completed'
-path: .meridian/.private/plans/{MODE}/{PLAN_ID}-{SHORT_NAME}/
+# ============================================================================
+# PHASE FILE TRACKING
+# ============================================================================
+# Track files created/modified in each phase
+# This helps plan reviewers understand the blast radius of each phase
 
-# =============================================================================
-# PRE-COMPUTED FILE PATHS
-# =============================================================================
+phases:
+  # -------------------------------------------------------------------------
+  # P-01: Foundation
+  # -------------------------------------------------------------------------
+  P-01:
+    title: "Foundation Phase"
+    status: pending  # pending | in_progress | completed | failed
+    started_at: null
+    completed_at: null
+    files:
+      created:
+        - path: "{path/to/file1}"
+          description: "What this file does"
+          type: code | config | test | doc | other
+        - path: "{path/to/file2}"
+          description: "What this file does"
+          type: doc
+      modified:
+        - path: "{path/to/existing}"
+          description: "What changed"
+      deleted: []
 
-files:
-  # START prompt (required before P-01 can begin)
-  start_prompt: handoffs/HO-{PLAN_ID}-START-{START_PHASE_ID}-PROMPT.yaml
+  # -------------------------------------------------------------------------
+  # P-02: Implementation (Track sub-phases)
```

#### `MASTERPLAN.yaml.template` — duplicate_divergent

- **RWP path**: `packages/rhumb-protocol/templates/MASTERPLAN.yaml.template`
- **Meridian path**: `.meridian/templates/MASTERPLAN.yaml.template`
- **RWP SHA-256**: `6ae081905f040b42e75136c9436f2ae2ca87131e015b450fc9a1cf6dc6ffe959`
- **Meridian SHA-256**: `e943d983dd262a3480ddc0336eb05348b58e0a3fd9357a9188b9158dd94353d3`
- **Divergence summary**: Meridian-augmented overlay of RWP base — see §3.1 canonical pattern. Diff inlined below.
- **Direction of truth**: RWP (default per KD-14.8) — override iff explicit reason.
- **Recommended resolution**: Default per §3.1 (Meridian consumes RWP base + applies overlay); see §3.2 override table for per-file investigate flags. Final disposition locked in P-02 (KD-14.3); refactor in P-03.
- **Resolution owner**: MP-0275 P-03.
- **Consumers of Meridian copy**: See §7 Consumer Grep Sweep table for per-template counts and notable consumers. No template in this set has zero consumers; P-03 refactor MUST preserve the consumed path.

```diff
--- .meridian/templates/MASTERPLAN.yaml.template	2026-04-25 17:59:06
+++ packages/rhumb-protocol/templates/MASTERPLAN.yaml.template	2026-04-29 16:04:24
@@ -1,5 +1,4 @@
-meridian_version: "{{MERIDIAN_VERSION}}"
-# MDDF Master Plan Template
+# Rhumb Workflow Protocol: Master Plan Template
 # Layer 2: Defines HOW to build it
 #
 # HIERARCHY (4 levels):
@@ -16,34 +15,34 @@
 #     └── P-04-B (Sub-phase)
 #
 # Sub-phases are OPTIONAL and nested within phases:
-#   - Use when a phase exceeds ~60 minutes of work
-#   - Each sub-phase targets ~30 minutes for crash resilience
+#   - Consider using when a phase exceeds ~60 minutes of work
+#   - Each sub-phase should target ~30 minutes for crash resilience
 #   - Not all phases need sub-phases
 #
 # AUDIT phases are special:
 #   - Placed after every 3 phases (configurable)
-#   - Run tests, verify deliverables, create audit reports
+#   - Should run tests, verify deliverables, create audit reports
 #
 # Usage:
 #   1. Copy this template to your plan directory as MASTERPLAN.yaml
 #   2. Replace placeholders with actual content
 #   3. Add sub-phases only where needed for complex phases
-#   4. Customize audit_schedule if defaults don't fit
-#
-# Reference: packages/yakkl-meridian-orchestrator/src/types/decomposition.ts
+#   4. Consider customizing audit_schedule if defaults don't fit
 
 master_plan:
   # =============================================================================
   # PLAN METADATA
   # =============================================================================
 
-  id: "MP-NNNN"                                  # Canonical identity in PLAN.md frontmatter
-  # request_id and title: see PLAN.md frontmatter (single source of truth)
+  id: "RWP-NNNN"
+  request_id: "REQ-NNNNN | null"          # Origin request that spawned this plan
+  title: "{Plan Title}"
   description: "{Detailed description of the plan}"
   intake_id: "INT-NNNN"
   status: planning  # planning | approved | processing | completed | failed | cancelled
   classification: public  # public | confidential
   created: "{ISO 8601}"
+  rwp_version: "0.25.1"                    # Rhumb Workflow Protocol version
 
   # =============================================================================
   # ARCHITECTURE METADATA (Optional)
@@ -55,10 +54,11 @@
     layers_touched: []  # ui | service | data | infra | config | test
 
   # =============================================================================
-  # EXECUTION TRACKING (MP-0186)
+  # EXECUTION TRACKING
   # =============================================================================
   # Controls how the plan integrates with session task systems for real-time
-  # progress visibility. When enabled, micro-tasks are hydrated as session tasks.
+  # progress visibility. Consider enabling for complex plans.
+  #
   execution_tracking:
     auto_hydrate: true           # Automatically create session tasks from micro-tasks
     sync_on_complete: true       # Sync task completion back to MASTERPLAN state
@@ -66,12 +66,15 @@
     track_duration: true         # Track actual duration of micro-tasks
 
   # =============================================================================
-  # AUDIT SCHEDULE (RULE-05, RULE-33, RULE-93)
+  # AUDIT SCHEDULE
   # =============================================================================
+  # Consider running audits at regular intervals (every 3 phases recommended).
+  # Audits verify quality and help catch issues early.
+  #
   audit_schedule:
     enabled: true
     checkpoint:
-      phase_frequency: 3  # AUDIT after every 3rd phase (P-03, P-06, P-09, ...)
+      phase_frequency: 3  # Audit after every 3rd phase (P-03, P-06, P-09, ...)
       type: checkpoint
       session: same_ok
     full:
@@ -109,7 +112,7 @@
     # P-01: Foundation / Setup (Simple phase - no sub-phases)
     # -------------------------------------------------------------------------
     - id: "P-01"
-      parent_mp: "MP-NNNN"
+      parent_mp: "RWP-NNNN"
       title: "{Phase 1 Title}"
       objective: "{What this phase accomplishes}"
       order: 1
@@ -135,12 +138,12 @@
     # P-02: Implementation (Complex phase WITH sub-phases)
     # -------------------------------------------------------------------------
     - id: "P-02"
-      parent_mp: "MP-NNNN"
+      parent_mp: "RWP-NNNN"
       title: "{Phase 2 Title - Complex}"
       objective: "{Overall objective for this phase}"
       order: 2
       dependencies: ["P-01"]
-      parallel_with: ["P-03"]  # Can run parallel with P-03 if no conflicts
+      parallel_with: ["P-03"]
       status: pending
       estimated_duration_minutes: 90  # Total for all sub-phases
 
@@ -150,8 +153,8 @@
           parent_phase: "P-02"
           title: "{Sub-Phase 2-A Title}"
           objective: "{Sub-phase A objective}"
-          order: 1  # Order within P-02
-          dependencies: []  # First sub-phase of P-02
+          order: 1
+          dependencies: []
           status: pending
           estimated_duration_minutes: 30
 
@@ -185,7 +188,7 @@
```

#### `PLAN-STATE.yaml.template` — duplicate_divergent

- **RWP path**: `packages/rhumb-protocol/templates/PLAN-STATE.yaml.template`
- **Meridian path**: `.meridian/templates/PLAN-STATE.yaml.template`
- **RWP SHA-256**: `eedd2f69eee9e508a4418ba569f15cb26cfc950ffef1160d4c333f17f0cff1ef`
- **Meridian SHA-256**: `e206a7387ce45f0ee052bf19bd9aa9fff46591c6fdd4e4170c01307e55ad9e4f`
- **Divergence summary**: Meridian-augmented overlay of RWP base — see §3.1 canonical pattern. Diff inlined below.
- **Direction of truth**: RWP (default per KD-14.8) — override iff explicit reason.
- **Recommended resolution**: Default per §3.1 (Meridian consumes RWP base + applies overlay); see §3.2 override table for per-file investigate flags. Final disposition locked in P-02 (KD-14.3); refactor in P-03.
- **Resolution owner**: MP-0275 P-03.
- **Consumers of Meridian copy**: See §7 Consumer Grep Sweep table for per-template counts and notable consumers. No template in this set has zero consumers; P-03 refactor MUST preserve the consumed path.

```diff
--- .meridian/templates/PLAN-STATE.yaml.template	2026-04-25 17:59:06
+++ packages/rhumb-protocol/templates/PLAN-STATE.yaml.template	2026-04-29 16:04:24
@@ -1,282 +1,133 @@
-meridian_version: "{{MERIDIAN_VERSION}}"
-# Plan State Template v1.2 (Updated for sub-phase support)
-# Copy to {plan-dir}/state.yaml when plan moves to processing/
+# Rhumb Workflow Protocol: Plan State Template
+# Real-time tracking of plan execution progress
 #
-# REQUIRED for (RULE-66):
-# - Any plan with status: processing
+# This file tracks the current state of a plan as it executes.
+# Update this file as phases complete to maintain accurate progress state.
 #
-# Purpose:
-# - Real-time execution tracking
-# - Failure detection (errors captured)
-# - Hang detection (heartbeat timeout)
-# - Progress visibility (current phase, tasks)
-# - Audit trail (history of events)
-# - Concurrent execution prevention via claims (RULE-90)
-#
-# SUB-PHASE SUPPORT:
-# - Default: Use P-XX-A, P-XX-B, P-XX-C format (~30 min each)
-# - Traditional: Use P-01, P-02 format (set sub_phases: false in PLAN.md)
-# - Each sub-phase gets its own handoff document
+# Usage:
+#   1. Create a copy of this template as state.yaml in your plan directory
+#   2. Update execution status and phase states as work progresses
+#   3. Create phase objects for each phase in your plan
+#   4. Update timestamps in ISO 8601 format
 
-plan_id: {PLAN_ID}  # e.g., MP-0122
-request_id: {REQUEST_ID}
-title: "{PLAN_TITLE}"
+# Plan identifier and metadata
+plan_id: RWP-NNNN
+request_id: REQ-NNNNN | null
+title: "{Plan Title}"
+rwp_version: "0.25.1"
 
-# ============================================================================
-# CLAIMS - Concurrent Execution Prevention (RULE-90, RULE-91)
-# ============================================================================
-# Claims track which agent is executing which phase.
-# If a claim exists for a phase, other agents MUST NOT start that phase.
-# Claims expire automatically after 30 minutes (for crashed agents).
-# Use: .meridian/scripts/meridian-phase-lock.zsh to manage claims
-claims: []
-  # Example claim:
-  # - claim_id: "claim-1705123456-12345"
-  #   plan_id: "MP-0122"
-  #   phase: "P-04"
-  #   agent_id: "agent-1705123456-12345"
-  #   claimed_at: "2026-01-21T23:50:00Z"
-  #   expires_at: "2026-01-22T00:20:00Z"
-  #   heartbeat: "2026-01-21T23:55:00Z"
-
-# ============================================================================
-# EXECUTION STATUS
-# ============================================================================
+# High-level execution status and timeline
 execution:
-  status: not_started  # not_started | in_progress | completed | failed | hung
-  current_phase: null   # P-01, P-02, etc.
-  started_at: null
-  completed_at: null
-  last_heartbeat: null  # Update every 5-10 min during work (RULE-67)
-  heartbeat_timeout_minutes: 30  # Mark as hung if no update for this long
+  status: planning  # planning | in_progress | completed | paused | failed
+  current_phase: P-NN  # Currently active phase
+  started_at: "{ISO 8601 timestamp}"
+  completed_at: null  # Filled when plan completes
+  last_heartbeat: "{ISO 8601 timestamp}"  # When state was last updated
+  heartbeat_timeout_minutes: 30  # Minutes before considering phase stale
 
-# ============================================================================
-# PHASE TRACKING - SUB-PHASES: {SUB_PHASES}
-# ============================================================================
-# For sub-phase pattern (default): Use P-XX-A, P-XX-B, P-XX-C
-# For traditional pattern: Use P-01, P-02, etc.
-phases:
-  # --- Sub-Phase Pattern Examples ---
-  P-01-A:
-    title: "{PHASE_01_A_TITLE}"
-    status: pending  # pending | in_progress | completed | skipped | failed
-    started_at: null
-    completed_at: null
-    tasks: []  # Add tasks as needed
-    handoff_created: false
-    handoff_validated: false
-    handoff_score: null
-
-  P-01-B:
-    title: "{PHASE_01_B_TITLE}"
-    status: pending
-    started_at: null
-    completed_at: null
-    tasks: []
-    handoff_created: false
-    handoff_validated: false
-    handoff_score: null
-
-  P-01-C:
-    title: "{PHASE_01_C_TITLE}"
-    status: pending
-    started_at: null
-    completed_at: null
-    tasks: []
-    handoff_created: false
-    handoff_validated: false
-    handoff_score: null
-
-  P-02-A:
-    title: "{PHASE_02_A_TITLE}"
-    status: pending
-    started_at: null
-    completed_at: null
-    tasks: []
-    handoff_created: false
```

#### `PLAN.md.template` — duplicate_divergent

- **RWP path**: `packages/rhumb-protocol/templates/PLAN.md.template`
- **Meridian path**: `.meridian/templates/PLAN.md.template`
- **RWP SHA-256**: `c208a95257afb59f73218fc18008153d0a836e5160e14ec7563036388cc3e6dd`
- **Meridian SHA-256**: `1ac0e9088ae39e49be3082399277624ce938836a201cda4fd03f275e7cb0dd1d`
- **Divergence summary**: Meridian-augmented overlay of RWP base — see §3.1 canonical pattern. Diff inlined below.
- **Direction of truth**: RWP (default per KD-14.8) — override iff explicit reason.
- **Recommended resolution**: Default per §3.1 (Meridian consumes RWP base + applies overlay); see §3.2 override table for per-file investigate flags. Final disposition locked in P-02 (KD-14.3); refactor in P-03.
- **Resolution owner**: MP-0275 P-03.
- **Consumers of Meridian copy**: See §7 Consumer Grep Sweep table for per-template counts and notable consumers. No template in this set has zero consumers; P-03 refactor MUST preserve the consumed path.

```diff
--- .meridian/templates/PLAN.md.template	2026-04-25 17:59:06
+++ packages/rhumb-protocol/templates/PLAN.md.template	2026-04-29 16:04:24
@@ -1,66 +1,45 @@
 ---
-meridian_version: "{{MERIDIAN_VERSION}}"
-# Master Plan (MP) Template — Rust frontmatter schema v1.0.0 (MP-0279 DD-MP-0279-02)
-#
-# Frontmatter is the canonical CommonFrontmatter shape (MP-0279 P-01/P-02).
-# Status values pinned to the Rust `MpStatus` enum (AVD-0005 KD-05.5):
-#   drafts     → unpromoted scratch draft; no REQUEST-NORMALIZED yet
-#   planning   → REQUEST-NORMALIZED frozen; MP artifacts authored; P-01 not started
-#   queued     → waiting on a dependency; ready to start but gated
-#   processing → at least one phase has started; plan is under active execution
-#   completed  → all phases complete + AUDIT-FINAL passed
-#   onhold     → paused by author or team; resumable
-#   cancelled  → abandoned; no further work planned
-#   archived   → terminal historical state; moved to the archive tree
-schema_version: "{{MERIDIAN_VERSION}}"   # Rust frontmatter schema version (MP-0279)
-id: MP-{{NEXT_ID}}                       # Canonical id (replaces legacy plan_id)
-type: plan                               # Frontmatter enum tag (MP-0279 DD-MP-0279-06)
-title: "{{TITLE}}"                       # Canonical title (replaces legacy name)
-status: planning                         # drafts | planning | queued | processing | completed | onhold | cancelled | archived
-classification: {{CLASSIFICATION}}       # public | confidential
+# Rhumb Workflow Protocol: Plan Document
+
+plan_id: RWP-NNNN
+request_id: REQ-NNNNN | null             # Origin request that spawned this plan
+name: [Plan Title]
+classification: public | confidential
+status: planning | processing | completed | on_hold
 created: {{TIMESTAMP}}
-updated: {{TIMESTAMP}}
-authors:
-  - name: "{{AUTHOR}}"
-    role: "author"                       # author | engineer | architect | reviewer | other
-tags: []                                 # Domain tags for indexing
-custom_sections: []                      # Project-specific sub-headings for indexing
+author: [AI/Human name]
+phases: NN
+current_phase: P-NN
+started: {{TIMESTAMP}}
+completed: {{TIMESTAMP}}
+parent: RWP-NNNN | null
+rwp_version: "0.25.1"                     # Rhumb Workflow Protocol version
+dependencies:
+  - RWP-NNNN
+  - RWP-NNNN
+packages:
+  - package-name-1
+  - package-name-2
 
-# ─── Chain linkage ────────────────────────────────────────────────────
-parent: null                             # Parent MP if this is a sub-plan (MP-NNNN or null)
-children: []                             # Child plan/phase artifact IDs
-depends_on: []                           # Other MPs this plan depends on
-related_to: []                           # Loosely related artifacts
-blocks: []                               # Other MPs this plan blocks
-supersedes: null                         # Prior MP this plan replaces (MP-NNNN or null)
+# =============================================================================
+# TRACKING (optional - customize to your workflow)
+# =============================================================================
+# Consider integrating with your issue tracker for better visibility.
+# All fields are optional. Use what helps your team, ignore the rest.
 
-# ─── Plan-specific fields ─────────────────────────────────────────────
-request_id: REQ-{{NEXT_ID}}              # Originating REQUEST-NORMALIZED (or null)
-phases: 0                                # Total phase count (update on decomposition)
-sub_phases: 0                            # Total sub-phase count
-current_phase: null                      # P-NN or null before start
-started: null                            # ISO-8601 (null until P-01 starts)
-completed: null                          # ISO-8601 (null until AUDIT-FINAL passes)
-packages: []                             # Affected package directories
-# Optional fields for classified plans:
-# classification_reason: [Reason for classification level]
-# blocked_by: []
-# priority: P0 | P1 | P2 | P3
-
-# ─── Tracking (optional — customize to your workflow) ─────────────────
-# See TRACKING-SCHEMA.yaml for full documentation and integration examples.
 tracking:
-  ticket: null                          # JIRA-123, GH-456, LINEAR-789, etc.
-  epic: null                            # Parent epic if applicable
-  external_url: null                    # Link to external tracker
-  assigned_to: null                     # Person or team
-  estimate: null                        # Your format: points, t-shirt, time, etc.
-  actual: null                          # Fill after completion for calibration
-  priority: null                        # Your format: P0-P3, High/Med/Low, 1-5
-  labels: []                            # Custom tags for filtering
+  ticket: null              # JIRA-123, GH-456, LINEAR-789, etc.
+  epic: null                # Parent epic if applicable
+  external_url: null        # Link to external tracker
+  assigned_to: null         # Person or team
+  estimate: null            # Your format: points, t-shirt, time, etc.
+  actual: null              # Fill after completion for calibration
+  priority: null            # Your format: P0-P3, High/Med/Low, 1-5
+  labels: []                # Custom tags for filtering
+
 ---
 
-# MP-NNNN: [Plan Title]
+# RWP-NNNN: [Plan Title]
 
 ---
 
@@ -76,22 +55,9 @@
 
 ---
 
-## Orchestration Configuration
-
-<!-- Optional: Enable multi-agent orchestration for this plan -->
-```yaml
-orchestration:
-  enabled: false          # Set to true for /execute-plan support
-  max_parallel: 2         # Maximum concurrent agents
-  require_approval: false # Pause between phases for approval
-  phase_timeout: 600000   # Timeout per phase (ms)
-```
-
----
```

#### `PROMPT.md.template` — duplicate_divergent

- **RWP path**: `packages/rhumb-protocol/templates/PROMPT.md.template`
- **Meridian path**: `.meridian/templates/PROMPT.md.template`
- **RWP SHA-256**: `2e3441f870b78c8086faab054f8e81b17304bf14449166c74ab255e82279d1b6`
- **Meridian SHA-256**: `9264722ddfd3a7b6000578990f2920376678162e2e08e8ceba705277504c7dea`
- **Divergence summary**: Meridian-augmented overlay of RWP base — see §3.1 canonical pattern. Diff inlined below.
- **Direction of truth**: RWP (default per KD-14.8) — override iff explicit reason.
- **Recommended resolution**: Default per §3.1 (Meridian consumes RWP base + applies overlay); see §3.2 override table for per-file investigate flags. Final disposition locked in P-02 (KD-14.3); refactor in P-03.
- **Resolution owner**: MP-0275 P-03.
- **Consumers of Meridian copy**: See §7 Consumer Grep Sweep table for per-template counts and notable consumers. No template in this set has zero consumers; P-03 refactor MUST preserve the consumed path.

```diff
--- .meridian/templates/PROMPT.md.template	2026-04-25 17:59:06
+++ packages/rhumb-protocol/templates/PROMPT.md.template	2026-04-29 16:04:24
@@ -1,535 +1,44 @@
----
-meridian_version: "{{MERIDIAN_VERSION}}"
-template_type: "prompt"
-template_description: "Generic phase handoff prompt template"
----
+═══════════════════════════════════════════════════════════════════════════════
+                      CONTINUING TO NEXT PHASE
+═══════════════════════════════════════════════════════════════════════════════
 
-# Handoff Prompt: [PLAN-ID] Phase P-NN, Subphase SP-XX (if applicable and -XX represents the subphase number/letters)
+CURRENT STATUS
 
-**Plan**: [PLAN-ID] - [Plan Title]
-**Request**: [REQ-NNNNN | null]
-**Phase**: P-NN
-**Subphase**: SP-XX-X - [Sub-Phase Title] (if applicable)
-**Classification**: [PUBLIC|PRIVATE|CONFIDENTIAL|TOP_SECRET]
-**Started**: {{TIMESTAMP}}
-**Completed**: {{TIMESTAMP}}
-**Date**: {{TIMESTAMP}}
-**Duration**: ~30 minutes
+  Plan:           {{PLAN_ID}} - {{PLAN_TITLE}}
+  Completing:     {{CURRENT_PHASE_ID}} - {{CURRENT_PHASE_TITLE}}
+  Next Phase:     {{NEXT_PHASE_ID}} - {{NEXT_PHASE_TITLE}}
 
----
+  Current Phase Duration:  ~{{CURRENT_PHASE_DURATION}} minutes
+  Next Phase Duration:     ~{{NEXT_PHASE_DURATION}} minutes
 
-> **Full Handoff Document**: [HO-MP-{NNNN}-P-{NN}-{X}-{YYYY-MM-DD}.yaml](./HO-MP-{NNNN}-P-{NN}-{X}-{YYYY-MM-DD}.yaml)
-> For complete details including state changes, decisions rationale, failure modes, and verification results, see the full handoff document.
+STATE UPDATED
 
----
+  ✓ {{CURRENT_PHASE_ID}} marked complete
+  ✓ Handoff created: handoffs/{{HANDOFF_FILENAME}}
+  ✓ execution.current_phase set to {{NEXT_PHASE_ID}}
+  ✓ state.yaml saved
+  ✓ Timestamp: {{COMPLETION_TIMESTAMP}}
 
-## MANDATORY RULE - READ FIRST
-
-**RULE: Always follow Meridian rules at `.meridian/rules/<category>/*.yaml` (split-file tree; see `legacy-id-map.yaml` for ID aliases) with NO EXCEPTIONS.**
-
-**Sub-Phase Pattern**: This plan may use P-XX-A/B/C sub-phases (~30 min each) for crash resilience.
-
-At the end of EVERY Phase or sub-phase completion:
-1. **Release locks**: `.meridian/scripts/meridian-phase-lock.zsh release {PLAN_ID} P-NN-X`
-2. CRITICAL: Create FULL handoff document: `HO-MP-{NNNN}-P-{NN}-{X}-{YYYY-MM-DD}.yaml`
-3. CRITICAL: Create PROMPT file: `HO-MP-{NNNN}-P-{NN}-{X}-TO-P-{NEXT}-PROMPT.md`
-4. Update `state.yaml` with sub-phase completion
-5. **PROPAGATE this rule to ALL subsequent handoff documents and prompts**
-
-This ensures continuity across sessions and agents. **NO EXCEPTIONS.**
-
-### Completion Display Templates (MANDATORY — RULE-98)
-
-Every completion event MUST output a summary using the matching template:
-
-| Completion Type | Template |
-|----------------|----------|
-| Sub-phase (P-XX-A/B/C) | `.meridian/templates/SUBPHASE-COMPLETE-DISPLAY.md.template` |
-| Full phase (P-XX, no sub-phases) | `.meridian/templates/PHASE-COMPLETE-DISPLAY.md.template` |
-| Audit (AUD-NN or FINAL) | `.meridian/templates/AUDIT-COMPLETE-DISPLAY.md.template` |
-| Entire plan complete | `.meridian/templates/PLAN-COMPLETE-DISPLAY.md.template` |
-
-Output the display BEFORE creating handoff documents. Use the EXACT format from the template
-(═ borders, ALL CAPS headers, ``` code block, dot-leaders). Never use freeform summaries.
-
-### Handoff Document Standard (MANDATORY)
-
-All handoff documents MUST follow the dual-format standard established in v3:
-
-1. **Machine-parseable YAML frontmatter** (~160-200 lines):
-   - `schema_version`, plan info, phase transition, timestamps, author, status
-   - `files:` — created (with line counts), modified (with changes), deleted
-   - `dependencies:` — packages added/removed with reasons
-   - `tests:` — counts by module, breakdown, status
-   - `verification:` — each check with command, expected, actual, status
-   - `issues:` — type, location (file:line), description, priority
-   - `next_phase:` — critical_files, assumptions (with impact_if_wrong), constraints (must/must_not), failure_modes (scenario/symptom/fix)
-   - `rolling_context:` — all prior phases with one-paragraph summaries
-   - See: `.meridian/templates/HANDOFF.yaml.template` for full schema
-   - See: `.meridian/templates/snippets/HANDOFF-FRONTMATTER.md` for quick reference
-
-2. **Narrative markdown content** (~250-300 lines):
-   - Completion checklist (all tasks with checkmarks)
-   - "What Was Done" with code snippets for key implementations
-   - Decisions table with rationale AND alternatives rejected
-   - State changes YAML block (files created/modified, deps, test counts before/after)
-   - Known issues / tech debt (numbered)
-   - Context for next phase (numbered action items)
-   - Failure modes table for next phase (scenario | symptom | fix)
-   - Explicit next steps (numbered, with file paths)
-   - Logical phase status table (if applicable)
-   - Verification table (command | expected | actual | status)
-   - Rolling context summary (paragraph per completed phase)
-   - Session metrics YAML block
-   - Completion verification table
-
-**Target size**: 400-1000 lines total. **Minimum**: 300 lines.
-**Reference gold standard**: Historical handoffs (MP-0074, MP-0090, MP-0155) average 400-700 lines; complex multi-surface migrations and decomposition-heavy sub-phases may reach 700-1000 lines.
-
-**DO NOT** produce handoffs under 200 lines — this indicates critical sections are missing.
-
----
-
-## Read Policy Reminder (RULE-03)
-
-**You already read PLAN.md earlier in this session. Do NOT re-read it.**
-For this phase, read ONLY:
-1. `state.yaml` — current phase status, claims, locks
-2. This handoff prompt — context for this phase
-3. The prior handoff document (referenced above) — what was done, what's next
-
-Read `manifest.yaml` or `dependencies.yaml` ONLY if you need audit schedule or blocker info.
-**Never read** `INTAKE.yaml` or `EXECUTION-CONTRACT.yaml` during execution.
```

#### `START-PROMPT.md.template` — duplicate_divergent

- **RWP path**: `packages/rhumb-protocol/templates/START-PROMPT.md.template`
- **Meridian path**: `.meridian/templates/START-PROMPT.md.template`
- **RWP SHA-256**: `389cbe7b59c56a1d2e6ff8af5f3b75e7c264fac9251d81105f7eb08adadf35be`
- **Meridian SHA-256**: `5f912fc1bed76ff2fa66acd8c3e7c10fe8a46badb7f958600fe67afaaa400555`
- **Divergence summary**: Meridian-augmented overlay of RWP base — see §3.1 canonical pattern. Diff inlined below.
- **Direction of truth**: RWP (default per KD-14.8) — override iff explicit reason.
- **Recommended resolution**: Default per §3.1 (Meridian consumes RWP base + applies overlay); see §3.2 override table for per-file investigate flags. Final disposition locked in P-02 (KD-14.3); refactor in P-03.
- **Resolution owner**: MP-0275 P-03.
- **Consumers of Meridian copy**: See §7 Consumer Grep Sweep table for per-template counts and notable consumers. No template in this set has zero consumers; P-03 refactor MUST preserve the consumed path.

```diff
--- .meridian/templates/START-PROMPT.md.template	2026-04-29 15:37:14
+++ packages/rhumb-protocol/templates/START-PROMPT.md.template	2026-04-29 16:04:24
@@ -1,436 +1,50 @@
----
-meridian_version: "{{MERIDIAN_VERSION}}"
-template_type: "prompt"
-template_description: "Phase start prompt template (paste-and-go entry point)"
----
+═══════════════════════════════════════════════════════════════════════════════
+                        INITIALIZING NEW PLAN
+═══════════════════════════════════════════════════════════════════════════════
 
-# Start Prompt: [PLAN-ID] P-01-A
+I'll gather information to create a new plan. Please provide:
 
-**Plan**: [PLAN-ID] - [Plan Title]
-**Phase**: P-01-A - [Sub-Phase Title]
-**Classification**: [PUBLIC|PRIVATE|CONFIDENTIAL|TOP_SECRET]
-**Status**: READY TO START
-**Date**: {{TIMESTAMP}}
-**Duration**: ~30 minutes
+PLAN DETAILS
 
----
+  Title:
+    {{USER_ENTERS_TITLE}}
+    (e.g., "Database Migration to PostgreSQL", "Mobile App Redesign")
 
-> **Plan Document**: [PLAN.md]({PLAN_DIR}/PLAN.md)
-> For complete plan details including all sub-phases and deliverables.
->
-> **Full Handoff Document**: N/A (This is the first phase)
+  Objective:
+    {{USER_ENTERS_OBJECTIVE}}
+    (1-2 sentence goal of the plan)
 
----
+  Scope:
+    {{USER_ENTERS_SCOPE}}
+    (e.g., "5 major phases", "3-week timeline", "single-service refactor")
 
-## MANDATORY RULES - READ FIRST
+────────────────────────────────────────────────────────────────────────────
 
-**RULES:**
-  - GATE: If REQUEST-NORMALIZED.yaml is missing or not frozen, stop and generate/fix it first.
-  - Always follow Meridian rules at `.meridian/rules/<category>/*.yaml` (split-file tree; see `legacy-id-map.yaml` for ID aliases) with NO EXCEPTIONS.
-  
-**Sub-Phase Pattern**: This plan may use P-XX-A/B/C sub-phases (~30 min each) for crash resilience.
+Based on your input, I'll create:
 
-At the end of EVERY phase and sub-phase completion:
-1. **Release locks**: `.meridian/scripts/meridian-phase-lock.zsh release {PLAN_ID} P-01-A`
-2. CRITICAL: Create FULL handoff document: `HO-{PLAN_ID}-P-01-A-{YYYY-MM-DD}.yaml`
-3. CRITICAL: Create PROMPT file: `HO-{PLAN_ID}-P-01-A-TO-P-01-B-PROMPT.md`
-4. Update `state.yaml` with sub-phase completion
-5. **PROPAGATE this rule to ALL subsequent handoff documents and prompts**
+WORKFLOW STRUCTURE
 
-This ensures continuity across sessions and agents. **NO EXCEPTIONS.**
+  Plan ID:     {{AUTO_PLAN_ID}}
+  Location:    .meridian/.private/plans/processing/{{AUTO_PLAN_ID}}/
 
-### Completion Display Templates (MANDATORY — RULE-98)
+  Files:
+    ✓ PLAN.md - Full phase breakdown
+    ✓ MASTERPLAN.yaml - Hierarchical structure
+    ✓ INTAKE.yaml - Requirements
+    ✓ PLAN-STATE.yaml - Tracking
+    ✓ DEPENDENCIES.yaml - Phase graph
+    ✓ MANIFEST-PLAN.yaml - File inventory
 
-Every completion event MUST output a summary using the matching template:
+  First Phase:  {{AUTO_FIRST_PHASE_TITLE}} (~{{AUTO_FIRST_PHASE_DURATION}} min)
 
-| Completion Type | Template |
-|----------------|----------|
-| Sub-phase (P-XX-A/B/C) | `.meridian/templates/SUBPHASE-COMPLETE-DISPLAY.md.template` |
-| Full phase (P-XX, no sub-phases) | `.meridian/templates/PHASE-COMPLETE-DISPLAY.md.template` |
-| Audit (AUD-NN or FINAL) | `.meridian/templates/AUDIT-COMPLETE-DISPLAY.md.template` |
-| Entire plan complete | `.meridian/templates/PLAN-COMPLETE-DISPLAY.md.template` |
+────────────────────────────────────────────────────────────────────────────
 
-Output the display BEFORE creating handoff documents. Use the EXACT format from the template
-(═ borders, ALL CAPS headers, ``` code block, dot-leaders). Never use freeform summaries.
+Ready to create this plan officially?
 
----
+  → Say 'yes' or 'create it' and I'll commit to the workflow
+  → Or ask me to adjust anything before creating
 
-## Pre-Execution Checklist (MANDATORY - RULE-64, RULE-66, RULE-90)
-
-Before ANY work, complete these checks **IN ORDER**:
-
-### 0. CONCURRENT EXECUTION CHECK (RULE-90) - DO THIS FIRST
-
-**This is the START phase, but still check for concurrent execution (e.g., restarts).**
-
-```bash
-# Check if sub-phase is already being executed
-.meridian/scripts/meridian-phase-lock.zsh check {PLAN_ID} P-01-A
-```
-
-**Expected**: `Overall: AVAILABLE - Safe to execute`
-
-**If LOCKED**:
-```
-STOP - DO NOT PROCEED
-Another agent is already executing this sub-phase.
-Wait for completion or check if locks are stale.
-```
-
-**If lock is stale (expired)**:
-```bash
-# Only clear if you're CERTAIN no other agent is running
-.meridian/scripts/meridian-phase-lock.zsh clear {PLAN_ID} P-01-A --force
-```
-
-### 1. ACQUIRE LOCKS (RULE-90, RULE-91, RULE-92)
```


## 4. `rwp_only` — exists in RWP, not in Meridian

These files exist in the RWP source-of-truth tree but Meridian does
not currently consume them. After consume-not-duplicate refactor,
they will be available to Meridian via the chosen mechanism.

- `architecture/ACS-TEMPLATE.md`
- `architecture/AVD-TEMPLATE.md`
- `display/HANDOFF-COMPLETE-DISPLAY.md.template`
- `display/PHASE-COMPLETE-DISPLAY.md.template`
- `display/PLAN-COMMIT-DISPLAY.md.template`
- `display/PLAN-DRAFT-DISPLAY.md.template`
- `reference/HANDOFF-TEMPLATE.md`
- `reference/PHASE-AUDIT.md`
- `sequences.yaml.template`

## 5. `meridian_only` — exists in Meridian, not in RWP

These files are Meridian-internal and not part of RWP scope. They
remain as-is post-refactor (Meridian-internal infrastructure is
not subject to consume-not-duplicate).

- `ACS-PROMPT.md.template`
- `ACS.md.template`
- `acs/content/api-surface.yaml.template`
- `acs/content/architecture.yaml.template`
- `acs/content/components.yaml.template`
- `acs/content/constraints.yaml.template`
- `acs/content/cost-estimates.yaml.template`
- `acs/content/data-model.yaml.template`
- `acs/content/decisions.yaml.template`
- `acs/content/goals.yaml.template`
- `acs/content/phases.yaml.template`
- `acs/content/questions.yaml.template`
- `acs/content/risks.yaml.template`
- `acs/content/summary.yaml.template`
- `acs/diagrams/.gitkeep.template`
- `acs/meta.yaml.template`
- `acs/README.md`
- `acs/structure.yaml.template`
- `ADD.md.template`
- `AGENTS.md.template`
- `ARCHITECTURE-ACTUAL.yaml.template`
- `ARCHITECTURE-DECISION.md.template`
- `ARCHITECTURE-DIFF.md.template`
- `ARCHITECTURE-INPUT.yaml.template`
- `ARCHITECTURE-PROPOSED.yaml.template`
- `ARCHITECTURE.yaml.template`
- `AUDIT-COMPLETE-DISPLAY.md.template`
- `AUDIT-PROMPT.md.template`
- `AUDIT.md.template`
- `AUTHORITY-MAP.md.template`
- `AVD-PROMPT.md.template`
- `AVD.md.template`
- `avd/content/api-surface.yaml.template`
- `avd/content/architecture.yaml.template`
- `avd/content/components.yaml.template`
- `avd/content/constraints.yaml.template`
- `avd/content/cost-estimates.yaml.template`
- `avd/content/data-model.yaml.template`
- `avd/content/decisions.yaml.template`
- `avd/content/goals.yaml.template`
- `avd/content/phases.yaml.template`
- `avd/content/questions.yaml.template`
- `avd/content/risks.yaml.template`
- `avd/content/summary.yaml.template`
- `avd/diagrams/.gitkeep.template`
- `avd/meta.yaml.template`
- `avd/README.md`
- `avd/structure.yaml.template`
- `bitbucket/issue_templates/meridian-bug-report.md`
- `bitbucket/issue_templates/meridian-feature-request.md`
- `bitbucket/issue_templates/meridian-plan-request.md`
- `bitbucket/issue_templates/meridian-support-escalation.md`
- `bitbucket/PULL_REQUEST_TEMPLATE.md`
- `bitbucket/README.md`
- `BUG-REPORT.md.template`
- `CLASSIFICATION-HEADER.md.template`
- `CLAUDE-MERIDIAN.md.template`
- `CLAUDE.md.template`
- `claude/.claudeignore`
- `claude/CLAUDE-MERIDIAN.md`
- `claude/CLAUDE.md`
- `claude/commands/meridian/meridian-audit.md`
- `claude/commands/meridian/meridian-final-phase.md`
- `claude/commands/meridian/meridian-manifest.md`
- `claude/commands/meridian/meridian-phase-complete.md`
- `claude/commands/meridian/meridian-phase-start.md`
- `claude/commands/meridian/meridian-status.md`
- `claude/commands/meridian/meridian-validate.md`
- `claude/commands/shared/plan.md`
- `claude/EXECUTION.yaml.template`
- `claude/MagicString.txt`
- `CODEX.md.template`
- `codex/config.toml.template`
- `codex/README.md`
- `codex/requirements.toml.template`
- `codex/rules/meridian.rules.template`
- `codex/scripts/find-monorepo-root.sh`
- `codex/scripts/meridian-sounds-install.sh`
- `codex/scripts/meridian-sounds.sh`
- `codex/scripts/meridian/meridian-sounds.sh`
- `codex/scripts/meridian/sounds/bell.wav`
- `codex/scripts/meridian/sounds/chime.wav`
- `codex/scripts/meridian/sounds/error.wav`
- `codex/scripts/meridian/sounds/input.wav`
- `codex/scripts/meridian/sounds/README.md`
- `codex/scripts/meridian/sounds/success.wav`
- `codex/scripts/meridian/sounds/warning.wav`
- `codex/scripts/notify.sh`
- `codex/skills/architecture-enforcer/SKILL.md`
- `codex/skills/changelog/SKILL.md`
- `codex/skills/checkpoint/SKILL.md`
- `codex/skills/context/SKILL.md`
- `codex/skills/doc-updater/SKILL.md`
- `codex/skills/documentation-updater/SKILL.md`
- `codex/skills/execute-plan/SKILL.md`
- `codex/skills/list/SKILL.md`
- `codex/skills/meridian-audit/SKILL.md`
- `codex/skills/meridian-final-phase/SKILL.md`
- `codex/skills/meridian-manifest/SKILL.md`
- `codex/skills/meridian-phase-complete/SKILL.md`
- `codex/skills/meridian-phase-start/SKILL.md`
- `codex/skills/meridian-sounds/SKILL.md`
- `codex/skills/meridian-status/SKILL.md`
- `codex/skills/meridian-validate/SKILL.md`
- `codex/skills/performance-optimizer/SKILL.md`
- `codex/skills/plan/SKILL.md`
- `codex/skills/README.md`
- `codex/skills/rules/SKILL.md`
- `codex/skills/security-auditor/SKILL.md`
- `codex/skills/sounds/SKILL.md`
- `codex/skills/test-runner/SKILL.md`
- `CODING_GUIDELINES.md.template`
- `CONTRIBUTING.md.template`
- `DECISION-INPUT.md.template`
- `DEPENDENCY-ERROR-MESSAGES.md.template`
- `dependency/DEPENDENCY-TREE.md.template`
- `dependency/dependency-tree.yaml.template`
- `dependency/README.md`
- `EVIDENCE.md.template`
- `examples/HANDOFF-EXAMPLE.md`
- `examples/MANIFEST.yaml`
- `examples/PLACEMENT-MANIFEST-EXAMPLE.yaml`
- `EXECUTION-CONTRACT.yaml.template`
- `FINAL-PROMPT.md.template`
- `FINAL.md.template`
- `FIX-PLAN.md.template`
- `FIX-REPORT.md.template`
- `GAP-MATRIX.md.template`
- `GEMINI.md.template`
- `gemini/commands/meridian/meridian-phase-complete.md`
- `gemini/commands/meridian/meridian-phase-start.md`
- `gemini/commands/meridian/meridian-plan.md`
- `gemini/commands/meridian/meridian-status.md`
- `gemini/commands/sound.md`
- `gemini/README.md`
- `gemini/scripts/meridian-sounds.sh`
- `gemini/settings.json`
- `github/ISSUE_TEMPLATE/config.yml`
- `github/ISSUE_TEMPLATE/meridian-bug-report.yml`
- `github/ISSUE_TEMPLATE/meridian-feature-request.yml`
- `github/ISSUE_TEMPLATE/meridian-plan-request.yml`
- `github/ISSUE_TEMPLATE/meridian-support-escalation.yml`
- `github/pull_request_template.md`
- `github/README.md`
- `gitlab/issue_templates/meridian-bug-report.md`
- `gitlab/issue_templates/meridian-feature-request.md`
- `gitlab/issue_templates/meridian-plan-request.md`
- `gitlab/issue_templates/meridian-support-escalation.md`
- `gitlab/merge_request_templates/Default.md`
- `gitlab/README.md`
- `GLOBAL-STATE.yaml.template`
- `HANDOFF-SESSION.yaml.template`
- `HANDOFF.yaml.template`
- `IDEA.md.template`
- `INVENTORY.md.template`
- `ISSUES-REMEDIATION-PROMPT.md.template`
- `ISSUES.md.template`
- `MERIDIAN-CONFIG.yaml.template`
- `MRH-BLOCK.md.template`
- `NOTE.md.template`
- `OO.md.template`
- `PARALLEL-PROMPT.md.template`
- `PHASE-COMPLETE-DISPLAY.md.template`
- `PHASE.md.template`
- `PHASES.yaml.template`
- `PLAN-COMMIT-DISPLAY.md.template`
- `PLAN-COMPLETE-DISPLAY.md.template`
- `PLAN-DRAFT-DISPLAY.md.template`
- `plan.yaml.template`
- `README.md`
- `REQUEST-NORMALIZED.yaml.template`
- `REQUEST.yaml.template`
- `RESEARCH.md.template`
- `ROLLING-CONTEXT.md.template`
- `RQ.md.template`
- `RULES.yaml.template`
- `SECURITY-REVIEW.md.template`
- `sequences.yaml`
- `snippets/HANDOFF-FRONTMATTER.md`
- `SUBPHASE-COMPLETE-DISPLAY.md.template`
- `TRACKING-SCHEMA.yaml`
- `TRANSITION-PROMPT.md.template`

## 6. Layout Variance — same basename, different relative path

These basenames appear in both trees but at different relative paths.
Strict relative-path classification puts them in *_only* buckets, but
they may be conceptual duplicates with structural divergence. Each
needs a P-02 / P-03 disposition: harmonize layout, or treat as
legitimate non-overlap.

| Basename | RWP path | Meridian path |
|----------|----------|----------------|
| `PHASE-COMPLETE-DISPLAY.md.template` | `display/PHASE-COMPLETE-DISPLAY.md.template` | `PHASE-COMPLETE-DISPLAY.md.template` |
| `PLAN-COMMIT-DISPLAY.md.template` | `display/PLAN-COMMIT-DISPLAY.md.template` | `PLAN-COMMIT-DISPLAY.md.template` |
| `PLAN-DRAFT-DISPLAY.md.template` | `display/PLAN-DRAFT-DISPLAY.md.template` | `PLAN-DRAFT-DISPLAY.md.template` |

---

## 7. Consumer Grep Sweep

For every file in the `duplicate_divergent` and layout-variant buckets,
this sweep finds Meridian-side consumers (rules, scripts, source code,
docs). Templates with a non-trivial consumer count cannot be deleted —
the consumption mechanism (P-02) must materialize them at the same
relative path, or every consumer must be updated to look up the new path.

| Template                                  | Bucket             | Consumer count (sample) | Notable consumers                                                                                                  |
|-------------------------------------------|--------------------|--------------------------|--------------------------------------------------------------------------------------------------------------------|
| `DEPENDENCIES.yaml.template`              | divergent          | 8                        | `meridian-cli/build.rs`, `meridian-templates/src/types.rs`, `TEMPLATE-COMMAND-MAP.yaml`, `RULES.yaml`              |
| `INTAKE.yaml.template`                    | divergent          | 9                        | `template-resolver.ts`, `meridian-cli/src/commands/plan_rwp_templates.rs`, `TEMPLATE-COMMAND-MAP.yaml`             |
| `MANIFEST-PLAN.yaml.template`             | divergent          | 4                        | `meridian-templates/src/types.rs`, `TEMPLATE-COMMAND-MAP.yaml`, `meridian-cli/build.rs`                            |
| `MASTERPLAN.yaml.template`                | divergent          | 7                        | `meridian-templates/src/types.rs`, `TEMPLATE-COMMAND-MAP.yaml`, `README.md`, `TEMPLATE-USAGE-GUIDE.md`             |
| `PLAN-STATE.yaml.template`                | divergent          | 11                       | `version-sync.ts`, `RULES.yaml`, `meridian-validate-execution.zsh`, `meridian-templates/src/types.rs`              |
| `PLAN.md.template`                        | divergent          | 20+                      | `template-resolver.ts`, `meridian-cli/src/commands/render.rs`, `meridian-init-state.zsh`, `meridian-templates/src/registry.rs` |
| `PROMPT.md.template`                      | divergent          | 16                       | `RULES.yaml`, `naming.yaml`, `meridian-init-state.zsh`, `meridian-cli/src/commands/render.rs`                      |
| `START-PROMPT.md.template`                | divergent          | 10                       | `RULES.yaml`, `naming.yaml`, `multi-ai-setup.md`, `TEMPLATE-COMMAND-MAP.yaml`                                      |
| `PHASE-COMPLETE-DISPLAY.md.template`      | layout-variant     | 6                        | `enforcement.yaml`, `RULES.yaml`, `meridian-templates/src/types.rs`, `TEMPLATE-COMMAND-MAP.yaml`                   |
| `PLAN-COMMIT-DISPLAY.md.template`         | layout-variant     | 3                        | `meridian-cli/build.rs`, `meridian-templates/src/types.rs`, `TEMPLATE-COMMAND-MAP.yaml`                            |
| `PLAN-DRAFT-DISPLAY.md.template`          | layout-variant     | 3                        | `meridian-cli/build.rs`, `meridian-templates/src/types.rs`, `TEMPLATE-COMMAND-MAP.yaml`                            |

**Key findings from the sweep**:

1. **No "safe to delete with zero consumers"**. Every divergent and
   layout-variant template has live Meridian-side consumers. The P-03
   refactor MUST preserve the path each consumer looks up — materialize
   to the same relative path, or update every consumer.
2. **`meridian-templates/src/types.rs` and `TEMPLATE-COMMAND-MAP.yaml`
   are universal consumers**. Every divergent template appears in both.
   These are the central registry the consumption mechanism plumbs into.
3. **`meridian-cli/build.rs` is a universal consumer**. Build-time
   template registration; the consumption mechanism MUST integrate with
   the CLI build step (per ACS-0014 §6 build-step integration contract).
4. **`RULES.yaml` references several templates by name**. The rules
   tree treats specific template paths as authoritative; the
   consumption mechanism must preserve those paths or trigger a
   `RULES.yaml` rewrite.
5. **`template-resolver.ts` (yakkl-meridian)**. The TypeScript SDK
   side of Meridian also resolves templates. Both Rust and TypeScript
   surfaces consume the templates; consumption mechanism must serve both.

**Search command** (reproducible):
```bash
rg -l "<basename>" packages/yakkl-meridian-rs/ packages/yakkl-meridian/ \
   .meridian/scripts/ .meridian/rules/ .meridian/MANIFEST.yaml \
   .meridian/MERIDIAN.yaml | grep -v "/templates/" | grep -v "DRIFT-REPORT"
```

## 8. Rules-Tree Scan

Per AVD-0004 §4: "rule DSL grammar" is RWP-owned; "rule evaluators" are
Meridian. This scan checks whether `.meridian/rules/<category>/*.yaml`
contains any artifact whose grammar (not policy) is mirrored in
`packages/rhumb-protocol/spec/`.

**Meridian rules tree** (15 files, all `*.yaml` rule-evaluator policy):
```
.meridian/rules/audit/audit.yaml
.meridian/rules/classification/classification.yaml
.meridian/rules/concurrency/concurrency.yaml
.meridian/rules/coordination/coordination.yaml
.meridian/rules/core/foundation.yaml
.meridian/rules/enforcement/enforcement.yaml
.meridian/rules/file-operations/file-ops.yaml
.meridian/rules/handoff/handoff.yaml
.meridian/rules/lifecycle/lifecycle.yaml
.meridian/rules/multi-ai/multi-ai.yaml
.meridian/rules/parallel/parallel.yaml
.meridian/rules/quality/quality.yaml
.meridian/rules/review/review.yaml
.meridian/rules/session-state/session-state.yaml
.meridian/rules/templates/templates.yaml
```

**RWP `spec/` content** (artifact-format schemas, not rule grammars):
```
packages/rhumb-protocol/spec/schemas/handoff.schema.json
packages/rhumb-protocol/spec/schemas/intake.schema.json
packages/rhumb-protocol/spec/schemas/manifest.schema.json
packages/rhumb-protocol/spec/schemas/plan.schema.json
packages/rhumb-protocol/spec/schemas/state.schema.json
packages/rhumb-protocol/spec/sequence.grammar
```

**Result**: **No overlap.** RWP `spec/` defines artifact-document
schemas (JSON Schema for handoff, intake, manifest, plan, state) and a
sequence-parser grammar — these describe document SHAPES that RWP claims
authority over. Meridian's `.meridian/rules/` defines policy/behavior
that Meridian applies WHEN authoring or processing those documents
(audit triggers, file-ops policy, lifecycle gates). The two are
orthogonal:

- **RWP spec/ → "what does a valid handoff.yaml look like?"** (schema)
- **Meridian rules/ → "when must Meridian write a handoff?"** (policy)

Rules engine is Meridian-internal per AVD-0004 §4 — confirmed by direct
inspection. R10 risk does not fire.

**One caveat for P-02 awareness**: `.meridian/rules/RULES.yaml` is a
deprecated fallback (per CLAUDE.md). Several divergent templates are
referenced by name in `RULES.yaml` (see §7). When P-03 materializes
templates, those references must continue to resolve. P-02 should plan
for this in the mechanism choice.

## 9. P-01 Disposition

- All bucket counts above feed P-02 mechanism selection (KD-14.3 lock).
- `duplicate_identical` count drives no debate — these are the safe
  deletions in P-03.
- `duplicate_divergent` count is the surface area where direction-of-truth
  decisions are made. If count > 15 with substantive divergence,
  ACS-0014 §8 R5 fires — flag in handoff.
- Layout-variant count is informational; harmonization is a P-02 design
  decision, not a P-01 finding.

---

Produced:
  - when: 2026-04-30T20:53:35Z
  - by: YAKKL® Meridian™— drift-audit.zsh (MP-0275 P-01)
  - copyright: Copyright © 2026 YAKKL Inc. All Rights Reserved.
