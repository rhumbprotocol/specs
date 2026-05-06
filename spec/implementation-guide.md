# RWP Implementation Guide

> Part of the [Rhumb Workflow Protocol (RWP)](../docs/PROTOCOL.md) - see also: [UUID Format](./uuid.format.md), [Sequence Parser](./sequence-parser.md), [Conformance Levels](./conformance-levels.md)

## Overview

This guide provides best practices and patterns for implementing RWP UUID generation and sequence parsing in different programming languages. Reference implementations are available in TypeScript, Python, Rust, and Go in the `util/` directory.

## General Principles

### 1. CSPRNG Quality First

**Never compromise on entropy quality:**

- Use OS-level CSPRNG only: `/dev/urandom` (Unix), `CryptGenRandom` (Windows)
- Never use `Math.random()`, `rand()`, or weak PRNGs
- Test entropy with statistical methods (chi-squared test recommended)
- Log CSPRNG failures with alerts

### 2. Error Handling

All implementations should handle these error cases:

```
├── CSPRNG Unavailable
│   └── Log error, attempt fallback, fail if no fallback
├── Weak Entropy Detected
│   └── Log warning, reject UUID, retry
├── Parse Errors
│   └── Provide position info and helpful context
├── Invalid Input
│   └── Validate format before processing
└── Resource Limits
    └── Implement timeouts for large operations
```

### 3. Testing Strategy

Minimum test coverage:

1. **Happy path**: Valid inputs produce valid outputs
2. **Format validation**: Reject malformed UUIDs/sequences
3. **Entropy verification**: Statistical tests for CSPRNG quality
4. **Collision detection**: Verify uniqueness in batches
5. **Edge cases**: Boundary conditions, large inputs
6. **Error cases**: All error conditions with helpful messages

### 4. Performance Considerations

#### UUID Generation

Typical single UUID generation:
- **Python**: 5-10 µs
- **TypeScript**: 3-5 µs
- **Rust**: 2-3 µs
- **Go**: 4-8 µs

For high-throughput scenarios (>10,000 UUIDs/second):
1. Use batch generation with threading
2. Cache entropy sources where safe
3. Pre-generate UUIDs if workload allows

#### Sequence Parsing

Typical parse times (ABNF-compliant input):
- **Python**: 50-100 µs
- **TypeScript**: 20-50 µs
- **Rust**: 10-20 µs
- **Go**: 30-60 µs

Optimization strategies:
1. Cache compiled parsers
2. Use memoization for repeated sequences
3. Compile patterns at startup

### 5. Validation Patterns

#### UUID Validation

All implementations should check:

```
Input → Type Check → Format Check → Checksum (if applicable) → Return Boolean
```

**Never throw on invalid UUID** in validation functions. Return boolean.

#### Sequence Validation

All implementations should check:

```
Input → Tokenize → Parse → Semantic Check → Return AST or Error
```

**Do throw on parsing errors** with position information.

## Language-Specific Guidance

### Python

**Best Practices:**

```python
# ✓ Use secrets module for CSPRNG
from secrets import token_bytes
import uuid

# ✓ Handle exceptions explicitly
try:
    uuid_obj = uuid.uuid4()
except Exception as e:
    logger.error(f"UUID generation failed: {e}")
    raise

# ✓ Use dataclasses for type safety
from dataclasses import dataclass

@dataclass
class PhaseId:
    phase_num: int
    sub_phase: str | None = None

# ✓ Use Protocol for parser interface
from typing import Protocol

class Parser(Protocol):
    def parse(self, input: str) -> ASTNode: ...

# ✗ Avoid global state in parsers
# ✗ Avoid unchecked type coercion
```

**Standard Library Dependencies:**

```
uuid        - for UUID parsing and validation
secrets     - for CSPRNG (Python 3.6+)
typing      - for type hints
re          - for regex patterns
dataclasses - for AST nodes (Python 3.7+)
```

**Testing:**

