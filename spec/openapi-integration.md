# OpenAPI Integration & API Patterns

> Part of the [Rhumb Workflow Protocol (RWP)](../docs/PROTOCOL.md) - see also: [Integration Points](../docs/PROTOCOL.md#integration-points)

**Version**: 0.27.0
**Date**: 2026-03-04
**Classification**: Public

---

## Overview

This document explains how to use RWP artifact schemas in OpenAPI 3.0+ API specifications, enabling:
1. Direct integration of RWP schemas
2. API endpoints for plan/intake/manifest management
3. RESTful patterns aligned with RWP
4. Schema reusability across implementations

---

## OpenAPI + RWP Integration Pattern

### Basic Structure

```yaml
openapi: 3.0.3
info:
  title: RWP Workflow API
  version: 0.27.0
  description: REST API for managing RWP artifacts

servers:
  - url: https://api.example.com/v1
    description: Production

components:
  schemas:
    Plan:
      $ref: './schemas/plan.schema.json'
    Intake:
      $ref: './schemas/intake.schema.json'
    Manifest:
      $ref: './schemas/manifest.schema.json'
    State:
      $ref: './schemas/state.schema.json'
    Handoff:
      $ref: './schemas/handoff.schema.json'

paths:
  /plans:
    post:
      summary: Create a new plan
      requestBody:
        required: true
        content:
          application/json:
            schema:
              $ref: '#/components/schemas/Plan'
      responses:
        '201':
          description: Plan created
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/Plan'
```

---

## Complete Example: RWP Workflow Management API

### OpenAPI 3.0.3 Specification

```yaml
openapi: 3.0.3
info:
  title: Rhumb Workflow Protocol (RWP) API
  version: 0.27.0
  description: >
    REST API for creating, managing, and tracking RWP workflow artifacts.
    Implements the Rhumb Workflow Protocol for structured AI workflow management.
  contact:
    name: RWP Community
    url: https://rhumbprotocol.dev
  license:
    name: Apache 2.0
    url: https://opensource.org/licenses/Apache-2.0

servers:
  - url: https://api.rhumbprotocol.dev/v1
    description: Production server
  - url: https://staging-api.rhumbprotocol.dev/v1
    description: Staging server
  - url: http://localhost:3000/v1
    description: Local development

tags:
  - name: Plans
    description: Plan management endpoints
  - name: Intakes
    description: Intake request endpoints
  - name: Manifests
    description: Manifest management endpoints
  - name: State
    description: Execution state tracking
  - name: Handoffs
    description: Phase handoff documentation

components:
  schemas:
    Plan:
      $ref: './schemas/plan.schema.json'

    Intake:
      $ref: './schemas/intake.schema.json'

    Manifest:
      $ref: './schemas/manifest.schema.json'

    State:
      $ref: './schemas/state.schema.json'

    Handoff:
      $ref: './schemas/handoff.schema.json'

    Error:
      type: object
      required:
        - code
        - message
      properties:
        code:
          type: string
          enum: ['INVALID_SCHEMA', 'NOT_FOUND', 'CONFLICT', 'UNAUTHORIZED']
        message:
          type: string
        details:
          type: object

    PaginatedResponse:
      type: object
      required:
        - data
        - pagination
      properties:
        data:
          type: array
        pagination:
          type: object
          properties:
            page:
              type: integer
            limit:
              type: integer
            total:
              type: integer

  securitySchemes:
    bearerAuth:
      type: http
      scheme: bearer
      bearerFormat: JWT
      description: JWT token for authentication

security:
  - bearerAuth: []

paths:
  /plans:
    get:
      summary: List all plans
      tags:
        - Plans
      parameters:
        - name: limit
          in: query
          schema:
            type: integer
            default: 20
        - name: offset
          in: query
          schema:
            type: integer
            default: 0
        - name: classification
          in: query
          schema:
            type: string
            enum: [public, private, confidential]
        - name: owner
          in: query
          schema:
            type: string
      responses:
        '200':
          description: List of plans
          content:
            application/json:
              schema:
                type: object
                properties:
                  data:
                    type: array
                    items:
                      $ref: '#/components/schemas/Plan'
                  pagination:
                    type: object
                    properties:
                      page:
                        type: integer
                      limit:
                        type: integer
                      total:
                        type: integer

    post:
      summary: Create a new plan
      tags:
        - Plans
      description: >
        Creates a new RWP plan. All REQUIRED fields must be provided.
        RECOMMENDED and OPTIONAL fields are optional.
      requestBody:
        required: true
        content:
          application/json:
            schema:
              $ref: '#/components/schemas/Plan'
            examples:
              minimal:
                summary: Minimal valid plan (REQUIRED fields only)
                value:
                  title: "Quick Infrastructure Task"
                  overview: "Update DNS records"
                  created_at: "2026-03-04T10:00:00Z"
                  phases:
                    - phase_id: "P-01"
                      title: "Update Records"
                      objective: "Add DNS entries"
                      deliverables: ["DNS updated"]
                      tasks: ["Update Route53"]
                      verification: ["nslookup example.com"]

              recommended:
                summary: Plan with RECOMMENDED fields (best practice)
                value:
                  title: "Q1 Infrastructure Upgrade"
                  overview: "Modernize deployment pipeline"
                  created_at: "2026-01-15T09:00:00Z"
                  updated_at: "2026-03-04T15:30:00Z"
                  owner: "platform-team"
                  classification: "confidential"
                  goals_and_success_criteria:
                    goals:
                      - "Reduce deployment time by 50%"
                    success_criteria:
                      - "Deploy < 5 minutes"
                  phases:
                    - phase_id: "P-01"
                      title: "Assess Current State"
                      objective: "Understand existing infrastructure"
                      duration_minutes: 480
                      deliverables:
                        - "Architecture documentation"
                        - "Bottleneck analysis"
                      tasks:
                        - "Map current deployment process"
                        - "Identify critical path items"
                      verification:
                        - "Documentation reviewed by team"
                        - "Presentation delivered"

      responses:
        '201':
          description: Plan created successfully
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/Plan'

        '400':
          description: Invalid request body
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/Error'
              example:
                code: 'INVALID_SCHEMA'
                message: 'Missing required field: title'
                details:
                  field: 'title'
                  required: true

        '401':
          description: Unauthorized

  /plans/{plan_id}:
    get:
      summary: Get a plan by ID
      tags:
        - Plans
      parameters:
        - name: plan_id
          in: path
          required: true
          schema:
            type: string
      responses:
        '200':
          description: Plan found
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/Plan'
        '404':
          description: Plan not found

    put:
      summary: Update a plan
      tags:
        - Plans
      parameters:
        - name: plan_id
          in: path
          required: true
          schema:
            type: string
      requestBody:
        required: true
        content:
          application/json:
            schema:
              $ref: '#/components/schemas/Plan'
      responses:
        '200':
          description: Plan updated
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/Plan'
        '404':
          description: Plan not found

    delete:
      summary: Delete a plan
      tags:
        - Plans
      parameters:
        - name: plan_id
          in: path
          required: true
          schema:
            type: string
      responses:
        '204':
          description: Plan deleted

  /intakes:
    post:
      summary: Create a new intake
      tags:
        - Intakes
      requestBody:
        required: true
        content:
          application/json:
            schema:
              $ref: '#/components/schemas/Intake'
            examples:
              basic:
                summary: Basic intake request
                value:
                  id: "INT-0001"
                  title: "API performance degradation"
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
      responses:
        '201':
          description: Intake created
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/Intake'

  /manifests:
    post:
      summary: Create a new manifest
      tags:
        - Manifests
      requestBody:
        required: true
        content:
          application/json:
            schema:
              $ref: '#/components/schemas/Manifest'
      responses:
        '201':
          description: Manifest created
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/Manifest'

  /plans/{plan_id}/state:
    get:
      summary: Get execution state for a plan
      tags:
        - State
      parameters:
        - name: plan_id
          in: path
          required: true
          schema:
            type: string
      responses:
        '200':
          description: Current execution state
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/State'

    patch:
      summary: Update execution state
      tags:
        - State
      parameters:
        - name: plan_id
          in: path
          required: true
          schema:
            type: string
      requestBody:
        required: true
        content:
          application/json:
            schema:
              type: object
              properties:
                status:
                  type: string
                  enum: [pending, in_progress, completed, failed]
                current_phase:
                  type: string
                  pattern: '^P-[0-9]{2}(-[A-Z])?$'
      responses:
        '200':
          description: State updated
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/State'

  /plans/{plan_id}/handoffs:
    get:
      summary: Get all handoffs for a plan
      tags:
        - Handoffs
      parameters:
        - name: plan_id
          in: path
          required: true
          schema:
            type: string
      responses:
        '200':
          description: List of handoffs
          content:
            application/json:
              schema:
                type: object
                properties:
                  data:
                    type: array
                    items:
                      $ref: '#/components/schemas/Handoff'

    post:
      summary: Create a phase handoff
      tags:
        - Handoffs
      parameters:
        - name: plan_id
          in: path
          required: true
          schema:
            type: string
      requestBody:
        required: true
        content:
          application/json:
            schema:
              $ref: '#/components/schemas/Handoff'
      responses:
        '201':
          description: Handoff created
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/Handoff'
```

---

## Schema References in OpenAPI

### Option 1: Direct File Reference

```yaml
components:
  schemas:
    Plan:
      $ref: './schemas/plan.schema.json'
```

**Pros**: Schemas stay in sync, single source of truth
**Cons**: Requires file access, may not work in all OpenAPI viewers

### Option 2: Embedded Copy

```yaml
components:
  schemas:
    Plan:
      $schema: "http://json-schema.org/draft-07/schema#"
      type: object
      title: "RWP Plan Schema"
      required: ["title", "overview", "phases"]
      properties:
        title:
          type: string
          minLength: 5
        overview:
          type: string
          minLength: 20
        phases:
          type: array
          minItems: 1
          items:
            # ... phase schema
```

**Pros**: Self-contained, works everywhere
**Cons**: Maintenance burden, schema drift risk

### Option 3: Remote URL Reference

```yaml
components:
  schemas:
    Plan:
      $ref: 'https://rhumbprotocol.dev/schemas/plan.schema.json'
```

**Pros**: Always latest version
**Cons**: Network dependency, external API contract

---

## Code Examples

### Create Plan via API (cURL)

```bash
curl -X POST https://api.example.com/v1/plans \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $TOKEN" \
  -d '{
    "title": "Q1 Infrastructure Upgrade",
    "overview": "Modernize deployment pipeline and reduce costs",
    "created_at": "2026-01-15T09:00:00Z",
    "owner": "platform-team",
    "classification": "confidential",
    "phases": [
      {
        "phase_id": "P-01",
        "title": "Assessment Phase",
        "objective": "Understand current state",
        "deliverables": ["Architecture doc"],
        "tasks": ["Map infrastructure"],
        "verification": ["Team review"]
      }
    ]
  }'
```

### TypeScript Client

```typescript
import axios from 'axios';

interface Plan {
  title: string;
  overview: string;
  created_at: string;
  phases: Phase[];
}

async function createPlan(plan: Plan): Promise<Plan> {
  const response = await axios.post<Plan>(
    'https://api.example.com/v1/plans',
    plan,
    {
      headers: {
        'Authorization': `Bearer ${process.env.API_TOKEN}`
      }
    }
  );
  return response.data;
}

// Usage
const newPlan = await createPlan({
  title: "Q1 Upgrade",
  overview: "Modernize infrastructure",
  created_at: new Date().toISOString(),
  phases: [...]
});
```

### Python Client

```python
import requests
import json
from datetime import datetime

def create_plan(plan_data):
    """Create a new RWP plan via API"""
    headers = {
        'Content-Type': 'application/json',
        'Authorization': f'Bearer {os.getenv("API_TOKEN")}'
    }

    response = requests.post(
        'https://api.example.com/v1/plans',
        json=plan_data,
        headers=headers
    )

    if response.status_code == 201:
        return response.json()
    else:
        raise Exception(f"Error: {response.status_code} - {response.text}")

# Usage
plan = {
    "title": "Q1 Infrastructure Upgrade",
    "overview": "Modernize deployment pipeline",
    "created_at": datetime.now().isoformat() + "Z",
    "phases": [...]
}

result = create_plan(plan)
print(f"Plan created: {result['id']}")
```

---

## Best Practices

### 1. Use Semantic Versioning for API

```yaml
info:
  title: RWP Workflow API
  version: 0.27.0  # Major.Minor.Patch
```

When schema version changes:
- **Patch (1.0.1)**: OPTIONAL field added
- **Minor (1.1.0)**: RECOMMENDED field added
- **Major (2.0.0)**: REQUIRED field added/changed

### 2. Document Conformance Levels

```yaml
paths:
  /plans:
    post:
      description: >
        Creates an RWP plan.

        **REQUIRED fields** (must be provided):
        - title
        - overview
        - created_at
        - phases (at least 1)

        **RECOMMENDED fields** (best practice):
        - owner
        - classification

        **OPTIONAL fields** (implementation-specific):
        - goals_and_success_criteria
        - x-* (custom fields)
```

### 3. Provide Multiple Examples

```yaml
requestBody:
  content:
    application/json:
      examples:
        minimal:
          summary: "Minimum viable plan"
          value: { ... }

        comprehensive:
          summary: "Full-featured plan with all fields"
          value: { ... }

        custom_fields:
          summary: "Plan with custom extensions"
          value: { ... }
```

### 4. Handle Validation Errors Clearly

```yaml
responses:
  '400':
    description: Invalid request
    content:
      application/json:
        schema:
          type: object
          properties:
            code:
              type: string
              example: "INVALID_SCHEMA"
            message:
              type: string
              example: "Missing required field: title"
            validation_errors:
              type: array
              items:
                type: object
                properties:
                  field:
                    type: string
                  error:
                    type: string
                  conformance:
                    type: string
                    enum: [required, recommended, optional]
```

---

## Testing OpenAPI Integration

### Validate OpenAPI Spec

```bash
npm install -g swagger-cli

swagger-cli validate openapi.yaml
```

### Test Endpoints

```bash
# Use Swagger UI
docker run -p 8080:8080 \
  -e SWAGGER_JSON=/foo/openapi.yaml \
  -v $(pwd):/foo \
  swaggerapi/swagger-ui

# Use Postman
# Import openapi.yaml → Collections → Run
```

### Generate Client Code

```bash
# Generate TypeScript client
npx openapi-generator-cli generate \
  -i openapi.yaml \
  -g typescript-axios \
  -o ./generated-client

# Generate Python client
openapi-generator-cli generate \
  -i openapi.yaml \
  -g python \
  -o ./generated-client
```

---

## Summary

| Aspect | Pattern | Example |
|---|---|---|
| **Schema integration** | `$ref` external files | `$ref: './schemas/plan.schema.json'` |
| **Base URL** | Server definitions | `https://api.example.com/v1` |
| **Resource paths** | RESTful naming | `/plans`, `/plans/{id}` |
| **Conformance** | Documented in descriptions | "REQUIRED fields: ..." |
| **Examples** | Multiple per endpoint | minimal, recommended, custom_fields |
| **Versioning** | Semantic versioning | `0.27.0` → `1.1.0` → `2.0.0` |

OpenAPI integration enables seamless API-driven workflow management with full RWP schema validation.

---

Produced: 2026-03-04T04:45:00Z
By: YAKKL® Meridian™- https://meridian.yakkl.com
Copyright: Copyright © 2026 YAKKL Inc. All Rights Reserved.
