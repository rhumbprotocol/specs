# Conformance Levels & Implementation Guidance

> Part of the [Rhumb Workflow Protocol (RWP)](../docs/PROTOCOL.md) - see also: [Conformance Levels](../docs/PROTOCOL.md#conformance-levels)

**Version**: 0.25.1
**Date**: 2026-03-04
**Classification**: Public

---

## Overview

RWP defines three conformance levels that guide implementation and validation:

1. **REQUIRED**: Must be present in all conformant implementations
2. **RECOMMENDED**: Should be present; best practice, improves interoperability
3. **OPTIONAL**: Implementation-specific extensions or future compatibility

This document explains the rationale, usage, and implications of each level.

---

## Conformance Level Definitions

### REQUIRED

**Definition**: Fields and features that MUST be present in all RWP-conformant implementations.

**Implication for Validators**:
- Validation FAILS if missing
- No exceptions or workarounds
- Core protocol functionality depends on these

**Implication for Implementers**:
- Non-negotiable; implement first
- Required in artifact instances
- Cannot be optional for backwards-compatibility

**Examples**:
- `title` (all artifacts)
- `phase_id` (phases)
- `captured` (intake)
- `artifact_type` (any artifact)

**Typical Percentage**: 30-40% of total fields

---

### RECOMMENDED

**Definition**: Fields and features that SHOULD be present in high-quality implementations, improving interoperability and usability.

**Implication for Validators**:
- Validation PASSES without these fields
- But tools may emit WARNINGS if missing
- Encourages best practices

**Implication for Implementers**:
- Implement after REQUIRED fields
- Provide sensible defaults if absent
- Document when/why they're omitted

**Examples**:
- `owner` (plans)
- `approval_date` (intake)
- `created_at` (manifests)
- `risks` (phases)

**Typical Percentage**: 40-50% of total fields

**When to Implement**:
1. Basic implementation complete and tested
2. Protocol adoption stabilizes
3. Specific use case requires these fields

---

### OPTIONAL

**Definition**: Fields for vendor extensions, experimental features, or future protocol evolution.

**Implication for Validators**:
- Validation ignores these fields
- No warnings or errors
- Tools should handle gracefully (or silently ignore)

**Implication for Implementers**:
- Implement based on specific needs
- Use custom field naming convention (`x-*`)
- Don't affect core workflow

**Examples**:
- `x-mycompany-billing-code` (custom)
- `x-analytics-metrics` (custom)
- Future fields in protocol v2.0

**Typical Percentage**: 10-20% of total fields

---

## Schema Annotation Pattern

Each field in RWP schemas can be annotated with conformance level:

```json
{
  "type": "object",
  "properties": {
    "title": {
      "type": "string",
      "conformance": "required",
      "description": "Human-readable plan title (REQUIRED)"
    },
    "overview": {
      "type": "string",
      "conformance": "required",
      "description": "Executive summary (REQUIRED)"
    },
    "owner": {
      "type": "string",
      "conformance": "recommended",
      "description": "Plan owner or team responsible (RECOMMENDED)"
    },
    "created_at": {
      "type": "string",
      "conformance": "recommended",
      "description": "ISO 8601 timestamp (RECOMMENDED)"
    },
    "custom_tracking_id": {
      "type": "string",
      "conformance": "optional",
      "description": "Implementation-specific tracking field (OPTIONAL)"
    }
  }
}
```

---

## Field-by-Field Conformance (All Artifacts)

### PLAN Artifact

| Field | Conformance | Notes |
|---|---|---|
| `title` | REQUIRED | Plan must have identifying title |
| `overview` | REQUIRED | Executive summary mandatory |
| `created_at` | REQUIRED | Timestamp for audit trail |
| `phases` | REQUIRED | At least one phase required |
| `owner` | RECOMMENDED | Best practice to identify responsible party |
| `classification` | RECOMMENDED | Helps with access control |
| `updated_at` | OPTIONAL | Updated timestamp |
| `goals_and_success_criteria` | OPTIONAL | Strategic context |
| `x-*` | OPTIONAL | Custom fields (vendor extensions) |

**Minimal Valid Plan** (REQUIRED only):

```yaml
title: "Quick Infrastructure Task"
overview: "Update DNS records for migration"
created_at: "2026-03-04T10:00:00Z"
phases:
  - phase_id: "P-01"
    title: "Update Records"
    objective: "Add new DNS entries"
    deliverables: ["DNS updated", "Verified"]
    tasks: ["Update Route53"]
    verification: ["nslookup"]
```

**Best Practice Plan** (REQUIRED + RECOMMENDED):

```yaml
title: "Q1 Infrastructure Upgrade"
overview: "Comprehensive modernization..."
created_at: "2026-03-04T10:00:00Z"
updated_at: "2026-03-04T15:30:00Z"
owner: "platform-team"
classification: "confidential"
goals_and_success_criteria:
  goals:
    - "Reduce deployment time by 50%"
    - "Improve reliability to 99.99%"
  success_criteria:
    - "Deploy time < 5 minutes"
    - "Uptime > 99.99%"
phases: [...]
```

---

### INTAKE Artifact

| Field | Conformance | Notes |
|---|---|---|
| `id` | REQUIRED | Unique intake identifier |
| `title` | REQUIRED | Clear problem statement |
| `captured` | REQUIRED | When intake was documented |
| `pain_points` | REQUIRED | Problems being addressed |
| `requirements` | REQUIRED | What must be built |
| `constraints` | REQUIRED | Limitations and boundaries |
| `success_criteria` | REQUIRED | How to measure success |
| `summary` | RECOMMENDED | Brief context overview |
| `approved_by` | RECOMMENDED | Approval chain |
| `approval_date` | RECOMMENDED | When approved |
| `classification` | RECOMMENDED | Security/access level |
| `background` | OPTIONAL | Historical context |
| `stakeholders` | OPTIONAL | Key parties |
| `x-*` | OPTIONAL | Custom fields |

**Minimal Valid Intake** (REQUIRED only):

```yaml
id: "INT-0001"
title: "API gateway performance degradation"
captured: "2026-03-04T12:00:00Z"
pain_points:
  - id: "PP-001"
    description: "p99 latency > 1s"
    impact: "User-facing degradation"
requirements:
  - id: "REQ-001"
    description: "Reduce p99 to < 200ms"
constraints:
  - "Cannot require database migration"
success_criteria:
  - "p99 < 200ms"
  - "tp99 improvement measured and verified"
```

---

### MANIFEST Artifact

| Field | Conformance | Notes |
|---|---|---|
| `id` | REQUIRED | Unique manifest identifier |
| `name` | REQUIRED | Human-readable name |
| `created_at` | REQUIRED | Creation timestamp |
| `artifacts` | REQUIRED | List of included artifacts |
| `description` | RECOMMENDED | Purpose and scope |
| `version` | RECOMMENDED | Manifest version |
| `updated_at` | OPTIONAL | Last update timestamp |
| `owners` | OPTIONAL | Multiple owners |
| `tags` | OPTIONAL | Categorization |
| `x-*` | OPTIONAL | Custom fields |

---

### STATE Artifact

| Field | Conformance | Notes |
|---|---|---|
| `plan_id` | REQUIRED | Associated plan identifier |
| `execution` | REQUIRED | Current execution status |
| `phases` | REQUIRED | Phase-by-phase tracking |
| `request_id` | RECOMMENDED | Link to original request |
| `audits` | RECOMMENDED | Audit checkpoint history |
| `error` | OPTIONAL | Error tracking (if occurred) |
| `history` | OPTIONAL | Event log |
| `x-*` | OPTIONAL | Custom fields |

---

### HANDOFF Artifact

| Field | Conformance | Notes |
|---|---|---|
| `id` | REQUIRED | Unique handoff identifier |
| `from_phase` | REQUIRED | Source phase |
| `to_phase` | REQUIRED | Destination phase |
| `created_at` | REQUIRED | Handoff timestamp |
| `context_summary` | REQUIRED | What was accomplished |
| `summary` | RECOMMENDED | Brief overview |
| `lessons_learned` | RECOMMENDED | Key insights |
| `next_phase_prompt` | OPTIONAL | Prompt for next phase |
| `verified_by` | OPTIONAL | Who verified handoff |
| `x-*` | OPTIONAL | Custom fields |

---

## Implementation Guidance by Conformance Level

### For REQUIRED Fields

1. **Always validate presence**
   ```bash
   jq '.title | select(. == null) | error("title is REQUIRED")' plan.json
   ```

2. **Never make optional**
   ```typescript
   // ✓ Good: Always validate
   const title = artifact.title;
   if (!title) throw new Error("title REQUIRED");

   // ✗ Bad: Making required field optional
   const title = artifact.title || 'Untitled';
   ```

3. **Document in specifications**
   ```markdown
   ## Required Fields
   All RWP artifacts MUST include:
   - title
   - created_at
   - ...
   ```

---

### For RECOMMENDED Fields

1. **Warn if absent (optional)**
   ```typescript
   if (!artifact.owner) {
     console.warn("⚠ RECOMMENDED field missing: owner");
   }
   ```

2. **Provide sensible defaults**
   ```typescript
   const owner = artifact.owner || process.env.DEFAULT_OWNER || 'team';
   ```

3. **Document in templates**
   ```markdown
   ## Recommended Fields
   Include these for best practices:
   - owner: Name of responsible party
   - classification: public|private|confidential
   ```

---

### For OPTIONAL Fields

1. **Silently ignore if absent**
   ```typescript
   const customField = artifact['x-mycompany-id'];  // undefined is OK
   if (customField) {
     processCustomField(customField);
   }
   ```

2. **Support graceful degradation**
   ```typescript
   // Tool works with or without custom field
   const trackingId = artifact['x-tracking-id'];
   if (trackingId) {
     logToTracker(trackingId);
   }
   // Continue regardless
   ```

3. **Version custom field contracts separately**
   ```yaml
   # In artifact
   x-mycompany-custom-fields-version: "0.25.1"
   x-mycompany-billing-code: "ENG-2026-Q1"
   ```

---

## Conformance Validation Matrix

Create a validation configuration:

```yaml
conformance_rules:
  strict:
    # Validate all levels
    required: validate_present
    recommended: validate_present
    optional: ignore

  moderate:
    # Validate REQUIRED, warn on RECOMMENDED
    required: validate_present
    recommended: warn_if_absent
    optional: ignore

  permissive:
    # Validate REQUIRED only
    required: validate_present
    recommended: ignore
    optional: ignore
```

**Usage**:

```bash
# Strict validation (all fields)
ajv validate -s plan.schema.json -d plan.json --conformance=strict

# Moderate validation (warn on RECOMMENDED)
ajv validate -s plan.schema.json -d plan.json --conformance=moderate

# Permissive validation (REQUIRED only)
ajv validate -s plan.schema.json -d plan.json --conformance=permissive
```

---

## Migration: Adding New Fields

When adding fields to RWP:

1. **Start as OPTIONAL** (v0.25.1)
   - Tools can ignore safely
   - No breaking changes
   - Example: `x-experimental-feature`

2. **Promote to RECOMMENDED** (v1.1.0 or v2.0.0)
   - Document why promotion happened
   - Tools should start using
   - Example: `classification`

3. **Promote to REQUIRED** (v3.0.0+)
   - Major version bump
   - All tools must support
   - Carefully considered (rare)

**Example Timeline**:

```markdown
## Field: `owner`

- **v0.1**: OPTIONAL (experimental)
- **v1.0**: RECOMMENDED (best practice adopted widely)
- **v2.0**: RECOMMENDED (no change needed)
- **v3.0**: REQUIRED? (under consideration)
```

---

## Conformance Certification

For tools claiming "RWP Conformance":

### Level 1: Basic Conformance

✓ Validates all REQUIRED fields
✓ Reads at least one artifact type
✗ No RECOMMENDED field support
✗ No extension support

**Example**: Artifact validator

### Level 2: Standard Conformance

✓ Validates all REQUIRED fields
✓ Supports all artifact types
✓ Handles RECOMMENDED fields
✗ No extension support

**Example**: Reference implementation

### Level 3: Full Conformance

✓ Validates all REQUIRED fields
✓ Supports all artifact types
✓ Supports all RECOMMENDED fields
✓ Handles extensions (custom fields)
✓ Validates schema composition

**Example**: Production framework

---

## Summary Table

| Level | Prevalence | Validator Behavior | Implementation Priority |
|---|---|---|---|
| **REQUIRED** | 30-40% | FAIL if missing | Implement first |
| **RECOMMENDED** | 40-50% | PASS, optional WARN | Implement after REQUIRED |
| **OPTIONAL** | 10-20% | PASS, always ignore | Implement based on needs |

---

Produced: 2026-03-04T04:45:00Z
By: YAKKL® Meridian™- https://meridian.yakkl.com
Copyright: Copyright © 2026 YAKKL Inc. All Rights Reserved.
