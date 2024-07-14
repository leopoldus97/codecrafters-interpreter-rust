use crate::scanner::token::Token;

use super::expr::{self, Expr};

pub struct Unary<R, E> {
    operator: Token,
    right: Box<dyn Expr<R, E>>,
}

impl<R, E> Unary<R, E> {
    pub fn new(operator: Token, right: Box<dyn Expr<R, E>>) -> Self {
        Self { operator, right }
    }

    pub fn operator(&self) -> &Token {
        &self.operator
    }

    pub fn right(&self) -> &dyn Expr<R, E> {
        self.right.as_ref()
    }
}

impl<R, E> Expr<R, E> for Unary<R, E> {
    fn accept(&self, visitor: &mut dyn expr::Visitor<R, E>) -> Result<R, E> {
        visitor.visit_unary_expr(self)
    }
}
