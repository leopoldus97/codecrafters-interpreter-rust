use crate::{scanner::token::Object, utils::error::Error};

use super::expr::{self, Expr};

pub struct Literal {
    pub value: Object,
}

impl Literal {
    pub fn new(value: Object) -> Self {
        Self { value }
    }

    pub fn value(&self) -> &Object {
        &self.value
    }
}

impl<R> Expr<R> for Literal {
    fn accept(&self, visitor: &mut dyn expr::Visitor<R>) -> Result<R, Error> {
        visitor.visit_literal_expr(self)
    }
}
