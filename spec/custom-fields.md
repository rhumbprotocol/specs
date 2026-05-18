# Custom Fields & Extension Patterns

> Part of the [Rhumb Workflow Protocol (RWP)](../docs/PROTOCOL.md) - see also: [Extension Mechanism](../docs/PROTOCOL.md#extension-mechanism)

**Version**: 0.29.0
**Date**: 2026-03-04
**Classification**: Public

---

## Overview

RWP schemas support extension through custom fields, allowing implementations to add vendor-specific, domain-specific, or project-specific metadata without modifying the core artifact structure.

This document defines:
1. **Naming conventions** for custom fields
2. **Namespacing patterns** to prevent collisions
3. **Validation rules** and best practices
4. **Real-world examples** in YAML and JSON formats

---

## Custom Field Naming Convention

### Namespace Prefix Pattern

All custom fields MUST use the namespace prefix pattern:

```
x-<vendor>-<domain>-<field-name>
```

**Components**:
- **Prefix**: `x-` (vendor extension marker, following JSON/HTTP standards)
- **Vendor**: lowercase identifier for the organization or tool (e.g., `yakkl`, `meridian`, `acme`)
- **Domain**: lowercase category (e.g., `billing`, `security`, `analytics`, `ui`)
- **Field name**: lowercase with hyphens (e.g., `cost-center`, `compliance-level`, `theme-preset`)

### Examples

```yaml
# Meridian-specific fields
x-meridian-budget-token-limit: 500000
x-meridian-budget-tier: individual
x-meridian-analytics-tool: claude-code

# Project-specific fields
x-acme-project-billing-code: BIL-2026-Q1
x-acme-security-clearance-level: high
x-acme-compliance-frameworks: [sox, hipaa, pci-dss]

# Tool-specific fields
x-github-actions-runner-label: ubuntu-latest
x-datadog-environment-tags: [prod, critical]
```

---

## Validation Rules

### Required

1. **All custom fields MUST start with `x-`**
   - Prevents collision with standard RWP fields
   - Follows JSON Schema and OpenAPI conventions

2. **Vendor and domain components MUST be lowercase ASCII**
   - Valid: `a-z`, `0-9`, `-` (no leading/trailing dashes)
   - Pattern: `^[a-z0-9]([a-z0-9-]*[a-z0-9])?$`

3. **Custom field values MUST be JSON-serializable**
   - Strings, numbers, booleans, arrays, objects
   - No functions, dates (use ISO 8601 strings), undefined values

4. **Custom fields MUST NOT shadow standard fields**
   - Standard field names are reserved (title, overview, phases, etc.)
   - Use namespaced fields instead

### Recommended

1. **Document custom fields in a metadata section**
   - Include in INTAKE, MANIFEST, or comments
   - Explain purpose, expected values, and usage

2. **Use consistent vendor naming**
   - Pick one vendor identifier per organization
   - Register it in your implementation docs (optional)

3. **Validate custom field presence at runtime**
   - Tools should gracefully handle missing custom fields
   - Don't require custom fields for core functionality

4. **Version custom field schemas separately**
   - If custom fields change, bump your own version
   - RWP version changes don't affect custom field contracts

---

## Extension Patterns by Artifact Type

### Plan Extensions

Common extensions for PLAN artifacts:

```yaml
title: "Q1 Infrastructure Upgrade"
overview: "Modernize deployment pipeline and reduce costs"
created_at: "2026-01-15T09:00:00Z"

# Custom fields: billing, analytics, security
x-billing-cost-center: "ENG-2026-Q1"
x-billing-estimated-cost-usd: 45000
x-analytics-project-type: "infrastructure"
x-security-requires-review: true
x-security-compliance-frameworks: ["sox2", "hipaa"]

phases: [...]
```

### Intake Extensions

Common extensions for INTAKE artifacts:

```yaml
id: "INT-0042"
title: "Performance degradation in API gateway"
captured: "2026-02-28T14:30:00Z"

# Custom fields: routing, priority, stakeholders
x-routing-on-call-team: "platform-infrastructure"
x-routing-escalation-manager: "jane.smith"
x-priority-business-impact: "critical"
x-priority-affected-users-count: 15000
```

### Manifest Extensions

Common extensions for MANIFEST artifacts:

```yaml
id: "MAN-2026-Q1-PLATFORM"
name: "Platform Modernization Manifest"
created_at: "2026-01-01T00:00:00Z"

# Custom fields: deployment, monitoring
x-deployment-target-environment: "production"
x-deployment-blue-green-enabled: true
x-monitoring-alert-threshold-p99-ms: 500
x-monitoring-slo-target: "99.95"
```

### State Extensions

Common extensions for STATE artifacts:

```yaml
plan_id: "MP-0235-rhumb-workflow-protocol"
execution:
  status: "in_progress"

# Custom fields: tracking, notifications
x-tracking-slack-channel: "#meridian-updates"
x-tracking-github-milestone: "2026-q1"
x-notifications-email-list: ["team@example.com"]
x-notifications-on-completion: true
```

### Handoff Extensions

Common extensions for HANDOFF artifacts:

```yaml
id: "HO-MP-0235-P-02-A"
from_phase: "P-02-A"
to_phase: "P-02-B"

# Custom fields: lessons, risks, recommendations
x-lessons-learned-tags: ["schema-validation", "test-coverage"]
x-lessons-learned-key-insights:
  - "Automated validation catches most issues early"
  - "Custom fields need clear documentation"
x-recommendations-for-next-phase: ["increase test coverage", "document edge cases"]
x-risks-identified: ["schema drift", "version incompatibility"]
```

---

## Real-World Examples

### Example 1: Multi-Vendor Extensions

A plan used across multiple tools might have:

```yaml
title: "Payment System Migration"

# Meridian (orchestration)
x-meridian-budget-limit: 1000000
x-meridian-approval-required: true

# DataDog (monitoring)
x-datadog-dashboard-id: "abc123def456"
x-datadog-slos: ["api-99.9", "db-99.5"]

# GitHub (source control)
x-github-repository: "company/payment-system"
x-github-branch-protection-enabled: true

# Internal (billing)
x-internal-billing-cost-code: "PROD-2026-Q1"
x-internal-billing-budget-owner: "finance-team"

# PagerDuty (incidents)
x-pagerduty-escalation-policy: "platform-oncall"
x-pagerduty-severity-level: "critical"
```

### Example 2: Compliance & Audit

An intake capturing security requirements:

```yaml
id: "INT-0123"
title: "SOX 2 Compliance Gap Remediation"

x-compliance-frameworks: ["sox2", "hipaa", "pci-dss"]
x-compliance-audit-id: "AUDIT-2026-001"
x-compliance-certification-deadline: "2026-06-30T23:59:59Z"

x-security-required-reviews:
  - type: "security-architecture"
    reviewer: "security-team"
  - type: "penetration-test"
    reviewer: "external-security-vendor"

x-security-data-classification: "confidential"
x-security-requires-encryption-at-rest: true
x-security-requires-encryption-in-transit: true
```

### Example 3: Analytics & Tracking

A manifest with detailed tracking:

```yaml
id: "MAN-2026-ANALYTICS"
name: "Analytics Platform Rollout"

x-analytics-cohort-size: 2500
x-analytics-experiment-duration-days: 14
x-analytics-metrics:
  - metric: "query-latency-p99"
    threshold: 250
    unit: "milliseconds"
  - metric: "feature-adoption-rate"
    threshold: 0.75
    unit: "percentage"

x-analytics-reporting-cadence: "daily"
x-analytics-stakeholder-email: "data-team@company.com"
```

---

## Validation JSON Schema

Here's a JSON Schema pattern for validating custom fields at the document level:

```json
{
  "type": "object",
  "patternProperties": {
    "^x-[a-z0-9]([a-z0-9-]*[a-z0-9])?(-[a-z0-9]([a-z0-9-]*[a-z0-9])?)*$": {
      "description": "Custom field following x-vendor-domain-field-name pattern"
    }
  },
  "errorMessage": {
    "patternProperties": "Custom fields must start with 'x-' followed by lowercase vendor, domain, and field name separated by hyphens"
  }
}
```

To validate a custom field name:

```typescript
const customFieldPattern = /^x-[a-z0-9]([a-z0-9-]*[a-z0-9])?(-[a-z0-9]([a-z0-9-]*[a-z0-9])?)*$/;
const isValid = customFieldPattern.test(fieldName);
```

---

## Best Practices

### 1. Document Your Custom Namespace

In your implementation or tool documentation:

```markdown
## Custom Fields Reference

### x-mycompany-* namespace

All custom fields in this section are specific to MyCompany's workflow platform.

#### x-mycompany-billing-cost-center

- **Type**: string
- **Required**: no
- **Example**: "ENG-2026-Q1"
- **Description**: Cost center code for chargeback

#### x-mycompany-security-audit-id

- **Type**: string
- **Required**: no
- **Example**: "AUDIT-2026-001"
- **Description**: Associated audit or compliance review ID
```

### 2. Handle Missing Custom Fields Gracefully

```typescript
// ✓ Good: optional custom fields
const costCenter = artifact['x-mycompany-billing-cost-center'] || 'UNKNOWN';

// ✗ Bad: assumes custom field exists
const costCenter = artifact['x-mycompany-billing-cost-center'].toUpperCase();
```

### 3. Separate Custom Schema from Core

Keep custom field validation separate:

```typescript
interface RWPPlan {
  // Core fields (required by spec)
  title: string;
  overview: string;
  phases: Phase[];

  // Custom fields (extension mechanism)
  [key: string]: any;  // Catch-all for x-* fields
}
```

### 4. Version Custom Field Contracts

If you define custom fields, version them:

```yaml
# In your implementation docs
custom_fields_version: "0.29.0"

# In artifacts
x-mycompany-custom-fields-version: "0.29.0"
```

### 5. Audit Custom Field Usage

Track which custom fields are actually used:

```bash
# Find all custom fields in a plan
grep -o 'x-[a-z0-9]*(-[a-z0-9]*)*' PLAN.md | sort -u

# Count usage
grep -o 'x-[a-z0-9]*(-[a-z0-9]*)*' *.yaml | cut -d: -f2 | sort | uniq -c
```

---

## Migration & Deprecation

### Adding a Custom Field

1. Document in your implementation guide
2. Add to examples in INTAKE or MANIFEST
3. Make optional (not required in schemas)
4. Test handling in tools

### Deprecating a Custom Field

1. Mark in documentation: "Deprecated as of version X.Y.Z"
2. Suggest replacement field
3. Continue supporting in tools (don't break)
4. Remove in major version bump

### Example

```markdown
#### x-mycompany-old-field (DEPRECATED)

**Deprecated**: Use `x-mycompany-new-field` instead (as of 1.5.0)
**Removed**: Will be removed in version 2.0.0

[Legacy behavior details...]
```

---

## Conflict Resolution

### Collision Avoidance

If two vendors want similar fields:

```yaml
# Meridian team
x-meridian-budget-token-limit: 500000

# Anthropic team
x-anthropic-quota-max-tokens: 500000

# Both are valid - no collision because vendor namespace is different
```

### If Collision Occurs

Choose the most authoritative source:

1. **Official implementations** take precedence over third-party
2. **Older registrations** take precedence over newer
3. **Establish a registration** to prevent future collisions

---

## Summary

| Aspect | Rule | Example |
|--------|------|---------|
| **Pattern** | `x-vendor-domain-field` | `x-meridian-budget-limit` |
| **Vendor** | Lowercase, no special chars | `yakkl`, `anthropic`, `acme` |
| **Domain** | Lowercase, no special chars | `budget`, `security`, `billing` |
| **Field** | Lowercase with hyphens | `token-limit`, `cost-center` |
| **Values** | JSON-serializable | strings, numbers, booleans, arrays, objects |
| **Required** | No (always optional) | Safe to add without breaking |
| **Collision** | Namespace prevents | Each vendor isolated |

Custom fields provide unlimited extensibility while preserving schema stability and interoperability.

---

Produced: 2026-03-04T04:45:00Z
By: YAKKL® Meridian™- https://meridian.yakkl.com
Copyright: Copyright © 2026 YAKKL Inc. All Rights Reserved.
