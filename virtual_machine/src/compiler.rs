use crate::{
    chunk::{Chunk, OpCode},
    scanner::{Scanner, Token, TokenType},
};

const DEBUG_PRINT_CODE: bool = true;

type PrefixFn = fn(&mut Compiler) -> ();
type InfixFn = fn(&mut Compiler, can_assign: bool) -> ();

pub struct ParseRule {
    pub prefix: Option<PrefixFn>,
    pub infix: Option<InfixFn>,
    pub precedence: Precedence,
}

pub const PARSE_RULES: &[ParseRule] = &[
    // 0: LeftParen
    ParseRule {
        prefix: Some(grouping_prefix),
        infix: None,
        precedence: Precedence::None,
    },
    // 1: RightParen
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    // 2: LeftBrace
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    // 3: RightBrace
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    // 4: Comma
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    // 5: Dot
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    // 6: Minus
    ParseRule {
        prefix: Some(unary_prefix),
        infix: Some(binary_infix),
        precedence: Precedence::Term,
    },
    // 7: Plus
    ParseRule {
        prefix: None,
        infix: Some(binary_infix),
        precedence: Precedence::Term,
    },
    // 8: Semicolon
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    // 9: Slash
    ParseRule {
        prefix: None,
        infix: Some(binary_infix),
        precedence: Precedence::Factor,
    },
    // 10: Star
    ParseRule {
        prefix: None,
        infix: Some(binary_infix),
        precedence: Precedence::Factor,
    },
    // 11: Bang
    ParseRule {
        prefix: Some(unary_prefix),
        infix: None,
        precedence: Precedence::None,
    },
    // 12: BangEqual
    ParseRule {
        prefix: None,
        infix: Some(binary_infix),
        precedence: Precedence::Equality,
    },
    // 13: Equal
    ParseRule {
        prefix: None,
        infix: None, // handled as assignment, not expression operator here
        precedence: Precedence::None,
    },
    // 14: EqualEqual
    ParseRule {
        prefix: None,
        infix: Some(binary_infix),
        precedence: Precedence::Equality,
    },
    // 15: Greater
    ParseRule {
        prefix: None,
        infix: Some(binary_infix),
        precedence: Precedence::Comparison,
    },
    // 16: GreaterEqual
    ParseRule {
        prefix: None,
        infix: Some(binary_infix),
        precedence: Precedence::Comparison,
    },
    // 17: Less
    ParseRule {
        prefix: None,
        infix: Some(binary_infix),
        precedence: Precedence::Comparison,
    },
    // 18: LessEqual
    ParseRule {
        prefix: None,
        infix: Some(binary_infix),
        precedence: Precedence::Comparison,
    },
    // 19: Identifier
    ParseRule {
        prefix: None,
        // prefix: Some(variable_prefix),
        infix: None,
        precedence: Precedence::None,
    },
    // 20: String
    ParseRule {
        prefix: None,
        // prefix: Some(string_prefix),
        infix: None,
        precedence: Precedence::None,
    },
    // 21: Number
    ParseRule {
        prefix: Some(number_prefix),
        infix: None,
        precedence: Precedence::None,
    },
    // 22: And
    ParseRule {
        prefix: None,
        infix: Some(binary_infix),
        precedence: Precedence::And,
    },
    // 23: Class
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    // 24: Else
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    // 25: False
    ParseRule {
        prefix: None,
        // prefix: Some(literal_prefix),
        infix: None,
        precedence: Precedence::None,
    },
    // 26: For
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    // 27: Fun
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    // 28: If
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    // 29: Nil
    ParseRule {
        prefix: None,
        // prefix: Some(literal_prefix),
        infix: None,
        precedence: Precedence::None,
    },
    // 30: Or
    ParseRule {
        prefix: None,
        infix: Some(binary_infix),
        precedence: Precedence::Or,
    },
    // 31: Print
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    // 32: Return
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    // 33: Super
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    // 34: This
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    // 35: True
    ParseRule {
        prefix: None,
        // prefix: Some(literal_prefix),
        infix: None,
        precedence: Precedence::None,
    },
    // 36: Var
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    // 37: While
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    // 38 Error
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    // 39: Eof
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
];

fn grouping_prefix(c: &mut Compiler) {
    c.grouping();
}

fn number_prefix(c: &mut Compiler) {
    c.number();
}

fn unary_prefix(c: &mut Compiler) {
    c.unary();
}

fn binary_infix(c: &mut Compiler, _can_assign: bool) {
    c.binary();
}

// fn string_prefix<'a>(c: &mut Compiler<'a>) {
//     c.string();
// }

// fn literal_prefix<'a>(c: &mut Compiler<'a>) {
//     c.literal();
// }

// fn variable_prefix<'a>(c: &mut Compiler<'a>) {
//     c.variable();
// }

#[derive(Default)]
pub struct Parser<'a> {
    current: Token<'a>,
    previous: Token<'a>,
    had_error: bool,
    panic_mode: bool,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Precedence {
    None = 0,
    Assignment, // =
    Or,         // or
    And,        // and
    Equality,   // == !=
    Comparison, // < > <= >=
    Term,       // + -
    Factor,     // * /
    Unary,      // ! -
    Call,       // . ()
    Primary,
}

pub struct Compiler<'a, 'b> {
    scanner: Scanner<'a>,
    parser: Parser<'a>,
    current_chunk: Option<&'b mut Chunk>,
}

