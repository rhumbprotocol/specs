/**
 * Tests for RWP UUID Generator
 */

import {
  generateRWPUuid,
  validateRWPUuid,
  batchGenerateWithEntropyCheck,
  batchGenerateWithUniquenessCheck,
  generateBatchRWPUuids,
  formatUuidForDisplay,
  extractUuid,
} from './uuid-generator';

describe('UUID Generator', () => {
  describe('generateRWPUuid', () => {
    test('generates valid UUIDs', () => {
      const uuid = generateRWPUuid();
      expect(validateRWPUuid(uuid)).toBe(true);
    });

    test('generates lowercase UUIDs', () => {
      const uuid = generateRWPUuid();
      expect(uuid).toBe(uuid.toLowerCase());
    });

    test('generates UUIDs in canonical format', () => {
      const uuid = generateRWPUuid();
      const uuidRegex = /^[0-9a-f]{8}-[0-9a-f]{4}-4[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$/;
      expect(uuid).toMatch(uuidRegex);
    });

    test('generates UUIDs with 36 characters', () => {
      const uuid = generateRWPUuid();
      expect(uuid).toHaveLength(36);
    });

    test('has hyphens at correct positions', () => {
      const uuid = generateRWPUuid();
      expect(uuid[8]).toBe('-');
      expect(uuid[13]).toBe('-');
      expect(uuid[18]).toBe('-');
      expect(uuid[23]).toBe('-');
    });

    test('has version 4 indicator at position 14', () => {
      const uuid = generateRWPUuid();
      expect(uuid[14]).toBe('4');
    });
  });

  describe('validateRWPUuid', () => {
    test('validates correct UUIDs', () => {
      const uuid = generateRWPUuid();
      expect(validateRWPUuid(uuid)).toBe(true);
    });

    test('rejects uppercase UUIDs', () => {
      const uuid = generateRWPUuid();
      expect(validateRWPUuid(uuid.toUpperCase())).toBe(false);
    });

    test('rejects UUIDs without hyphens', () => {
      expect(validateRWPUuid('550e8400e29b41d4a716446655440000')).toBe(false);
    });

    test('rejects UUIDs with braces', () => {
      expect(validateRWPUuid('{550e8400-e29b-41d4-a716-446655440000}')).toBe(false);
    });

    test('rejects UUIDs with wrong version', () => {
      // UUIDv1: xxxxxxxx-xxxx-1xxx-yxxx-xxxxxxxxxxxx
      expect(validateRWPUuid('550e8400-e29b-11d4-a716-446655440000')).toBe(false);
    });

    test('rejects UUIDs with invalid variant', () => {
      // Invalid variant (should be 8, 9, a, or b)
      expect(validateRWPUuid('550e8400-e29b-41d4-c716-446655440000')).toBe(false);
    });

    test('rejects empty strings', () => {
      expect(validateRWPUuid('')).toBe(false);
    });

    test('rejects null/undefined', () => {
      expect(validateRWPUuid(null as any)).toBe(false);
      expect(validateRWPUuid(undefined as any)).toBe(false);
    });

    test('rejects partial UUIDs', () => {
      expect(validateRWPUuid('550e8400-e29b-41d4-a716')).toBe(false);
    });
  });

  describe('batchGenerateWithEntropyCheck', () => {
    test('generates requested number of UUIDs', async () => {
      const [uuids] = await batchGenerateWithEntropyCheck(100);
      expect(uuids).toHaveLength(100);
    });

    test('returns chi-squared statistic', async () => {
      const [, chiSquared] = await batchGenerateWithEntropyCheck(100);
      expect(typeof chiSquared).toBe('number');
      expect(chiSquared).toBeGreaterThan(0);
    });

    test('passes entropy test for 1000 UUIDs', async () => {
      // Should not throw
      const [uuids, chiSquared] = await batchGenerateWithEntropyCheck(1000);
      expect(uuids).toHaveLength(1000);
      // Critical value for df=1, α=0.05 is 3.841
      expect(chiSquared).toBeLessThan(3.841);
    });

    test('all generated UUIDs are valid', async () => {
      const [uuids] = await batchGenerateWithEntropyCheck(100);
      for (const uuid of uuids) {
        expect(validateRWPUuid(uuid)).toBe(true);
      }
    });

    test('default count is 1000', async () => {
      const [uuids] = await batchGenerateWithEntropyCheck();
      expect(uuids).toHaveLength(1000);
    });
  });

  describe('batchGenerateWithUniquenessCheck', () => {
    test('generates requested number of unique UUIDs', () => {
      const [uuids, ratio] = batchGenerateWithUniquenessCheck(100);
      expect(uuids).toHaveLength(100);
      expect(ratio).toBe(1.0); // 100% unique
    });

    test('returns uniqueness ratio', () => {
      const [, ratio] = batchGenerateWithUniquenessCheck(100);
      expect(typeof ratio).toBe('number');
      expect(ratio).toBeGreaterThan(0);
      expect(ratio).toBeLessThanOrEqual(1);
    });

    test('detects duplicate UUIDs', () => {
      // This should never actually happen with cryptographic RNG
      // But the function is designed to detect it if it does
      const [uuids] = batchGenerateWithUniquenessCheck(1000);
      const seen = new Set(uuids);
      expect(seen.size).toBe(uuids.length); // No duplicates
    });

    test('all generated UUIDs are valid', () => {
      const [uuids] = batchGenerateWithUniquenessCheck(100);
      for (const uuid of uuids) {
        expect(validateRWPUuid(uuid)).toBe(true);
      }
    });
  });

  describe('generateBatchRWPUuids', () => {
    test('generates batch of UUIDs', () => {
      const uuids = generateBatchRWPUuids(50);
      expect(uuids).toHaveLength(50);
    });

    test('all generated UUIDs are valid', () => {
      const uuids = generateBatchRWPUuids(50);
      for (const uuid of uuids) {
        expect(validateRWPUuid(uuid)).toBe(true);
      }
    });

    test('generates unique UUIDs', () => {
      const uuids = generateBatchRWPUuids(100);
      const seen = new Set(uuids);
      expect(seen.size).toBe(100);
    });
  });

  describe('formatUuidForDisplay', () => {
    test('formats UUID without prefix', () => {
      const uuid = generateRWPUuid();
      const formatted = formatUuidForDisplay(uuid);
      expect(formatted).toBe(uuid);
    });

    test('formats UUID with prefix', () => {
      const uuid = generateRWPUuid();
      const formatted = formatUuidForDisplay(uuid, 'plan-');
      expect(formatted).toBe(`plan-${uuid}`);
    });

    test('rejects invalid UUIDs', () => {
      expect(() => formatUuidForDisplay('invalid-uuid')).toThrow();
    });
  });

  describe('extractUuid', () => {
    test('extracts UUID from formatted string', () => {
      const uuid = generateRWPUuid();
      const formatted = `plan-${uuid}-suffix`;
      const extracted = extractUuid(formatted);
      expect(extracted).toBe(uuid);
    });

    test('extracts UUID in different positions', () => {
      const uuid = generateRWPUuid();
      const formatted = `prefix-${uuid}`;
      const extracted = extractUuid(formatted);
      expect(extracted).toBe(uuid);
    });

    test('throws on no UUID found', () => {
      expect(() => extractUuid('no-uuid-here')).toThrow('No UUID found in');
    });

    test('is case-insensitive for extraction', () => {
      const uuid = generateRWPUuid();
      const formatted = uuid.toUpperCase();
      const extracted = extractUuid(formatted);
      expect(extracted).toBe(uuid.toLowerCase());
    });
  });

  describe('Collision resistance (statistical)', () => {
    test('generates 10000 UUIDs with zero collisions', () => {
      const uuids = generateBatchRWPUuids(10000);
      const seen = new Set(uuids);
      expect(seen.size).toBe(10000);
    });

    test('probability of collision is astronomically low', () => {
      // With 1 million UUIDs, collision probability is still ~0.00001%
      // This test just validates the function can handle high counts
      const uuids = generateBatchRWPUuids(1000);
      expect(uuids.length).toBe(1000);
    });
  });
});