```python
import pytest

def test_generate_uuid():
    uuid_str = generate_rwp_uuid()
    assert validate_rwp_uuid(uuid_str)
    assert uuid_str == uuid_str.lower()

@pytest.mark.parametrize("invalid", [
    "invalid",
    "550e8400-e29b-41d4-a716-446655440000".upper(),
])
def test_reject_invalid_uuid(invalid):
    assert not validate_rwp_uuid(invalid)
```

### TypeScript/Node.js

**Best Practices:**

```typescript
// ✓ Use crypto module from Node.js
import { randomUUID, randomBytes } from 'crypto';

// ✓ Type all inputs and outputs
function generateRWPUuid(): string {
  return randomUUID().toLowerCase();
}

// ✓ Use discriminated unions for AST
type ASTNode =
  | { kind: 'phase'; phaseNum: number; subPhase: string | null }
  | { kind: 'binop'; operator: 'AND' | 'OR'; left: ASTNode; right: ASTNode }
  | { kind: 'group'; expr: ASTNode };

// ✓ Use Map for efficiency
const uuidCache = new Map<string, boolean>();

// ✗ Avoid 'any' type
// ✗ Avoid implicit coercion
// ✗ Avoid mutable shared state
```

**Testing Framework:**

```typescript
import { describe, test, expect } from '@jest/globals';

describe('UUID Generator', () => {
  test('generates valid UUIDs', () => {
    const uuid = generateRWPUuid();
    expect(validateRWPUuid(uuid)).toBe(true);
  });
});
```

### Rust

**Best Practices:**

```rust
// ✓ Use uuid crate for UUIDs
use uuid::Uuid;

// ✓ Use enums for AST nodes
#[derive(Debug, Clone, PartialEq)]
pub enum ASTNode {
    PhaseId { phase_num: u32, sub_phase: Option<char> },
    BinaryOp { operator: BinaryOp, left: Box<ASTNode>, right: Box<ASTNode> },
    Group(Box<ASTNode>),
}

// ✓ Use Result for error handling
pub fn generate_rwp_uuid() -> Result<String, Box<dyn std::error::Error>> {
    Ok(Uuid::new_v4().to_string().to_lowercase())
}

// ✓ Use lifetime parameters for zero-copy parsing
fn parse_phase_id<'a>(input: &'a str) -> Result<(&'a str, PhaseId), ParseError> {
    // ...
}

// ✗ Avoid panics in library code
// ✗ Avoid unnecessary allocations
// ✗ Avoid mutable global state
```

**Testing:**

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_uuid() {
        let uuid = generate_rwp_uuid().unwrap();
        assert!(validate_rwp_uuid(&uuid));
    }

    #[test]
    fn test_entropy_check() {
        let (uuids, chi_squared) = batch_generate_with_entropy_check(1000).unwrap();
        assert_eq!(uuids.len(), 1000);
        assert!(chi_squared < 3.841); // Critical value
    }
}
```

### Go

**Best Practices:**

```go
// ✓ Use crypto/rand for CSPRNG
import "crypto/rand"

// ✓ Use interfaces for parser abstraction
type Parser interface {
    Parse(input string) (ASTNode, error)
}

// ✓ Use error wrapping (Go 1.13+)
if err := someFunc(); err != nil {
    return fmt.Errorf("operation failed: %w", err)
}

// ✓ Use context for timeouts
func generateWithContext(ctx context.Context) (string, error) {
    select {
    case <-ctx.Done():
        return "", ctx.Err()
    default:
        // generate UUID
    }
}

// ✗ Avoid panic() for recoverable errors
// ✗ Avoid unchecked type assertions
// ✗ Avoid global state
```

**Testing:**

```go
package rwp

import (
    "testing"
)

