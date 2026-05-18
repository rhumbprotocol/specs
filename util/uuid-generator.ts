/**
 * RWP UUID Generator - TypeScript Reference Implementation
 *
 * Provides cryptographically secure UUID generation and validation
 * for the Rhumb Workflow Protocol (RWP).
 */

import { randomUUID, randomBytes } from 'crypto';

/**
 * Generate a cryptographically secure RWP UUID.
 *
 * Uses Node.js crypto.randomUUID() which calls OpenSSL's RAND_bytes()
 * on Unix-like systems or CryptGenRandom() on Windows.
 *
 * @returns UUIDv4 in canonical lowercase format (xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx)
 * @throws Error if CSPRNG fails
 */
export function generateRWPUuid(): string {
  try {
    const uuid = randomUUID();
    return uuid.toLowerCase();
  } catch (error) {
    throw new Error(
      `UUID generation failed: ${error instanceof Error ? error.message : String(error)}`
    );
  }
}

/**
 * Validate a string as a compliant RWP UUID.
 *
 * Checks:
 * - Format matches UUIDv4 specification
 * - Version byte is 4
 * - Variant bits match RFC 4122
 * - String is in canonical lowercase form with hyphens
 *
 * @param uuidStr String to validate
 * @returns True if valid RWP UUID, false otherwise
 */
export function validateRWPUuid(uuidStr: string): boolean {
  if (!uuidStr || typeof uuidStr !== 'string') {
    return false;
  }

  // UUIDv4 format: xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx
  // where y is one of [8, 9, a, b]
  const uuidRegex = /^[0-9a-f]{8}-[0-9a-f]{4}-4[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$/;

  if (!uuidRegex.test(uuidStr)) {
    return false;
  }

  // Verify lowercase (canonical form)
  return uuidStr === uuidStr.toLowerCase();
}

/**
 * Batch generate UUIDs and verify entropy quality.
 *
 * Performs chi-squared test on bit distribution to verify CSPRNG quality.
 * Uses the entropy test from the RWP UUID Generation specification.
 *
 * @param count Number of UUIDs to generate and test (default: 1000)
 * @returns Tuple of [uuids, chi_squared_statistic]
 * @throws Error if entropy test fails (χ² > critical value)
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

    // Convert hex to bits and count distribution
    const hexDigits = uuid.replace(/-/g, '');
    for (const char of hexDigits) {
      const byte = parseInt(char, 16);
      // Count bits in 4-bit hex digit
      for (let j = 0; j < 4; j++) {
        if (byte & (1 << j)) {
          bitCounts[1]++; // ones
        } else {
          bitCounts[0]++; // zeros
        }
      }
    }
  }

  // Chi-squared test: χ² = Σ((observed - expected)² / expected)
  // For df=1, α=0.05, critical value ≈ 3.841
  const expected = (count * 128) / 2; // 32 hex chars × 4 bits = 128 bits total
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

/**
 * Batch generate UUIDs and check for uniqueness.
 *
 * Generates multiple UUIDs and verifies no duplicates exist.
 * For cryptographically generated UUIDs, duplicates should never occur.
 *
 * @param count Number of UUIDs to generate
 * @returns Tuple of [uuids, uniqueness_ratio]
 * @throws Error if any duplicates are detected
 */
export function batchGenerateWithUniquenessCheck(count: number): [string[], number] {
  const uuids: string[] = [];
  const seen = new Set<string>();

  for (let i = 0; i < count; i++) {
    const uuid = generateRWPUuid();

    if (seen.has(uuid)) {
      throw new Error(
        `Duplicate UUID detected after ${i + 1} generations: ${uuid}. ` +
          `CSPRNG may be compromised.`
      );
    }

    uuids.push(uuid);
    seen.add(uuid);
  }

  const uniquenessRatio = seen.size / count;
  return [uuids, uniquenessRatio];
}

/**
 * Generate multiple UUIDs (simple batch).
 *
 * @param count Number of UUIDs to generate
 * @returns Array of generated UUIDs
 */
export function generateBatchRWPUuids(count: number): string[] {
  const uuids: string[] = [];
  for (let i = 0; i < count; i++) {
    uuids.push(generateRWPUuid());
  }
  return uuids;
}

/**
 * Format UUID for display (with optional prefix).
 *
 * @param uuid UUID string
 * @param prefix Optional prefix (e.g., "plan-" → "plan-xxxxxxxx-...")
 * @returns Formatted UUID string
 */
export function formatUuidForDisplay(uuid: string, prefix?: string): string {
  if (!validateRWPUuid(uuid)) {
    throw new Error(`Invalid UUID: ${uuid}`);
  }

  if (prefix) {
    return `${prefix}${uuid}`;
  }

  return uuid;
}

/**
 * Extract UUID from formatted string.
 *
 * @param formatted Formatted UUID string (e.g., "plan-xxxxxxxx-...")
 * @returns UUID string without prefix
 */
export function extractUuid(formatted: string): string {
  // Try to match UUID pattern anywhere in the string
  const uuidMatch = formatted.match(/[0-9a-f]{8}-[0-9a-f]{4}-4[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}/i);
  if (!uuidMatch) {
    throw new Error(`No UUID found in: ${formatted}`);
  }
  return uuidMatch[0].toLowerCase();
}
