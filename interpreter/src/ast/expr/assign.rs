use std::rc::Rc;

use crate::{
    scanner::token::{Object, Token},
    utils::error::Error,
};

use super::Expr;

pub struct Assign {
    name: Token,
    value: Rc<dyn Expr>,
}

impl Assign {
    pub fn new(name: Token, value: Rc<dyn Expr>) -> Self {
        Self { name, value }
    }

    pub fn name(&self) -> &Token {
        &self.name
    }

    pub fn value(&self) -> &dyn Expr {
        self.value.as_ref()
    }
}

impl Expr for Assign {
    fn accept(&self, visitor: &mut dyn super::Visitor) -> Result<Object, Error> {
        visitor.visit_assign_expr(self)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
