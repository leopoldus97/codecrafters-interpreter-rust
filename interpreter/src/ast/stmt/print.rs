use crate::{ast::expr::Expr, scanner::token::Object, utils::error::Error};

use super::Stmt;

pub struct Print {
    expression: Box<dyn Expr<Object>>,
}

impl Print {
    pub fn new(expression: Box<dyn Expr<Object>>) -> Self {
        Self { expression }
    }

    pub fn expression(&self) -> &dyn Expr<Object> {
        self.expression.as_ref()
    }
}

impl Stmt for Print {
    fn accept(&self, visitor: &mut dyn super::Visitor) -> Result<Object, Error> {
        visitor.visit_print_stmt(self)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
