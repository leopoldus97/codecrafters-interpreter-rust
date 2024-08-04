use std::rc::Rc;

use crate::{
    ast::expr::Expr,
    scanner::token::{Object, Token},
    utils::error::Error,
};

use super::Stmt;

pub struct Var {
    name: Token,
    initializer: Option<Rc<dyn Expr>>,
}

impl Var {
    pub fn new(name: Token, initializer: Option<Rc<dyn Expr>>) -> Self {
        Self { name, initializer }
    }

    pub fn name(&self) -> &Token {
        &self.name
    }

    pub fn initializer(&self) -> &Option<Rc<dyn Expr>> {
        &self.initializer
    }
}

impl Stmt for Var {
    fn accept(&self, visitor: &mut dyn super::Visitor) -> Result<Object, Error> {
        visitor.visit_var_stmt(self)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
