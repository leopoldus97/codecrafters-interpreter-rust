mod error;

use std::ops::Neg;

use error::{runtime_error, RuntimeError};

use crate::{
    ast::{binary::Binary, expr::{self, Expr}, expression::Expression, grouping::Grouping, literal::Literal, print::Print, stmt, unary::Unary},
    scanner::{token::Object, token_type::TokenType},
};

pub struct Interpreter {}

impl Interpreter {
    pub fn new() -> Self {
        Self {}
    }

    pub fn interpret(&mut self, expr: Box<dyn Expr<Object, RuntimeError>>) {
        match evaluate(&*expr) {
            Ok(value) => println!("{}", value),
            Err(e) => runtime_error(e),
        }
    }
}

impl expr::Visitor<Object, RuntimeError> for Interpreter {
    fn visit_binary_expr(
        &mut self,
        expr: &Binary<Object, RuntimeError>,
    ) -> Result<Object, RuntimeError> {
        let left = evaluate(expr.left())?;
        let right = evaluate(expr.right())?;

        match expr.operator().token_type() {
            TokenType::Minus => {
                if let (Object::Num(l), Object::Num(r)) = (left, right) {
                    Ok(Object::Num(l - r))
                } else {
                    Err(RuntimeError::new(
                        String::from("Both operands must be numbers"),
                        expr.operator().to_owned(),
                    ))
                }
            }
            TokenType::Slash => {
                if let (Object::Num(l), Object::Num(r)) = (left, right) {
                    Ok(Object::Num(l / r))
                } else {
                    Err(RuntimeError::new(
                        String::from("Both operands must be numbers"),
                        expr.operator().to_owned(),
                    ))
                }
            }
            TokenType::Star => {
                if let (Object::Num(l), Object::Num(r)) = (left, right) {
                    Ok(Object::Num(l * r))
                } else {
                    Err(RuntimeError::new(
                        String::from("Both operands must be numbers"),
                        expr.operator().to_owned(),
                    ))
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
                )),
            },
            TokenType::Greater => {
                if let (Object::Num(l), Object::Num(r)) = (left, right) {
                    Ok(Object::Bool(l > r))
                } else {
                    Err(RuntimeError::new(
                        String::from("Both operands must be numbers"),
                        expr.operator().to_owned(),
                    ))
                }
            }
            TokenType::GreaterEqual => {
                if let (Object::Num(l), Object::Num(r)) = (left, right) {
                    Ok(Object::Bool(l >= r))
                } else {
                    Err(RuntimeError::new(
                        String::from("Both operands must be numbers"),
                        expr.operator().to_owned(),
                    ))
                }
            }
            TokenType::Less => {
                if let (Object::Num(l), Object::Num(r)) = (left, right) {
                    Ok(Object::Bool(l < r))
                } else {
                    Err(RuntimeError::new(
                        String::from("Both operands must be numbers"),
                        expr.operator().to_owned(),
                    ))
                }
            }
            TokenType::LessEqual => {
                if let (Object::Num(l), Object::Num(r)) = (left, right) {
                    Ok(Object::Bool(l <= r))
                } else {
                    Err(RuntimeError::new(
                        String::from("Both operands must be numbers"),
                        expr.operator().to_owned(),
                    ))
                }
            }
            TokenType::BangEqual => Ok(Object::Bool(!is_equal(left, right))),
            TokenType::EqualEqual => Ok(Object::Bool(is_equal(left, right))),
            _ => Ok(Object::Nil),
        }
    }

    fn visit_grouping_expr(
        &mut self,
        expr: &Grouping<Object, RuntimeError>,
    ) -> Result<Object, RuntimeError> {
        evaluate(expr.expression())
    }

    fn visit_literal_expr(&mut self, expr: &Literal) -> Result<Object, RuntimeError> {
        Ok(expr.value.to_owned())
    }

    fn visit_unary_expr(
        &mut self,
        expr: &Unary<Object, RuntimeError>,
    ) -> Result<Object, RuntimeError> {
        let right = evaluate(expr.right())?;

        let result = match expr.operator().token_type() {
            TokenType::Minus => {
                if let Object::Num(n) = right {
                    Ok(Object::Num(n.neg()))
                } else {
                    Err(RuntimeError::new(
                        String::from("Unary minus must be applied to a number"),
                        expr.operator().to_owned(),
                    ))
                }
            }
            TokenType::Bang => Ok(Object::Bool(!right.is_truthy())),
            _ => Ok(Object::Nil),
        };

        result
    }
}

impl stmt::Visitor<(), RuntimeError> for Interpreter {
    fn visit_expression_stmt(&mut self, stmt: &Expression<(), RuntimeError>) -> Result<(), RuntimeError> {
        evaluate(stmt.expression())?;
        Ok(())
    }

    fn visit_print_stmt(&mut self, stmt: &Print<(), RuntimeError>) -> Result<(), RuntimeError> {
        let value = evaluate(stmt.expression())?;
        println!("{}", value);
        Ok(())
    }

}

impl Default for Interpreter {
    fn default() -> Self {
        Self::new()
    }
}

fn evaluate(expr: &dyn Expr<Object, RuntimeError>) -> Result<Object, RuntimeError> {
    expr.accept(&mut Interpreter {})
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
