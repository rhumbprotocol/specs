# RWP Sequence Parser Specification

> Part of the [Rhumb Workflow Protocol (RWP)](../docs/PROTOCOL.md) - see also: [Sequence Grammar](./sequence.grammar), [Implementation Guide](./implementation-guide.md)

## Overview

This specification details how to implement a parser for Rhumb Workflow Protocol (RWP) phase sequence notation. The grammar is formally defined in `sequence.grammar` (ABNF RFC 5234). This document provides parser architecture, pseudocode, AST design, and evaluator algorithms.

## Parser Architecture

### High-Level Flow

```
Input String (e.g., "P-01, (P-02 + P-03), P-04")
    ↓
Tokenizer (lexical analysis)
    ↓
Token Stream: [P, -, 01, ',', '(', P, -, 02, '+', ...]
    ↓
Parser (syntax analysis)
    ↓
AST (abstract syntax tree)
    ↓
Evaluator (semantic analysis)
    ↓
Execution Order: [P-01, then (P-02 || P-03), then P-04]
```

### Three-Stage Design

1. **Tokenizer**: Converts raw string to tokens
2. **Parser**: Builds AST from tokens using recursive descent
3. **Evaluator**: Walks AST to compute execution order

## Token Types

```
PHASE_ID       : P-\d{2}[A-Z]?   (e.g., P-01, P-01-A)
COMMA          : ,                (sequential/OR operator)
PLUS           : +                (parallel/AND operator)
LPAREN         : (                (grouping open)
RPAREN         : )                (grouping close)
LBRACKET       : [                (grouping open)
RBRACKET       : ]                (grouping close)
EOF            : end of input     (terminator)
WHITESPACE     : [ \t\n\r]+       (ignored)
```

## AST Node Types

```
PhaseId(phase_num: int, sub_phase: str | null)
  Example: PhaseId(1, "A") represents P-01-A

BinaryOp(operator: "AND" | "OR", left: Node, right: Node)
  Example: BinaryOp("AND", PhaseId(1), PhaseId(2))
           represents P-01 + P-02

Group(expr: Node)
  Example: Group(BinaryOp(...))
           represents (P-01 + P-02)

Sequence(nodes: [Node, ...])
  Example: Sequence([PhaseId(1), PhaseId(2)])
           represents the top-level parsed expression
```

## Pseudocode: Tokenizer

```python
def tokenize(input_str: str) -> List[Token]:
    """Convert input string to token stream."""
    tokens = []
    i = 0

    while i < len(input_str):
        # Skip whitespace
        if input_str[i] in ' \t\n\r':
            i += 1
            continue

        # PHASE_ID: P-\d{2}[A-Z]?
        if i + 2 < len(input_str) and input_str[i:i+2] == 'P-':
            if input_str[i+2:i+4].isdigit():
                phase_num = input_str[i+2:i+4]
                i += 4
                sub_phase = None
                if i < len(input_str) and input_str[i].isalpha():
                    sub_phase = input_str[i]
                    i += 1
                tokens.append(Token('PHASE_ID', f'P-{phase_num}-{sub_phase if sub_phase else ""}'))
                continue

        # Single-character tokens
        if input_str[i] == ',':
            tokens.append(Token('COMMA', ','))
            i += 1
        elif input_str[i] == '+':
            tokens.append(Token('PLUS', '+'))
            i += 1
        elif input_str[i] == '(':
            tokens.append(Token('LPAREN', '('))
            i += 1
        elif input_str[i] == ')':
            tokens.append(Token('RPAREN', ')'))
            i += 1
        elif input_str[i] == '[':
            tokens.append(Token('LBRACKET', '['))
            i += 1
        elif input_str[i] == ']':
            tokens.append(Token('RBRACKET', ']'))
            i += 1
        else:
            raise ParseError(f"Unexpected character: {input_str[i]} at position {i}")

    tokens.append(Token('EOF', ''))
    return tokens
```

## Pseudocode: Recursive Descent Parser

