use crate::scanner::token::Object;

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

impl<R> crate::ast::Expr<R> for Literal {
    fn accept(&self, visitor: &mut dyn crate::ast::Visitor<R>) -> R {
        visitor.visit_literal_expr(self)
    }
}