use std::rc::Rc;

use crate::{ast::expr::Expr, scanner::token::Object, utils::error::Error};

use super::Stmt;

pub struct Expression {
    expression: Rc<dyn Expr>,
}

impl Expression {
    pub fn new(expression: Rc<dyn Expr>) -> Self {
        Self { expression }
    }

    pub fn expression(&self) -> &dyn Expr {
        self.expression.as_ref()
    }
}

impl Stmt for Expression {
    fn accept(&self, visitor: &mut dyn super::Visitor) -> Result<Object, Error> {
        visitor.visit_expression_stmt(self)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
