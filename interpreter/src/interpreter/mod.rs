pub mod environment;
mod error;

use std::{cell::RefCell, ops::Neg};

use environment::Environment;
use error::runtime_error;

use crate::{
    ast::{
        assign::Assign,
        binary::Binary,
        block::Block,
        expr::{self, Expr},
        expression::Expression,
        grouping::Grouping,
        literal::Literal,
        print::Print,
        stmt::{self, Stmt},
        unary::Unary,
        var::Var,
        variable::Variable,
    },
    scanner::{token::Object, token_type::TokenType},
    utils::error::{Error, RuntimeError},
};

pub struct Interpreter {
    environment: Environment,
}

impl Interpreter {
    pub fn new(environment: Environment) -> Self {
        Self { environment }
    }

    pub fn interpret(&mut self, statements: Vec<Box<dyn Stmt>>) {
        for statement in statements {
            if let Err(e) = execute(statement.as_ref(), self) {
                match e {
                    Error::RuntimeError(e) => runtime_error(e),
                    _ => println!("{}", e),
                }
            }
        }
    }

    fn execute_block(
        &mut self,
        statements: &Vec<Box<dyn Stmt>>,
        environment: Environment,
    ) -> Result<(), Error> {
        let previous = self.environment.clone();
        self.environment = environment;
        for statement in statements {
            if let Err(e) = execute(statement.as_ref(), self) {
                self.environment = previous;
                return Err(e);
            }
        }
        self.environment = previous;
        Ok(())
    }
}

impl expr::Visitor<Object> for Interpreter {
    fn visit_assign_expr(&mut self, expr: &Assign<Object>) -> Result<Object, Error> {
        let value = evaluate(expr.value(), self)?;
        self.environment.assign(expr.name(), value.clone())?;
        Ok(value)
    }

    fn visit_binary_expr(&mut self, expr: &Binary<Object>) -> Result<Object, Error> {
        let left = evaluate(expr.left(), self)?;
        let right = evaluate(expr.right(), self)?;

        match expr.operator().token_type() {
            TokenType::Minus => {
                if let (Object::Num(l), Object::Num(r)) = (left, right) {
                    Ok(Object::Num(l - r))
                } else {
                    Err(RuntimeError::new(
                        String::from("Both operands must be numbers"),
                        expr.operator().to_owned(),
                    )
                    .into())
                }
            }
            TokenType::Slash => {
                if let (Object::Num(l), Object::Num(r)) = (left, right) {
                    Ok(Object::Num(l / r))
                } else {
                    Err(RuntimeError::new(
                        String::from("Both operands must be numbers"),
                        expr.operator().to_owned(),
                    )
                    .into())
                }
            }
            TokenType::Star => {
                if let (Object::Num(l), Object::Num(r)) = (left, right) {
                    Ok(Object::Num(l * r))
                } else {
                    Err(RuntimeError::new(
                        String::from("Both operands must be numbers"),
                        expr.operator().to_owned(),
                    )
                    .into())
                }
            }
            TokenType::Plus => match (left, right) {
                (Object::Num(l), Object::Num(r)) => Ok(Object::Num(l + r)),
                (Object::Str(l), Object::Str(r)) => Ok(Object::Str(format!("{}{}", l, r))),
                (Object::Str(l), Object::Num(r)) => Ok(Object::Str(format!("{}{}", l, r))),
                (Object::Num(l), Object::Str(r)) => Ok(Object::Str(format!("{}{}", l, r))),
                _ => Err(RuntimeError::new(
                    String::from("Both operands must be numbers or strings"),
                    expr.operator().to_owned(),
                )
                .into()),
            },
            TokenType::Greater => {
                if let (Object::Num(l), Object::Num(r)) = (left, right) {
                    Ok(Object::Bool(l > r))
                } else {
                    Err(RuntimeError::new(
                        String::from("Both operands must be numbers"),
                        expr.operator().to_owned(),
                    )
                    .into())
                }
            }
            TokenType::GreaterEqual => {
                if let (Object::Num(l), Object::Num(r)) = (left, right) {
                    Ok(Object::Bool(l >= r))
                } else {
                    Err(RuntimeError::new(
                        String::from("Both operands must be numbers"),
                        expr.operator().to_owned(),
                    )
                    .into())
                }
            }
            TokenType::Less => {
                if let (Object::Num(l), Object::Num(r)) = (left, right) {
                    Ok(Object::Bool(l < r))
                } else {
                    Err(RuntimeError::new(
                        String::from("Both operands must be numbers"),
                        expr.operator().to_owned(),
                    )
                    .into())
                }
            }
            TokenType::LessEqual => {
                if let (Object::Num(l), Object::Num(r)) = (left, right) {
                    Ok(Object::Bool(l <= r))
                } else {
                    Err(RuntimeError::new(
                        String::from("Both operands must be numbers"),
                        expr.operator().to_owned(),
                    )
                    .into())
                }
            }
            TokenType::BangEqual => Ok(Object::Bool(!is_equal(left, right))),
            TokenType::EqualEqual => Ok(Object::Bool(is_equal(left, right))),
            _ => Ok(Object::Nil),
        }
    }

