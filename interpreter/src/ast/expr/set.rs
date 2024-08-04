use std::rc::Rc;

use crate::scanner::token::{Object, Token};

use super::Expr;

pub struct Set {
    object: Rc<dyn Expr>,
    name: Token,
    value: Rc<dyn Expr>,
}

impl Set {
    pub fn new(object: Rc<dyn Expr>, name: Token, value: Rc<dyn Expr>) -> Self {
        Self {
            object,
            name,
            value,
        }
    }

    pub fn object(&self) -> Rc<dyn Expr> {
        Rc::clone(&self.object)
    }

    pub fn name(&self) -> &Token {
        &self.name
    }

    pub fn value(&self) -> Rc<dyn Expr> {
        Rc::clone(&self.value)
    }
}

impl Expr for Set {
    fn accept(
        &self,
        visitor: &mut dyn super::Visitor,
    ) -> Result<Object, crate::utils::error::Error> {
        visitor.visit_set_expr(self)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
