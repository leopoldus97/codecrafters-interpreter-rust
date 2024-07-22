use crate::{ast::expr::Expr, scanner::token::Object, utils::error::Error};

use super::Stmt;

pub struct Expression {
    expression: Box<dyn Expr<Object>>,
}

impl Expression {
    pub fn new(expression: Box<dyn Expr<Object>>) -> Self {
        Self { expression }
    }

    pub fn expression(&self) -> &dyn Expr<Object> {
        self.expression.as_ref()
    }
}

impl Stmt for Expression {
    fn accept(&self, visitor: &mut dyn super::Visitor) -> Result<(), Error> {
        visitor.visit_expression_stmt(self)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}