use crate::{
    scanner::token::{Object, Token},
    utils::error::Error,
};

use super::Expr;

#[derive(Clone, PartialEq)]
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

impl Expr for Variable {
    fn accept(&self, visitor: &mut dyn super::Visitor) -> Result<Object, Error> {
        visitor.visit_variable_expr(self)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl std::fmt::Display for Variable {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}
