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

impl Expr for Literal {
    fn accept(&self, visitor: &mut dyn super::Visitor) -> Result<Object, Box<Error>> {
        visitor.visit_literal_expr(self)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
