use super::expr::{self, Expr};

pub struct Grouping<R, E> {
    expression: Box<dyn Expr<R, E>>,
}

impl<R, E> Grouping<R, E> {
    pub fn new(expression: Box<dyn Expr<R, E>>) -> Self {
        Self { expression }
    }

    pub fn expression(&self) -> &dyn Expr<R, E> {
        self.expression.as_ref()
    }
}

impl<R, E> Expr<R, E> for Grouping<R, E> {
    fn accept(&self, visitor: &mut dyn expr::Visitor<R, E>) -> Result<R, E> {
        visitor.visit_grouping_expr(self)
    }
}