func TestGenerateUUID(t *testing.T) {
    uuid, err := GenerateRWPUUID()
    if err != nil {
        t.Fatalf("unexpected error: %v", err)
    }
    if !ValidateRWPUUID(uuid) {
        t.Errorf("invalid UUID: %s", uuid)
    }
}
```

## Architectural Patterns

### Parser Architecture

Recommended three-stage architecture:

```
┌─────────────────────────────────────────────────────┐
│ Input: "P-01, (P-02 + P-03)"                         │
└────────────────────┬────────────────────────────────┘
                     │
         ┌───────────▼──────────────┐
         │  Tokenizer               │
         │  (lexical analysis)      │
         │                          │
         │  Token stream:           │
         │  [PHASE(01), COMMA,      │
         │   LPAREN, PHASE(02), ... │
         │   EOF]                   │
         └───────────┬──────────────┘
                     │
         ┌───────────▼──────────────────┐
         │ Parser (recursive descent)   │
         │ (syntax analysis)            │
         │                              │
         │ Produces: AST (abstract      │
         │           syntax tree)       │
         └───────────┬──────────────────┘
                     │
         ┌───────────▼──────────────┐
         │ Evaluator                │
         │ (semantic analysis)      │
         │                          │
         │ Output: Execution order  │
         │ [Batch 1: [P-01],        │
         │  Batch 2: [P-02, P-03],] │
         └──────────┬───────────────┘
                    │
            ┌───────▼─────────┐
            │ Output: JSON or  │
            │ user-friendly    │
            │ format           │
            └──────────────────┘
```

### Error Handling Pattern

```python
# Define error types
class RWPError(Exception):
    """Base RWP error"""
    pass

class ParseError(RWPError):
    def __init__(self, message: str, position: int, input: str):
        super().__init__(
            f"{message}\n"
            f"  Input: {input}\n"
            f"  {' ' * position}^"
        )

class EntropyError(RWPError):
    def __init__(self, chi_squared: float, threshold: float):
        super().__init__(
            f"Entropy test failed: χ² = {chi_squared:.2f} > {threshold}"
        )

# Use in functions
def parse(input: str) -> ASTNode:
    try:
        tokens = tokenize(input)
        return parser.parse(tokens)
    except Exception as e:
        raise ParseError("failed to parse sequence", 0, input) from e
```

## Testing Best Practices

### Unit Tests

```python
# Test structure
def test_single_phase():
    """Verify single phase is parsed correctly."""
    ast = parse("P-01")
    assert ast.kind == "phase"
    assert ast.phase_num == 1

def test_sequential_phases():
    """Verify sequential phases create separate batches."""
    order = compute_order("P-01, P-02")
    assert len(order) == 2

def test_parallel_phases():
    """Verify parallel phases combine into single batch."""
    order = compute_order("P-01 + P-02")
    assert len(order) == 1
    assert len(order[0].phases) == 2
```

### Property-Based Testing

```python
from hypothesis import given, strategies as st

@given(st.integers(min_value=1, max_value=99))
def test_all_phase_numbers_valid(phase_num):
    """Test that any valid phase number can be parsed."""
    phase_str = f"P-{phase_num:02d}"
    ast = parse(phase_str)
    assert ast.phase_num == phase_num

@given(st.just(''))
def test_empty_input_fails(input_str):
    """Test that empty input raises error."""
    with pytest.raises(ParseError):
        parse(input_str)
```

### Performance Tests

```python
import timeit

def test_uuid_generation_performance():
    """Verify UUID generation meets performance targets."""
    def generate_1000():
        for _ in range(1000):
            generate_rwp_uuid()

    time_taken = timeit.timeit(generate_1000, number=1) / 1000
    assert time_taken < 10_000  # microseconds
```

## Integration Examples

### Example 1: Generate UUIDs for Workflow Artifacts

```python
from datetime import datetime

def create_workflow(title: str, author: str) -> dict:
    """Create a new workflow with unique UUID."""
    workflow_id = generate_rwp_uuid()

    return {
        'id': workflow_id,
        'title': title,
        'author': author,
        'created': datetime.utcnow().isoformat() + 'Z',
        'status': 'draft'
    }
```

### Example 2: Parse and Execute Sequence

```typescript
import { computeExecutionOrder } from '@rwp/sequence-parser';

async function executeWorkflow(sequence: string): Promise<void> {
  const order = computeExecutionOrder(sequence);

  for (const batch of order) {
    // Execute phases in parallel
    const promises = batch.phases.map(phase => executePhase(phase));
    await Promise.all(promises);
  }
}
```

### Example 3: Validate Artifacts

```rust
use uuid::Uuid;
use rwp_parser::parse_sequence;