```python
class Parser:
    def __init__(self, tokens: List[Token]):
        self.tokens = tokens
        self.pos = 0

    def current_token(self) -> Token:
        if self.pos < len(self.tokens):
            return self.tokens[self.pos]
        return Token('EOF', '')

    def consume(self, expected_type: str):
        token = self.current_token()
        if token.type != expected_type:
            raise ParseError(
                f"Expected {expected_type}, got {token.type} at position {self.pos}"
            )
        self.pos += 1
        return token

    def parse(self) -> Node:
        """Entry point: parse logical expression."""
        return self.parse_or_expr()

    def parse_or_expr(self) -> Node:
        """Parse: and_expr (',' and_expr)*"""
        left = self.parse_and_expr()

        while self.current_token().type == 'COMMA':
            self.consume('COMMA')
            right = self.parse_and_expr()
            left = BinaryOp('OR', left, right)

        return left

    def parse_and_expr(self) -> Node:
        """Parse: primary_expr ('+' primary_expr)*"""
        left = self.parse_primary_expr()

        while self.current_token().type == 'PLUS':
            self.consume('PLUS')
            right = self.parse_primary_expr()
            left = BinaryOp('AND', left, right)

        return left

    def parse_primary_expr(self) -> Node:
        """Parse: PHASE_ID | '(' or_expr ')' | '[' or_expr ']'"""
        token = self.current_token()

        # PHASE_ID
        if token.type == 'PHASE_ID':
            self.consume('PHASE_ID')
            # Parse P-\d{2}[A-Z]?
            match = re.match(r'P-(\d{2})(?:-([A-Z]))?', token.value)
            if match:
                phase_num = int(match.group(1))
                sub_phase = match.group(2)
                return PhaseId(phase_num, sub_phase)
            else:
                raise ParseError(f"Invalid phase ID: {token.value}")

        # Parenthesized expression
        elif token.type == 'LPAREN':
            self.consume('LPAREN')
            expr = self.parse_or_expr()
            self.consume('RPAREN')
            return Group(expr)

        # Bracketed expression
        elif token.type == 'LBRACKET':
            self.consume('LBRACKET')
            expr = self.parse_or_expr()
            self.consume('RBRACKET')
            return Group(expr)

        else:
            raise ParseError(
                f"Expected PHASE_ID, '(', or '[', got {token.type} at position {self.pos}"
            )
```

## Pseudocode: Evaluator

```python
class Evaluator:
    """Convert AST to execution order."""

    def evaluate(self, node: Node) -> ExecutionOrder:
        """Evaluate AST and return execution order."""
        return self._eval_node(node)

    def _eval_node(self, node: Node) -> ExecutionOrder:
        if isinstance(node, PhaseId):
            return ExecutionOrder(
                type='phase',
                phases=[node.phase_id_str()],
                parallelism=False
            )

        elif isinstance(node, Group):
            return self._eval_node(node.expr)

        elif isinstance(node, BinaryOp):
            left = self._eval_node(node.left)
            right = self._eval_node(node.right)

            if node.operator == 'OR':
                # Sequential execution
                return ExecutionOrder(
                    type='sequence',
                    steps=[left, right],
                    parallelism=False
                )
            else:  # 'AND'
                # Parallel execution
                return ExecutionOrder(
                    type='parallel',
                    steps=[left, right],
                    parallelism=True
                )

    def compute_order(self, node: Node) -> List[List[str]]:
        """Compute sequential groups for execution.

        Returns:
            List[List[str]]: Execution batches. Each batch is a list of phases
                            that can execute in parallel.

        Example:
            Input: P-01, (P-02 + P-03), P-04
            Output: [[P-01], [P-02, P-03], [P-04]]
        """
        order = self._eval_node(node)
        batches = []
        self._flatten_order(order, batches)
        return batches

    def _flatten_order(self, order: ExecutionOrder, batches: List[List[str]], current_batch: List[str] = None):
        if current_batch is None:
            current_batch = []

        if order.type == 'phase':
            current_batch.extend(order.phases)

        elif order.type == 'sequence':
            # Flush current batch before sequence
            if current_batch:
                batches.append(current_batch)
                current_batch = []

            # Evaluate each step
            for step in order.steps:
                self._flatten_order(step, batches, current_batch)
                if current_batch:
                    batches.append(current_batch)
                    current_batch = []

        elif order.type == 'parallel':
            # Collect parallel phases in current batch
            for step in order.steps:
                self._flatten_order(step, batches, current_batch)

        if current_batch:
            batches.append(current_batch)
```

## Error Handling

### Common Parsing Errors

| Error | Example | Handling |
|-------|---------|----------|
| Invalid phase ID | `P-A` | Reject; expect `P-\d{2}[A-Z]?` |
| Unmatched parentheses | `(P-01` | Reject; expect matching `)` |
| Unexpected operator | `P-01 +` | Reject; expect phase ID or grouping |
| Duplicate operators | `P-01 ++ P-02` | Reject; extra `+` is unexpected |
| Empty grouping | `[]` or `()` | Reject; groups must contain phases |

### Error Messages

Implementations SHOULD provide helpful error messages with position:

```
Error at position 8: Unexpected ')' - expected PHASE_ID, '(', or '['
  Input: (P-01,)P-02
                ^
```

## Language-Specific Examples

### Reference: TypeScript Parser

