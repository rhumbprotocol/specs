/**
 * RWP Sequence Parser - TypeScript Reference Implementation
 *
 * Parses phase sequence notation according to the ABNF grammar defined
 * in packages/rhumbprotocol/spec/sequence.grammar.
 *
 * Grammar (RFC 5234 ABNF):
 *   sequence = logical-expr
 *   logical-expr = or-expr
 *   or-expr = and-expr *( "," and-expr )
 *   and-expr = primary-expr *( "+" primary-expr )
 *   primary-expr = phase-group / phase-id / "(" logical-expr ")"
 *   phase-group = "[" logical-expr "]"
 *   phase-id = phase-number [ "-" sub-phase ]
 *   phase-number = "P-" 2DIGIT
 *   sub-phase = 1ALPHA
 */

/**
 * Token type
 */
interface Token {
  type: TokenType;
  value: string;
  position: number;
}

type TokenType = 'PHASE_ID' | 'COMMA' | 'PLUS' | 'LPAREN' | 'RPAREN' | 'LBRACKET' | 'RBRACKET' | 'EOF';

/**
 * AST Node types
 */
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

/**
 * Execution order representation
 */
interface ExecutionBatch {
  batch: number;
  phases: string[];
  parallel: boolean;
}

/**
 * Parse error with position information
 */
export class ParseError extends Error {
  constructor(
    message: string,
    public position: number,
    public input: string
  ) {
    super(
      `${message}\n` +
        `  Input: ${input}\n` +
        `  ${' '.repeat(position)}^`
    );
    this.name = 'ParseError';
  }
}

/**
 * Tokenizer: Convert input string to token stream
 */
class Tokenizer {
  private input: string;
  private position: number;

  constructor(input: string) {
    this.input = input;
    this.position = 0;
  }

  tokenize(): Token[] {
    const tokens: Token[] = [];

    while (this.position < this.input.length) {
      // Skip whitespace
      if (this.isWhitespace(this.current())) {
        this.advance();
        continue;
      }

      // PHASE_ID: P-\d{2}[A-Z]?
      if (this.match('P-')) {
        const start = this.position;
        this.advance(); // P
        this.advance(); // -

        if (!this.isDigit(this.current())) {
          throw new ParseError('Expected digit after "P-"', this.position, this.input);
        }

        const num1 = this.current();
        this.advance();

        if (!this.isDigit(this.current())) {
          throw new ParseError('Expected two-digit phase number', this.position, this.input);
        }

        const num2 = this.current();
        this.advance();

        let phaseId = `P-${num1}${num2}`;

        // Optional sub-phase: -[A-Z]
        if (this.current() === '-' && this.position + 1 < this.input.length) {
          const next = this.input[this.position + 1];
          if (this.isAlpha(next)) {
            this.advance(); // -
            this.advance(); // letter
            phaseId += `-${next}`;
          }
        }

        tokens.push({
          type: 'PHASE_ID',
          value: phaseId,
          position: start,
        });
        continue;
      }

      // Single-character tokens
      const char = this.current();
      if (char === ',') {
        tokens.push({ type: 'COMMA', value: ',', position: this.position });
        this.advance();
      } else if (char === '+') {
        tokens.push({ type: 'PLUS', value: '+', position: this.position });
        this.advance();
      } else if (char === '(') {
        tokens.push({ type: 'LPAREN', value: '(', position: this.position });
        this.advance();
      } else if (char === ')') {
        tokens.push({ type: 'RPAREN', value: ')', position: this.position });
        this.advance();
      } else if (char === '[') {
        tokens.push({ type: 'LBRACKET', value: '[', position: this.position });
        this.advance();
      } else if (char === ']') {
        tokens.push({ type: 'RBRACKET', value: ']', position: this.position });
        this.advance();
      } else {
        throw new ParseError(`Unexpected character: '${char}'`, this.position, this.input);
      }
    }

    tokens.push({ type: 'EOF', value: '', position: this.position });
    return tokens;
  }

  private current(): string {
    if (this.position >= this.input.length) {
      return '';
    }
    return this.input[this.position];
  }

  private advance(): void {
    this.position++;
  }

  private match(str: string): boolean {
    return this.input.slice(this.position, this.position + str.length) === str;
  }

  private isWhitespace(char: string): boolean {
    return /\s/.test(char);
  }

  private isDigit(char: string): boolean {
    return /\d/.test(char);
  }

  private isAlpha(char: string): boolean {
    return /[A-Za-z]/.test(char);
  }
}

/**
 * Parser: Build AST from token stream using recursive descent
 */
class Parser {
  private tokens: Token[];
  private position: number;

  constructor(tokens: Token[]) {
    this.tokens = tokens;
    this.position = 0;
  }

  parse(): ASTNode {
    const result = this.parseOrExpr();
    if (this.currentToken().type !== 'EOF') {
      throw new ParseError(
        `Unexpected token: ${this.currentToken().type}`,
        this.currentToken().position,
        this.getInputForError()
      );
    }
    return result;
  }

