# RWP UUID Generation Specification

> Part of the [Rhumb Workflow Protocol (RWP)](../docs/PROTOCOL.md) - see also: [UUID Format](./uuid.format.md), [Implementation Guide](./implementation-guide.md)

## Overview

This specification details the cryptographic requirements and implementation strategies for generating compliant Rhumb Workflow Protocol (RWP) UUIDs. While the UUID format itself is defined in `uuid.format.md`, this document focuses on **how to generate** valid UUIDs with proper entropy validation, collision detection, and language-specific guidance.

## Cryptographic Requirements

### Entropy Source

All UUIDs MUST be generated using a cryptographically secure pseudorandom number generator (CSPRNG):

- **Minimum Entropy**: 128 bits from the CSPRNG
- **Quality Standard**: NIST SP 800-90A compliant
- **Acceptable Sources**:
  - Linux/macOS: `/dev/urandom`
  - Node.js: `crypto.getRandomValues()` or `crypto.randomBytes()`
  - Python: `secrets` module
  - Rust: `rand::rngs::OsRng`
  - Go: `crypto/rand`

### Entropy Validation

After generating a UUID from a CSPRNG, implementations SHOULD verify entropy quality using statistical tests:

#### Chi-Squared Test (Recommended)

Collect N generated UUIDs (minimum 1000) and perform a chi-squared test on the bit distribution:

```
Null hypothesis (H0): Bits are uniformly distributed
Significance level: α = 0.05
Critical value: χ²(255) ≈ 293.0

If χ² < 293.0: Accept H0 → Entropy is acceptable
If χ² ≥ 293.0: Reject H0 → CSPRNG may be weak
```

#### Practical Implementation

For each UUID generated:
1. Convert to 128-bit binary representation
2. Count occurrence of each bit value (0 and 1)
3. Expected: ~64 zeros, ~64 ones
4. If deviation > 10%: Log warning and consider CSPRNG health check

## Language-Specific Implementations

### Python 3.6+

```python
import uuid
from secrets import SystemRandom
import hashlib

def generate_rwp_uuid() -> str:
    """Generate a cryptographically secure RWP UUID.

    Uses the secrets module which wraps os.urandom on Unix-like systems
    and CryptGenRandom on Windows.

    Returns:
        str: UUIDv4 in canonical lowercase format (xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx)

    Raises:
        RuntimeError: If CSPRNG fails to provide sufficient entropy
    """
    try:
        # uuid.uuid4() internally uses os.urandom() on all platforms
        uuid_obj = uuid.uuid4()
        # Return canonical lowercase format
        return str(uuid_obj).lower()
    except Exception as e:
        raise RuntimeError(f"UUID generation failed: {e}")


def validate_rwp_uuid(uuid_str: str) -> bool:
    """Validate a string as a compliant RWP UUID.

    Args:
        uuid_str: String to validate

    Returns:
        bool: True if valid RWP UUID, False otherwise
    """
    if not uuid_str:
        return False

    try:
        uuid_obj = uuid.UUID(uuid_str)
        # Verify it's UUIDv4
        if uuid_obj.version != 4:
            return False
        # Verify canonical form (lowercase with hyphens)
        canonical = str(uuid_obj).lower()
        return uuid_str == canonical
    except (ValueError, AttributeError):
        return False


def batch_generate_with_entropy_check(count: int = 1000) -> tuple[list[str], float]:
    """Generate multiple UUIDs and verify entropy quality.

    Uses chi-squared test on bit distribution to verify CSPRNG quality.

    Args:
        count: Number of UUIDs to generate and test

    Returns:
        Tuple of (uuid_list, chi_squared_statistic)

    Raises:
        RuntimeError: If chi-squared statistic indicates weak entropy
    """
    uuids = []
    bit_counts = [0, 0]  # [zeros, ones]

    # Generate UUIDs and count bit distribution
    for _ in range(count):
        uuid_str = generate_rwp_uuid()
        uuids.append(uuid_str)

        # Convert to bits and count
        uuid_obj = uuid.UUID(uuid_str)
        uuid_int = uuid_obj.int
        for i in range(128):
            if uuid_int & (1 << i):
                bit_counts[1] += 1
            else:
                bit_counts[0] += 1

    # Chi-squared test
    expected = count * 128 / 2  # Expected zeros and ones per category
    chi_squared = sum((observed - expected) ** 2 / expected for observed in bit_counts)

    # Critical value for α=0.05, df=1
    critical_value = 3.841

    if chi_squared > critical_value:
        raise RuntimeError(
            f"Entropy test failed: χ² = {chi_squared:.2f} > {critical_value} (p < 0.05). "
            "CSPRNG may be weak."
        )

    return uuids, chi_squared
```

