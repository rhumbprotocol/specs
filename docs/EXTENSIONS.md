# Extending RWP™

A guide to extending the Rhumb Workflow Protocol™ with custom fields, artifacts, and domain-specific additions.

> See also: [PROTOCOL.md](./PROTOCOL.md) (full specification), [Custom Fields](../spec/custom-fields.md), [Schema Composition](../spec/schema-composition.md), [Conformance Levels](../spec/conformance-levels.md)

---

## Why Extend RWP™?

RWP™ provides a structured foundation for workflow management, but every organization has unique needs - billing codes, compliance requirements, monitoring hooks, team-specific metadata. Rather than baking these into the core protocol, RWP™ provides formal extension points that let you add domain-specific capabilities while preserving interoperability.

Extensions fall into three categories:

1. **Custom fields** - Add metadata to existing artifact types
2. **Custom artifact types** - Define entirely new document types
3. **Integration extensions** - Adapt RWP for specific tools or platforms

---

## Custom Fields

The simplest and most common extension pattern. Custom fields add key-value pairs to standard RWP™ artifacts without modifying the core schema.

### Naming Convention

All custom fields use the `x-` prefix with vendor namespacing:

```
x-<vendor>-<domain>-<field-name>
```

**Examples**:

```yaml
# Organization-specific
x-acme-billing-cost-center: "ENG-2026-Q1"
x-acme-security-clearance: "confidential"

# Tool-specific
x-datadog-dashboard-id: "abc123"
x-slack-notification-channel: "#deploys"

# Domain-specific
x-compliance-framework: "sox2"
x-analytics-cohort-size: 2500
```

### Where to Use Custom Fields

Custom fields can be added to any RWP™ artifact type:

| Artifact | Common Extensions |
|----------|------------------|
| **Plan** | Billing codes, cost estimates, approval workflows |
| **Intake** | Priority scoring, routing rules, SLA targets |
| **State** | Notification channels, dashboard links |
| **Manifest** | Deployment targets, monitoring configuration |
| **Handoff** | Lesson tags, risk assessments, reviewer notes |

### Validation

Custom fields are always optional. Tools should handle them gracefully:

```typescript
// Good: check before using
const billingCode = artifact['x-acme-billing-cost-center'];
if (billingCode) {
  assignToCostCenter(billingCode);
}

// Bad: assume field exists
const billingCode = artifact['x-acme-billing-cost-center'].toUpperCase();
```

For detailed patterns, naming rules, and real-world examples, see the [Custom Fields specification](../spec/custom-fields.md).

---

## Custom Artifact Types

For workflows that need document types beyond the standard five (Plan, Intake, Manifest, State, Handoff), RWP™ supports defining custom artifact types.

### When to Create a Custom Artifact

Consider a custom artifact type when:
- Your data doesn't fit naturally into any existing artifact
- You need a distinct lifecycle (different from phases or handoffs)
- Multiple teams would benefit from the new type
- The document has its own schema and validation rules

**Good candidates**: Risk registers, decision logs, architecture records, test plans, runbooks.

**Not needed**: If data fits as custom fields on an existing artifact, use custom fields instead.

### Defining a Custom Artifact

A custom artifact type needs:

1. **A schema** - JSON Schema describing the structure
2. **A template** - Starting point for creating instances
3. **Lifecycle rules** - How the artifact transitions between states
4. **Integration with standard artifacts** - How it references Plans, Phases, etc.

**Example: Decision Log artifact**

```yaml
# decision-log.yaml
artifact_type: "x-decision-log"
rwp_version: "0.31.0"
id: "DL-0001"
plan_id: "MP-0042-dark-mode-toggle"

decisions:
  - id: "DEC-001"
    title: "Use PostgreSQL over DynamoDB"
    phase: "P-01-A"
    date: "2026-03-04T10:00:00Z"
    status: "approved"
    context: "Need ACID transactions for payment processing"
    options_considered:
      - name: "PostgreSQL"
        pros: ["ACID", "SQL", "mature tooling"]
        cons: ["Operational overhead"]
      - name: "DynamoDB"
        pros: ["Managed", "auto-scaling"]
        cons: ["No ACID", "vendor lock-in"]
    decision: "PostgreSQL"
    rationale: "Payment integrity requires ACID guarantees"
    consequences: ["Need RDS provisioning", "Add pgbouncer for connection pooling"]
```