    fn visit_grouping_expr(&mut self, expr: &Grouping<Object>) -> Result<Object, Error> {
        evaluate(expr.expression(), self)
    }

    fn visit_literal_expr(&mut self, expr: &Literal) -> Result<Object, Error> {
        Ok(expr.value.to_owned())
    }

    fn visit_unary_expr(&mut self, expr: &Unary<Object>) -> Result<Object, Error> {
        let right = evaluate(expr.right(), self)?;

        let result = match expr.operator().token_type() {
            TokenType::Minus => {
                if let Object::Num(n) = right {
                    Ok(Object::Num(n.neg()))
                } else {
                    Err(RuntimeError::new(
                        String::from("Unary minus must be applied to a number"),
                        expr.operator().to_owned(),
                    )
                    .into())
                }
            }
            TokenType::Bang => Ok(Object::Bool(!right.is_truthy())),
            _ => Ok(Object::Nil),
        };

        result
    }

    fn visit_variable_expr(&mut self, expr: &Variable) -> Result<Object, Error> {
        self.environment.get(expr.name()).map(|v| v.to_owned())
    }
}

impl stmt::Visitor for Interpreter {
    fn visit_block_stmt(&mut self, stmt: &Block) -> Result<(), Error> {
        let inner_environment =
            Environment::new(Some(RefCell::new(self.environment.clone()).into()));
        self.execute_block(&stmt.statements, inner_environment)
    }

    fn visit_expression_stmt(&mut self, stmt: &Expression) -> Result<(), Error> {
        evaluate(stmt.expression(), self)?;
        Ok(())
    }

    fn visit_print_stmt(&mut self, stmt: &Print) -> Result<(), Error> {
        let value = evaluate(stmt.expression(), self)?;
        println!("{}", value);
        Ok(())
    }

    #[allow(unused_variables)]
    fn visit_var_stmt(&mut self, stmt: &Var) -> Result<(), Error> {
        let value = if let Some(initializer) = stmt.initializer() {
            evaluate(initializer.as_ref(), self)?
        } else {
            Object::Nil
        };

        self.environment
            .define(stmt.name().lexeme().to_owned(), value);
        Ok(())
    }
}

impl Default for Interpreter {
    fn default() -> Self {
        let environment = Environment::new(None);
        Self::new(environment)
    }
}

fn evaluate(expr: &dyn Expr<Object>, visitor: &mut Interpreter) -> Result<Object, Error> {
    expr.accept(visitor)
}

fn execute(stmt: &dyn Stmt, visitor: &mut Interpreter) -> Result<(), Error> {
    stmt.accept(visitor)
}

fn is_equal(a: Object, b: Object) -> bool {
    if a == Object::Nil && b == Object::Nil {
        return true;
    }

    if a == Object::Nil {
        return false;
    }

    a == b
}
