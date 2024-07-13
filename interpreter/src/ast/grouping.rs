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

impl<R> Expr<R> for Grouping<R> {
    fn accept(&self, visitor: &mut dyn crate::ast::Visitor<R>) -> R {
        visitor.visit_grouping_expr(self)
    }
}