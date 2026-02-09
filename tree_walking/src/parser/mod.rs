mod error;

use std::rc::Rc;

use crate::{
    ast::{
        expr::{
            assign::Assign, binary::Binary, call::Call, get::Get, grouping::Grouping,
            literal::Literal, logical::Logical, set::Set, super_keyword::Super, this::This,
            unary::Unary, variable::Variable, Expr,
        },
        stmt::{
            block::Block, class::Class, expression::Expression, function::Function, print::Print,
            r#if::If, r#return::Return, r#while::While, var::Var, Stmt,
        },
    },
    parser::error::error,
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

    pub fn parse(&mut self) -> Result<Vec<Rc<dyn Stmt>>, Box<Error>> {
        let mut statements: Vec<Rc<dyn Stmt>> = Vec::new();
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
    fn declaration(&mut self) -> Option<Rc<dyn Stmt>> {
        let result = if self.match_token_types(&[TokenType::Class]) {
            self.class_declaration()
        } else if self.match_token_types(&[TokenType::Fun]) {
            self.function("function")
        } else if self.match_token_types(&[TokenType::Var]) {
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

    fn class_declaration(&mut self) -> Result<Rc<dyn Stmt>, ParseError> {
        let name = self.consume(TokenType::Identifier, "Expect class name.")?;

        let superclass = if self.match_token_types(&[TokenType::Less]) {
            self.consume(TokenType::Identifier, "Expect superclass name.")?;
            Some(Variable::new(self.previous()))
        } else {
            None
        };

        self.consume(TokenType::LeftBrace, "Expect '{' before class body.")?;

        let mut methods = vec![];
        while !self.check(&TokenType::RightBrace) && !self.is_at_end() {
            let method = self.function("method")?;
            if let Some(function) = method.as_any().downcast_ref::<Function>() {
                methods.push(function.to_owned());
            }
        }

        self.consume(TokenType::RightBrace, "Expect '}' after class body.")?;

        Ok(Rc::new(Class::new(name, methods, superclass)))
    }

    fn statement(&mut self) -> Result<Rc<dyn Stmt>, ParseError> {
        if self.match_token_types(&[TokenType::For]) {
            self.for_statement()
        } else if self.match_token_types(&[TokenType::If]) {
            self.if_statement()
        } else if self.match_token_types(&[TokenType::Print]) {
            self.print_statement()
        } else if self.match_token_types(&[TokenType::Return]) {
            self.return_statement()
        } else if self.match_token_types(&[TokenType::While]) {
            self.while_statement()
        } else if self.match_token_types(&[TokenType::LeftBrace]) {
            Ok(Rc::new(Block::new(self.block()?)))
        } else {
            self.expression_statement()
        }
    }

    fn function(&mut self, kind: &str) -> Result<Rc<dyn Stmt>, ParseError> {
        let name = self.consume(TokenType::Identifier, &format!("Expect {} name.", kind))?;
        self.consume(
            TokenType::LeftParen,
            &format!("Expect '(' after {} name.", kind),
        )?;

        let mut parameters: Vec<Token> = Vec::new();
        if !self.check(&TokenType::RightParen) {
            loop {
                if parameters.len() >= 255 {
                    let error = self.error(self.peek(), "Cannot have more than 255 parameters.");
                    return Err(error);
                }

                parameters.push(self.consume(TokenType::Identifier, "Expect parameter name.")?);

                if !self.match_token_types(&[TokenType::Comma]) {
                    break;
                }
            }
        }

        self.consume(TokenType::RightParen, "Expect ')' after parameters.")?;
        self.consume(
            TokenType::LeftBrace,
            &format!("Expect '{{' before {} body.", kind),
        )?;
        let body = self.block()?;
        Ok(Rc::new(Function::new(name, parameters, body)))
    }

    fn for_statement(&mut self) -> Result<Rc<dyn Stmt>, ParseError> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'for'.")?;

        let initializer = if self.match_token_types(&[TokenType::Semicolon]) {
            None
        } else if self.match_token_types(&[TokenType::Var]) {
            Some(self.var_declaration()?)
        } else {
            Some(self.expression_statement()?)
        };

        let condition = if !self.check(&TokenType::Semicolon) {
            self.expression()?
        } else {
            Rc::new(Literal::new(Object::Bool(true)))
        };
        self.consume(TokenType::Semicolon, "Expect ';' after loop condition.")?;

        let increment = if !self.check(&TokenType::RightParen) {
            Some(self.expression()?)
        } else {
            None
        };
        self.consume(TokenType::RightParen, "Expect ')' after for clauses.")?;

        let mut body = self.statement()?;

        if let Some(increment) = increment {
            body = Rc::new(Block::new(vec![body, Rc::new(Expression::new(increment))]));
        }

        body = Rc::new(While::new(condition, body));

        if let Some(initializer) = initializer {
            body = Rc::new(Block::new(vec![initializer, body]));
        }

        Ok(body)
    }

    fn if_statement(&mut self) -> Result<Rc<dyn Stmt>, ParseError> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'if'.")?;
        let condition = self.expression()?;
        self.consume(TokenType::RightParen, "Expect ')' after if condition.")?;

        let then_branch = self.statement()?;
        let else_branch = if self.match_token_types(&[TokenType::Else]) {
            Some(self.statement()?)
        } else {
            None
        };

        Ok(Rc::new(If::new(condition, then_branch, else_branch)))
    }

    fn print_statement(&mut self) -> Result<Rc<dyn Stmt>, ParseError> {
        let value = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect ';' after value.")?;
        Ok(Rc::new(Print::new(value)))
    }

    fn return_statement(&mut self) -> Result<Rc<dyn Stmt>, ParseError> {
        let keyword = self.previous();
        let value = if !self.check(&TokenType::Semicolon) {
            Some(self.expression()?)
        } else {
            None
        };
        self.consume(TokenType::Semicolon, "Expect ';' after return value.")?;
        Ok(Rc::new(Return::new(keyword, value)))
    }

    fn var_declaration(&mut self) -> Result<Rc<dyn Stmt>, ParseError> {
        let name = self.consume(TokenType::Identifier, "Expect variable name.")?;
        let initializer = if self.match_token_types(&[TokenType::Equal]) {
            Some(self.expression()?)
        } else {
            None
        };
        self.consume(
            TokenType::Semicolon,
            "Expect ';' after variable declaration.",
        )?;
        Ok(Rc::new(Var::new(name, initializer)))
    }

    fn while_statement(&mut self) -> Result<Rc<dyn Stmt>, ParseError> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'while'.")?;
        let condition = self.expression()?;
        self.consume(TokenType::RightParen, "Expect ')' after condition.")?;
        let body = self.statement()?;
        Ok(Rc::new(While::new(condition, body)))
    }

    fn expression_statement(&mut self) -> Result<Rc<dyn Stmt>, ParseError> {
        let expr = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect ';' after value.")?;
        Ok(Rc::new(Expression::new(expr)))
    }

    fn block(&mut self) -> Result<Vec<Rc<dyn Stmt>>, ParseError> {
        let mut statements: Vec<Rc<dyn Stmt>> = Vec::new();

        while !self.check(&TokenType::RightBrace) && !self.is_at_end() {
            let declaration = self.declaration();
            if let Some(declaration) = declaration {
                statements.push(declaration);
            }
        }

        self.consume(TokenType::RightBrace, "Expect '}' after block.")?;
        Ok(statements)
    }

    fn expression(&mut self) -> Result<Rc<dyn Expr>, ParseError> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Rc<dyn Expr>, ParseError> {
        let expr = self.or()?;

        if self.match_token_types(&[TokenType::Equal]) {
            let equals = self.previous();
            let value = self.assignment()?;

            if let Some(variable) = expr.as_any().downcast_ref::<Variable>() {
                let name = variable.name().to_owned();
                return Ok(Rc::new(Assign::new(name, value)));
            } else if let Some(get) = expr.as_any().downcast_ref::<Get>() {
                let get = get.to_owned();
                return Ok(Rc::new(Set::new(
                    get.object(),
                    get.name().to_owned(),
                    value,
                )));
            }

            let error = self.error(&equals, "Invalid assignment target.");
            return Err(error);
        }

        Ok(expr)
    }

    fn or(&mut self) -> Result<Rc<dyn Expr>, ParseError> {
        let mut expr = self.and()?;

        while self.match_token_types(&[TokenType::Or]) {
            let operator = self.previous();
            let right = self.and()?;
            expr = Rc::new(Logical::new(expr, operator, right));
        }

        Ok(expr)
    }

    fn and(&mut self) -> Result<Rc<dyn Expr>, ParseError> {
        let mut expr = self.equality()?;

        while self.match_token_types(&[TokenType::And]) {
            let operator = self.previous();
            let right = self.equality()?;
            expr = Rc::new(Logical::new(expr, operator, right));
        }

        Ok(expr)
    }

    fn equality(&mut self) -> Result<Rc<dyn Expr>, ParseError> {
        let mut expr = self.comparison()?;

        while self.match_token_types(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous();
            let right = self.comparison()?;
            expr = Rc::new(Binary::new(expr, operator, right));
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Rc<dyn Expr>, ParseError> {
        let mut expr = self.term()?;

        while self.match_token_types(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous();
            let right = self.term()?;
            expr = Rc::new(Binary::new(expr, operator, right));
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Rc<dyn Expr>, ParseError> {
        let mut expr = self.factor()?;

        while self.match_token_types(&[TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous();
            let right = self.factor()?;
            expr = Rc::new(Binary::new(expr, operator, right));
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Rc<dyn Expr>, ParseError> {
        let mut expr = self.unary()?;

        while self.match_token_types(&[TokenType::Slash, TokenType::Star]) {
            let operator = self.previous();
            let right = self.unary()?;
            expr = Rc::new(Binary::new(expr, operator, right));
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Rc<dyn Expr>, ParseError> {
        if self.match_token_types(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous();
            let right = self.unary()?;
            Ok(Rc::new(Unary::new(operator, right)))
        } else {
            self.call()
        }
    }

    fn call(&mut self) -> Result<Rc<dyn Expr>, ParseError> {
        let mut expr = self.primary()?;

        loop {
            if self.match_token_types(&[TokenType::LeftParen]) {
                expr = self.finish_call(expr)?;
            } else if self.match_token_types(&[TokenType::Dot]) {
                let name =
                    self.consume(TokenType::Identifier, "Expect property name after '.'.")?;
                expr = Rc::new(Get::new(expr, name));
            } else {
                break;
            }
        }

        Ok(expr)
    }

    fn finish_call(&mut self, callee: Rc<dyn Expr>) -> Result<Rc<dyn Expr>, ParseError> {
        let mut arguments: Vec<Rc<dyn Expr>> = Vec::new();

        if !self.check(&TokenType::RightParen) {
            loop {
                if arguments.len() >= 255 {
                    error(
                        self.peek(),
                        String::from("Cannot have more than 255 arguments."),
                    );
                }

                arguments.push(self.expression()?);

                if !self.match_token_types(&[TokenType::Comma]) {
                    break;
                }
            }
        }

        let paren = self.consume(TokenType::RightParen, "Expect ')' after arguments.")?;
        Ok(Rc::new(Call::new(callee, paren, arguments)))
    }

    fn primary(&mut self) -> Result<Rc<dyn Expr>, ParseError> {
        if self.match_token_types(&[TokenType::False]) {
            Ok(Rc::new(Literal::new(Object::Bool(false))))
        } else if self.match_token_types(&[TokenType::True]) {
            Ok(Rc::new(Literal::new(Object::Bool(true))))
        } else if self.match_token_types(&[TokenType::Nil]) {
            Ok(Rc::new(Literal::new(Object::Nil)))
        } else if self.match_token_types(&[TokenType::Number, TokenType::String]) {
            Ok(Rc::new(Literal::new(self.previous().literal().clone())))
        } else if self.match_token_types(&[TokenType::Super]) {
            let keyword = self.previous();
            self.consume(TokenType::Dot, "Expect '.' after 'super'.")?;
            let method = self.consume(TokenType::Identifier, "Expect superclass method name.")?;
            Ok(Rc::new(Super::new(keyword, method)))
        } else if self.match_token_types(&[TokenType::This]) {
            Ok(Rc::new(This::new(self.previous())))
        } else if self.match_token_types(&[TokenType::Identifier]) {
            Ok(Rc::new(Variable::new(self.previous().to_owned())))
        } else if self.match_token_types(&[TokenType::LeftParen]) {
            let expr = self.expression()?;
            self.consume(TokenType::RightParen, "Expect ')' after expression.")?;
            Ok(Rc::new(Grouping::new(expr)))
        } else {
            let error = self.error(self.peek(), "Expect expression.");
            Err(error)
        }
    }
}
