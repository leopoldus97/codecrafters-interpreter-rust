use crate::utils::error::Error;

use super::Expr;

pub struct Grouping<R> {
    expression: Box<dyn Expr<R>>,
}

impl<R> Grouping<R> {
    pub fn new(expression: Box<dyn Expr<R>>) -> Self {
        Self { expression }
    }

    pub fn expression(&self) -> &dyn Expr<R> {
        self.expression.as_ref()
    }
}

impl<R: 'static> Expr<R> for Grouping<R> {
    fn accept(&self, visitor: &mut dyn super::Visitor<R>) -> Result<R, Error> {
        visitor.visit_grouping_expr(self)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}