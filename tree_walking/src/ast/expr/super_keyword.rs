use crate::{
    scanner::token::{Object, Token},
    utils::{error::Error, next_id},
};

use super::Expr;

#[derive(Clone)]
pub struct Super {
    keyword: Token,
    method: Token,
    id: u64,
}

impl Super {
    pub fn new(keyword: Token, method: Token) -> Self {
        Self {
            keyword,
            method,
            id: next_id(),
        }
    }

    pub fn keyword(&self) -> &Token {
        &self.keyword
    }

    pub fn method(&self) -> &Token {
        &self.method
    }
}

impl Expr for Super {
    fn id(&self) -> u64 {
        self.id
    }

    fn accept(&self, visitor: &mut dyn super::Visitor) -> Result<Object, Box<Error>> {
        visitor.visit_super_expr(self)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
