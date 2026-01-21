use std::rc::Rc;

use crate::{
    scanner::token::{Object, Token},
    utils::{error::Error, next_id},
};

use super::Expr;

#[derive(Clone)]
pub struct Assign {
    name: Token,
    value: Rc<dyn Expr>,
    id: u64,
}

impl Assign {
    pub fn new(name: Token, value: Rc<dyn Expr>) -> Self {
        Self {
            name,
            value,
            id: next_id(),
        }
    }

    pub fn name(&self) -> &Token {
        &self.name
    }

    pub fn value(&self) -> &dyn Expr {
        self.value.as_ref()
    }
}

impl Expr for Assign {
    fn id(&self) -> u64 {
        self.id
    }

    fn accept(&self, visitor: &mut dyn super::Visitor) -> Result<Object, Error> {
        visitor.visit_assign_expr(self)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
