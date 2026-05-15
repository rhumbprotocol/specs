# Rhumb Workflow Protocol (RWP) v0.27.0

## Executive Summary

The Rhumb Workflow Protocol (RWP) is a formal, open-source protocol specification for structured AI workflow management. Designed to standardize how AI systems and human agents collaborate on complex, multi-phase projects, RWP provides a language-agnostic framework for planning, tracking, and completing work with full transparency and auditability.

RWP is an independent Apache-2.0 licensed protocol, created by YAKKL Inc. and complementary to the YAKKL Meridian reference implementation (much as LSP is complementary to VS Code, or MCP is complementary to Claude). This specification enables third-party implementations in any programming language or AI platform, ensuring interoperability across the AI workflow ecosystem.

---

## Purpose & Scope

### Purpose

The Rhumb Workflow Protocol establishes a vendor-neutral, platform-agnostic standard for:

1. **Workflow Definition** - Specifying multi-phase projects with dependencies and scheduling
2. **Request Capture** - Formally documenting requirements, pain points, and constraints
3. **Execution Tracking** - Recording real-time progress, state transitions, and phase completion
4. **Phase Coordination** - Managing handoffs between phases and coordinating parallel work
5. **Audit & Compliance** - Creating immutable records of what was planned, executed, and delivered

RWP addresses the need for structured collaboration between humans and AI systems on complex projects where multiple phases, dependencies, and decision points require transparent coordination.

### Scope

This specification defines:

- **Artifact Types** - The core documents (PLAN.md, INTAKE.yaml, manifest.yaml, state.yaml, HO-*.yaml) that together describe a workflow
- **Lifecycle State Machine** - Phase and plan states, transitions, error handling, and recovery
- **Data Formats** - YAML, Markdown, JSON schemas for all artifacts
- **Protocol Versioning** - Version numbering, backward compatibility, and deprecation
- **Conformance Levels** - Three levels of protocol compliance for different use cases
- **Extension Mechanisms** - How to extend RWP for domain-specific needs

Out of scope:
- Implementation details of specific tools (e.g., how Meridian implements RWP)
- Business logic or domain-specific workflows
- Performance requirements or benchmarks
- Security or encryption mechanisms (left to implementers)

---

## Terminology & Definitions

### Core Concepts

**Workflow** - A multi-phase project with defined objectives, phases, dependencies, and completion criteria. Workflows are coordinated by RWP artifacts.

**Phase** - A logical unit of work within a workflow. Phases are sequential or parallel, have explicit dependencies, and may have sub-phases for finer granularity.

**Sub-Phase** - A subdivision of a phase using the notation P-XX-A/B/C (where XX is the phase number). Sub-phases allow crash resilience: if a 45-minute sub-phase fails mid-way, only that sub-phase restarts.

**Artifact** - A formal document created as part of workflow execution. RWP defines 5 core artifact types: Plan, Intake, Manifest, State, Handoff.

**Plan** - The master workflow document (PLAN.md) specifying all phases, deliverables, and tasks.

**Intake** - A formal request specification (INTAKE.yaml) capturing pain points, requirements, constraints, and success criteria.

**Manifest** - An artifact registry (manifest.yaml) tracking all files, directories, and deliverables produced by a workflow.

**State** - The execution state document (state.yaml) recording which phases have started/completed, current heartbeat, and error status.

**Handoff** - A phase completion summary (HO-*.yaml) documenting what was accomplished, decisions made, and rolling context for the next phase.

**Lifecycle** - The progression of a phase through states: pending → in_progress → completed (or error → recoverable).

**Dependency** - A requirement that one phase complete before another can start. Dependencies are explicit in the plan and verified before phase execution.

**Conformance** - A tool's level of RWP support, declared as Level 1 (minimal), Level 2 (standard), or Level 3 (advanced).

**Heartbeat** - A periodic timestamp indicating a phase is still executing. Used to detect timeouts and zombie processes.

**Audit** - A formal review of one or more completed phases, verifying deliverables, testing outputs, and approving progress.

---

## Audience

This specification is written for:

1. **Protocol Implementers** - Engineers building RWP support into their tools (IDEs, AI platforms, CI/CD systems)
2. **Workflow Users** - Project managers and developers using RWP-compliant tools to manage work
3. **AI System Designers** - LLM engineers designing agent orchestration systems on top of RWP
4. **Open Source Contributors** - Community members extending RWP with domain-specific templates or integrations

No prior knowledge of the YAKKL Meridian system is required. This specification is self-contained and implementable in isolation.

---

## Artifact Types & Schemas

### Overview

RWP defines 5 core artifact types that together describe a complete workflow. All artifacts are human-readable and version-controllable (suitable for git).

### 1. Plan (PLAN.md)

**Purpose**: Master specification of the entire workflow. Defines phases, deliverables, tasks, verification commands, and temporal ordering. The Plan is the single source of truth for what the workflow accomplishes.

**File Format**: Markdown with optional YAML frontmatter

**Key Responsibilities**:
- Enumerate all phases in execution order
- Specify dependencies and temporal relationships
- List deliverables for each phase
- Define success criteria and verification steps
- Provide context for executors (requirements, constraints, rationale)
- Include estimated duration for planning purposes
- Reference supporting documentation (designs, specs, requirements)

**Example Structure**:
```markdown
# Plan: Build Analytics Platform

## Overview
This plan outlines the 3-phase development of a real-time analytics dashboard
for enterprise customers. The solution integrates with existing PostgreSQL
infrastructure and provides live metrics via WebSocket push.

## Goals & Success Criteria
- Deliver dashboard to 5 pilot customers by 2026-03-31
- Support 1000+ concurrent WebSocket connections
- Query latency <200ms p95
- 95%+ test coverage
- Zero data loss in pipeline

## Phases

### Phase P-01: Data Pipeline Setup (12 hours)
**Objective**: Build the data collection and normalization layer.

**Deliverables**:
- schema.sql with 15 tables for metrics collection
- Python ETL job with 90%+ unit test coverage
- Database initialization script
- Migration strategy documentation

**Tasks**:
1. Design normalized schema for time-series data
2. Implement ETL job for data ingestion
3. Write unit and integration tests (target: 90%+ coverage)
4. Document schema and ETL process
5. Create database initialization/migration scripts

**Verification**:
```bash
# Unit tests pass with 90%+ coverage
pytest tests/ --cov=etl --cov-report=term

# Schema passes linting
sqlfluff lint migrations/001_create_schema.sql

# ETL processes sample data without errors
python etl/ingest.py --test-data samples/metrics-1000.jsonl

# Performance baseline meets requirements
./scripts/benchmark-schema.sh  # Should show <100ms query time
```

**Dependencies**: None (first phase)

**Risks**:
- PostgreSQL connection pooling may not scale to 1000 concurrent connections
  - Mitigation: Test with pgBouncer under load (load test in P-02)

---

### Phase P-02: REST API & WebSocket Gateway (16 hours)
**Objective**: Build the API layer exposing metrics to clients.

**Deliverables**:
- REST API with 12 endpoints
- WebSocket server for real-time push
- OpenAPI schema
- Integration tests with sample client

**Tasks**:
1. Implement REST API endpoints
2. Build WebSocket gateway with authentication
3. Add request validation and error handling
4. Create comprehensive OpenAPI schema
5. Write integration tests
6. Performance-test for 1000+ concurrent connections

**Verification**:
```bash
# API integration tests pass
pytest tests/integration/test_api.py -v

# WebSocket stress test succeeds (1000 concurrent)
./scripts/websocket-load-test.sh --clients=1000 --duration=300

# OpenAPI schema validates
openapi-generator-cli validate -i openapi.yaml

# Performance baseline: <200ms p95 latency
./scripts/benchmark-api.sh --percentile=95
```

**Dependencies**:
- Depends on P-01 (data pipeline must be operational)
- Note: P-02 can start after P-01 handoff is created (parallelizable setup work)

---

### Phase P-03: Dashboard UI & Deployment (20 hours)
**Objective**: Build the web dashboard and deploy to production.

**Deliverables**:
- React SPA dashboard with 5 main views
- Responsive design (mobile, tablet, desktop)
- Deployment automation (Docker, Kubernetes)
- User documentation
- Production runbook

**Tasks**:
1. Build React components and state management
2. Implement real-time data visualization (recharts)
3. Add user authentication and authorization
4. Create responsive design (mobile-first)
5. Build Docker image and Helm charts
6. Deploy to staging, run smoke tests
7. Deploy to production
8. Monitor for first 24 hours

**Verification**:
```bash
# UI component tests pass
npm test -- --coverage --coveragePathIgnorePatterns=/node_modules/

# E2E tests pass in staging environment
npx playwright test --config=e2e/playwright.config.ts

# Production deployment health check
./scripts/healthcheck-production.sh --timeout=300

# Dashboard responds to real-time updates
./scripts/test-realtime-updates.sh --duration=60
```

**Dependencies**:
- Depends on P-02 (API and WebSocket must be ready)

---

## Project Structure

All phase descriptions should follow this pattern:

| Field | Required | Notes |
|-------|----------|-------|
| Phase title & estimated duration | Yes | e.g., "P-01: Data Pipeline (12 hours)" |
| Objective (1-2 sentences) | Yes | What will be accomplished |
| Deliverables (bulleted list) | Yes | Concrete output; measurable items |
| Tasks (numbered list) | Yes | Steps to accomplish deliverables |
| Verification (bash commands) | Yes | How to prove completion |
| Dependencies | Yes | Explicit list of prior phases |
| Risks (optional) | Recommended | What could go wrong; mitigations |
| Time estimates | Recommended | Duration for each task |
| Resources (optional) | Recommended | Team members, tools, infrastructure needed |

```

