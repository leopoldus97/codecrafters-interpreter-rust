use crate::scanner::token::Token;

use super::Expr;

pub struct Unary<R> {
    operator: Token,
    right: Box<dyn Expr<R>>,
}

impl<R> Unary<R> {
    pub fn new(operator: Token, right: Box<dyn Expr<R>>) -> Self {
        Self { operator, right }
    }

    pub fn operator(&self) -> &Token {
        &self.operator
    }

    pub fn right(&self) -> &Box<dyn Expr<R>> {
        &self.right
    }
}

impl<R> Expr<R> for Unary<R> {
    fn accept(&self, visitor: &mut dyn crate::ast::Visitor<R>) -> R {
        visitor.visit_unary_expr(self)
    }
}