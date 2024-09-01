use std::rc::Rc;

use crate::scanner::token::Token;

use super::Expr;

pub struct Get {
    object: Rc<dyn Expr>,
    name: Token,
}

impl Get {
    pub fn new(object: Rc<dyn Expr>, name: Token) -> Self {
        Self { object, name }
    }

    pub fn object(&self) -> Rc<dyn Expr> {
        Rc::clone(&self.object)
    }

    pub fn name(&self) -> &Token {
        &self.name
    }
}

impl Expr for Get {
    fn accept(
        &self,
        visitor: &mut dyn super::Visitor,
    ) -> Result<crate::scanner::token::Object, crate::utils::error::Error> {
        visitor.visit_get_expr(self)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
