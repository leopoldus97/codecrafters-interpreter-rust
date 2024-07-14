use super::{expr::Expr, stmt::{self, Stmt}};

pub struct Expression<R, E> {
    expression: Box<dyn Expr<R, E>>,
}

impl<R, E> Expression<R, E> {
    pub fn new(expression: Box<dyn Expr<R, E>>) -> Self {
        Self { expression }
    }

    pub fn expression(&self) -> &dyn Expr<R, E> {
        self.expression.as_ref()
    }
}

impl<R, E> Stmt<R, E> for Expression<R, E> {
    fn accept(&self, visitor: &mut dyn stmt::Visitor<R, E>) -> Result<R, E> {
        visitor.visit_expression_stmt(self)
    }
}