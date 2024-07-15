use crate::{scanner::token::Token, utils::error::Error};

use super::expr::{self, Expr};

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

    pub fn right(&self) -> &dyn Expr<R> {
        self.right.as_ref()
    }
}

impl<R: 'static> Expr<R> for Unary<R> {
    fn accept(&self, visitor: &mut dyn expr::Visitor<R>) -> Result<R, Error> {
        visitor.visit_unary_expr(self)
    }
    
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
