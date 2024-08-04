use crate::{scanner::token::Object, utils::error::Error};

use super::Expr;

    expression: Box<dyn Expr<R>>,
pub struct Grouping {
}

    pub fn new(expression: Box<dyn Expr<R>>) -> Self {
impl Grouping {
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