### Schema for Custom Artifacts

Define a JSON Schema that extends the base RWP™ artifact pattern:

```json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "title": "Decision Log",
  "description": "Custom RWP™ artifact for recording architectural decisions",
  "type": "object",
  "required": ["artifact_type", "id", "decisions"],
  "properties": {
    "artifact_type": {
      "type": "string",
      "const": "x-decision-log"
    },
    "rwp_version": {
      "type": "string"
    },
    "id": {
      "type": "string",
      "pattern": "^DL-\\d{4}$"
    },
    "plan_id": {
      "type": "string"
    },
    "decisions": {
      "type": "array",
      "items": { "$ref": "#/$defs/decision" }
    }
  }
}
```

For composition patterns (inheritance, combining schemas), see [Schema Composition](../spec/schema-composition.md).

---

## Domain-Specific Extensions

Some domains benefit from coordinated sets of custom fields and artifacts. Here are patterns for common domains.

### DevOps / Infrastructure

```yaml
# In PLAN artifacts
x-infra-terraform-workspace: "production"
x-infra-cloud-provider: "aws"
x-infra-cost-estimate-monthly-usd: 3200
x-infra-rollback-plan: "Revert to previous Terraform state"

# Custom artifact: Runbook
artifact_type: "x-runbook"
triggers:
  - condition: "p99 > 500ms"
    action: "Scale horizontally"
  - condition: "error_rate > 5%"
    action: "Rollback deployment"
```

### Compliance / Regulated Industries

```yaml
# In INTAKE artifacts
x-compliance-frameworks: ["sox2", "hipaa", "pci-dss"]
x-compliance-audit-deadline: "2026-06-30T23:59:59Z"
x-compliance-data-classification: "confidential"
x-compliance-requires-pen-test: true

# In HANDOFF artifacts
x-compliance-reviewer: "security-team"
x-compliance-sign-off-date: "2026-03-15T12:00:00Z"
```

### Machine Learning / Data Science

```yaml
# In PLAN artifacts
x-ml-experiment-tracking-url: "https://mlflow.internal/exp/42"
x-ml-dataset-version: "v2.3.1"
x-ml-model-registry: "production/fraud-detection"

# Custom artifact: Experiment Log
artifact_type: "x-experiment-log"
experiments:
  - id: "EXP-001"
    model: "XGBoost v1.7"
    metrics:
      accuracy: 0.943
      f1_score: 0.921
    status: "promoted"
```

### Multi-Team / Enterprise

```yaml
# In STATE artifacts
x-enterprise-slack-channel: "#platform-updates"
x-enterprise-jira-epic: "PLAT-1234"
x-enterprise-okr-alignment: "O3-KR2"
x-enterprise-stakeholder-review-required: true
```

---

## Integration Extensions

RWP provides [integration adapters](../integrations/) for major AI platforms. You can extend these for your specific toolchain.

### Creating a Custom Integration

To add RWP support for a new tool:

1. **Create an adapter directory** under `integrations/your-tool/`
2. **Map RWP concepts** to the tool's native features (commands, settings, templates)
3. **Include a README** explaining setup and usage
4. **Reference PROTOCOL.md** for specification details

**Adapter structure**:

```
integrations/your-tool/
├── README.md              # Setup and usage guide
├── commands/              # Tool-specific commands (if applicable)
│   └── rwp-plan.md        # Plan creation command
├── settings/              # Tool configuration (if applicable)
│   └── rwp-rules.yaml     # Convention enforcement
└── templates/             # Tool-specific template adaptations
```

### Platform-Specific Considerations

