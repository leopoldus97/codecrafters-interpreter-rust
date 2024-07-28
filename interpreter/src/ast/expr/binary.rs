use crate::{scanner::token::Token, utils::error::Error};

use super::Expr;

pub struct Binary<R> {
    left: Box<dyn Expr<R>>,
    operator: Token,
    right: Box<dyn Expr<R>>,
}

impl<R> Binary<R> {
    pub fn new(left: Box<dyn Expr<R>>, operator: Token, right: Box<dyn Expr<R>>) -> Self {
        Self {
            left,
            operator,
            right,
        }
    }

    pub fn left(&self) -> &dyn Expr<R> {
        self.left.as_ref()
    }

    pub fn operator(&self) -> &Token {
        &self.operator
    }

    pub fn right(&self) -> &dyn Expr<R> {
        self.right.as_ref()
    }
}

impl<R: 'static> Expr<R> for Binary<R> {
    fn accept(&self, visitor: &mut dyn super::Visitor<R>) -> Result<R, Error> {
        visitor.visit_binary_expr(self)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
