use std::rc::Rc;

use crate::{scanner::token::Object, utils::error::Error};

use super::Expr;

pub struct Grouping {
    expression: Rc<dyn Expr>,
}

impl Grouping {
    pub fn new(expression: Rc<dyn Expr>) -> Self {
        Self { expression }
    }

    pub fn expression(&self) -> &dyn Expr {
        self.expression.as_ref()
    }
}

impl Expr for Grouping {
    fn accept(&self, visitor: &mut dyn super::Visitor) -> Result<Object, Error> {
        visitor.visit_grouping_expr(self)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
