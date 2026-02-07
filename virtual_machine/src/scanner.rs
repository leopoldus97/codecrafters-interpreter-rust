#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier,
    String,
    Number,

    // Keywords.
    And,
    Class,
    Else,
    False,
    For,
    Fun,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Error,
    Eof,
}

#[derive(Clone, Copy)]
pub struct Token<'a> {
    pub token_type: TokenType,
    pub start: &'a str,
    pub length: usize,
    pub line: usize,
}

impl Default for Token<'_> {
    fn default() -> Self {
        Self {
            token_type: TokenType::Eof,
            start: "",
            length: 0,
            line: 0,
        }
    }
}

pub struct Scanner<'a> {
    pub source: &'a str,
    pub start: usize,
    pub current: usize,
    pub line: usize,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_token(&mut self) -> Token<'a> {
        self.skip_whitespace();

        self.start = self.current;

        if self.is_at_end(None) {
            return self.make_token(TokenType::Eof);
        }

        let c = self.advance();

        match c {
            '(' => self.make_token(TokenType::LeftParen),
            ')' => self.make_token(TokenType::RightParen),
            '{' => self.make_token(TokenType::LeftBrace),
            '}' => self.make_token(TokenType::RightBrace),
            ';' => self.make_token(TokenType::Semicolon),
            ',' => self.make_token(TokenType::Comma),
            '.' => self.make_token(TokenType::Dot),
            '-' => self.make_token(TokenType::Minus),
            '+' => self.make_token(TokenType::Plus),
            '/' => self.make_token(TokenType::Slash),
            '*' => self.make_token(TokenType::Star),
            '!' => {
                let token_type = if self.match_char('=') {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                };
                self.make_token(token_type)
            }
            '=' => {
                let token_type = if self.match_char('=') {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                };
                self.make_token(token_type)
            }
            '<' => {
                let token_type = if self.match_char('=') {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                };
                self.make_token(token_type)
            }
            '>' => {
                let token_type = if self.match_char('=') {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                };
                self.make_token(token_type)
            }
            '"' => self.string(),
            '0'..='9' => self.number(),
            'a'..='z' | 'A'..='Z' | '_' => self.identifier(),
            _ => self.error_token("Unexpected character."),
        }
    }

    fn skip_whitespace(&mut self) {
        loop {
            let c = match self.peek() {
                Some(ch) => ch,
                None => return,
            };
            match c {
                ' ' | '\r' | '\t' => {
                    self.current += 1;
                }
                '\n' => {
                    self.line += 1;
                    self.current += 1;
                }
                '/' => {
                    if self.peek_next() == Some('/') {
                        while let Some(ch) = self.peek() {
                            if ch == '\n' {
                                break;
                            }
                            self.current += 1;
                        }
                    } else {
                        return;
                    }
                }
                _ => return,
            }
        }
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end(None) {
            return false;
        }
        if self.peek().is_none_or(|val| val != expected) {
            return false;
        }

        self.current += 1;
        true
    }

    fn advance(&mut self) -> char {
        let c = self.peek().expect("Index out of bounds");
        self.current += 1;
        c
    }

    fn is_at_end(&self, offset: Option<usize>) -> bool {
        let offset = offset.unwrap_or(0);

        self.current + offset >= self.source.chars().count()
    }

    fn identifier(&mut self) -> Token<'a> {
        while let Some(c) = self.peek() {
            if !c.is_ascii_alphanumeric() && c != '_' {
                break;
            }
            self.current += 1;
        }

        self.make_token(self.identifier_type())
    }

    fn number(&mut self) -> Token<'a> {
        self.consume_digits();

        // Look for a fractional part.
        if self.peek() == Some('.') && self.peek_next().is_some_and(|c| c.is_ascii_digit()) {
            // Consume the "."
            self.current += 1;

            self.consume_digits();
        }

        self.make_token(TokenType::Number)
    }

    fn string(&mut self) -> Token<'a> {
        while let Some(c) = self.peek() {
            if c == '"' {
                break;
            }
            if c == '\n' {
                self.line += 1;
            }
            self.current += 1;
        }

        if self.is_at_end(None) {
            return self.error_token("Unterminated string.");
        }

        // The closing ".
        self.current += 1;
        self.make_token(TokenType::String)
    }

    fn make_token(&mut self, token_type: TokenType) -> Token<'a> {
        Token {
            token_type,
            start: &self.source[self.start..self.current],
            length: self.current - self.start,
            line: self.line,
        }
    }

    fn error_token(&self, message: &'a str) -> Token<'a> {
        Token {
            token_type: TokenType::Error,
            start: message,
            length: message.len(),
            line: self.line,
        }
    }

    fn peek(&mut self) -> Option<char> {
        if self.is_at_end(None) {
            None
        } else {
            Some(self.source.as_bytes()[self.current] as char)
        }
    }

    fn peek_next(&mut self) -> Option<char> {
        if self.is_at_end(Some(1)) {
            None
        } else {
            Some(self.source.as_bytes()[self.current + 1] as char)
        }
    }

    fn consume_digits(&mut self) {
        while let Some(c) = self.peek() {
            if !c.is_ascii_digit() {
                break;
            }
            self.current += 1;
        }
    }

    fn identifier_type(&self) -> TokenType {
        let slice = &self.source.as_bytes()[self.start..self.current];

        match slice {
            b"and" => TokenType::And,
            b"class" => TokenType::Class,
            b"else" => TokenType::Else,
            b"false" => TokenType::False,
            b"for" => TokenType::For,
            b"fun" => TokenType::Fun,
            b"if" => TokenType::If,
            b"nil" => TokenType::Nil,
            b"or" => TokenType::Or,
            b"print" => TokenType::Print,
            b"return" => TokenType::Return,
            b"super" => TokenType::Super,
            b"this" => TokenType::This,
            b"true" => TokenType::True,
            b"var" => TokenType::Var,
            b"while" => TokenType::While,
            _ => TokenType::Identifier,
        }
    }
}
