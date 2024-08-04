use crate::{
    scanner::token::{Object, Token},
    utils::error::Error,
};

use super::Expr;

    callee: Box<dyn Expr<R>>,
pub struct Call {
    paren: Token,
    arguments: Vec<Box<dyn Expr<R>>>,
}

    pub fn new(callee: Box<dyn Expr<R>>, paren: Token, arguments: Vec<Box<dyn Expr<R>>>) -> Self {
impl Call {
        Self {
            callee,
            paren,
            arguments,
        }
    }

    pub fn callee(&self) -> &dyn Expr<R> {
        self.callee.as_ref()
    }

    pub fn paren(&self) -> &Token {
        &self.paren
    }

    pub fn arguments(&self) -> &Vec<Box<dyn Expr<R>>> {
        &self.arguments
    }
}

impl<R: 'static> Expr<R> for Call<R> {
    fn accept(&self, visitor: &mut dyn super::Visitor<R>) -> Result<R, Error> {
        visitor.visit_call_expr(self)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
