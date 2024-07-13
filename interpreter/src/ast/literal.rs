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

impl<R, E> crate::ast::Expr<R, E> for Literal {
    fn accept(&self, visitor: &mut dyn crate::ast::Visitor<R, E>) -> Result<R, E> {
        visitor.visit_literal_expr(self)
    }
}
