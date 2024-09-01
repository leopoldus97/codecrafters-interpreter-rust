use std::rc::Rc;

use crate::{
    scanner::token::{Object, Token},
    utils::error::Error,
};

use super::Expr;

pub struct Call {
    callee: Rc<dyn Expr>,
    paren: Token,
    arguments: Vec<Rc<dyn Expr>>,
}

impl Call {
    pub fn new(callee: Rc<dyn Expr>, paren: Token, arguments: Vec<Rc<dyn Expr>>) -> Self {
        Self {
            callee,
            paren,
            arguments,
        }
    }

    pub fn callee(&self) -> &dyn Expr {
        self.callee.as_ref()
    }

    pub fn paren(&self) -> &Token {
        &self.paren
    }

    pub fn arguments(&self) -> &Vec<Rc<dyn Expr>> {
        &self.arguments
    }
}

impl Expr for Call {
    fn accept(&self, visitor: &mut dyn super::Visitor) -> Result<Object, Error> {
        visitor.visit_call_expr(self)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
