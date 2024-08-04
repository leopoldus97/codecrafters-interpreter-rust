use crate::{
    scanner::token::{Object, Token},
    utils::error::Error,
};

use super::Expr;

pub struct Unary {
    operator: Token,
    right: Box<dyn Expr<R>>,
}

    pub fn new(operator: Token, right: Box<dyn Expr<R>>) -> Self {
impl Unary {
        Self { operator, right }
    }

    pub fn operator(&self) -> &Token {
        &self.operator
    }

    pub fn right(&self) -> &dyn Expr {
        self.right.as_ref()
    }
}

impl Expr for Unary {
    fn accept(&self, visitor: &mut dyn super::Visitor) -> Result<Object, Error> {
        visitor.visit_unary_expr(self)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
