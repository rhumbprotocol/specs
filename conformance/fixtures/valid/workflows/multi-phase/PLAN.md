# Rhumb Workflow Protocol: Plan Document

---

plan_id: MP-0087-api-rate-limiter
request_id: REQ-00312
name: API Rate Limiter Service
classification: confidential
status: processing
created: 2026-02-15T09:30:00Z
author: Backend Tech Lead
phases: 5
current_phase: P-02-B
started: 2026-02-15T10:00:00Z
completed: null
parent: null
rwp_version: "0.29.0"
dependencies: []
packages:
  - rate-limiter
  - api-gateway
  - dashboard-frontend

tracking:
  ticket: PLAT-456
  epic: PLAT-400
  external_url: null
  assigned_to: Backend Team
  estimate: L
  actual: null
  priority: P0
  labels:
    - rate-limiting
    - infrastructure
    - enterprise-blocker

---

# MP-0087-api-rate-limiter: API Rate Limiter Service

---

## Executive Summary

Build a distributed rate limiting service backed by Redis sliding window counters.
The service integrates into the API gateway as middleware, adds rate limit headers to
every response, and exposes per-customer usage data to a dashboard. The current
per-instance rate limiting is inconsistent and allows burst abuse - this plan replaces
it with globally-consistent enforcement.

---

## Problem Statement

The public API currently enforces rate limits per gateway instance. With 8 instances
behind a load balancer, customers effectively get 8x their plan limit. This causes
billing inaccuracies, allows burst abuse that degrades other customers' experience,
and leaves support without visibility into usage patterns. An enterprise customer
contract renewal requires proper rate limiting before March 1, 2026.

---

## Task Hierarchy

```
P-01 (Phase)                    # Redis module + sliding window algorithm
P-02 (Phase)                    # Gateway middleware integration
  +-- P-02-A (Sub-phase)        # Middleware skeleton + request interception
  +-- P-02-B (Sub-phase)        # Rate limit headers + fail-open logic
  +-- P-02-C (Sub-phase)        # Dynamic configuration + hot reload
P-03 (Phase)                    # Testing + benchmarks
AUDIT                           # Post-implementation audit
P-04 (Phase)                    # Usage dashboard
P-05 (Phase)                    # Documentation + deployment
```

---

## Phase Breakdown

### P-01: Redis Module and Sliding Window Algorithm

**Depends On**: None
**Estimated Duration**: 90 minutes

**Objective**: Implement the core rate limiting logic as a standalone module with Redis-backed sliding window counters.

**Tasks**:
1. Create `packages/rate-limiter/src/window.ts` with sliding window counter implementation using Redis sorted sets
2. Create `packages/rate-limiter/src/config.ts` for rate limit configuration types (per-plan limits, window sizes)
3. Create `packages/rate-limiter/src/client.ts` for Redis connection management with connection pooling
4. Create `packages/rate-limiter/src/index.ts` exporting public API: `checkLimit()`, `getRemainingQuota()`, `resetLimit()`
5. Write unit tests in `packages/rate-limiter/tests/window.test.ts` using Redis mock

**Files to Create/Modify**:
| File | Action | Description |
|------|--------|-------------|
| `packages/rate-limiter/src/window.ts` | Create | Sliding window algorithm using ZRANGEBYSCORE |
| `packages/rate-limiter/src/config.ts` | Create | Configuration types and defaults |
| `packages/rate-limiter/src/client.ts` | Create | Redis client with connection pooling |
| `packages/rate-limiter/src/index.ts` | Create | Public API exports |
| `packages/rate-limiter/tests/window.test.ts` | Create | Unit tests with Redis mock |
| `packages/rate-limiter/package.json` | Create | Package configuration |

**Verification**:
```bash
cd packages/rate-limiter && pnpm test
```

**Expected Results**:
- `checkLimit(customerId, planId)` returns `{ allowed: boolean, remaining: number, resetAt: number }`
- Sliding window correctly handles boundary cases
- All unit tests pass with Redis mock
- Module exports clean public API

---

### P-02: Gateway Middleware Integration

**Depends On**: P-01
**Estimated Duration**: 90 minutes (split into sub-phases)

**Objective**: Integrate the rate limiter module into the API gateway as Express middleware with response headers, fail-open behavior, and dynamic configuration.

> **Note**: This phase uses sub-phases for crash resilience (~30 min each).

#### P-02-A: Middleware Skeleton and Request Interception (~30 min)

**Depends On**: P-01
**Objective**: Create the Express middleware that intercepts requests and calls the rate limiter.

**Tasks**:
1. Create `packages/api-gateway/src/middleware/rate-limit.ts` with Express middleware signature
2. Extract customer ID from API key in request header (`X-API-Key`)
3. Look up customer's plan from the auth service
4. Call `checkLimit()` and return 429 if denied

**Files to Create/Modify**:
| File | Action | Description |
|------|--------|-------------|
| `packages/api-gateway/src/middleware/rate-limit.ts` | Create | Express middleware |
| `packages/api-gateway/src/app.ts` | Modify | Mount rate limit middleware |

