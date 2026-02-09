use crate::{
    scanner::token::{Object, Token},
    utils::{error::Error, next_id},
};

use super::Expr;

#[derive(Clone)]
pub struct This {
    keyword: Token,
    id: u64,
}

impl This {
    pub fn new(keyword: Token) -> Self {
        Self {
            keyword,
            id: next_id(),
        }
    }

    pub fn keyword(&self) -> &Token {
        &self.keyword
    }
}

impl Expr for This {
    fn id(&self) -> u64 {
        self.id
    }

    fn accept(&self, visitor: &mut dyn super::Visitor) -> Result<Object, Box<Error>> {
        visitor.visit_this_expr(self)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
