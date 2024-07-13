mod error;

use error::ParseError;

use crate::{
    ast::{binary::Binary, grouping::Grouping, literal::Literal, unary::Unary, Expr},
    scanner::{
        token::{Object, Token},
        token_type::TokenType,
    },
};

// expression     → equality ;
// equality       → comparison ( ( "!=" | "==" ) comparison )* ;
// comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
// term           → factor ( ( "-" | "+" ) factor )* ;
// factor         → unary ( ( "/" | "*" ) unary )* ;
// unary          → ( "!" | "-" ) unary
//                | primary ;
// primary        → NUMBER | STRING | "true" | "false" | "nil"
//                | "(" expression ")" ;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse<R: 'static>(&mut self) -> Option<Box<dyn Expr<R>>> {
        match self.expression() {
            Ok(expr) => Some(expr),
            Err(_) => None,
        }
    }

    fn match_token_types(&mut self, types: &[TokenType]) -> bool {
        for token_type in types {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }

        false
    }

    fn check(&self, token_type: &TokenType) -> bool {
        if self.is_at_end() {
            false
        } else {
            self.peek().token_type() == token_type
        }
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type() == &TokenType::Eof
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> Token {
        self.tokens[self.current - 1].to_owned()
    }

    fn consume(&mut self, token_type: TokenType, message: &str) -> Result<Token, ParseError> {
        if self.check(&token_type) {
            Ok(self.advance())
        } else {
            let error = self.error(self.peek(), message);
            Err(error)
        }
    }

    fn error(&self, token: &Token, message: &str) -> ParseError {
        error::error(token, message.to_string());
        ParseError {}
    }

    #[allow(dead_code)]
    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().token_type() == &TokenType::Semicolon {
                return;
            }

            match self.peek().token_type() {
                TokenType::Class
                | TokenType::Fun
                | TokenType::Var
                | TokenType::For
                | TokenType::If
                | TokenType::While
                | TokenType::Print
                | TokenType::Return => return,
                _ => self.advance(),
            };
        }
    }
}

impl Parser {
    fn expression<R: 'static>(&mut self) -> Result<Box<dyn Expr<R>>, ParseError> {
        self.equality::<R>()
    }

    fn equality<R: 'static>(&mut self) -> Result<Box<dyn Expr<R>>, ParseError> {
        let mut expr = self.comparison::<R>()?;

        while self.match_token_types(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous();
            let right = self.comparison::<R>()?;
            expr = Box::new(Binary::new(expr, operator, right));
        }

        Ok(expr)
    }

    fn comparison<R: 'static>(&mut self) -> Result<Box<dyn Expr<R>>, ParseError> {
        let mut expr = self.term::<R>()?;

        while self.match_token_types(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous();
            let right = self.term::<R>()?;
            expr = Box::new(Binary::new(expr, operator, right));
        }

        Ok(expr)
    }

    fn term<R: 'static>(&mut self) -> Result<Box<dyn Expr<R>>, ParseError> {
        let mut expr = self.factor::<R>()?;

        while self.match_token_types(&[TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous();
            let right = self.factor::<R>()?;
            expr = Box::new(Binary::new(expr, operator, right));
        }

        Ok(expr)
    }

    fn factor<R: 'static>(&mut self) -> Result<Box<dyn Expr<R>>, ParseError> {
        let mut expr = self.unary::<R>()?;

        while self.match_token_types(&[TokenType::Slash, TokenType::Star]) {
            let operator = self.previous();
            let right = self.unary::<R>()?;
            expr = Box::new(Binary::new(expr, operator, right));
        }

        Ok(expr)
    }

    fn unary<R: 'static>(&mut self) -> Result<Box<dyn Expr<R>>, ParseError> {
        if self.match_token_types(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous();
            let right = self.unary::<R>()?;
            Ok(Box::new(Unary::new(operator, right)))
        } else {
            self.primary()
        }
    }

    fn primary<R: 'static>(&mut self) -> Result<Box<dyn Expr<R>>, ParseError> {
        if self.match_token_types(&[TokenType::False]) {
            Ok(Box::new(Literal::new(Object::Bool(false))))
        } else if self.match_token_types(&[TokenType::True]) {
            Ok(Box::new(Literal::new(Object::Bool(true))))
        } else if self.match_token_types(&[TokenType::Nil]) {
            Ok(Box::new(Literal::new(Object::Nil)))
        } else if self.match_token_types(&[TokenType::Number, TokenType::String]) {
            Ok(Box::new(Literal::new(self.previous().literal().clone())))
        } else if self.match_token_types(&[TokenType::LeftParen]) {
            let expr = self.expression::<R>()?;
            self.consume(TokenType::RightParen, "Expect ')' after expression.")?;
            Ok(Box::new(Grouping::new(expr)))
        } else {
            let error = self.error(self.peek(), "Expect expression.");
            Err(error)
        }
    }
}