use crate::scanner::token::Token;

use super::Expr;

#[derive(Clone)]
pub struct Super {
    keyword: Token,
    method: Token,
}

impl Super {
    pub fn new(keyword: Token, method: Token) -> Self {
        Self { keyword, method }
    }

    pub fn keyword(&self) -> &Token {
        &self.keyword
    }

    pub fn method(&self) -> &Token {
        &self.method
    }
}

impl Expr for Super {
    fn accept(
        &self,
        visitor: &mut dyn super::Visitor,
    ) -> Result<crate::scanner::token::Object, crate::utils::error::Error> {
        visitor.visit_super_expr(self)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
