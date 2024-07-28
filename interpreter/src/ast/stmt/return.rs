use crate::{
    ast::expr::Expr,
    scanner::token::{Object, Token},
    utils::error::Error,
};

use super::Stmt;

pub struct Return {
    keyword: Token,
    value: Option<Box<dyn Expr<Object>>>,
}

impl Return {
    pub fn new(keyword: Token, value: Option<Box<dyn Expr<Object>>>) -> Self {
        Self { keyword, value }
    }

    pub fn keyword(&self) -> &Token {
        &self.keyword
    }

    pub fn value(&self) -> &Option<Box<dyn Expr<Object>>> {
        &self.value
    }
}

impl Stmt for Return {
    fn accept(&self, visitor: &mut dyn super::Visitor) -> Result<(), Error> {
        visitor.visit_return_stmt(self)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