**Required Fields**:
- `title` - Human-readable plan title
- `phases` - List of all phases in execution order
- For each phase: `title`, `objective`, `deliverables`, `tasks`, `verification`
- Overall `summary` or `overview` explaining the plan's purpose and context

**Optional Fields**:
- `classification` (public/private/confidential)
- `created_at`, `updated_at` (ISO 8601 timestamps)
- `schedule` (estimated duration per phase)
- `owner`, `team` (assigned stakeholders)
- `risks_and_mitigations` (per-phase risk analysis)
- `dependencies` (cross-plan dependencies)

### 2. Intake (INTAKE.yaml)

**Purpose**: Formal capture of the request, requirements, constraints, and success criteria. Serves as the single source of truth for what the workflow should accomplish. Intake documents are frozen once a plan begins execution-changes are tracked as formal amendment requests.

**File Format**: YAML

**Key Responsibilities**:
- Document all stakeholder needs and pain points
- Specify functional and non-functional requirements
- List constraints and dependencies
- Define measurable success criteria
- Capture acceptance conditions for delivery
- Record approval chain (who requested, who approved)

**Example Structure**:
```yaml
# Intake for Analytics Platform Project
---
id: INT-0001
captured: "2026-03-04T09:00:00Z"
title: "Build Real-Time Analytics Platform"
classification: confidential
approved_by: "product-leadership"
approval_date: "2026-03-04T14:00:00Z"

summary: >
  Build a real-time analytics dashboard enabling enterprise customers
  to visualize operational metrics with <5 second latency. System must
  scale to 1000+ concurrent users and integrate with existing PostgreSQL
  data warehouse.

pain_points:
  - id: PP-01
    description: "Current analytics system (Tableau) requires 24-hour batch refresh"
    impact: high
    frequency: daily
    affected_teams: ["Product", "Operations", "Customer Success"]
    business_impact: >
      Customers cannot respond to operational incidents in real-time.
      This has caused 3 major incidents where delayed visibility cost
      $150K in customer support and SLA penalties.
    examples:
      - "Database performance degradation undetected for 2 hours"
      - "Customer traffic spike unnoticed, leading to cascade failures"

  - id: PP-02
    description: "Custom report requests take 2-3 weeks (requires data engineer time)"
    impact: medium
    frequency: weekly
    affected_teams: ["Customer Success", "Sales"]
    business_impact: "Sales delays closed deals; CS unable to provide quick answers"

  - id: PP-03
    description: "No alerting on metric anomalies; manual monitoring only"
    impact: critical
    frequency: constant
    affected_teams: ["Operations", "Engineering"]
    business_impact: "Operational team works nights/weekends to monitor systems manually"

requirements:
  - id: REQ-01
    type: functional
    description: "Real-time dashboard showing live metrics from all systems"
    acceptance: "Metrics update within 5 seconds of event generation"
    priority: P0
    verification: "Integration test: simulate 100 events/second, verify <5s latency"

  - id: REQ-02
    type: functional
    description: "Support 1000+ concurrent WebSocket connections"
    acceptance: "Zero dropped connections under sustained load"
    priority: P0
    verification: "Load test: 1000 concurrent clients, 30 min duration, zero errors"

  - id: REQ-03
    type: functional
    description: "Custom dashboard creation without engineer involvement"
    acceptance: "Support team can create new dashboards in <5 minutes"
    priority: P1
    verification: "Usability test with 3 support team members"

  - id: REQ-04
    type: non-functional
    description: "Query latency <200ms p95"
    acceptance: "95th percentile latency for all queries <200ms"
    priority: P0
    verification: "Production monitoring: APM dashboard shows p95 latency"

  - id: REQ-05
    type: non-functional
    description: "Mobile-responsive design"
    acceptance: "Dashboard usable on iPhone, iPad, desktop"
    priority: P1
    verification: "E2E tests on Chrome mobile, iPad, desktop"

  - id: REQ-06
    type: non-functional
    description: "95%+ unit test coverage"
    acceptance: "Code coverage report shows 95%+ coverage"
    priority: P0
    verification: "`pytest --cov` output shows ≥95% coverage"

constraints:
  - id: CON-01
    type: technical
    description: "Must use PostgreSQL as primary data store"
    rationale: "Existing data warehouse is Postgres; moving would require year-long migration"
    negotiable: false

  - id: CON-02
    type: technical
    description: "Must not require changes to data warehouse schema"
    rationale: "Data warehouse is shared; schema changes affect 50+ downstream systems"
    negotiable: false

  - id: CON-03
    type: technical
    description: "Must integrate with existing authentication (OAuth via OIDC)"
    rationale: "Corporate policy requires centralized auth; no new auth systems allowed"
    negotiable: false

  - id: CON-04
    type: business
    description: "Must be delivered by 2026-03-31"
    rationale: "Customer committed to announce feature at conference; penalty $500K if missed"
    negotiable: false

  - id: CON-05
    type: business
    description: "Budget limit: $75K (engineering time + infrastructure)"
    rationale: "VP approval capped at this level; higher budget requires re-approval"
    negotiable: true (request re-approval)

  - id: CON-06
    type: compliance
    description: "All data must comply with customer SOC2 requirements"
    rationale: "Enterprise customer contract requires Level 2 SOC2 compliance"
    negotiable: false

success_criteria:
  - "✓ All P0 requirements met"
  - "✓ 5 pilot customers onboarded and providing positive feedback"
  - "✓ Performance benchmarks achieved (latency <200ms, 1000 concurrent)"
  - "✓ Test coverage ≥95%"
  - "✓ Production runbook and on-call playbook created"
  - "✓ Team trained on system architecture and incident response"
  - "✓ SLO agreed: 99.5% uptime, <5 second metric latency"

assumptions:
  - "Data warehouse will remain stable during development (no major schema changes)"
  - "OIDC server will be accessible and stable"
  - "5 pilot customers will be available for UAT in final week"
  - "Team has existing expertise in WebSockets and React"

open_questions:
  - "Should alerts trigger for custom metrics? (deferred to P2)"
  - "What is the retention policy for metrics? (30 days? unlimited?)"
  - "Should historical dashboards be available? (e.g., replay metrics from yesterday)"

dependencies:
  - "Requires data warehouse access (already granted)"
  - "Requires staging environment (available now)"
  - "Requires 1 DevOps engineer from infrastructure team (TBD availability)"

stakeholders:
  - role: "Product Owner"
    name: "Alice Chen"
    responsibilities: ["Requirement verification", "Pilot customer coordination"]
  - role: "Engineering Lead"
    name: "Bob Martinez"
    responsibilities: ["Technical planning", "Architecture decisions"]
  - role: "QA Lead"
    name: "Carol Williams"
    responsibilities: ["Test strategy", "UAT coordination"]
```

**Required Fields**:
- `id` - Unique intake identifier (e.g., INT-0001)
- `title` - Request title
- `captured`, `approval_date` - Timestamps with approvals
- `pain_points` - List of problems being solved (with impact/frequency)
- `requirements` - All functional and non-functional requirements with acceptance criteria
- `constraints` - Technical, legal, business constraints (negotiable vs. non-negotiable)
- `success_criteria` - Measurable definition of completion

**Optional Fields**:
- `created`, `updated` (ISO 8601 timestamps)
- `classification` (public/private/confidential)
- `affected_teams`, `priority`, `assigned_to`, `stakeholders`
- `assumptions`, `open_questions`, `dependencies`
- `business_impact`, `financial_impact` (quantified where possible)

### 3. Manifest (manifest.yaml)

**Purpose**: Registry of all files, directories, and artifacts produced by the workflow. Enables validation that promised deliverables actually exist and tracking of work output across phases. Used to generate completion reports and verify handoff accuracy.

**File Format**: YAML

**Key Responsibilities**:
- Track all files created/modified per phase
- Organize artifacts by type (source, test, schema, documentation, etc.)
- Record metadata for each artifact (size, checksum, modification time)
- Cross-reference with Plan deliverables
- Support auditing and verification