impl<'a, 'b> Compiler<'a, 'b> {
    pub fn new(source: &'a str) -> Self {
        Self {
            scanner: Scanner::new(source),
            parser: Parser::default(),
            current_chunk: None,
        }
    }

    pub fn compile(&mut self, chunk: &'b mut Chunk) -> bool {
        self.current_chunk = Some(chunk);

        // Compilation logic goes here
        self.advance();
        self.expression();
        self.consume(TokenType::Eof, "Expect end of expression.");

        self.end_compiler();
        !self.parser.had_error
    }

    fn advance(&mut self) {
        self.parser.previous = self.parser.current;

        loop {
            self.parser.current = self.scanner.scan_token();
            if !matches!(self.parser.current.token_type, TokenType::Error) {
                break;
            }

            self.error_at_current(self.parser.current.start);
        }
    }

    fn consume(&mut self, token_type: TokenType, message: &str) {
        if self.parser.current.token_type == token_type {
            self.advance();
            return;
        }

        self.error_at_current(message);
    }

    fn emit_byte(&mut self, byte: u8) {
        if let Some(chunk) = &mut self.current_chunk {
            chunk.write(byte, self.parser.previous.line);
        }
    }

    fn emit_bytes(&mut self, byte1: u8, byte2: u8) {
        self.emit_byte(byte1);
        self.emit_byte(byte2);
    }

    fn end_compiler(&mut self) {
        self.emit_return();
        if DEBUG_PRINT_CODE && !self.parser.had_error {
            if let Some(chunk) = &self.current_chunk {
                chunk.disassemble("code");
            }
        }
    }

    fn get_rule(&self, token_type: TokenType) -> &'static ParseRule {
        &PARSE_RULES[token_type as usize]
    }

    fn binary(&mut self) {
        let operator_type = self.parser.previous.token_type;
        let rule = self.get_rule(operator_type);

        // Compile the right operand.
        self.parse_precedence(rule.precedence as u8 + 1);

        // Emit the operator instruction.
        match operator_type {
            TokenType::Plus => self.emit_byte(OpCode::OpAdd as u8),
            TokenType::Minus => self.emit_byte(OpCode::OpSubtract as u8),
            TokenType::Star => self.emit_byte(OpCode::OpMultiply as u8),
            TokenType::Slash => self.emit_byte(OpCode::OpDivide as u8),
            _ => (), // Unreachable.
        }
    }

    fn unary(&mut self) {
        let operator_type = self.parser.previous.token_type;

        // Compile the operand.
        self.parse_precedence(Precedence::Unary as u8);

        // Emit the operator instruction.
        // match operator_type {
        //     TokenType::Minus => self.emit_byte(OpCode::OpNegate as u8),
        //     TokenType::Bang => self.emit_byte(OpCode::OpNot as u8),
        //     _ => (), // Unreachable.
        // }
        if operator_type == TokenType::Minus {
            self.emit_byte(OpCode::OpNegate as u8);
        }
    }

    fn parse_precedence(&mut self, precedence: u8) {
        self.advance();
        let operator_type = self.parser.previous.token_type;
        let prefix_rule = self.get_rule(operator_type).prefix;
        if let Some(prefix_fn) = prefix_rule {
            prefix_fn(self);
        } else {
            self.error("Expect expression.");
            return;
        }

        while precedence <= self.get_rule(self.parser.current.token_type).precedence as u8 {
            self.advance();
            let infix_rule = self.get_rule(self.parser.previous.token_type).infix;
            if let Some(infix_fn) = infix_rule {
                infix_fn(self, false);
            }
        }
    }

    fn grouping(&mut self) {
        self.expression();
        self.consume(TokenType::RightParen, "Expect ')' after expression.");
    }

    fn number(&mut self) {
        let value: f64 = match self.parser.previous.start.parse() {
            Ok(num) => num,
            Err(_) => {
                self.parser.had_error = true;
                self.error("Invalid number.");
                return;
            }
        };
        self.emit_constant(value);
    }

    fn expression(&mut self) {
        self.parse_precedence(Precedence::Assignment as u8);
    }

    fn emit_return(&mut self) {
        self.emit_byte(OpCode::OpReturn as u8);
    }

    fn make_constant(&mut self, value: f64) -> u8 {
        if let Some(chunk) = &mut self.current_chunk {
            let constant = chunk.add_constant(value);
            if constant > u8::MAX as usize {
                self.error("Too many constants in one chunk.");
                return 0;
            }
            constant as u8
        } else {
            0
        }
    }

    fn emit_constant(&mut self, value: f64) {
        let constant = self.make_constant(value);
        self.emit_bytes(OpCode::OpConstant as u8, constant);
    }

    fn error_at_current(&mut self, message: &str) {
        let token = self.parser.current;

        self.error_at(&token, message);
    }

    fn error(&mut self, message: &str) {
        let token = self.parser.previous;

        self.error_at(&token, message);
    }

    fn error_at(&mut self, token: &Token, message: &str) {
        if self.parser.panic_mode {
            return;
        }
        self.parser.panic_mode = true;
        eprint!("[line {}] Error", token.line);

        if matches!(token.token_type, TokenType::Eof) {
            eprint!(" at end");
        } else if matches!(token.token_type, TokenType::Error) {
            // Nothing.
        } else {
            eprint!(" at '{}' (length {})", token.start, token.length);
        }

        eprintln!(": {}", message);

        self.parser.had_error = true;
    }
}
