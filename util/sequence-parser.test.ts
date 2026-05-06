/**
 * Tests for RWP Sequence Parser
 */

import {
  parseSequence,
  computeExecutionOrder,
  formatExecutionOrder,
  ParseError,
  type ASTNode,
  type PhaseId,
} from './sequence-parser';

describe('Sequence Parser', () => {
  describe('parseSequence', () => {
    test('parses single phase', () => {
      const ast = parseSequence('P-01');
      expect(ast.kind).toBe('phase');
      expect((ast as PhaseId).phaseNum).toBe(1);
      expect((ast as PhaseId).subPhase).toBeNull();
    });

    test('parses sub-phase', () => {
      const ast = parseSequence('P-02-A');
      expect(ast.kind).toBe('phase');
      expect((ast as PhaseId).phaseNum).toBe(2);
      expect((ast as PhaseId).subPhase).toBe('A');
    });

    test('parses sequential phases with comma', () => {
      const ast = parseSequence('P-01, P-02');
      expect(ast.kind).toBe('binop');
      expect(ast.operator).toBe('OR');
    });

    test('parses parallel phases with plus', () => {
      const ast = parseSequence('P-01 + P-02');
      expect(ast.kind).toBe('binop');
      expect(ast.operator).toBe('AND');
    });

    test('parses multiple sequential phases', () => {
      const ast = parseSequence('P-01, P-02, P-03');
      expect(ast.kind).toBe('binop');
      // Check nested structure: (P-01, (P-02, P-03))
    });

    test('parses multiple parallel phases', () => {
      const ast = parseSequence('P-01 + P-02 + P-03');
      expect(ast.kind).toBe('binop');
    });

    test('parses parenthesized expression', () => {
      const ast = parseSequence('(P-01 + P-02)');
      expect(ast.kind).toBe('group');
    });

    test('parses bracketed expression', () => {
      const ast = parseSequence('[P-01-A, P-01-B]');
      expect(ast.kind).toBe('group');
    });

    test('parses complex nested expression', () => {
      const ast = parseSequence('P-01, (P-02 + P-03), P-04');
      expect(ast.kind).toBe('binop');
    });

    test('respects operator precedence (AND > OR)', () => {
      // P-01, P-02 + P-03 should be parsed as P-01, (P-02 + P-03)
      const ast = parseSequence('P-01, P-02 + P-03');
      expect(ast.kind).toBe('binop');
      expect(ast.operator).toBe('OR');
    });

    test('handles whitespace correctly', () => {
      const ast1 = parseSequence('P-01, P-02');
      const ast2 = parseSequence('P-01 , P-02');
      const ast3 = parseSequence('P-01 ,  P-02');
      // All should produce equivalent results
      expect(ast1.kind).toBe(ast2.kind);
      expect(ast2.kind).toBe(ast3.kind);
    });

    test('parses mixed sub-phases and regular phases', () => {
      const ast = parseSequence('P-01-A, P-01-B, P-02');
      expect(ast.kind).toBe('binop');
    });
  });

  describe('Error handling', () => {
    test('rejects invalid phase IDs', () => {
      expect(() => parseSequence('P-A')).toThrow(ParseError);
      expect(() => parseSequence('P-1')).toThrow(ParseError);
      expect(() => parseSequence('01')).toThrow(ParseError);
    });

    test('rejects unmatched parentheses', () => {
      expect(() => parseSequence('(P-01')).toThrow(ParseError);
      expect(() => parseSequence('P-01)')).toThrow(ParseError);
      expect(() => parseSequence('(P-01, P-02')).toThrow(ParseError);
    });

    test('rejects unmatched brackets', () => {
      expect(() => parseSequence('[P-01')).toThrow(ParseError);
      expect(() => parseSequence('P-01]')).toThrow(ParseError);
    });

    test('rejects empty grouping', () => {
      expect(() => parseSequence('()')).toThrow(ParseError);
      expect(() => parseSequence('[]')).toThrow(ParseError);
    });

    test('rejects duplicate operators', () => {
      expect(() => parseSequence('P-01 ++ P-02')).toThrow(ParseError);
      expect(() => parseSequence('P-01 ,, P-02')).toThrow(ParseError);
    });

    test('rejects trailing operators', () => {
      expect(() => parseSequence('P-01,')).toThrow(ParseError);
      expect(() => parseSequence('P-01+')).toThrow(ParseError);
    });

    test('rejects leading operators', () => {
      expect(() => parseSequence(',P-01')).toThrow(ParseError);
      expect(() => parseSequence('+P-01')).toThrow(ParseError);
    });

    test('provides helpful error messages', () => {
      try {
        parseSequence('P-A');
      } catch (error) {
        expect(error).toBeInstanceOf(ParseError);
        expect((error as ParseError).message).toContain('Expected');
      }
    });
  });

  describe('computeExecutionOrder', () => {
    test('single phase creates one batch', () => {
      const order = computeExecutionOrder('P-01');
      expect(order).toHaveLength(1);
      expect(order[0].phases).toEqual(['P-01']);
      expect(order[0].parallel).toBe(false);
    });

    test('sequential phases create separate batches', () => {
      const order = computeExecutionOrder('P-01, P-02, P-03');
      expect(order).toHaveLength(3);
      expect(order[0].phases).toEqual(['P-01']);
      expect(order[1].phases).toEqual(['P-02']);
      expect(order[2].phases).toEqual(['P-03']);
    });

    test('parallel phases create single batch', () => {
      const order = computeExecutionOrder('P-01 + P-02 + P-03');
      expect(order).toHaveLength(1);
      expect(order[0].phases).toHaveLength(3);
      expect(order[0].phases).toContain('P-01');
      expect(order[0].phases).toContain('P-02');
      expect(order[0].phases).toContain('P-03');
    });

    test('mixed sequential and parallel creates multiple batches', () => {
      const order = computeExecutionOrder('P-01, (P-02 + P-03), P-04');
      expect(order).toHaveLength(3);
      expect(order[0].phases).toEqual(['P-01']);
      expect(order[1].phases.length).toBe(2); // P-02 and P-03 in parallel
      expect(order[2].phases).toEqual(['P-04']);
    });

    test('handles grouped sequential phases', () => {
      const order = computeExecutionOrder('[P-01-A, P-01-B]');
      expect(order).toHaveLength(2);
      expect(order[0].phases).toEqual(['P-01-A']);
      expect(order[1].phases).toEqual(['P-01-B']);
    });

    test('handles sub-phases correctly', () => {
      const order = computeExecutionOrder('P-01-A + P-01-B, P-02');
      expect(order).toHaveLength(2);
      expect(order[0].phases.length).toBe(2);
      expect(order[1].phases).toEqual(['P-02']);
    });

    test('batch numbers are sequential', () => {
      const order = computeExecutionOrder('P-01, P-02, P-03');
      expect(order[0].batch).toBe(1);
      expect(order[1].batch).toBe(2);
      expect(order[2].batch).toBe(3);
    });
  });

  describe('formatExecutionOrder', () => {
    test('formats single batch', () => {
      const order = computeExecutionOrder('P-01');
      const formatted = formatExecutionOrder(order);
      expect(formatted).toContain('Batch 1');
      expect(formatted).toContain('P-01');
    });

    test('formats multiple batches', () => {
      const order = computeExecutionOrder('P-01, P-02, P-03');
      const formatted = formatExecutionOrder(order);
      expect(formatted).toContain('Batch 1');
      expect(formatted).toContain('Batch 2');
      expect(formatted).toContain('Batch 3');
    });

    test('indicates parallel phases', () => {
      const order = computeExecutionOrder('P-01 + P-02');
      const formatted = formatExecutionOrder(order);
      expect(formatted).toContain('parallel');
    });
  });

  describe('Grammar compliance', () => {
    test('ABNF example: single phase', () => {
      const order = computeExecutionOrder('P-01');
      expect(order.map((b) => b.phases)).toEqual([['P-01']]);
    });

    test('ABNF example: sequential phases', () => {
      const order = computeExecutionOrder('P-01, P-02');
      expect(order).toHaveLength(2);
    });

    test('ABNF example: parallel phases with grouping', () => {
      const order = computeExecutionOrder('P-01, (P-02 + P-03), P-04');
      expect(order).toHaveLength(3);
      expect(order[1].phases.length).toBe(2);
    });

    test('ABNF example: sub-phase grouping', () => {
      const order = computeExecutionOrder('[P-01-A, P-01-B, P-01-C]');
      expect(order).toHaveLength(3);
      expect(order[0].phases).toEqual(['P-01-A']);
      expect(order[1].phases).toEqual(['P-01-B']);
      expect(order[2].phases).toEqual(['P-01-C']);
    });

    test('ABNF example: complex nesting', () => {
      const order = computeExecutionOrder('P-01 + [P-02-A, P-02-B]');
      expect(order).toHaveLength(2);
      expect(order[0].phases).toContain('P-01');
      expect(order[0].phases).toContain('P-02-A');
    });
  });

  describe('Edge cases', () => {
    test('high phase numbers', () => {
      const order = computeExecutionOrder('P-99');
      expect(order[0].phases).toEqual(['P-99']);
    });

    test('all sub-phase letters', () => {
      const order = computeExecutionOrder('P-01-A + P-01-B + P-01-C + P-01-D');
      expect(order[0].phases).toHaveLength(4);
    });

    test('deeply nested expressions', () => {
      const order = computeExecutionOrder('(((P-01)))');
      expect(order[0].phases).toEqual(['P-01']);
    });

    test('mixed brackets and parentheses', () => {
      const order = computeExecutionOrder('[P-01 + (P-02, P-03)]');
      expect(order).toBeDefined();
    });

    test('large sequence', () => {
      const phases = Array.from({ length: 20 }, (_, i) =>
        `P-${String(i + 1).padStart(2, '0')}`
      ).join(', ');
      const order = computeExecutionOrder(phases);
      expect(order).toHaveLength(20);
    });
  });
});