### TypeScript/Node.js

```typescript
import { randomUUID, randomBytes } from 'crypto';

/**
 * Generate a cryptographically secure RWP UUID.
 *
 * Uses Node.js crypto.randomUUID() which calls OpenSSL's RAND_bytes()
 * on Unix-like systems or CryptGenRandom() on Windows.
 *
 * @returns UUIDv4 in canonical lowercase format
 * @throws Error if CSPRNG fails
 */
export function generateRWPUuid(): string {
  try {
    const uuid = randomUUID();
    return uuid.toLowerCase();
  } catch (error) {
    throw new Error(`UUID generation failed: ${error instanceof Error ? error.message : String(error)}`);
  }
}

/**
 * Validate a string as a compliant RWP UUID.
 *
 * @param uuidStr String to validate
 * @returns True if valid RWP UUID
 */
export function validateRWPUuid(uuidStr: string): boolean {
  if (!uuidStr || typeof uuidStr !== 'string') {
    return false;
  }

  // UUIDv4 format: xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx
  const uuidRegex = /^[0-9a-f]{8}-[0-9a-f]{4}-4[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$/i;
  if (!uuidRegex.test(uuidStr)) {
    return false;
  }

  // Verify lowercase
  return uuidStr === uuidStr.toLowerCase();
}

/**
 * Batch generate UUIDs with entropy verification.
 *
 * Performs chi-squared test on bit distribution to verify CSPRNG quality.
 *
 * @param count Number of UUIDs to generate
 * @returns Tuple of [uuids, chi_squared_statistic]
 * @throws Error if entropy test fails
 */
export async function batchGenerateWithEntropyCheck(
  count: number = 1000
): Promise<[string[], number]> {
  const uuids: string[] = [];
  const bitCounts = [0, 0]; // [zeros, ones]

  // Generate UUIDs and count bit distribution
  for (let i = 0; i < count; i++) {
    const uuid = generateRWPUuid();
    uuids.push(uuid);

    // Convert hex to bits
    const hexDigits = uuid.replace(/-/g, '');
    for (const char of hexDigits) {
      const byte = parseInt(char, 16);
      for (let j = 0; j < 4; j++) {
        if (byte & (1 << j)) {
          bitCounts[1]++;
        } else {
          bitCounts[0]++;
        }
      }
    }
  }

  // Chi-squared test (df=1, α=0.05, critical value ≈ 3.841)
  const expected = (count * 128) / 2;
  const chiSquared = bitCounts.reduce(
    (sum, observed) => sum + Math.pow(observed - expected, 2) / expected,
    0
  );

  const criticalValue = 3.841;
  if (chiSquared > criticalValue) {
    throw new Error(
      `Entropy test failed: χ² = ${chiSquared.toFixed(2)} > ${criticalValue} (p < 0.05). ` +
        `CSPRNG may be weak.`
    );
  }

  return [uuids, chiSquared];
}
```

### Rust

