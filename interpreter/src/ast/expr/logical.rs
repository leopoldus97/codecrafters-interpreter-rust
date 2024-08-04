use crate::{
    scanner::token::{Object, Token},
    utils::error::Error,
};

use super::Expr;

    pub left: Box<dyn Expr<R>>,
pub struct Logical {
    pub operator: Token,
    pub right: Box<dyn Expr<R>>,
}

    pub fn new(left: Box<dyn Expr<R>>, operator: Token, right: Box<dyn Expr<R>>) -> Self {
impl Logical {
        Self {
            left,
            operator,
            right,
        }
    }

    pub fn left(&self) -> &dyn Expr {
        self.left.as_ref()
    }

    pub fn operator(&self) -> &Token {
        &self.operator
    }

    pub fn right(&self) -> &dyn Expr {
        self.right.as_ref()
    }
}

impl Expr for Logical {
    fn accept(&self, visitor: &mut dyn super::Visitor) -> Result<Object, Error> {
        visitor.visit_logical_expr(self)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