pub fn validate_artifact(artifact: &Artifact) -> Result<(), Box<dyn std::error::Error>> {
    // Validate UUID
    Uuid::parse_str(&artifact.id)?;

    // Validate sequence if present
    if let Some(seq) = &artifact.sequence {
        let _ = parse_sequence(seq)?;
    }

    Ok(())
}
```

## Security Considerations

### CSPRNG Verification

Before deploying to production:

1. **Test CSPRNG availability** on all target platforms
2. **Verify entropy sources** (check /proc/sys/kernel/random/entropy_avail on Linux)
3. **Run statistical tests** (chi-squared, entropy tests)
4. **Monitor for failures** and log all CSPRNG errors

### Parser Security

Prevent parser denial-of-service:

```python
# Limit input size
MAX_SEQUENCE_LENGTH = 1000

def parse_with_limits(input: str) -> ASTNode:
    if len(input) > MAX_SEQUENCE_LENGTH:
        raise ParseError("input too long", 0, input)

    # Limit recursion depth
    parser = Parser(tokens, max_depth=100)
    return parser.parse()
```

### UUID Collision Monitoring

For high-volume systems:

```python
import logging

class UUIDCollisionDetector:
    def __init__(self, window_size: int = 10000):
        self.seen = set()
        self.window_size = window_size

    def check(self, uuid: str) -> None:
        if uuid in self.seen:
            logging.error(f"Collision detected: {uuid}")
            alert_security_team()

        self.seen.add(uuid)
        if len(self.seen) > self.window_size:
            self.seen.clear()
```

## Debugging Guide

### Common Issues

| Symptom | Cause | Solution |
|---------|-------|----------|
| Invalid UUIDs generated | Weak CSPRNG | Run entropy test, verify RNG source |
| Parser fails on valid input | Grammar mismatch | Compare against ABNF spec |
| Collision detected | CSPRNG broken | Investigate system entropy, restart service |
| Out of memory | Unbounded parser recursion | Add recursion depth limit |
| Slow UUID generation | System entropy pool empty | Check `/proc/sys/kernel/random/entropy_avail` |

### Debugging Checklist

When implementation fails:

1. **Verify CSPRNG** works standalone
2. **Test tokenizer** with known inputs
3. **Trace parser** with debug logging
4. **Check platform differences** (Windows vs Unix)
5. **Review recent changes** to system entropy source
6. **Compare against reference impl** in TypeScript

## Performance Optimization

### UUID Generation

```python
# Pre-generate batch
class UUIDPool:
    def __init__(self, size: int = 10000):
        self.pool = [generate_rwp_uuid() for _ in range(size)]
        self.index = 0

    def get(self) -> str:
        uuid = self.pool[self.index]
        self.pool[self.index] = generate_rwp_uuid()
        self.index = (self.index + 1) % len(self.pool)
        return uuid
```

### Sequence Parsing

```python
# Cache compiled parsers
@functools.lru_cache(maxsize=1000)
def parse_sequence_cached(input: str) -> ASTNode:
    return parse_sequence(input)

# Or precompile common sequences
COMMON_SEQUENCES = {
    'linear': parse_sequence('P-01, P-02, P-03'),
    'parallel': parse_sequence('P-01 + P-02 + P-03'),
}
```

## Maintenance & Updates

### Version Compatibility

RWP versions follow semantic versioning. When implementing:

1. **Store RWP version** in artifacts
2. **Support backward compatibility** for minor versions
3. **Document breaking changes** in changelog
4. **Test against multiple versions** before upgrade

### Updating Reference Implementations

When RWP specification changes:

1. Update grammar in `sequence.grammar`
2. Update TypeScript reference first
3. Verify all test cases pass
4. Update other language implementations
5. Update this guide with new patterns

---

Produced:
  - when: 2026-03-04T04:35:00Z
  - by: YAKKL® Meridian™- https://meridian.yakkl.com
  - copyright: Copyright © 2026 YAKKL Inc. All Rights Reserved.
