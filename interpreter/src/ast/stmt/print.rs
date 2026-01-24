use std::rc::Rc;

use crate::{ast::expr::Expr, scanner::token::Object, utils::error::Error};

use super::Stmt;

pub struct Print {
    expression: Rc<dyn Expr>,
}

impl Print {
    pub fn new(expression: Rc<dyn Expr>) -> Self {
        Self { expression }
    }

    pub fn expression(&self) -> &dyn Expr {
        self.expression.as_ref()
    }
}

impl Stmt for Print {
    fn accept(&self, visitor: &mut dyn super::Visitor) -> Result<Object, Box<Error>> {
        visitor.visit_print_stmt(self)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