  private currentToken(): Token {
    if (this.position < this.tokens.length) {
      return this.tokens[this.position];
    }
    return { type: 'EOF', value: '', position: -1 };
  }

  private consume(expected: TokenType): Token {
    const token = this.currentToken();
    if (token.type !== expected) {
      throw new ParseError(
        `Expected ${expected}, got ${token.type}`,
        token.position,
        this.getInputForError()
      );
    }
    this.position++;
    return token;
  }

  private parseOrExpr(): ASTNode {
    let left = this.parseAndExpr();

    while (this.currentToken().type === 'COMMA') {
      this.consume('COMMA');
      const right = this.parseAndExpr();
      left = {
        kind: 'binop',
        operator: 'OR',
        left,
        right,
      };
    }

    return left;
  }

  private parseAndExpr(): ASTNode {
    let left = this.parsePrimaryExpr();

    while (this.currentToken().type === 'PLUS') {
      this.consume('PLUS');
      const right = this.parsePrimaryExpr();
      left = {
        kind: 'binop',
        operator: 'AND',
        left,
        right,
      };
    }

    return left;
  }

  private parsePrimaryExpr(): ASTNode {
    const token = this.currentToken();

    if (token.type === 'PHASE_ID') {
      this.consume('PHASE_ID');
      return this.parsePhaseId(token.value);
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

    throw new ParseError(
      `Expected PHASE_ID, '(', or '[', got ${token.type}`,
      token.position,
      this.getInputForError()
    );
  }

  private parsePhaseId(phaseIdStr: string): PhaseId {
    // Format: P-\d{2}(-[A-Z])?
    const match = phaseIdStr.match(/^P-(\d{2})(?:-([A-Z]))?$/);
    if (!match) {
      throw new Error(`Invalid phase ID format: ${phaseIdStr}`);
    }

    return {
      kind: 'phase',
      phaseNum: parseInt(match[1], 10),
      subPhase: match[2] || null,
    };
  }

  private getInputForError(): string {
    return this.tokens.filter((t) => t.type !== 'EOF').map((t) => t.value).join('');
  }
}

/**
 * Evaluator: Walk AST to compute execution order
 */
class Evaluator {
  evaluate(node: ASTNode): ExecutionBatch[] {
    const batches: ExecutionBatch[] = [];
    let batchNum = 1;
    let currentBatch: string[] = [];

    this.walkNode(node, batches, currentBatch, (batch) => {
      batchNum++;
    });

    if (currentBatch.length > 0) {
      batches.push({ batch: batchNum, phases: currentBatch, parallel: false });
    }

    return batches;
  }

  private walkNode(
    node: ASTNode,
    batches: ExecutionBatch[],
    currentBatch: string[],
    onFlush: () => void
  ): void {
    if (node.kind === 'phase') {
      currentBatch.push(this.phaseIdString(node));
    } else if (node.kind === 'group') {
      this.walkNode(node.expr, batches, currentBatch, onFlush);
    } else if (node.kind === 'binop') {
      if (node.operator === 'OR') {
        // Sequential: flush current batch, evaluate left, flush, evaluate right
        if (currentBatch.length > 0) {
          const batchNum = batches.length + 1;
          batches.push({ batch: batchNum, phases: [...currentBatch], parallel: false });
          currentBatch.length = 0;
          onFlush();
        }

        this.walkNode(node.left, batches, currentBatch, onFlush);

        if (currentBatch.length > 0) {
          const batchNum = batches.length + 1;
          batches.push({ batch: batchNum, phases: [...currentBatch], parallel: false });
          currentBatch.length = 0;
          onFlush();
        }

        this.walkNode(node.right, batches, currentBatch, onFlush);
      } else {
        // AND: collect parallel phases in current batch
        this.walkNode(node.left, batches, currentBatch, onFlush);
        this.walkNode(node.right, batches, currentBatch, onFlush);
      }
    }
  }

  private phaseIdString(phase: PhaseId): string {
    return phase.subPhase ? `P-${String(phase.phaseNum).padStart(2, '0')}-${phase.subPhase}` : `P-${String(phase.phaseNum).padStart(2, '0')}`;
  }
}

/**
 * Public API: Parse sequence and compute execution order
 */
export function parseSequence(input: string): ASTNode {
  const tokenizer = new Tokenizer(input);
  const tokens = tokenizer.tokenize();

  const parser = new Parser(tokens);
  return parser.parse();
}

/**
 * Compute execution order from sequence string
 */
export function computeExecutionOrder(input: string): ExecutionBatch[] {
  const ast = parseSequence(input);
  const evaluator = new Evaluator();
  return evaluator.evaluate(ast);
}

/**
 * Format execution order for display
 */
export function formatExecutionOrder(batches: ExecutionBatch[]): string {
  return batches
    .map((b) => `Batch ${b.batch}: ${b.phases.join(b.phases.length > 1 ? ' (parallel) ' : ', ')}`)
    .join('\n');
}

/**
 * Export types for external use
 */
export type { ASTNode, PhaseId, BinaryOp, Group, ExecutionBatch, Token };
