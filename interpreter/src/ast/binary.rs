use crate::scanner::token::Token;

use super::Expr;

pub struct Binary<R, E> {
    left: Box<dyn Expr<R, E>>,
    operator: Token,
    right: Box<dyn Expr<R, E>>,
}

impl<R, E> Binary<R, E> {
    pub fn new(left: Box<dyn Expr<R, E>>, operator: Token, right: Box<dyn Expr<R, E>>) -> Self {
        Self {
            left,
            operator,
            right,
        }
    }

    pub fn left(&self) -> &dyn Expr<R, E> {
        self.left.as_ref()
    }

    pub fn operator(&self) -> &Token {
        &self.operator
    }

    pub fn right(&self) -> &dyn Expr<R, E> {
        self.right.as_ref()
    }
}

impl<R, E> Expr<R, E> for Binary<R, E> {
    fn accept(&self, visitor: &mut dyn crate::ast::Visitor<R, E>) -> Result<R, E> {
        visitor.visit_binary_expr(self)
    }
}
