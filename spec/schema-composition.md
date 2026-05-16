# Schema Composition & Inheritance

> Part of the [Rhumb Workflow Protocol (RWP)](../docs/PROTOCOL.md) - see also: [Artifact Types & Schemas](../docs/PROTOCOL.md#artifact-types--schemas)

**Version**: 0.28.0
**Date**: 2026-03-04
**Classification**: Public

---

## Overview

This document explains how to compose, extend, and combine RWP schemas for:
1. Creating specialized artifact types
2. Combining multiple schemas
3. Versioned schema evolution
4. Multi-level validation hierarchies

---

## Composition Patterns

### Pattern 1: Schema Inheritance with `allOf`

Combine base schemas with extensions:

```json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "title": "Extended Plan with Billing",
  "allOf": [
    { "$ref": "plan.schema.json" },
    {
      "type": "object",
      "properties": {
        "x-billing-cost-center": { "type": "string" },
        "x-billing-budget-usd": { "type": "number", "minimum": 0 },
        "x-billing-approved-date": { "type": "string", "format": "date-time" }
      }
    }
  ]
}
```

**Use when**: Adding custom fields while preserving all base schema requirements.

**Benefit**: Inheritance chain is clear; tools can validate against base schema first.

---

### Pattern 2: Polymorphic Schemas with `oneOf`

Choose one of several schema variants:

```json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "title": "RWP Artifact (Polymorphic)",
  "oneOf": [
    { "$ref": "plan.schema.json" },
    { "$ref": "intake.schema.json" },
    { "$ref": "manifest.schema.json" },
    { "$ref": "state.schema.json" },
    { "$ref": "handoff.schema.json" }
  ],
  "discriminator": {
    "propertyName": "artifact_type",
    "mapping": {
      "plan": "plan.schema.json",
      "intake": "intake.schema.json",
      "manifest": "manifest.schema.json",
      "state": "state.schema.json",
      "handoff": "handoff.schema.json"
    }
  }
}
```

**Use when**: A single schema field can be multiple types depending on discriminator.

**Benefit**: Validator can quickly route to correct schema based on type field.

---

### Pattern 3: Conditional Composition with `if/then/else`

Apply different schema rules based on conditions:

```json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "title": "Conditional Plan Validation",
  "type": "object",
  "properties": {
    "classification": { "type": "string", "enum": ["public", "private", "confidential"] },
    "owner": { "type": "string" }
  },
  "if": {
    "properties": { "classification": { "const": "confidential" } }
  },
  "then": {
    "required": ["owner"],
    "properties": {
      "x-security-audit-id": { "type": "string" },
      "x-security-requires-review": { "type": "boolean" }
    }
  },
  "else": {
    "properties": {
      "owner": { "type": "string" }
    }
  }
}
```

**Use when**: Required fields change based on property values.

**Benefit**: Single schema adapts to multiple scenarios.

---

## Real-World Examples

### Example 1: Plan with Custom Fields

**Goal**: Create a specialized plan schema for infrastructure projects.

```json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "title": "Infrastructure Plan (RWP Plan + Custom Fields)",
  "description": "Plan variant for infrastructure projects with deployment and monitoring extensions",
  "allOf": [
    {
      "$ref": "plan.schema.json"
    },
    {
      "type": "object",
      "properties": {
        "x-deployment-target-environment": {
          "type": "string",
          "enum": ["staging", "production"],
          "description": "Target environment for deployment"
        },
        "x-deployment-blue-green-enabled": {
          "type": "boolean",
          "description": "Whether to use blue-green deployment"
        },
        "x-deployment-rollback-plan": {
          "type": "string",
          "description": "Steps to rollback if deployment fails"
        },
        "x-monitoring-alert-threshold-p99": {
          "type": "number",
          "minimum": 0,
          "description": "P99 latency alert threshold in milliseconds"
        },
        "x-monitoring-slo-target": {
          "type": "string",
          "pattern": "^[0-9]{2}\\.[0-9]{2}%$",
          "description": "SLO target percentage (e.g., 99.95%)"
        }
      }
    }
  ]
}
```

**YAML Example**:

```yaml
title: "Kubernetes Cluster Upgrade - Q1 2026"
overview: "Upgrade EKS clusters from 1.27 to 1.29 in all regions"
created_at: "2026-01-15T10:00:00Z"
classification: "confidential"
owner: "platform-team"

# Custom infrastructure fields
x-deployment-target-environment: "production"
x-deployment-blue-green-enabled: true
x-deployment-rollback-plan: "Revert to 1.27 using previous AMI and re-apply Helm charts"
x-monitoring-alert-threshold-p99: 500
x-monitoring-slo-target: "99.95%"

phases:
  - phase_id: "P-01"
    title: "Staging Upgrade"
    objective: "Validate upgrade process in staging"
    ...
```

---

### Example 2: Handoff with Optional Analytics

**Goal**: Add optional analytics tracking to handoff documents.

```json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "title": "Handoff with Analytics (RWP Handoff + Extensions)",
  "allOf": [
    {
      "$ref": "handoff.schema.json"
    },
    {
      "type": "object",
      "properties": {
        "x-analytics-phase-duration-minutes": {
          "type": "integer",
          "minimum": 1,
          "description": "Actual phase execution time"
        },
        "x-analytics-total-tasks-completed": {
          "type": "integer",
          "minimum": 0,
          "description": "Number of tasks completed"
        },
        "x-analytics-blockers-encountered": {
          "type": "array",
          "items": { "type": "string" },
          "description": "Issues that blocked progress"
        },
        "x-analytics-context-file-reads": {
          "type": "integer",
          "minimum": 0,
          "description": "Number of files read during phase"
        },
        "x-analytics-tokens-used": {
          "type": "integer",
          "minimum": 0,
          "description": "Tokens consumed in this phase"
        }
      }
    }
  ]
}
```

**YAML Example**:

```yaml
id: "HO-MP-0235-P-02-A-2026-03-04"
from_phase: "P-02-A"
to_phase: "P-02-B"
created_at: "2026-03-04T04:30:00Z"
summary: "UUID specification complete, sequence parser reference implementation done"

# Analytics extensions
x-analytics-phase-duration-minutes: 45
x-analytics-total-tasks-completed: 5
x-analytics-blockers-encountered: []
x-analytics-context-file-reads: 12
x-analytics-tokens-used: 42000

context_summary: "P-02-A delivered all 5 tasks with reference implementations..."
lessons_learned: ["...", "..."]
next_phase_prompt: "..."
```

---

### Example 3: Conditional Security Requirements

**Goal**: Require security fields only for confidential plans.

```json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "title": "Conditional Security Plan",
  "description": "Plans marked confidential require security approval",
  "allOf": [
    {
      "$ref": "plan.schema.json"
    },
    {
      "if": {
        "properties": {
          "classification": { "const": "confidential" }
        }
      },
      "then": {
        "required": ["x-security-audit-id", "x-security-approved-date"],
        "properties": {
          "x-security-audit-id": {
            "type": "string",
            "pattern": "^AUDIT-[0-9]{4}$",
            "description": "Associated security audit ID"
          },
          "x-security-approved-date": {
            "type": "string",
            "format": "date-time",
            "description": "When security review was completed"
          },
          "x-security-compliance-frameworks": {
            "type": "array",
            "items": { "type": "string" },
            "enum": ["sox2", "hipaa", "pci-dss", "iso27001"],
            "description": "Applicable compliance frameworks"
          }
        }
      }
    }
  ]
}
```

**YAML Example**:

```yaml
title: "Payment System Security Upgrade"
classification: "confidential"
created_at: "2026-02-01T09:00:00Z"

# Required because classification = confidential
x-security-audit-id: "AUDIT-0001"
x-security-approved-date: "2026-02-28T17:00:00Z"
x-security-compliance-frameworks: ["pci-dss", "sox2"]

phases: [...]
```

---

## Composition in Code

### TypeScript Example: Validating with Composed Schemas

```typescript
import Ajv from 'ajv';
import planSchema from './schemas/plan.schema.json';
import infrastructurePlanSchema from './schemas/infrastructure-plan.schema.json';

const ajv = new Ajv();

// Compile both schemas
const validatePlan = ajv.compile(planSchema);
const validateInfraPlan = ajv.compile(infrastructurePlanSchema);

// Validate against base schema
const plan = {
  title: "Kubernetes Upgrade",
  overview: "Upgrade EKS to 1.29",
  created_at: "2026-01-15T10:00:00Z",
  phases: []
};

if (validatePlan(plan)) {
  console.log('✓ Valid RWP Plan');
} else {
  console.error('✗ Invalid:', validatePlan.errors);
}

// Validate with infrastructure extensions
const infraPlan = {
  ...plan,
  "x-deployment-target-environment": "production",
  "x-deployment-blue-green-enabled": true,
  "x-monitoring-slo-target": "99.95%"
};

if (validateInfraPlan(infraPlan)) {
  console.log('✓ Valid Infrastructure Plan');
} else {
  console.error('✗ Invalid:', validateInfraPlan.errors);
}
```

### Python Example: Loading and Composing

```python
import json
import jsonschema
from jsonschema import Draft7Validator

# Load base schema
with open('schemas/plan.schema.json') as f:
    plan_schema = json.load(f)

# Create infrastructure plan schema with composition
infra_plan_schema = {
    "$schema": "http://json-schema.org/draft-07/schema#",
    "title": "Infrastructure Plan",
    "allOf": [
        plan_schema,
        {
            "type": "object",
            "properties": {
                "x-deployment-target-environment": {
                    "type": "string",
                    "enum": ["staging", "production"]
                },
                "x-monitoring-slo-target": {"type": "string"}
            }
        }
    ]
}

# Validate
validator = Draft7Validator(infra_plan_schema)

plan = {
    "title": "Kubernetes Upgrade",
    "overview": "Upgrade to 1.29",
    "created_at": "2026-01-15T10:00:00Z",
    "phases": [],
    "x-deployment-target-environment": "production",
    "x-monitoring-slo-target": "99.95%"
}

if validator.is_valid(plan):
    print("✓ Valid Infrastructure Plan")
else:
    for error in validator.iter_errors(plan):
        print(f"✗ Error: {error.message}")
```

---

## Schema Versioning & Composition

### Versioned Schemas

Keep track of schema versions:

```json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "title": "Plan Schema v2.0.0",
  "description": "Breaking changes from v0.28.0: phases.duration_minutes renamed to duration_minutes",
  "version": "2.0.0",
  "type": "object",
  "properties": {
    "schema_version": {
      "type": "string",
      "pattern": "^[0-9]+\\.[0-9]+\\.[0-9]+$",
      "description": "Version of this schema"
    },
    "...": {}
  }
}
```

### Compatibility Matrix

Document how versions compose:

| Base Version | Extension Version | Compatible | Notes |
|---|---|---|---|
| 0.28.0 | 1.x.x | ✓ Yes | Patch updates always compatible |
| 0.28.0 | 2.0.0 | ⚠ Maybe | Breaking changes possible |
| 2.0.0 | 1.x.x | ✗ No | Extension requires newer base |

---

## Best Practices

### 1. Keep Composition Shallow

```json
// ✓ Good: 2 levels
{
  "allOf": [
    { "$ref": "base.schema.json" },
    { /* custom fields */ }
  ]
}

// ✗ Bad: deeply nested
{
  "allOf": [
    { "allOf": [
        { "allOf": [ /* ... */ ] }
      ]
    }
  ]
}
```

### 2. Use `$ref` for Clarity

```json
// ✓ Good: explicit reference
{ "$ref": "plan.schema.json" }

// ✗ Bad: duplicated content
{
  "type": "object",
  "properties": {
    "title": { "type": "string" },
    "overview": { "type": "string" },
    // ... copy-pasted from plan.schema.json
  }
}
```

### 3. Document Composition Assumptions

```markdown
## Schema Composition Notes

This schema (infrastructure-plan.schema.json) extends plan.schema.json:

- **Base**: plan.schema.json v0.28.0
- **Custom fields**: x-deployment-*, x-monitoring-*
- **Breaking changes**: None (backward compatible)
- **Requires**: AJV with `draft-07` support

### Validation Order

1. Validate against plan.schema.json base
2. Validate custom fields against infrastructure extensions
3. Apply conditional rules if classification = "confidential"
```

### 4. Test Composition Combinations

```bash
# Test base schema
ajv test -s plan.schema.json -d valid-plans.json

# Test extended schema
ajv test -s infrastructure-plan.schema.json -d valid-infra-plans.json

# Test composition chain
ajv compile -s plan.schema.json
ajv compile -s infrastructure-plan.schema.json
```

---

## Summary Table

| Pattern | Use Case | Example |
|---------|----------|---------|
| **allOf** | Extend with custom fields | Infrastructure Plan = Plan + x-deployment-* |
| **oneOf** | Multiple artifact types | Any RWP artifact: Plan \| Intake \| Manifest |
| **if/then/else** | Conditional requirements | Confidential plans require security fields |
| **$ref** | Reuse base schema | Every extension references a base |
| **Custom fields** | Add vendor metadata | x-mycompany-billing-code |
| **Version embedding** | Track schema version | `schema_version: "0.28.0"` |

Composition patterns provide unlimited extensibility while maintaining validation integrity and interoperability.

---

Produced: 2026-03-04T04:45:00Z
By: YAKKL® Meridian™- https://meridian.yakkl.com
Copyright: Copyright © 2026 YAKKL Inc. All Rights Reserved.