```rust
use uuid::Uuid;

/// Generate a cryptographically secure RWP UUID.
///
/// Uses `OsRng` from the `rand` crate, which accesses `/dev/urandom` on Unix
/// and CryptGenRandom on Windows.
///
/// # Errors
/// Returns error if CSPRNG fails (rare in practice).
pub fn generate_rwp_uuid() -> Result<String, Box<dyn std::error::Error>> {
    let uuid = Uuid::new_v4();
    Ok(uuid.to_string().to_lowercase())
}

/// Validate a string as a compliant RWP UUID.
pub fn validate_rwp_uuid(uuid_str: &str) -> bool {
    if uuid_str.is_empty() {
        return false;
    }

    // UUIDv4 format regex: xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx
    if let Ok(uuid) = Uuid::parse_str(uuid_str) {
        uuid.get_version() == Some(uuid::Version::Random) && uuid_str == uuid_str.to_lowercase()
    } else {
        false
    }
}

/// Batch generate UUIDs with entropy verification.
///
/// Performs chi-squared test on bit distribution.
pub fn batch_generate_with_entropy_check(
    count: usize,
) -> Result<(Vec<String>, f64), Box<dyn std::error::Error>> {
    let mut uuids = Vec::with_capacity(count);
    let mut bit_counts = [0u32; 2]; // [zeros, ones]

    for _ in 0..count {
        let uuid = Uuid::new_v4();
        let uuid_str = uuid.to_string().to_lowercase();
        uuids.push(uuid_str);

        // Count bit distribution
        let bytes = uuid.as_bytes();
        for byte in bytes {
            for i in 0..8 {
                if byte & (1 << i) != 0 {
                    bit_counts[1] += 1;
                } else {
                    bit_counts[0] += 1;
                }
            }
        }
    }

    // Chi-squared test
    let expected = (count as f64) * 128.0 / 2.0;
    let chi_squared: f64 = bit_counts
        .iter()
        .map(|&observed| {
            let obs_f = observed as f64;
            (obs_f - expected).powi(2) / expected
        })
        .sum();

    const CRITICAL_VALUE: f64 = 3.841; // df=1, α=0.05

    if chi_squared > CRITICAL_VALUE {
        return Err(format!(
            "Entropy test failed: χ² = {:.2} > {:.2} (p < 0.05). CSPRNG may be weak.",
            chi_squared, CRITICAL_VALUE
        )
        .into());
    }

    Ok((uuids, chi_squared))
}
```

### Go

```go
package rwp

import (
	"crypto/rand"
	"fmt"
	"strings"

	"github.com/google/uuid"
)

// GenerateRWPUUID generates a cryptographically secure RWP UUID.
//
// Uses Go's crypto/rand via the uuid package, which accesses /dev/urandom on Unix
// and CryptGenRandom on Windows.
func GenerateRWPUUID() (string, error) {
	u, err := uuid.NewRandom()
	if err != nil {
		return "", fmt.Errorf("UUID generation failed: %w", err)
	}
	return strings.ToLower(u.String()), nil
}

// ValidateRWPUUID validates a string as a compliant RWP UUID.
func ValidateRWPUUID(uuidStr string) bool {
	if uuidStr == "" {
		return false
	}

	u, err := uuid.Parse(uuidStr)
	if err != nil {
		return false
	}

	// Verify it's UUIDv4 and in canonical lowercase form
	return u.Version() == 4 && uuidStr == strings.ToLower(u.String())
}

// BatchGenerateWithEntropyCheck generates multiple UUIDs and verifies entropy.
func BatchGenerateWithEntropyCheck(count int) ([]string, float64, error) {
	uuids := make([]string, 0, count)
	bitCounts := [2]int{0, 0} // [zeros, ones]

	for i := 0; i < count; i++ {
		uuid, err := GenerateRWPUUID()
		if err != nil {
			return nil, 0, err
		}
		uuids = append(uuids, uuid)

		// Count bit distribution
		u, _ := uuid.Parse(uuid)
		bytes := u[:]
		for _, b := range bytes {
			for j := 0; j < 8; j++ {
				if (b & (1 << j)) != 0 {
					bitCounts[1]++
				} else {
					bitCounts[0]++
				}
			}
		}
	}

	// Chi-squared test
	expected := float64(count*128) / 2.0
	chiSquared := 0.0
	for _, observed := range bitCounts {
		obs := float64(observed)
		chiSquared += (obs - expected) * (obs - expected) / expected
	}

	const criticalValue = 3.841 // df=1, α=0.05

	if chiSquared > criticalValue {
		return nil, chiSquared, fmt.Errorf(
			"entropy test failed: χ² = %.2f > %.2f (p < 0.05). CSPRNG may be weak",
			chiSquared, criticalValue,
		)
	}

	return uuids, chiSquared, nil
}
```

## Collision Detection Strategies

### Expected Collision Rate

For UUIDv4 with proper CSPRNG:

- **1,000 UUIDs**: Collision probability ≈ 0 (1 in 10^36)
- **1,000,000 UUIDs**: Collision probability ≈ 0.0001% (1 in 10^30)
- **1 billion UUIDs**: Collision probability ≈ 0.00001% (1 in 10^24)

In practice, collisions with cryptographically generated UUIDs are astronomically unlikely and should never be observed.

### Testing for Collisions