**Example Structure**:
```yaml
plan_id: MP-0001-analytics-platform
manifest_version: "1.0"
created: "2026-03-04T08:00:00Z"
updated: "2026-03-04T12:00:00Z"

summary:
  total_files: 47
  total_directories: 8
  total_size_bytes: 2850432
  completed_phases: 2
  in_progress_phases: 1

files:
  # Phase P-01 deliverables
  - id: FILE-001
    path: "src/schema.sql"
    type: schema
    phase: P-01
    created: "2026-03-04T08:15:00Z"
    modified: "2026-03-04T09:45:00Z"
    size_bytes: 2048
    checksum_sha256: "abc123def456..."
    verified: true
    related_deliverable: "schema.sql with 15 tables"

  - id: FILE-002
    path: "etl/ingest.py"
    type: source
    phase: P-01
    language: python
    lines_of_code: 342
    created: "2026-03-04T08:30:00Z"
    modified: "2026-03-04T10:15:00Z"
    size_bytes: 4096
    checksum_sha256: "def456ghi789..."
    verified: true
    test_coverage: 92
    related_deliverable: "Python ETL job"

  - id: FILE-003
    path: "tests/test_ingest.py"
    type: test
    phase: P-01
    language: python
    test_count: 24
    created: "2026-03-04T09:00:00Z"
    modified: "2026-03-04T11:00:00Z"
    size_bytes: 3072
    checksum_sha256: "ghi789jkl012..."
    verified: true
    test_result: "24 passed, 0 failed"
    coverage_contribution: 92
    related_deliverable: "Unit tests (90%+ coverage)"

  - id: FILE-004
    path: "migrations/001_create_schema.sql"
    type: schema
    phase: P-01
    created: "2026-03-04T10:30:00Z"
    modified: "2026-03-04T10:45:00Z"
    size_bytes: 1024
    verified: true
    related_deliverable: "Database initialization script"

  # Phase P-02 deliverables
  - id: FILE-005
    path: "api/rest_server.py"
    type: source
    phase: P-02
    language: python
    lines_of_code: 1240
    created: "2026-03-04T10:00:00Z"
    modified: "2026-03-04T12:30:00Z"
    size_bytes: 12288
    endpoints: 12
    verified: false
    related_deliverable: "REST API with 12 endpoints"

  - id: FILE-006
    path: "api/openapi.yaml"
    type: schema
    phase: P-02
    created: "2026-03-04T11:45:00Z"
    modified: "2026-03-04T12:30:00Z"
    size_bytes: 5120
    verified: true
    endpoints_documented: 12
    related_deliverable: "OpenAPI schema"

  - id: FILE-007
    path: "tests/integration_api.py"
    type: test
    phase: P-02
    language: python
    test_count: 18
    created: "2026-03-04T11:00:00Z"
    modified: "2026-03-04T12:15:00Z"
    size_bytes: 6144
    verified: false
    test_result: "18 passed, 2 failed (expected: WebSocket timeout)"
    related_deliverable: "Integration tests"

  # Phase P-03 deliverables (in progress)
  - id: FILE-008
    path: "ui/src/Dashboard.tsx"
    type: source
    phase: P-03
    language: typescript
    lines_of_code: 680
    created: "2026-03-04T11:00:00Z"
    modified: "2026-03-04T12:45:00Z"
    size_bytes: 8192
    verified: false
    components: 8
    related_deliverable: "React SPA dashboard"

directories:
  - id: DIR-001
    path: "src/"
    type: source
    phase: P-01
    file_count: 5
    lines_of_code: 1240
    size_bytes: 15360
    verified: true

  - id: DIR-002
    path: "tests/"
    type: test
    phase: P-01
    file_count: 8
    test_cases: 42
    size_bytes: 24576
    verified: true

  - id: DIR-003
    path: "migrations/"
    type: schema
    phase: P-01
    file_count: 1
    size_bytes: 1024
    verified: true

  - id: DIR-004
    path: "api/"
    type: source
    phase: P-02
    file_count: 4
    lines_of_code: 1240
    size_bytes: 16384
    verified: false

  - id: DIR-005
    path: "ui/"
    type: source
    phase: P-03
    file_count: 12
    lines_of_code: 1850
    size_bytes: 20480
    verified: false

expected_outputs:
  P-01:
    deliverables:
      - "schema.sql with 15 tables"
      - "Python ETL job with 90%+ test coverage"
      - "Database initialization script"
      - "Schema documentation"
    delivered: true
    verified: true
    verification_date: "2026-03-04T09:30:00Z"
    verification_notes: "All deliverables present and tested"

  P-02:
    deliverables:
      - "REST API with 12 endpoints"
      - "WebSocket server for real-time push"
      - "OpenAPI schema"
      - "Integration tests"
    delivered: false
    status: "In progress"
    completion_estimate: "2026-03-04T14:00:00Z"

  P-03:
    deliverables:
      - "React SPA dashboard"
      - "Responsive design (mobile, tablet, desktop)"
      - "Docker image and Helm charts"
      - "User documentation"
    delivered: false
    status: "Not started"
    completion_estimate: "2026-03-05T08:00:00Z"

statistics:
  total_lines_of_code: 1240
  completed_lines_of_code: 1240
  in_progress_lines_of_code: 3070
  projected_lines_of_code: 5150
  phases_completed: 1
  phases_in_progress: 2
  phases_pending: 1
  average_files_per_phase: 8
  average_test_coverage_completed: 92
```

**Required Fields**:
- `plan_id` - Reference to parent plan
- `manifest_version` - Schema version
- `files` - List of artifacts created (with path, type, phase, metadata)
- `expected_outputs` - Per-phase deliverable summary

**Optional Fields**:
- `created`, `updated` (ISO 8601 timestamps)
- `summary` (totals: file count, size, etc.)
- `statistics` (lines of code, coverage, etc.)
- `directories` (directory-level tracking)
- `verification_notes`, `verification_by` (audit trail)

### 4. State (state.yaml)

**Purpose**: Runtime execution record. Tracks which phases have completed, current state, heartbeat, and any errors. Updated continuously during execution. State is a growing log of execution events-never rewound, only appended to ensure immutable audit trail.

**File Format**: YAML

**Key Responsibilities**:
- Record real-time execution status
- Maintain heartbeat for detecting timeouts
- Track phase entry/exit timestamps
- Log all error events with recovery strategy
- Store execution history for auditing
- Provide single source of truth for current workflow state

**Example Structure**:
```yaml
# State for Analytics Platform Project
---
plan_id: P-0001
state_version: "1.0"
created: "2026-03-04T08:00:00Z"

execution:
  status: in_progress                   # planning | in_progress | paused | completed | failed
  current_phase: P-02-A
  started_at: "2026-03-04T08:00:00Z"
  completed_at: null
  last_heartbeat: "2026-03-04T12:30:00Z"
  heartbeat_timeout_minutes: 30
  executor: "meridian-agent-v1.2.0"
  executor_host: "executor-prod-01"
  executor_pid: 42157

phases:
  P-01:
    title: "Data Pipeline Setup"
    status: completed
    started_at: "2026-03-04T08:00:00Z"
    completed_at: "2026-03-04T09:00:00Z"
    duration_minutes: 60
    tasks:
      - "Design schema"
      - "Initialize database"
      - "Write unit tests"
    handoff_created: true
    handoff_path: "handoffs/HO-P-0001-P-01-2026-03-04.md"
    handoff_validated: true
    handoff_score: 98
    handoff_validator: "automated-validation-v1"
    deliverables_verified: 4
    tests_passed: 24
    tests_failed: 0
    notes: "Schema optimization added 15 minutes; still within timeline"

  P-02-A:
    title: "REST API Implementation (sub-phase A)"
    status: in_progress
    started_at: "2026-03-04T09:15:00Z"
    completed_at: null
    duration_minutes: 75
    estimated_completion: "2026-03-04T10:30:00Z"
    progress_percent: 45
    tasks:
      - "[✓] Set up FastAPI scaffold"
      - "[✓] Implement authentication middleware"
      - "[ ] Implement 12 endpoints (7/12 complete)"
      - "[ ] Write endpoint tests"
      - "[ ] Performance profiling"
    executor_notes: "Rate limiting endpoint is taking longer than estimated; added 10 min buffer"

  P-02-B:
    title: "WebSocket Gateway (sub-phase B)"
    status: pending
    started_at: null
    completed_at: null
    dependencies: [P-02-A]
    estimated_start: "2026-03-04T10:30:00Z"
    estimated_duration_minutes: 90

  P-02-C:
    title: "Integration Tests (sub-phase C)"
    status: pending
    started_at: null
    completed_at: null
    dependencies: [P-02-A, P-02-B]
    estimated_start: "2026-03-04T12:00:00Z"
    estimated_duration_minutes: 60

  P-03:
    title: "Dashboard UI & Deployment"
    status: pending
    started_at: null
    completed_at: null
    dependencies: [P-02]
    estimated_start: "2026-03-05T08:00:00Z"

error:
  occurred: false
  phase: null
  message: null
  timestamp: null
  recoverable: null
  resolution_attempted: null

audit_trail:
  last_updated: "2026-03-04T12:30:45Z"
  last_updater: "meridian-agent-v1.2.0"
  update_count: 47
  events_logged: 89

history:
  - timestamp: "2026-03-04T08:00:00Z"
    event: "execution_started"
    phase: null
    executor: "meridian-agent-v1.2.0"
    details: "Workflow execution began"

  - timestamp: "2026-03-04T08:05:00Z"
    event: "phase_started"
    phase: P-01
    details: "Phase P-01 (Data Pipeline Setup) initiated"
    executor: "meridian-agent-v1.2.0"

  - timestamp: "2026-03-04T08:15:00Z"
    event: "heartbeat"
    phase: P-01
    details: "Heartbeat recorded; phase in progress"
    executor: "meridian-agent-v1.2.0"

  - timestamp: "2026-03-04T08:30:00Z"
    event: "heartbeat"
    phase: P-01
    details: "Heartbeat recorded; 3/5 tasks complete"

  - timestamp: "2026-03-04T09:00:00Z"
    event: "phase_completed"
    phase: P-01
    details: "All P-01 tasks completed; handoff created"
    executor: "meridian-agent-v1.2.0"
    verification:
      - "4 deliverables verified"
      - "24 unit tests passed"
      - "0 tests failed"
      - "92% code coverage"

  - timestamp: "2026-03-04T09:15:00Z"
    event: "phase_started"
    phase: P-02-A
    details: "Phase P-02-A (REST API Implementation) initiated"
    dependencies_verified: true
    executor: "meridian-agent-v1.2.0"

  - timestamp: "2026-03-04T09:30:00Z"
    event: "heartbeat"
    phase: P-02-A
    details: "Heartbeat recorded; 2/5 tasks complete"

  - timestamp: "2026-03-04T10:00:00Z"
    event: "heartbeat"
    phase: P-02-A
    details: "Heartbeat recorded; 4/12 endpoints implemented"

  - timestamp: "2026-03-04T10:30:00Z"
    event: "heartbeat"
    phase: P-02-A
    details: "Heartbeat recorded; 7/12 endpoints implemented"
    note: "Rate limiting endpoint took longer; adjusting timeline"

  - timestamp: "2026-03-04T12:00:00Z"
    event: "error_observed"
    phase: P-02-A
    severity: "warning"
    message: "Task 'Performance profiling' not started yet; phase 75% complete"
    recovery: "Adding 15 minutes to buffer; performance profiling deferred to optional phase"
    executor: "meridian-agent-v1.2.0"

  - timestamp: "2026-03-04T12:30:00Z"
    event: "heartbeat"
    phase: P-02-A
    details: "Heartbeat recorded; 10/12 endpoints complete; est. 1 hour to completion"

metrics:
  total_phases: 3
  completed_phases: 1
  in_progress_phases: 1
  pending_phases: 1
  phases_with_errors: 0
  average_phase_duration_minutes: 60
  total_execution_time_minutes: 150
  planned_total_time_minutes: 180
  variance_percent: -17
  error_rate: 0
  recovery_attempts: 0
```

