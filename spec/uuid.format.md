# RWP UUID Format Specification

> Part of the [Rhumb Workflow Protocol (RWP)](../docs/PROTOCOL.md) - see also: [UUID Generation](./uuid-generation.md), [Implementation Guide](./implementation-guide.md)

## Overview

The Rhumb Workflow Protocol (RWP) defines a standardized UUID format for uniquely identifying plans, intakes, phases, and other workflow artifacts. RWP UUIDs are cryptographically secure, URL-safe, and human-readable, enabling reliable deduplication and cross-reference tracking across distributed systems.

## UUID Type: UUIDv4 (Random)

RWP adopts **RFC 4122 UUIDv4 (random)** as its standard format for all workflow artifact identifiers.

### Characteristics

- **Version**: UUIDv4 (random UUID)
- **Length**: 128 bits (16 bytes)
- **Encoding**: Hexadecimal with hyphens (canonical form)
- **Entropy Source**: Cryptographically secure random number generator (CSPRNG)
- **Collision Probability**: ~1 in 5.3 × 10^36 (astronomically low)

### Format

```
xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx
```

Where:
- `x` = random hex digit (0-9, a-f)
- `4` = version 4 indicator (fixed at third segment)
- `y` = variant bits (8, 9, a, or b, indicating RFC 4122 compliance)

### Example UUIDs

```
550e8400-e29b-41d4-a716-446655440000
a3bb189e-8bf9-4067-b30c-28dde98ccc7f
7e89c4a2-3d9f-4f8e-b5c1-2a7d1f4e6c9b
```

## Canonical Representation

UUIDs in RWP MUST use the canonical lowercase hexadecimal representation with hyphens:

```
xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx
```

- **Lowercase**: Use lowercase hex digits (a-f), not uppercase (A-F)
- **Hyphenated**: Include the standard hyphens at positions 8, 13, 18, 23
- **No Braces**: Do not wrap in curly braces `{...}`
- **No URN Prefix**: Do not include `urn:uuid:` prefix

### Valid Examples

```
✓ 550e8400-e29b-41d4-a716-446655440000
✓ a3bb189e-8bf9-4067-b30c-28dde98ccc7f
✓ 7e89c4a2-3d9f-4f8e-b5c1-2a7d1f4e6c9b
```

### Invalid Examples

```
✗ 550E8400-E29B-41D4-A716-446655440000   (uppercase)
✗ 550e8400e29b41d4a716446655440000       (no hyphens)
✗ {550e8400-e29b-41d4-a716-446655440000} (braced)
✗ urn:uuid:550e8400-e29b-41d4-a716-446655440000 (URN prefixed)
```

## Usage in RWP Artifacts

### Plan Identifier

A Plan identifier is typically assigned by the RWP implementation and stored in the plan metadata:

```yaml
# In PLAN.md frontmatter (if used)
---
id: 550e8400-e29b-41d4-a716-446655440000
title: "Build Analytics Platform"
created: "2026-03-04T08:00:00Z"
---
```

Or tracked in `state.yaml`:

```yaml
plan_id: 550e8400-e29b-41d4-a716-446655440000
```

### Intake Identifier

Intakes are assigned a shorter human-readable ID for quick reference:

```yaml
id: INT-0001  # Human-readable
uuid: a3bb189e-8bf9-4067-b30c-28dde98ccc7f  # Machine-readable
```

### File & Artifact Identifiers

Individual files and artifacts can track a UUID for deduplication:

```yaml
files:
  - id: FILE-001
    uuid: 7e89c4a2-3d9f-4f8e-b5c1-2a7d1f4e6c9b
    path: "src/schema.sql"
```

## Generation

### For Third-Party Implementations

To generate RWP-compliant UUIDs, use your platform's built-in UUIDv4 generator:

**Python**:
```python
import uuid
plan_id = str(uuid.uuid4())  # e.g., 550e8400-e29b-41d4-a716-446655440000
```

**JavaScript/TypeScript**:
```typescript
import { v4 as uuidv4 } from 'uuid';
const planId = uuidv4();  // e.g., 550e8400-e29b-41d4-a716-446655440000
```

**Rust**:
```rust
use uuid::Uuid;
let plan_id = Uuid::new_v4().to_string();
```

**Go**:
```go
import "github.com/google/uuid"
planID := uuid.New().String()
```

### Entropy Requirements