| Platform Type | Key Consideration |
|--------------|------------------|
| **CLI tools** | Map RWP commands to tool's command system |
| **IDE plugins** | Surface RWP artifacts in the editor UI |
| **CI/CD** | Validate RWP artifacts in pipeline steps |
| **Web UIs** | Provide copy-paste instructions or API integration |

For existing integrations, see the [integrations directory](../integrations/).

---

## Extension Best Practices

### 1. Start with Custom Fields

Custom fields are the lowest-friction way to extend RWP. Only create custom artifact types when data truly doesn't fit existing artifacts.

### 2. Namespace Consistently

Pick one vendor prefix per organization and use it everywhere:

```yaml
# Consistent: one prefix
x-acme-billing-code: "..."
x-acme-security-level: "..."
x-acme-team-owner: "..."

# Inconsistent: multiple prefixes for same org
x-acme-billing-code: "..."
x-acme-corp-security-level: "..."
x-acmeinc-team-owner: "..."
```

### 3. Document Your Extensions

Maintain a reference document listing all custom fields your organization uses:

```markdown
## Custom Field Reference

### x-acme-billing-* namespace
| Field | Type | Description |
|-------|------|-------------|
| x-acme-billing-code | string | Cost center code |
| x-acme-billing-budget-usd | number | Allocated budget |

### x-acme-security-* namespace
| Field | Type | Description |
|-------|------|-------------|
| x-acme-security-level | string | Classification level |
| x-acme-security-pen-test | boolean | Requires pen test |
```

### 4. Keep Extensions Optional

Extensions should enhance workflows, not gate them. A plan without your custom fields should still be valid RWP:

```yaml
# Valid RWP plan - no custom fields needed
title: "Quick DNS Update"
overview: "Update DNS records"
created_at: "2026-03-04T10:00:00Z"
phases:
  - phase_id: "P-01"
    title: "Update Records"
```

### 5. Version Your Extensions

If your custom field contracts change, track that separately from the RWP protocol version:

```yaml
x-acme-custom-fields-version: "2.1.0"
x-acme-billing-code: "ENG-2026-Q2"
```

### 6. Contribute Back

If your extension solves a common problem, consider contributing it to RWP:
- Open an issue at the [RWP repository](https://rhumbprotocol.dev)
- Describe the use case and proposed fields
- See [CONTRIBUTING.md](../CONTRIBUTING.md) for the contribution process

---

## The extensions/ Directory

The RWP repository includes an `extensions/` directory for community-contributed extension packages:

```
extensions/
├── README.md              # Index of available extensions
├── compliance/            # Compliance field definitions
├── devops/                # DevOps-specific patterns
└── analytics/             # Analytics and tracking fields
```

To contribute an extension package:
1. Create a subdirectory under `extensions/`
2. Include a README.md describing the extension
3. Include a JSON Schema for validation (if applicable)
4. Submit a pull request following the [contribution guidelines](../CONTRIBUTING.md)

---

## Reference Implementation

[YAKKL® Meridian™](https://meridian.yakkl.com?utm_source=RWP_extension_doc) - the RWP™ reference implementation - demonstrates extension patterns including:
- Budget enforcement via custom fields (`x-meridian-budget-*`)
- Agent orchestration metadata
- Session tracking and analytics

Meridian's extensions illustrate how a production tool can layer capabilities on top of the RWP™ foundation while remaining protocol-compliant.

---

## Further Reading

- [PROTOCOL.md](./PROTOCOL.md) - Full RWP specification, including the Extension Mechanism section
- [Custom Fields](../spec/custom-fields.md) - Detailed naming conventions and validation rules
- [Schema Composition](../spec/schema-composition.md) - Patterns for combining and extending schemas
- [Conformance Levels](../spec/conformance-levels.md) - How fields are classified (Required, Recommended, Optional)
- [Getting Started](./GETTING-STARTED.md) - Practical introduction to RWP adoption

---

Rhumb Workflow Protocol™ (RWP™) v0.31.0
https://rhumbprotocol.dev