**Required Fields**:
- `plan_id` - Reference to parent plan
- `execution.status` - Current workflow state (planning | in_progress | paused | completed | failed)
- `execution.current_phase` - Active phase
- `execution.started_at`, `last_heartbeat` - Timestamps (ISO 8601)
- `phases` - Map with status for all phases (pending, in_progress, completed, failed, skipped)
- `error` - Error tracking (occurred: bool, phase, message, recoverable, resolution)

**Optional Fields**:
- `execution.completed_at`, `executor`, `executor_host`, `executor_pid`
- `execution.heartbeat_timeout_minutes`
- `history` - Immutable append-only event log
- `audit_trail` - Summary stats (update count, last updater)
- `metrics` - Execution statistics (durations, variance, error rates)

### 5. Handoff (HO-*.yaml)

**Purpose**: Phase completion documentation. Created when a phase finishes; summarizes what was accomplished, decisions made, metrics, and rolling context for the next phase.

**File Format**: Markdown

**Example Structure**:
```markdown
# Handoff: P-01 → P-02

## Summary
- Phase: P-01 (Data Pipeline Setup)
- Status: COMPLETED ✓
- Duration: 45 minutes
- Date: 2026-03-04T08:00:00Z to 2026-03-04T08:45:00Z

## Deliverables
- [x] schema.sql (2048 bytes)
- [x] Unit tests (96% coverage)
- [x] Documentation

## Key Decisions
1. Used PostgreSQL 14 (vs. 15) for broader compatibility
2. Implemented schema versioning for future migrations

## Session Metrics
- Turns: 12
- Tool calls: 8 (4 reads, 3 writes, 1 test run)
- Lines written: 342
- Tests: 18 passing, 0 failing
- Build attempts: 2, both successful

## Rolling Context for P-02
- Database is ready; API clients should connect via `localhost:5432`
- Schema includes comments explaining each table's purpose
- Version control history at: `git log --oneline -- src/schema.sql`

## Risks & Mitigations
- Risk: Next phase must account for schema fragility during development
  - Mitigation: Migrations are tested and documented
- Risk: Team unfamiliar with new schema layout
  - Mitigation: Added README with entity diagram

## Next Phase (P-02) Blockers
None identified. Ready to proceed.

---

*Handoff created: 2026-03-04T08:45:00Z by YAKKL Meridian*
```

**Required Sections**:
- Summary (phase, status, duration, date)
- Deliverables (with completion checkmarks)
- Key Decisions (why certain choices were made)
- Session Metrics (turns, tools, output)
- Rolling Context (what the next phase needs to know)
- Risks & Mitigations (what could go wrong)
- Next Phase Blockers (explicit "None" or list of blockers)

**Optional Sections**:
- Design decisions with rationale
- Verification test results
- References to external documentation
- Post-mortem analysis (if phase had errors)

---

## Lifecycle & State Machine

### Phase Lifecycle

Each phase progresses through a defined set of states:

```
┌──────────────────────────────────────────────────────────────┐
│                    RWP Phase Lifecycle                       │
└──────────────────────────────────────────────────────────────┘

              ┌────────────┐
              │  PENDING   │  Initial state; awaits start signal
              └──────┬─────┘
                     │ start()
                     ▼
         ┌───────────────────────────┐
         │    IN_PROGRESS            │  Active execution
         │ (with heartbeat tracking) │
         └───────┬─────────┬─────────┘
                 │         │
          success│         │error
                 │         │
                 ▼         ▼
         ┌──────────────────────────────┐
         │    COMPLETED                 │ ERROR
         │ (requires handoff)           │ (recoverable/fatal)
         └──────────────────────────────┘
                 │                   │
                 │                   └──► RECOVERABLE
                 │                        (retry sub-phase)
                 │
                 └──► Create Handoff Document
                      Update State File
                      Proceed to Next Phase

Timeout (no heartbeat for N minutes):
    IN_PROGRESS ──► ERROR (recoverable)
```

### Phase States

| State | Meaning | Transitions | Notes |
|-------|---------|-----------|-------|
| `pending` | Waiting to start | → in_progress (when dependencies met) | Explicit start signal required |
| `in_progress` | Currently executing | → completed, error | Must have heartbeat |
| `completed` | All tasks done, handoff created | → (final state) | Enables dependent phases |
| `error` | Execution failed | → in_progress (recoverable) or terminal | Blocks dependent phases |

### Plan-Level Lifecycle

Plans follow a similar pattern but at a coarser granularity:

```
┌─────────────────────────────────┐
│   Plan States                   │
└─────────────────────────────────┘

  planning     ──►  in_progress  ──►  completed
                         │
                         └────►  error
```

### Heartbeat Mechanism

During `in_progress` state, the executor must update `execution.last_heartbeat` at least every N minutes (default: 30). If heartbeat is missing for > N minutes:
- Phase transitions to `error` state
- Error state is marked `recoverable: true`
- Next execution can restart from the same phase

**Heartbeat Update**:
```yaml
execution:
  last_heartbeat: "2026-03-04T08:45:00Z"  # Update frequently
  heartbeat_timeout_minutes: 30            # Grace period
```

### Error Handling & Recovery

```
┌────────────────────────────────────────────┐
│   Error State & Recovery                   │
└────────────────────────────────────────────┘

Error occurs during phase execution:
   │
   ├─► Phase transitions to ERROR state
   ├─► error.occurred = true
   ├─► error.message = <exception details>
   ├─► error.recoverable = true|false
   └─► error.timestamp = <when error occurred>

Recovery decision:
   │
   ├─► If recoverable: Restart phase from beginning
   │   (sub-phases enable finer recovery: restart just P-02-B)
   │
   └─► If fatal: Escalate to human review
       (continue only with explicit override)
```

### Sub-Phases (P-XX-A/B/C)

For resilience, phases may be subdivided into sub-phases:

```
Phase P-02 (estimated 1.5 hours):
   ├── P-02-A (30 min)  ──► creates HO-P-02-A-*.md handoff
   ├── P-02-B (30 min)  ──► creates HO-P-02-B-*.md handoff
   └── P-02-C (30 min)  ──► creates HO-P-02-C-*.md handoff

Final: Create master HO-P-02-*.md summarizing all three.
```

If P-02-B fails mid-execution, only P-02-B restarts-not P-02-A or P-02-C.

---

## Protocol Versioning

### Version Scheme

RWP uses semantic versioning: **RWP-{MAJOR}.{MINOR}.{PATCH}**

- **MAJOR**: Breaking changes to artifact formats or state machine (e.g., renaming required fields)
- **MINOR**: Backward-compatible additions (e.g., new optional fields)
- **PATCH**: Bug fixes with no schema changes

**Examples**:
- RWP-0.27.0 - Initial release
- RWP-1.1.0 - Added `heartbeat_timeout_minutes` field (backward-compatible)
- RWP-1.1.1 - Fixed lifecycle diagram (documentation only)
- RWP-2.0.0 - Renamed `status` to `phase_status` (breaking change)

### Version Embedding

Each artifact includes the RWP version it conforms to:

```yaml
# In PLAN.md frontmatter or as a comment:
---
rwp_version: "0.27.0"
---

# In INTAKE.yaml:
rwp_version: "0.27.0"

# In state.yaml:
rwp_version: "0.27.0"
```

### Backward Compatibility Policy

- **Within MAJOR version**: All implementations must support previous MINOR/PATCH versions
- **Across MAJOR versions**: Implementations may require migration tools (e.g., migrate from RWP-1.x to RWP-2.x)
- **Deprecated fields**: Marked as deprecated for 2 full MAJOR versions before removal

### Version Declaration

Tools declare which RWP versions they support:

```yaml
# tool-config.yaml
rwp_conformance:
  level: 2
  supported_versions: ["0.27.0", "1.1.0"]
  auto_migrate: true  # Automatically upgrade artifacts to latest version
```

---

## Conformance Levels

