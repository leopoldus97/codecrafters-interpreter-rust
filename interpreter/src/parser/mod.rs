mod error;

use crate::{
    ast::{
        binary::Binary, expr::Expr, expression::Expression, grouping::Grouping, literal::Literal,
        print::Print, stmt::Stmt, unary::Unary, var::Var, variable::Variable,
    },
    scanner::{
        token::{Object, Token},
        token_type::TokenType,
    },
    utils::error::{Error, ParseError},
};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse<R: 'static>(&mut self) -> Result<Vec<Box<dyn Stmt>>, Error> {
        let mut statements: Vec<Box<dyn Stmt>> = Vec::new();
        while !self.is_at_end() {
            let declaration = self.declaration();
            if let Some(declaration) = declaration {
                statements.push(declaration);
            }
        }
        Ok(statements)
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
    fn declaration(&mut self) -> Option<Box<dyn Stmt>> {
        let result = if self.match_token_types(&[TokenType::Var]) {
            self.var_declaration()
        } else {
            self.statement()
        };

        match result {
            Ok(stmt) => Some(stmt),
            Err(_) => {
                self.synchronize();
                None
            }
        }
    }

    fn statement(&mut self) -> Result<Box<dyn Stmt>, ParseError> {
        if self.match_token_types(&[TokenType::Print]) {
            self.print_statement()
        } else {
            self.expression_statement()
        }
    }

    fn print_statement(&mut self) -> Result<Box<dyn Stmt>, ParseError> {
        let value = self.expression::<Object>()?;
        self.consume(TokenType::Semicolon, "Expect ';' after value.")?;
        Ok(Box::new(Print::new(value)))
    }

    fn var_declaration(&mut self) -> Result<Box<dyn Stmt>, ParseError> {
        let name = self.consume(TokenType::Identifier, "Expect variable name.")?;
        let initializer = if self.match_token_types(&[TokenType::Equal]) {
            Some(self.expression::<Object>()?)
        } else {
            None
        };
        self.consume(
            TokenType::Semicolon,
            "Expect ';' after variable declaration.",
        )?;
        Ok(Box::new(Var::new(name, initializer)))
    }

    fn expression_statement(&mut self) -> Result<Box<dyn Stmt>, ParseError> {
        let expr = self.expression::<Object>()?;
        self.consume(TokenType::Semicolon, "Expect ';' after value.")?;
        Ok(Box::new(Expression::new(expr)))
    }

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
        } else if self.match_token_types(&[TokenType::Identifier]) {
            Ok(Box::new(Variable::new(self.previous().to_owned())))
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
