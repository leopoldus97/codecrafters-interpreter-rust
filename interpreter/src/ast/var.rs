use crate::{
    scanner::token::{Object, Token},
    utils::error::Error,
};

use super::{
    expr::Expr,
    stmt::{self, Stmt},
};

pub struct Var {
    name: Token,
    initializer: Option<Box<dyn Expr<Object>>>,
}

impl Var {
    pub fn new(name: Token, initializer: Option<Box<dyn Expr<Object>>>) -> Self {
        Self { name, initializer }
    }

    pub fn name(&self) -> &Token {
        &self.name
    }

    pub fn initializer(&self) -> &Option<Box<dyn Expr<Object>>> {
        &self.initializer
    }
}

impl Stmt for Var {
    fn accept(&self, visitor: &mut dyn stmt::Visitor) -> Result<(), Error> {
        visitor.visit_var_stmt(self)
    }
    
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