```typescript
// Type definitions
type Token = { type: string; value: string };
type ASTNode = PhaseId | BinaryOp | Group;

interface PhaseId {
  kind: 'phase';
  phaseNum: number;
  subPhase: string | null;
}

interface BinaryOp {
  kind: 'binop';
  operator: 'AND' | 'OR';
  left: ASTNode;
  right: ASTNode;
}

interface Group {
  kind: 'group';
  expr: ASTNode;
}

// Tokenizer
function tokenize(input: string): Token[] {
  const tokens: Token[] = [];
  let i = 0;

  while (i < input.length) {
    if (/\s/.test(input[i])) {
      i++;
      continue;
    }

    // PHASE_ID
    if (input[i] === 'P' && input[i + 1] === '-') {
      const match = input.slice(i).match(/^P-(\d{2})([A-Z])?/);
      if (match) {
        tokens.push({
          type: 'PHASE_ID',
          value: match[0],
        });
        i += match[0].length;
        continue;
      }
    }

    // Single-character tokens
    const charTokens: { [key: string]: string } = {
      ',': 'COMMA',
      '+': 'PLUS',
      '(': 'LPAREN',
      ')': 'RPAREN',
      '[': 'LBRACKET',
      ']': 'RBRACKET',
    };

    if (input[i] in charTokens) {
      tokens.push({
        type: charTokens[input[i]],
        value: input[i],
      });
      i++;
      continue;
    }

    throw new Error(`Unexpected character '${input[i]}' at position ${i}`);
  }

  tokens.push({ type: 'EOF', value: '' });
  return tokens;
}

// Parser
class Parser {
  private tokens: Token[];
  private pos: number;

  constructor(tokens: Token[]) {
    this.tokens = tokens;
    this.pos = 0;
  }

  parse(): ASTNode {
    return this.parseOrExpr();
  }

  private currentToken(): Token {
    return this.pos < this.tokens.length ? this.tokens[this.pos] : { type: 'EOF', value: '' };
  }

  private consume(expectedType: string): Token {
    const token = this.currentToken();
    if (token.type !== expectedType) {
      throw new Error(`Expected ${expectedType}, got ${token.type} at position ${this.pos}`);
    }
    this.pos++;
    return token;
  }

  private parseOrExpr(): ASTNode {
    let left = this.parseAndExpr();

    while (this.currentToken().type === 'COMMA') {
      this.consume('COMMA');
      const right = this.parseAndExpr();
      left = { kind: 'binop', operator: 'OR', left, right };
    }

    return left;
  }

  private parseAndExpr(): ASTNode {
    let left = this.parsePrimaryExpr();

    while (this.currentToken().type === 'PLUS') {
      this.consume('PLUS');
      const right = this.parsePrimaryExpr();
      left = { kind: 'binop', operator: 'AND', left, right };
    }

    return left;
  }

  private parsePrimaryExpr(): ASTNode {
    const token = this.currentToken();

    if (token.type === 'PHASE_ID') {
      this.consume('PHASE_ID');
      const match = token.value.match(/P-(\d{2})(?:-([A-Z]))?/);
      if (match) {
        return {
          kind: 'phase',
          phaseNum: parseInt(match[1], 10),
          subPhase: match[2] || null,
        };
      }
      throw new Error(`Invalid phase ID: ${token.value}`);
    } else if (token.type === 'LPAREN') {
      this.consume('LPAREN');
      const expr = this.parseOrExpr();
      this.consume('RPAREN');
      return { kind: 'group', expr };
    } else if (token.type === 'LBRACKET') {
      this.consume('LBRACKET');
      const expr = this.parseOrExpr();
      this.consume('RBRACKET');
      return { kind: 'group', expr };
    }

    throw new Error(`Unexpected token: ${token.type} at position ${this.pos}`);
  }
}

// Usage
function parseSequence(input: string): ASTNode {
  const tokens = tokenize(input);
  const parser = new Parser(tokens);
  return parser.parse();
}
```

## Test Cases

All implementations SHOULD pass these test cases:

```python
test_cases = [
    # Single phase
    ("P-01", expected_ast),

    # Sequential phases
    ("P-01, P-02", expected_ast),
    ("P-01, P-02, P-03", expected_ast),

    # Parallel phases
    ("P-01 + P-02", expected_ast),
    ("P-01 + P-02 + P-03", expected_ast),

    # Sub-phases
    ("P-01-A", expected_ast),
    ("P-01-A, P-01-B", expected_ast),

    # Grouping with parentheses
    ("(P-01 + P-02), P-03", expected_ast),
    ("P-01, (P-02 + P-03)", expected_ast),

    # Grouping with brackets
    ("[P-01-A, P-01-B, P-01-C]", expected_ast),

    # Complex nesting
    ("P-01, (P-02 + P-03), P-04", expected_ast),
    ("(P-01 + [P-02-A, P-02-B]), P-03", expected_ast),

    # Whitespace handling
    ("P-01 , P-02", expected_ast),
    ("P-01+P-02", expected_ast),
    ("( P-01 + P-02 ) , P-03", expected_ast),
]

for input_str, expected in test_cases:
    result = parse_sequence(input_str)
    assert result == expected, f"Failed: {input_str}"
```

## Validation Checklist

✓ Parser correctly tokenizes all valid inputs
✓ Parser rejects invalid inputs with clear error messages
✓ Parser handles all ABNF grammar rules
✓ Parser correctly sets operator precedence (AND > OR)
✓ Parser correctly handles parentheses and brackets
✓ Parser correctly parses sub-phase identifiers (P-XX-Y)
✓ Parser ignores whitespace
✓ Parser produces valid AST
✓ Evaluator correctly computes execution order
✓ Evaluator correctly identifies parallel and sequential steps

---

Produced:
  - when: 2026-03-04T04:20:00Z
  - by: YAKKL® Meridian™- https://meridian.yakkl.com
  - copyright: Copyright © 2026 YAKKL Inc. All Rights Reserved.