RWP defines three conformance levels. Tools declare their level, enabling users to understand what features are supported and make informed decisions about tool selection. Each level builds on the previous, ensuring that workflows are portable: a Level 2 workflow can run on Level 3 tools, though it won't use Level 3 features.

### Level 1: Minimal

**Supported Artifacts**: Plan, State

**Capabilities**:
- Create and track basic workflows
- Record phase start/completion
- Support heartbeat mechanism
- Transition between phase states
- Basic error tracking
- Execute phases sequentially with explicit start signals
- Log execution history to state file

**What Level 1 Does NOT Support**:
- Formal requirements capture (no Intake)
- Deliverable tracking (no Manifest)
- Handoff documentation (no Handoff artifacts)
- Sub-phases (P-XX-A/B/C only with manual splitting)
- Schema validation
- Workflow import/export
- External system integrations

**Tools**: Basic workflow trackers, simple phase management, spreadsheet-based tracking

**Example Use Case**: "Track which phases have started/completed in a spreadsheet or simple database. A human manually manages phase transitions, and state is updated by editing YAML files."

**User Base**: Internal teams managing simple workflows; educational/learning contexts

**Typical Workflow Size**: < 5 phases, single person or small team

### Level 2: Standard

**Supported Artifacts**: Plan, Intake, Manifest, State, Handoff

**Capabilities**:
- Everything in Level 1, plus:
- Capture formal requirements (Intake)
- Track deliverables and verify completion (Manifest)
- Generate handoff documentation between phases
- Verify handoff accuracy against Manifest
- Support sub-phases (P-XX-A/B/C) for crash resilience
- Automatic phase dependency verification
- Structured error handling with recovery suggestions
- Handoff validation (automated checks)
- Basic audit trail
- Text-based workflow visualization
- Version control integration (git-friendly formats)

**What Level 2 Does NOT Support**:
- JSON Schema validation against custom schemas
- External system integrations (Jira, Slack, GitHub, etc.)
- Parallel phase execution
- Advanced error recovery and retry logic
- Cryptographic audit verification
- Real-time dashboards
- Custom field validation
- Multi-workspace isolation

**Tools**: Most AI workflow systems, project management tools, CI/CD integration, CLI-based tools

**Example Use Case**: "Full workflow with requirements, execution tracking, and handoffs between phases. Suitable for professional teams managing complex projects where requirements must be documented and deliverables verified before proceeding."

**User Base**: Product teams, engineering teams, AI systems, independent developers

**Typical Workflow Size**: 5-15 phases, 2-10 participants

### Level 3: Advanced

**Supported Artifacts**: All Level 2 + extended schemas, integrations, domain-specific extensions, custom artifacts

**Capabilities**:
- Everything in Level 2, plus:
- JSON/YAML schema validation with custom and extended schemas
- Integration adapters for external systems:
  - Jira (sync issues, create tickets)
  - GitHub (sync PRs, commits, deployment status)
  - Slack (notifications, approvals)
  - Email (escalations, reports)
  - Webhooks (custom integrations)
  - Custom API plugins
- Parallel phase execution with automatic dependency resolution
- Advanced error recovery:
  - Automatic retry with exponential backoff
  - Human-in-the-loop approval for recovery
  - Rollback to previous phase state
  - Dead letter queues for failed phases
- Cryptographic audit trail with signatures
- Custom field extensions with validation rules
- Multi-team coordination with role-based access control
- Real-time dashboard and visualization
- Workflow templates and reusable components
- Cost and resource tracking
- Advanced reporting and analytics
- Workflow marketplace (share/import community workflows)

**What Level 3 Requires**:
- Sophisticated tooling (Web UI, APIs, databases)
- Administration and configuration management
- User authentication and authorization
- Audit logging infrastructure
- Integration with enterprise systems

**Tools**: Enterprise systems, large-scale AI platforms, regulated environments, premium SaaS tools

**Example Use Case**: "Enterprise deployment with compliance audits, multi-team coordination, and external system integrations. Used by organizations with 50+ concurrent workflows, regulatory requirements, and complex inter-team dependencies."

**User Base**: Large enterprises, regulated industries (healthcare, finance), AI research organizations

**Typical Workflow Size**: 15-100+ phases, 10-1000+ participants, mission-critical workflows

### Conformance Comparison Table

| Feature | Level 1 | Level 2 | Level 3 |
|---------|---------|---------|---------|
| **Artifacts Supported** | Plan, State | All 5 core | + custom artifacts |
| **Heartbeat Mechanism** | ✓ | ✓ | ✓ |
| **Formal Requirements** | ✗ | ✓ | ✓ |
| **Deliverable Tracking** | ✗ | ✓ | ✓ |
| **Handoff Documentation** | ✗ | ✓ | ✓ |
| **Sub-Phases (P-XX-A/B/C)** | Manual | ✓ | ✓ |
| **Dependency Verification** | Manual | Automatic | Automatic + parallel |
| **Schema Validation** | ✗ | Basic | Full + custom |
| **Error Recovery** | Manual | Suggested | Automated + human-in-loop |
| **External Integrations** | ✗ | ✗ | ✓ |
| **Real-time Dashboard** | ✗ | ✗ | ✓ |
| **Audit Trail** | Basic | Structured | Cryptographic |
| **Multi-team Support** | ✗ | ✗ | ✓ |
| **API/SDK** | Optional | Recommended | Required |
| **Typical Users** | Solo/learning | Teams | Enterprises |

### Declaring Conformance

Tools declare their conformance level in documentation or configuration:

```yaml
# In tool documentation (README, docs)
rwp_conformance:
  level: 2
  description: >
    Standard RWP Level 2 implementation. Supports all 5 core artifact types,
    automatic dependency verification, handoff validation, and sub-phases.
  supported_versions: ["0.27.0", "1.1.0"]
  features:
    schema_validation: true
    sub_phases: true
    handoff_generation: true
    error_recovery: basic
    integrations: []

# In tool's API response (if tool has API)
{
  "rwp_conformance": {
    "level": 2,
    "version": "0.27.0",
    "supported_artifacts": ["PLAN", "INTAKE", "MANIFEST", "STATE", "HANDOFF"],
    "features": {
      "automatic_dependency_verification": true,
      "handoff_validation": true,
      "sub_phase_support": true,
      "parallel_execution": false,
      "custom_integrations": false
    }
  }
}
```

### Upgrading Conformance

Tools may upgrade to higher conformance levels over time. When upgrading:

1. **Backward Compatibility**: Higher levels must support all lower-level workflows
2. **Graceful Degradation**: If a Level 3 feature is not available, tool should indicate this in error messages rather than silently ignoring requests
3. **Migration Path**: Provide tools to migrate workflows from lower to higher conformance (e.g., auto-fill Manifest fields)
4. **Documentation**: Clearly document new features and how they differ from lower levels

### When to Choose Each Level

**Choose Level 1** if:
- You're learning RWP
- Simple workflows with few phases (<5)
- Single person or tightly coupled team
- No formal requirement for traceability

**Choose Level 2** if:
- Professional team managing workflows
- Need formal requirements and deliverable tracking
- Multi-person teams with dependencies
- Audit/compliance requirements
- Want crash resilience (sub-phases)

**Choose Level 3** if:
- Enterprise environment with 50+ concurrent workflows
- Regulatory/compliance requirements (SOC2, HIPAA, etc.)
- Need integration with existing tools (Jira, GitHub, Slack)
- Multi-team coordination
- Mission-critical workflows requiring real-time visibility

---

## Foundation Templates

RWP provides 17 foundation templates organized into four categories. All templates use advisory language ("consider", "we recommend") and contain no enforcement directives. Templates are located in the `templates/` directory.

### Core Plan Templates (7)

| Template | Purpose | Format |
|----------|---------|--------|
| `PLAN.md.template` | Master plan document - phases, deliverables, verification | Markdown |
| `INTAKE.yaml.template` | Requirements capture - pain points, constraints, success criteria | YAML |
| `MASTERPLAN.yaml.template` | Execution masterplan - micro-tasks, scheduling, resource allocation | YAML |
| `PLAN-STATE.yaml.template` | Runtime execution state - phase progress, heartbeat, errors | YAML |
| `DEPENDENCIES.yaml.template` | Dependency tracking - inter-phase and external dependencies | YAML |
| `MANIFEST-PLAN.yaml.template` | File/deliverable registry - tracks all artifacts produced | YAML |
| `sequences.yaml.template` | Sequence configuration - 13 sequence types for ID generation | YAML |

### Architecture Templates (4)

| Template | Purpose | Location |
|----------|---------|----------|
| `AVD.md.template` | Architecture Vision Document - goals, topology, cost analysis | `templates/` |
| `AVD-PROMPT.md.template` | Prompt for generating or revising AVD artifacts | `templates/` |
| `ACS.md.template` | Architecture Component Spec - subsystem specification | `templates/` |
| `ACS-PROMPT.md.template` | Prompt for generating or revising ACS artifacts | `templates/` |

### Display & Prompt Templates (5)

| Template | Purpose | Location |
|----------|---------|----------|
| `PLAN-DRAFT-DISPLAY.md.template` | Conversational output for plan drafts | `templates/display/` |
| `PLAN-COMMIT-DISPLAY.md.template` | Confirmatory output after plan commit | `templates/display/` |
| `PHASE-COMPLETE-DISPLAY.md.template` | Phase completion summary with metrics | `templates/display/` |
| `SUBPHASE-COMPLETE-DISPLAY.md.template` | Sub-phase completion summary | `templates/` |
| `START-PROMPT.md.template` | Session initialization flow | `templates/` |
| `PROMPT.md.template` | Phase continuation flow | `templates/` |