**Expected Results**:
- Requests with valid API key are rate-checked
- 429 response returned when limit exceeded
- Requests without API key pass through (public endpoints)

---

#### P-02-B: Rate Limit Headers and Fail-Open Logic (~30 min)

**Depends On**: P-02-A
**Objective**: Add standard rate limit headers to all responses and implement graceful degradation.

**Tasks**:
1. Add `X-RateLimit-Limit`, `X-RateLimit-Remaining`, `X-RateLimit-Reset` headers to all responses
2. Add `Retry-After` header to 429 responses
3. Implement fail-open: if Redis is unreachable, allow traffic and log warning
4. Add circuit breaker pattern: after N consecutive Redis failures, skip rate limiting for M seconds

**Files to Create/Modify**:
| File | Action | Description |
|------|--------|-------------|
| `packages/api-gateway/src/middleware/rate-limit.ts` | Modify | Add headers + fail-open |
| `packages/rate-limiter/src/health.ts` | Create | Redis health check + circuit breaker |

**Expected Results**:
- All API responses include rate limit headers
- Redis failure causes fail-open with log warnings
- Circuit breaker prevents Redis reconnection storms

---

#### P-02-C: Dynamic Configuration and Hot Reload (~30 min)

**Depends On**: P-02-B
**Objective**: Allow rate limit configuration changes without redeployment.

**Tasks**:
1. Store rate limit configs in Redis hash (`rate-limits:config:{planId}`)
2. Add config watcher that polls for changes every 30 seconds
3. Create admin API endpoint `PUT /admin/rate-limits/:planId` for config updates
4. Add config validation to reject invalid values (negative limits, zero windows)

**Files to Create/Modify**:
| File | Action | Description |
|------|--------|-------------|
| `packages/rate-limiter/src/config.ts` | Modify | Redis-backed config with polling |
| `packages/api-gateway/src/routes/admin.ts` | Modify | Add config update endpoint |

**Expected Results**:
- Rate limit changes take effect within 30 seconds without restart
- Admin endpoint validates and persists config changes
- Invalid configs rejected with clear error messages

---

### P-03: Testing and Benchmarks

**Depends On**: P-02
**Estimated Duration**: 60 minutes

**Objective**: Comprehensive testing including integration tests with real Redis and performance benchmarks.

**Tasks**:
1. Write integration tests with Docker Redis: multi-instance simulation, boundary burst test, fail-open test
2. Write performance benchmark: measure p99 latency overhead of rate limit check
3. Write load test script: simulate 1000 req/sec across 4 gateway instances
4. Verify p99 overhead < 5ms (REQ-04)

**Files to Create/Modify**:
| File | Action | Description |
|------|--------|-------------|
| `packages/rate-limiter/tests/integration.test.ts` | Create | Integration tests with real Redis |
| `packages/rate-limiter/tests/benchmark.ts` | Create | Latency benchmark |
| `packages/api-gateway/tests/rate-limit-load.test.ts` | Create | Multi-instance load test |

**Verification**:
```bash
pnpm test -- --integration
pnpm run benchmark
```

**Expected Results**:
- All integration tests pass with real Redis
- p99 latency overhead < 5ms
- Multi-instance test confirms global limit enforcement
- Boundary burst test confirms sliding window correctness

---

### AUDIT: Post-Implementation Audit

**Depends On**: P-03
**Estimated Duration**: 30 minutes

**Objective**: Verify all prior work meets quality and performance requirements before dashboard work.

**Audit Requirements**:
- Consider verifying all unit and integration tests pass
- Consider confirming p99 latency benchmark < 5ms
- Consider reviewing fail-open behavior under Redis failure
- Consider verifying rate limit headers in API responses
- Consider checking dynamic config reload works
- Consider reviewing code for security issues (rate limit bypass, injection)

**Verification**:
```bash
pnpm test
pnpm run benchmark
```

**Audit Report**: `audits/AUD-MP-0087-api-rate-limiter-2026-02-20.md`

---

### P-04: Usage Dashboard

**Depends On**: AUDIT
**Estimated Duration**: 90 minutes

**Objective**: Build a customer-facing usage dashboard showing real-time rate limit consumption.

**Tasks**:
1. Create API endpoint `GET /api/usage/:apiKey` returning current window usage, limit, and remaining
2. Create dashboard component showing usage bar chart per API key
3. Add WebSocket stream for real-time usage updates (optional, falls back to polling)
4. Add usage history chart (last 24 hours, 1-hour buckets)

**Files to Create/Modify**:
| File | Action | Description |
|------|--------|-------------|
| `packages/api-gateway/src/routes/usage.ts` | Create | Usage API endpoint |
| `packages/dashboard-frontend/src/components/UsageChart.svelte` | Create | Usage bar chart |
| `packages/dashboard-frontend/src/components/UsageHistory.svelte` | Create | 24h history chart |
| `packages/dashboard-frontend/src/routes/usage/+page.svelte` | Create | Usage dashboard page |

