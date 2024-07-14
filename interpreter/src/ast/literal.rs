use crate::scanner::token::Object;

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

impl<R, E> Expr<R, E> for Literal {
    fn accept(&self, visitor: &mut dyn expr::Visitor<R, E>) -> Result<R, E> {
        visitor.visit_literal_expr(self)
    }
}