### Closure Templates (5)

| Template | Purpose | Location |
|----------|---------|----------|
| `HANDOFF.yaml.template` | Structured handoff document | `templates/` |
| `AUDIT.md.template` | Audit checkpoint report | `templates/` |
| `AUDIT-PROMPT.md.template` | Prompt for audit execution | `templates/` |
| `FINAL.md.template` | Final report for a completed plan | `templates/` |
| `FINAL-PROMPT.md.template` | Prompt for plan finalization | `templates/` |

All templates include `{{PLACEHOLDER}}` substitution markers for dynamic content (plan ID, timestamps, phase numbers, RWP version). See the [Extension Mechanism](#extension-mechanism) section for creating custom templates.

---

## Extension Mechanism

RWP is designed to be extended without breaking backward compatibility. This section describes how tools can add functionality while remaining conformant to the core specification.

### Custom Fields

Artifacts may include custom fields outside the core spec. RWP requires that tools gracefully ignore unknown fields, enabling safe extension:

```yaml
# state.yaml with custom fields:
plan_id: P-0001

# Standard fields:
execution:
  status: in_progress
  current_phase: P-02
  started_at: "2026-03-04T08:00:00Z"

# Custom extension fields (ignored by non-supporting tools):
custom_analytics:
  phase_velocity_minutes: [45, 35, 40]
  average_velocity: 40
  estimated_remaining: 120
  confidence: 0.92
  trend: improving

custom_integrations:
  jira_project: "PROJ-123"
  slack_channel: "#workflows"
  github_repo: "myorg/myrepo"

environment:
  deployment_target: production
  database_version: "14.2"
  api_version: "v2"
  feature_flags:
    - "enable_parallel_phases"
    - "enable_ai_suggestions"
```

**Rules for Custom Fields**:
1. All custom fields must be optional (tools without extension support can still read the artifact)
2. Custom field names should follow kebab-case (e.g., `custom_analytics`, not `customAnalytics`)
3. Do not override or shadow core RWP fields
4. Document custom extensions in your tool's configuration or README
5. Consider contributing domain-specific extensions back to the community

### Custom Artifact Types

Organizations may define domain-specific artifact types extending RWP. These should follow the same structure as core artifacts:

```yaml
# Example: Risk Register (custom artifact)
---
artifact_type: risk_register
parent_plan: P-0001
created: "2026-03-04T08:00:00Z"

risks:
  - id: RISK-001
    title: "Database performance degrades under load"
    likelihood: medium
    impact: high
    phase: P-03
    owner: "database-team"
    mitigation: "Implement connection pooling"
    status: open

  - id: RISK-002
    title: "API rate limiting causes test failures"
    likelihood: low
    impact: medium
    phase: P-04
    owner: "api-team"
    mitigation: "Use test rate limit bypass credentials"
    status: resolved
```

### Domain-Specific Extensions

Organizations may create template libraries and extension packages extending RWP. The recommended structure:

```
rhumbprotocol/                  # Core protocol
  ├── docs/
  │   └── PROTOCOL.md             # This specification
  ├── spec/
  │   ├── schemas/
  │   │   ├── plan.schema.json
  │   │   ├── intake.schema.json
  │   │   ├── manifest.schema.json
  │   │   ├── state.schema.json
  │   │   └── handoff.schema.json
  │   ├── uuid.format.md          # UUID format specification
  │   ├── uuid-generation.md      # UUID generation guidance
  │   ├── versioning.format.md    # Version embedding format
  │   ├── sequence.grammar        # Phase sequence ABNF grammar
  │   ├── sequence-parser.md      # Sequence parser specification
  │   ├── implementation-guide.md # Implementation best practices
  │   ├── conformance-levels.md   # Conformance level guidance
  │   ├── custom-fields.md        # Custom field extension patterns
  │   ├── schema-composition.md   # Schema composition & inheritance
  │   └── openapi-integration.md  # OpenAPI integration patterns
  ├── templates/
  │   ├── PLAN.md.template              # Master plan document
  │   ├── INTAKE.yaml.template          # Requirements capture
  │   ├── MASTERPLAN.yaml.template      # Execution masterplan
  │   ├── PLAN-STATE.yaml.template      # Runtime execution state
  │   ├── DEPENDENCIES.yaml.template    # Dependency tracking
  │   ├── MANIFEST-PLAN.yaml.template   # File/deliverable registry
  │   ├── sequences.yaml.template       # Sequence configuration
  │   ├── AVD.md.template               # Architecture Vision Document
  │   ├── AVD-PROMPT.md.template        # AVD generation prompt
  │   ├── ACS.md.template               # Architecture Component Spec
  │   ├── ACS-PROMPT.md.template        # ACS generation prompt
  │   ├── display/
  │   │   ├── PLAN-DRAFT-DISPLAY.md.template      # Plan draft output
  │   │   ├── PLAN-COMMIT-DISPLAY.md.template     # Plan commit output
  │   │   ├── PHASE-COMPLETE-DISPLAY.md.template  # Phase completion
  │   ├── SUBPHASE-COMPLETE-DISPLAY.md.template # Sub-phase completion
  │   ├── START-PROMPT.md.template      # Session initialization
  │   ├── PROMPT.md.template            # Phase continuation
  │   ├── HANDOFF.yaml.template         # Handoff document structure
  │   ├── AUDIT.md.template             # Audit checkpoint report
  │   ├── AUDIT-PROMPT.md.template      # Audit execution prompt
  │   ├── FINAL.md.template             # Final report
  │   └── FINAL-PROMPT.md.template      # Finalization prompt
  ├── integrations/                # Platform-specific adapters
  │   ├── claude-code/             # Claude Code (CLI)
  │   ├── codex/                   # OpenAI Codex
  │   ├── gemini-cli/              # Google Gemini CLI
  │   ├── claude-ai/               # Claude.ai (browser)
  │   ├── chatgpt/                 # ChatGPT (browser)
  │   └── gemini-web/              # Gemini (browser)
  └── extensions/                  # Third-party extensions
      ├── ai-research/
      │   ├── EXTENSION-README.md
      │   ├── templates/
      │   │   ├── research-plan.template.md
      │   │   └── experiment-phase.template.md
      │   ├── schemas/
      │   │   └── experiment-results.schema.json
      │   └── rules/
      │       └── research-validation.yaml
      ├── healthcare/
      │   ├── EXTENSION-README.md
      │   ├── templates/
      │   │   └── clinical-workflow.template.md
      │   ├── schemas/
      │   │   └── compliance-checklist.schema.json
      │   └── rules/
      │       └── hipaa-audit-trail.yaml
      ├── fintech/
      │   ├── EXTENSION-README.md
      │   ├── templates/
      │   │   └── trading-workflow.template.md
      │   ├── schemas/
      │   │   └── transaction-log.schema.json
      │   └── rules/
      │       └── sec-compliance.yaml
      └── community/
          └── (contributed by community members)
```

### Integration Points

Compliant implementations should support these integration points:

1. **Artifact Import/Export**
   - Read YAML/Markdown artifacts in canonical format
   - Write artifacts in canonical format
   - Support optional field additions without breaking reads
   - Version artifacts to match tool's supported RWP version

2. **Version Control Integration**
   - Store artifacts in git repositories
   - Ensure artifacts diff cleanly (no unnecessary reformatting)
   - Support branching and merging of workflows
   - Provide git hooks for validation (optional)

3. **Audit Trail & Logging**
   - Log all state changes with timestamps
   - Record who/what triggered each phase transition
   - Maintain immutable history (state file grows, never shrinks)
   - Support audit export for compliance/review

4. **Notifications & Webhooks**
   - Alert when phases complete or errors occur
   - Support outbound webhooks (e.g., Slack, email, webhook URLs)
   - Enable custom notification rules per workflow
   - Log all notifications for audit purposes

5. **API/SDK Interface**
   - RESTful API for workflow management (CRUD on artifacts)
   - GraphQL API for flexible querying (optional)
   - SDK for popular languages (Python, TypeScript, Go, Rust)
   - OpenAPI/GraphQL schema for tool integration

6. **CLI Interface**
   - `rwp init` - Initialize new workflow
   - `rwp phase start <phase>` - Start a phase
   - `rwp phase status` - Check phase status
   - `rwp artifact validate <artifact>` - Validate artifact against schema
   - `rwp export` - Export workflow for sharing
   - `rwp import` - Import workflow from file

7. **UI/Visualization**
   - Web dashboard showing phase progress
   - Real-time heartbeat indicator
   - Phase dependency graph visualization
   - Timeline view of past completions
   - Handoff viewer with rolling context
   - Error/alert management interface

### Recommended Extension Patterns

#### Pattern 1: Enhanced Metadata

Add structured metadata to enhance auditing without changing core fields:

```yaml
# In state.yaml custom section:
rwp_metadata:
  organization: "Acme Corp"
  team: "Engineering"
  environment: "production"
  cost_center: "ENG-1000"
  slack_channel: "#eng-workflows"
  jira_epic: "ENG-500"
  okr_alignment: "Q2-OKR-3"
```

#### Pattern 2: Compliance & Audit Extensions

Add compliance-specific tracking for regulated industries:

```yaml
# In custom audit section:
compliance:
  framework: "SOC2"
  reviewer: "compliance@company.com"
  review_date: "2026-03-04T15:00:00Z"
  approved: true
  approval_reason: "Phase meets compliance requirements"
  next_review: "2026-06-04T00:00:00Z"
```

#### Pattern 3: Cost & Resource Tracking

Integrate resource utilization and cost estimates:

```yaml
# In custom resources section:
resource_consumption:
  compute_minutes: 120
  estimated_cost_usd: 12.50
  ai_api_calls: 450
  storage_gb: 2.1
  headcount_days: 1.5
```

#### Pattern 4: AI Suggestions & Insights

Add AI-generated analysis and recommendations:

```yaml
# In custom insights section:
ai_analysis:
  phase_complexity_score: 7.2
  risk_score: 4.1
  suggestion: "Consider parallelizing P-03-B and P-03-C to save 15 minutes"
  similar_workflows: 3
  estimated_time_remaining: "2 hours 30 minutes"
```

---

## Best Practices for RWP Implementations

This section provides recommendations for tools and organizations implementing RWP:

### File Management

1. **Version Control**: Always store RWP artifacts in git (or equivalent VCS)
   - Enables workflow history, audit trails, and collaboration
   - Use `.gitignore` to exclude sensitive data (e.g., API keys in state)
   - Tag commits corresponding to phase completions

2. **Directory Structure**: Organize artifacts in a consistent directory layout
   ```
   my-project/
   ├── .meridian/
   │   ├── plans/
   │   │   ├── PLAN.md
   │   │   ├── INTAKE.yaml
   │   │   └── manifest.yaml
   │   └── state.yaml
   ├── handoffs/
   │   ├── HO-P-0001-P-01-2026-03-04.md
   │   └── HO-P-0001-P-02-2026-03-04.md
   ├── src/
   └── tests/
   ```

3. **Artifact Naming**: Use consistent naming conventions
   - Plans: `PLAN.md` (singular)
   - Intake: `INTAKE.yaml` (singular)
   - Handoffs: `HO-{PLAN_ID}-{PHASE}-{DATE}.yaml`
   - State: `state.yaml` (singular)

### Execution Best Practices

1. **Heartbeat Discipline**: Update heartbeat at least every 15-30 minutes
   - Prevents false "timeout" errors
   - Provides granular insight into phase progress
   - Enables early detection of stalled phases

2. **Handoff Quality**: Invest time in thorough handoff documentation
   - Next phase often depends on implicit context captured in handoff
   - Detailed handoffs reduce context-switching time
   - Include rolling context, not just deliverables

3. **Phase Planning**: Be realistic about phase duration estimates
   - Use historical data from similar projects
   - Include buffer time (10-20%) for unknowns
   - Document assumptions that could invalidate estimates

4. **Dependency Management**: Make dependencies explicit, never implicit
   - List all prerequisites before starting a phase
   - Verify dependencies are truly met before proceeding
   - Create explicit blockers if dependencies are missing

### For Tool Builders

1. **Schema Validation**: Implement validation against JSON/YAML schemas
   - Catches format errors early
   - Prevents invalid state transitions
   - Improves developer experience with clear error messages

2. **Artifact Export**: Provide export/import capabilities
   - Enable workflow portability across tools
   - Support multiple export formats (YAML, JSON, Markdown)
   - Test round-trip (export then import) for fidelity

3. **Error Handling**: Design recoverable error states
   - Distinguish between fatal and recoverable errors
   - Provide clear recovery instructions
   - Log error context for debugging

4. **Notifications**: Implement phase completion notifications
   - Alert stakeholders when phases complete or error
   - Support multiple notification channels (email, Slack, webhooks)
   - Make notifications actionable (include links, summary)

### For Workflow Designers

1. **Phase Sizing**: Keep phases to 4-8 hours estimated duration
   - Phases that are too small add overhead
   - Phases that are too large make recovery difficult
   - Sub-phases enable finer granularity without overhead

2. **Deliverable Clarity**: Make deliverables concrete and measurable
   - "Implement authentication" (vague) vs. "Add OAuth 2.0 PKCE flow with tests and documentation" (concrete)
   - Measurable deliverables enable objective completion verification

3. **Verification Commands**: Include exact commands to verify completion
   - Don't rely on subjective assessment
   - Verification commands must be deterministic and reproducible
   - Include success criteria (e.g., "0 test failures")

4. **Risk Documentation**: Identify risks and mitigations early
   - Risks often impact timeline; addressing them proactively saves time
   - Mitigations should be concrete actions, not wishful thinking

## Summary & Next Steps

The Rhumb Workflow Protocol defines a vendor-neutral set of artifacts and lifecycle rules for AI-assisted workflow management. Tools that implement this specification can move plans, state, and handoffs between platforms without losing the audit trail.

RWP is designed to be:

- **Vendor-Neutral** - Not tied to any single tool or platform
- **Language-Agnostic** - YAML and Markdown are universally supported
- **Simple** - Five artifact types and straightforward state machine
- **Extensible** - Custom fields and domain-specific extensions supported
- **Auditable** - Complete history with immutable event log
- **Version-Controlled** - All artifacts suitable for git

### For Protocol Implementers

1. **Start Small** - Begin with Level 1 (Plan + State) to validate core concepts
2. **Add Incrementally** - Adopt Level 2 features (Intake, Manifest, Handoff) based on user feedback
3. **Plan Extensions** - Design extensibility before implementing Level 3 features
4. **Implement Validation** - Build JSON/YAML schema validators early; they catch many errors
5. **Declare Conformance** - Clearly document which RWP version and conformance level you support
6. **Test Interoperability** - Exchange workflows with other RWP-compliant tools to verify compatibility

### For Workflow Users

1. **Start with Planning** - Invest time in detailed Plan and Intake documents upfront
2. **Use Version Control** - Store all artifacts in git; never edit state.yaml directly in production
3. **Create Handoffs** - Document what worked, what didn't, and key context for the next phase
4. **Verify Completeness** - Use Manifest validation to ensure all promised deliverables exist
5. **Archive Workflows** - Keep completed workflows for reference and learning
6. **Share Workflows** - Export and share workflows with teams; reuse templates for similar projects

### For Organizations

1. **Standardize on RWP** - Adopt RWP as the standard for multi-phase projects
2. **Build Templates** - Create organization-specific templates for common workflow types
3. **Invest in Tooling** - Choose Level 2+ tools for professional teams; Level 3 for enterprises
4. **Document Patterns** - Capture and document successful workflow patterns
5. **Measure & Optimize** - Track metrics (phase duration, error rates, rework) and optimize over time

### Reference Implementation

YAKKL Meridian (https://meridian.yakkl.com) provides a Level 3 reference implementation in TypeScript/Rust, supporting all RWP features including integrations, parallel execution, and real-time dashboards. See the Meridian documentation for implementation examples, best practices, and advanced techniques.

### Contributing to RWP

RWP is an open standard. Contributions are welcome:

1. **Report Issues** - Found a problem with the spec? File an issue on GitHub
2. **Propose Extensions** - Have an idea for a domain-specific extension? Start a discussion
3. **Build Implementations** - Implement RWP in your favorite language/platform
4. **Share Workflows** - Contribute workflow templates to the community marketplace

---

## Final Notes

This specification represents v0.27.0 of the Rhumb Workflow Protocol. As adoption grows and real-world use cases emerge, the protocol will evolve. However, the core principles will remain:

- **Simplicity**: Five artifacts, clear state machine, no magic
- **Interoperability**: Any RWP tool can read any RWP workflow
- **Auditability**: Complete history, immutable events
- **Extensibility**: Grow the protocol without breaking compatibility

Welcome to RWP. Let's build better workflows together.

---

## Questions & Feedback

RWP is an open standard developed by the community. For questions, feedback, or contributions:

- **Specification**: https://rhumbprotocol.dev
- **GitHub**: https://github.com/rhumbprotocol/specs
- **Created by**: YAKKL Inc. (https://yakkl.com)
- **Reference Implementation**: YAKKL Meridian (https://meridian.yakkl.com)

RWP is licensed under Apache-2.0, enabling free use, modification, and redistribution for any purpose.

---

## Quick Reference

### Artifact Types at a Glance

| Artifact | Purpose | Format | When Created | Updated By |
|----------|---------|--------|--------------|-----------|
| **PLAN.md** | Master workflow specification | Markdown | Project start | Project owner |
| **INTAKE.yaml** | Requirements & constraints | YAML | Before execution | As requirements change |
| **manifest.yaml** | Deliverable registry | YAML | After each phase | Executor |
| **state.yaml** | Runtime execution state | YAML | Execution start | Continuously (executor) |
| **HO-*.yaml** | Phase completion handoff | Markdown | After each phase | Phase executor |

### Key Acronyms & Abbreviations

- **RWP** - Rhumb Workflow Protocol
- **HO** - Handoff document
- **P-XX** - Phase numbering (e.g., P-01, P-02-A)
- **ISO 8601** - Date/time standard (YYYY-MM-DDTHH:MM:SSZ)
- **YAML** - Data serialization format (human-readable)
- **REQ** - Requirement identifier
- **PP** - Pain point identifier
- **CON** - Constraint identifier

### Phase Lifecycle Diagram (Text Format)

```
planning
    │
    ├─ start signal
    │
    ▼
in_progress (must have heartbeat)
    │
    ├─ timeout (no heartbeat) → error (recoverable)
    │
    ├─ task failure → error (check if recoverable)
    │
    ├─ all tasks complete → create handoff
    │
    ▼
completed (creates handoff document)
    │
    ├─ handoff validation
    │
    ▼
next phase can start (if dependencies met)
```

### Conformance Quick Decision Tree

```
Are you building a tool?
├─ No → Skip to user section
│
└─ Yes → How many concurrent workflows?
   ├─ < 10 or learning → Level 1
   ├─ 10-1000 → Level 2
   └─ > 1000 or enterprise → Level 3
```

### Validation Checklist for Phase Completion

Before marking a phase complete:

- [ ] All listed deliverables exist and are accessible
- [ ] Verification commands run without errors
- [ ] Tests pass with expected coverage
- [ ] Manifest is updated with new files
- [ ] Handoff document is created and formatted correctly
- [ ] Rolling context for next phase is included
- [ ] No open blockers or unresolved errors
- [ ] State file is updated with completion timestamp

### Common Workflow Sizes

| Workflow Type | Phases | Duration | Team Size |
|---|---|---|---|
| Small project | 3-5 | 1-2 weeks | 1-2 |
| Medium project | 6-10 | 2-4 weeks | 3-5 |
| Large project | 10-20 | 4-12 weeks | 5-15 |
| Enterprise system | 20+ | 12+ weeks | 15+ |

### Frequently Asked Questions

**Q: Can I use RWP without version control?**
A: Technically yes, but you lose audit trail. Version control (git) is strongly recommended for production workflows.

**Q: What if a phase takes longer than estimated?**
A: Update the Plan's estimate and state.yaml's heartbeat to reflect reality. Adjust dependent phases' start times. Document reasons in handoff for future reference.

**Q: Can phases run in parallel?**
A: RWP Level 1-2 assume sequential phases. Level 3 tools may support parallel execution if phases don't depend on each other. Check manifest dependencies.

**Q: How do I rollback to a previous phase?**
A: RWP doesn't support rollback by design (immutable audit trail). Instead, create a new sub-phase to undo changes or fix issues.

**Q: Can I migrate from one tool to another?**
A: Yes, if both tools are RWP-compliant. Export artifacts from the source tool, import to the target. Check artifact validity with schema validation.

**Q: What if I lose the state.yaml file?**
A: State can be reconstructed from git history and handoff documents, but the immutable event log is lost. Always backup your repository.

**Q: Is RWP suitable for agile/iterative workflows?**
A: RWP works best for waterfall-style structured projects. For agile workflows, consider creating a short RWP plan for each sprint.

## Protocol Versioning

The Rhumb Workflow Protocol uses semantic versioning (SemVer 2.0.0) to track specification changes and ensure compatibility across implementations.

### Versioning Scheme

```
MAJOR.MINOR.PATCH

MAJOR = Breaking changes (schema structure changes, required field removals)
MINOR = Backward-compatible additions (new optional fields, new artifact types)
PATCH = Non-breaking clarifications (documentation, example updates, typo fixes)
```

**Current Version**: 0.27.0
**Stability**: Stable (no breaking changes until 2.0.0)

### Artifact Versioning Fields

The `rwp_version` field MUST be embedded in the following artifact types to track protocol compliance:

| Artifact Type | Field | Format | Required | Inherited | Notes |
|---|---|---|---|---|---|
| Plan | `rwp_version` | SemVer 2.0.0 | ✓ MUST | N/A | Toplevel version |
| Masterplan | `rwp_version` | SemVer 2.0.0 | ✓ MUST | Plan | Inherits from parent plan |
| State | `rwp_version` | SemVer 2.0.0 | ✓ MUST | Plan | Immutable at creation |
| Intake | `rwp_version` | SemVer 2.0.0 | ✓ MUST | Plan | Request normalization |
| Manifest | `rwp_version` | SemVer 2.0.0 | ✓ MUST | Plan | File tracking |
| Handoff | `rwp_version` | SemVer 2.0.0 | ✓ MUST | Parent phase | Inherits from plan |
| Phase | `rwp_version` | SemVer 2.0.0 | ✗ SHOULD | Parent plan | Inherited, not re-assigned |
| Audit | `rwp_version` | SemVer 2.0.0 | ✓ MUST | Parent phase | Spec version at audit time |

**Key Rules**:
- All **top-level artifacts** (Plan, State, Intake, Manifest) MUST carry `rwp_version`
- **Child artifacts** (Handoff, Phase, Audit) inherit version from parent phase/plan
- Version field is **immutable** once created - no retroactive version updates
- Version MUST be in strict SemVer 2.0.0 format: `MAJOR.MINOR.PATCH[-prerelease]`, not `vMAJOR.MINOR.PATCH` or `MAJOR.MINOR.PATCH.BUILD`

### Compatibility Rules

**Backward Compatibility (MINOR/PATCH)**:

1. New optional fields MAY be added (minor version bump)
2. Existing required fields MUST NOT be removed
3. Field types MUST NOT change (string remains string)
4. Array/object structure MUST NOT fundamentally change
5. Old tools can safely ignore unknown optional fields

**Breaking Changes (MAJOR)**:

1. Removing required fields
2. Changing field types (string → number)
3. Restructuring nested objects
4. Removing or renaming lifecycle states
5. Changing core schema constraints

**Examples**:

| Change | Version | Reason |
|--------|---------|--------|
| Add optional `custom_metadata` field | 1.1.0 | New optional field (MINOR) |
| Clarify `created` timestamp format | 1.0.1 | Documentation (PATCH) |
| Change `phase_id` from required to optional | 2.0.0 | Backward-incompatible change (MAJOR) |
| Add new artifact type "research" | 1.1.0 | Additive, backward-compatible (MINOR) |
| Rename "plan" to "workflow" everywhere | 2.0.0 | Breaking change (MAJOR) |

### Version Detection at Runtime

When processing artifacts, tools MUST detect and validate protocol version:

**Algorithm (Priority Order)**:

1. **Explicit Field**: Check for `rwp_version` in artifact metadata
   ```yaml
   rwp_version: "0.27.0"
   ```

2. **Parent Inheritance**: If version field missing, inherit from parent artifact
   ```yaml
   # Handoff inherits from parent phase's plan
   parent_phase: "P-02-C"
   rwp_version: null  # Use parent plan's version
   ```

3. **Schema Inference**: If both missing, infer from schema features
   ```
   If artifact has optional_field_X → minimum version 1.1.0
   If artifact has optional_field_Y → minimum version 1.2.0
   ```

4. **Fallback Default**: Assume 0.27.0 (earliest version on record)
   ```
   Last resort when all else fails
   ```

### Version Compatibility Assertion

Before processing an artifact, tools SHOULD assert version compatibility:

```
Tool supports RWP versions: [0.27.0, 1.1.0]
Artifact version: 1.0.5
Result: COMPATIBLE (1.0.5 is patch of 0.27.0, covered by support)

Tool supports RWP versions: [0.27.0]
Artifact version: 1.1.0
Result: CAUTION (tool doesn't support new optional fields)

Tool supports RWP versions: [0.27.0]
Artifact version: 2.0.0
Result: INCOMPATIBLE (breaking changes, reject processing)
```

### Migration & Upgrade Path

**For Users**:
1. Existing RWP 0.27.0 workflows continue working indefinitely (backward compatibility)
2. Optional new fields in 1.x can be ignored by 1.0.0-only tools
3. Upgrade to newer tools when you want to use 1.x features

**For Tool Builders**:
1. Always support at minimum the protocol version you're designed for
2. Add support for newer MINOR/PATCH versions automatically
3. Warn users when encountering newer MAJOR versions
4. Provide clear migration docs when releasing 2.0.0+

**Checking Protocol Version of Installed Tools**:

```bash
# Example: Meridian CLI
meridian --version
# Output: Meridian 0.61.0 (RWP 0.27.0 compatible)

# Example: Codex
codex info
# Output: Codex with RWP 0.27.0 support

# Your Tool
./your-tool --check-rwp
# Output: Your Tool v1.0 supports RWP 0.27.0 - 1.2.0
```

### Examples: Version in Artifacts

**RWP 0.27.0 Plan** (minimal):

```yaml
rwp_version: "0.27.0"
plan_id: "MP-0001-example-plan"
title: "My Project"
```

**RWP 1.1.0 Plan** (with new optional field):

```yaml
rwp_version: "1.1.0"
plan_id: "MP-0001-example-plan"
title: "My Project"
custom_metadata: {...}  # New in 1.1.0, ignored by 0.27.0 tools
```

**RWP 1.0.5 Handoff** (inherits version):

```yaml
rwp_version: "0.27.0"  # Inherited from parent plan
handoff_id: "HO-MP-0001-P-01-2026-03-04.md"
parent_plan: "MP-0001-example-plan"
```

### External Resources

- **Specification**: https://rhumbprotocol.dev
- **GitHub Organization**: https://github.com/rhumbprotocol
- **Created By**: YAKKL Inc. (https://yakkl.com)
- **Reference Implementation**: YAKKL Meridian (https://meridian.yakkl.com)
- **License**: Apache-2.0 (https://opensource.org/licenses/Apache-2.0)
- **Community Forum**: https://rhumbprotocol.dev/community
- **Issues & Feedback**: https://github.com/rhumbprotocol/specs/issues

---

*Specification Version: 0.27.0*
*Status: Released*
*Last Updated: 2026-03-04*
*License: Apache-2.0*
*Copyright: Copyright © 2026 YAKKL® Inc. All Rights Reserved.*