**Expected Results**:
- Usage endpoint returns accurate real-time data
- Dashboard shows consumption vs. limit for each API key
- History chart shows 24-hour usage pattern

---

### P-05: Documentation and Deployment

**Depends On**: P-04
**Estimated Duration**: 45 minutes
**is_final**: true

**Objective**: Complete documentation and deploy to staging.

**Tasks**:
1. Write API documentation for rate limit headers and usage endpoint
2. Write runbook for rate limit configuration changes
3. Write deployment guide with Redis requirements
4. Deploy to staging environment
5. Run smoke tests on staging

**Files to Create/Modify**:
| File | Action | Description |
|------|--------|-------------|
| `packages/rate-limiter/README.md` | Create | Module documentation |
| `docs/rate-limiting.md` | Create | API documentation for customers |
| `docs/runbooks/rate-limit-config.md` | Create | Operations runbook |

**Expected Results**:
- Complete API documentation published
- Runbook covers common operations
- Staging deployment successful
- Smoke tests pass

---

## Phase Dependency Graph

```
P-01 (Redis Module)
  |
  +---- P-02 (Gateway Integration)
  |       +-- P-02-A (Middleware)
  |       +-- P-02-B (Headers + Fail-Open)
  |       +-- P-02-C (Dynamic Config)
  |
  +---- P-03 (Testing) ----- AUDIT
                                |
                                +---- P-04 (Dashboard)
                                        |
                                        +---- P-05 (Docs + Deploy)
```

---

## Dependencies

| Dependency | Type | Status | Notes |
|------------|------|--------|-------|
| Redis 7.x cluster | Infrastructure | Met | Existing cluster with spare capacity |
| Express.js 4.x | Package | Met | Current gateway framework |
| ioredis 5.x | Package | Met | Already in gateway dependencies |
| Svelte 5 | Package | Met | Dashboard frontend framework |
| Auth service API | External | Met | Provides customer plan lookup |

---

## Files Reference

### New Files

| File | Purpose |
|------|---------|
| `packages/rate-limiter/` | Standalone rate limiter module (6 files) |
| `packages/api-gateway/src/middleware/rate-limit.ts` | Express middleware |
| `packages/api-gateway/src/routes/usage.ts` | Usage API endpoint |
| `packages/dashboard-frontend/src/components/UsageChart.svelte` | Usage visualization |
| `docs/rate-limiting.md` | Customer-facing API docs |
| `docs/runbooks/rate-limit-config.md` | Operations runbook |

### Modified Files

| File | Changes |
|------|---------|
| `packages/api-gateway/src/app.ts` | Mount rate limit middleware |
| `packages/api-gateway/src/routes/admin.ts` | Add config update endpoint |

---

## Success Criteria

1. Global rate limiting works across all gateway instances (REQ-01)
2. Sliding window prevents boundary burst attacks (REQ-02)
3. Rate limit headers present in all API responses (REQ-03)
4. Rate limit check adds < 5ms p99 latency (REQ-04)
5. Fail-open behavior when Redis is unavailable (REQ-05)
6. Usage dashboard shows real-time consumption (REQ-06)
7. Configuration changeable without redeployment (CON-04)

---

## Risk Assessment

| Risk | Likelihood | Impact | Mitigation |
|------|------------|--------|------------|
| Redis latency spikes under load | Medium | High | Circuit breaker + fail-open (P-02-B) |
| Race conditions in sliding window | Low | High | Atomic Redis operations (MULTI/EXEC) |
| Dashboard overloads usage endpoint | Low | Medium | Cache usage data with 5s TTL |
| Config change causes outage | Low | High | Validation in admin endpoint (P-02-C) |
| Enterprise customer deadline missed | Medium | Critical | P-01-P-03 prioritized over dashboard |

---

## Rollback Plan

If this plan needs to be reverted:

1. Remove rate limit middleware from `api-gateway/src/app.ts`
2. Revert admin route changes
3. Rate limiter package can remain (unused) - no side effects
4. Redis keys expire automatically (TTL on sorted sets)

**Rollback Verification**:
```bash
# Verify middleware removed
grep -r "rate-limit" packages/api-gateway/src/app.ts
# Verify API returns no rate limit headers
curl -I https://api.example.com/v1/health | grep -i x-ratelimit
```

---

## Notes

- Enterprise customer renewal is March 1, 2026 - this is the primary driver
- Dashboard (P-04) is P2 priority; if timeline is tight, ship P-01-P-03 first
- Consider migrating from sorted sets to Redis Streams in a future iteration for better memory efficiency

---

## Changelog

| Date | Phase | Author | Changes |
|------|-------|--------|---------|
| 2026-02-15 | P-01 | Backend Lead | Initial plan created |
| 2026-02-15 | P-01 | AI Agent | P-01 completed, core module delivered |
| 2026-02-15 | P-02-A | AI Agent | Middleware skeleton complete |

---

Produced: 2026-02-15T09:30:00Z
By: Rhumb Protocol Contributors - https://rhumbprotocol.dev
