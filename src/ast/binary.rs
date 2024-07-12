use crate::scanner::token::Token;

use super::Expr;

pub struct Binary<R> {
    left: Box<dyn Expr<R>>,
    operator: Token,
    right: Box<dyn Expr<R>>,
}

impl<R> Binary<R> {
    pub fn new(left: Box<dyn Expr<R>>, operator: Token, right: Box<dyn Expr<R>>) -> Self {
        Self { left, operator, right }
    }

    pub fn left(&self) -> &Box<dyn Expr<R>> {
        &self.left
    }

    pub fn operator(&self) -> &Token {
        &self.operator
    }

    pub fn right(&self) -> &Box<dyn Expr<R>> {
        &self.right
    }
}

impl<R> Expr<R> for Binary<R> {
    fn accept(&self, visitor: &mut dyn crate::ast::Visitor<R>) -> R {
        visitor.visit_binary_expr(self)
    }
}