- **Minimum Entropy**: 128 bits of cryptographic entropy
- **CSPRNG Required**: Use `crypto.getRandomValues()` (JavaScript), `os.urandom()` (Python), or equivalent
- **No Weak Sources**: Do not use `Math.random()`, `rand()`, or other non-cryptographic PRNGs

## Validation

### Regex Pattern

To validate an RWP UUID in canonical form:

```regex
^[a-f0-9]{8}-[a-f0-9]{4}-4[a-f0-9]{3}-[89ab][a-f0-9]{3}-[a-f0-9]{12}$
```

### Implementation Examples

**Python**:
```python
import re
import uuid

pattern = r'^[a-f0-9]{8}-[a-f0-9]{4}-4[a-f0-9]{3}-[89ab][a-f0-9]{3}-[a-f0-9]{12}$'

def is_valid_rwp_uuid(s: str) -> bool:
    return bool(re.match(pattern, s))

# Usage
assert is_valid_rwp_uuid('550e8400-e29b-41d4-a716-446655440000')
assert not is_valid_rwp_uuid('550E8400-E29B-41D4-A716-446655440000')  # uppercase
```

**TypeScript**:
```typescript
const pattern = /^[a-f0-9]{8}-[a-f0-9]{4}-4[a-f0-9]{3}-[89ab][a-f0-9]{3}-[a-f0-9]{12}$/;

function isValidRWPUuid(s: string): boolean {
  return pattern.test(s);
}

// Usage
assert(isValidRWPUuid('550e8400-e29b-41d4-a716-446655440000'));
assert(!isValidRWPUuid('550E8400-E29B-41D4-A716-446655440000')); // uppercase
```

## Scope & Limits

### What RWP UUIDs Identify

- Workflow plan instances
- Request/intake documents
- Individual handoff artifacts
- Phase execution records
- File and artifact registries
- Audit trail entries

### Uniqueness Guarantees

- **Globally Unique**: Every RWP UUID is unique across all RWP implementations, globally
- **Collision Probability**: Negligible for all practical purposes (~1 in 5.3 × 10^36 with UUIDv4)
- **No Coordination Required**: No centralized registry needed; implementers can generate UUIDs independently

### Non-Unique Identifiers

For quick human reference, RWP uses shorter, non-unique identifiers:

- **Phase IDs**: P-01, P-02, ..., P-99, P-01-A, P-01-B, P-01-C (tied to a plan)
- **Intake IDs**: INT-0001, INT-0002, ... (tied to a team/organization)
- **File IDs**: FILE-001, FILE-002, ... (tied to a manifest)
- **Pain Point IDs**: PP-01, PP-02, ... (tied to an intake)

These are designed for human readability within a single context (plan, intake, manifest). For cross-context identification, use UUIDs.

## Backward Compatibility

RWP v0.25.1 supports both:

1. **UUID-only identification** (recommended for new implementations)
2. **Human-readable IDs with optional UUID** (recommended for human-facing tools)

Tools may store both forms:

```yaml
id: INT-0001                                    # Human-readable
uuid: a3bb189e-8bf9-4067-b30c-28dde98ccc7f    # Machine-readable (optional in v1.0)
```

Future versions may require UUIDs for all artifacts; tools should prepare for this transition.

## Security Considerations

### Cryptographic Strength

RWP UUIDs use UUIDv4 (random), which provides 122 bits of entropy. This is sufficient for:

- **Deduplication**: No practical collision risk in any single organization
- **Authentication**: Not recommended for security tokens; use dedicated crypto for that
- **Audit Trails**: Sufficient for immutable artifact tracking

### What UUIDs Do NOT Provide

- **Confidentiality**: UUIDs are not encrypted; don't encode sensitive data
- **Authentication**: UUIDs can be predicted if the CSPRNG is weak; use crypto keys for auth
- **Integrity**: UUIDs don't verify data hasn't been modified; use HMAC/signatures for that

## References

- **RFC 4122**: A Universally Unique IDentifier (UUID) URN Namespace - https://tools.ietf.org/html/rfc4122
- **Python uuid Module**: https://docs.python.org/3/library/uuid.html
- **JavaScript uuid Package**: https://github.com/uuidjs/uuid
- **Go uuid Package**: https://github.com/google/uuid

## Version History

| Version | Date | Changes |
|---------|------|---------|
| 0.25.1 | 2026-03-04 | Initial specification; UUIDv4 required |

---

*Specification produced by YAKKL® - https://yakkl.com*
*RWP Reference Implementation: YAKKL Meridian - https://meridian.yakkl.com*