```python
# Python: Simple uniqueness test
from collections import Counter

uuids = [generate_rwp_uuid() for _ in range(10000)]
counts = Counter(uuids)
duplicates = [uuid for uuid, count in counts.items() if count > 1]

if duplicates:
    print(f"WARNING: Found {len(duplicates)} duplicate UUIDs!")
else:
    print(f"✓ Generated {len(uuids)} UUIDs with zero collisions")
```

```typescript
// TypeScript: Uniqueness test
const uuids = new Set<string>();
for (let i = 0; i < 10000; i++) {
  const uuid = generateRWPUuid();
  if (uuids.has(uuid)) {
    console.error(`Collision detected: ${uuid}`);
    process.exit(1);
  }
  uuids.add(uuid);
}
console.log(`✓ Generated ${uuids.size} UUIDs with zero collisions`);
```

## Performance Benchmarks

### Generation Speed

Typical performance on modern systems (single-threaded):

| Language | 1,000 UUIDs | 100,000 UUIDs | Notes |
|----------|------------|--------------|-------|
| Python | ~5ms | ~0.5s | Using `uuid.uuid4()` |
| TypeScript | ~3ms | ~0.3s | Using `randomUUID()` |
| Rust | ~2ms | ~0.2s | Using `Uuid::new_v4()` |
| Go | ~4ms | ~0.4s | Using `uuid.NewRandom()` |

For high-throughput scenarios, consider:
1. **Batch generation**: Generate in batches to amortize overhead
2. **Threading**: Parallelize UUID generation across CPU cores
3. **Pre-generation**: Pre-generate UUIDs and cache them

### Entropy Validation Overhead

Chi-squared test for 1,000 UUIDs:
- Python: ~5ms additional
- TypeScript: ~3ms additional
- Rust: ~2ms additional
- Go: ~4ms additional

This overhead is negligible and recommended for production systems.

## Security Considerations

### CSPRNG Availability

Before generating UUIDs in security-critical contexts:

```python
# Python: Verify CSPRNG availability
import secrets
try:
    random_bytes = secrets.token_bytes(16)
    print("✓ CSPRNG available")
except Exception as e:
    print(f"✗ CSPRNG unavailable: {e}")
```

```typescript
// TypeScript: Verify CSPRNG availability
import { randomBytes } from 'crypto';
try {
  randomBytes(16);
  console.log('✓ CSPRNG available');
} catch (e) {
  console.error(`✗ CSPRNG unavailable: ${e}`);
}
```

### Weak CSPRNG Detection

If entropy tests consistently fail:
1. Check system entropy pool (e.g., `cat /proc/sys/kernel/random/entropy_avail` on Linux)
2. Verify CSPRNG algorithm (should be ChaCha20 or AES-CTR minimum)
3. Check for virtualization issues (VMs may have weak entropy)
4. Seed additional entropy from hardware devices if available

## Integration Examples

### Example: Generate UUID for New Plan

```python
from datetime import datetime

def create_new_plan(title: str, author: str) -> dict:
    """Create a new RWP plan with generated UUID."""
    plan_id = generate_rwp_uuid()

    return {
        'id': plan_id,
        'title': title,
        'author': author,
        'created': datetime.utcnow().isoformat() + 'Z',
        'status': 'draft'
    }

# Usage
plan = create_new_plan('Build Analytics', 'alice@example.com')
print(f"Created plan: {plan['id']}")
```

### Example: Validate UUID Before Storage

```typescript
async function storePlanArtifact(
  planUuid: string,
  artifact: PlanArtifact
): Promise<void> {
  if (!validateRWPUuid(planUuid)) {
    throw new Error(`Invalid plan UUID: ${planUuid}`);
  }

  await db.plans.insert({
    id: planUuid,
    ...artifact,
  });
}
```

## Summary

- **Always use CSPRNG**: Never use `Math.random()`, `rand()`, or weak PRNGs
- **Validate format**: Check canonical lowercase form with hyphens
- **Test entropy**: Run chi-squared test in production systems
- **Check collisions**: Verify uniqueness in safety-critical scenarios
- **Document source**: Record which CSPRNG was used for auditing

---

Produced:
  - when: 2026-03-04T04:15:00Z
  - by: YAKKL® Meridian™- https://meridian.yakkl.com
  - copyright: Copyright © 2026 YAKKL Inc. All Rights Reserved.
