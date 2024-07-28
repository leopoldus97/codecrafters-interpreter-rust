use crate::{scanner::token::Object, utils::error::Error};

use super::Expr;

pub struct Literal {
    value: Object,
}

impl Literal {
    pub fn new(value: Object) -> Self {
        Self { value }
    }

    pub fn value(&self) -> &Object {
        &self.value
    }
}

impl<R: 'static> Expr<R> for Literal {
    fn accept(&self, visitor: &mut dyn super::Visitor<R>) -> Result<R, Error> {
        visitor.visit_literal_expr(self)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
