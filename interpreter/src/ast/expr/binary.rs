use std::rc::Rc;

use crate::{
    scanner::token::{Object, Token},
    utils::error::Error,
};

use super::Expr;

pub struct Binary {
    left: Rc<dyn Expr>,
    operator: Token,
    right: Rc<dyn Expr>,
}

impl Binary {
    pub fn new(left: Rc<dyn Expr>, operator: Token, right: Rc<dyn Expr>) -> Self {
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

impl Expr for Binary {
    fn accept(&self, visitor: &mut dyn super::Visitor) -> Result<Object, Error> {
        visitor.visit_binary_expr(self)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
