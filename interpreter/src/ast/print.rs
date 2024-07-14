use super::{expr::Expr, stmt::{self, Stmt}};

pub struct Print<R, E> {
    expression: Box<dyn Expr<R, E>>,
}

impl<R, E> Print<R, E> {
    pub fn new(expression: Box<dyn Expr<R, E>>) -> Self {
        Self { expression }
    }

    pub fn expression(&self) -> &dyn Expr<R, E> {
        self.expression.as_ref()
    }
}

impl<R, E> Stmt<R, E> for Print<R, E> {
    fn accept(&self, visitor: &mut dyn stmt::Visitor<R, E>) -> Result<R, E> {
        visitor.visit_print_stmt(self)
    }
}

