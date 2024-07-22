use crate::{scanner::token::Token, utils::error::Error};

use super::Expr;

pub struct Variable {
    name: Token,
}

impl Variable {
    pub fn new(name: Token) -> Self {
        Self { name }
    }

    pub fn name(&self) -> &Token {
        &self.name
    }
}

impl<R: 'static> Expr<R> for Variable {
    fn accept(&self, visitor: &mut dyn super::Visitor<R>) -> Result<R, Error> {
        visitor.visit_variable_expr(self)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}