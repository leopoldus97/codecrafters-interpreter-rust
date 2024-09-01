use crate::scanner::token::{Object, Token};

use super::Expr;

#[derive(Clone)]
pub struct This {
    keyword: Token,
}

impl This {
    pub fn new(keyword: Token) -> Self {
        Self { keyword }
    }

    pub fn keyword(&self) -> &Token {
        &self.keyword
    }
}

impl Expr for This {
    fn accept(
        &self,
        visitor: &mut dyn super::Visitor,
    ) -> Result<Object, crate::utils::error::Error> {
        visitor.visit_this_expr(self)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
