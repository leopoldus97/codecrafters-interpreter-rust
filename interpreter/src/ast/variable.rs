use crate::{scanner::token::Token, utils::error::Error};

use super::expr::{self, Expr};

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

impl<R> Expr<R> for Variable {
    fn accept(&self, visitor: &mut dyn expr::Visitor<R>) -> Result<R, Error> {
        visitor.visit_variable_expr(self)
    }
}