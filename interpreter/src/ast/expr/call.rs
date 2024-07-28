use crate::{scanner::token::Token, utils::error::Error};

use super::Expr;

pub struct Call<R> {
    callee: Box<dyn Expr<R>>,
    paren: Token,
    arguments: Vec<Box<dyn Expr<R>>>,
}

impl<R> Call<R> {
    pub fn new(callee: Box<dyn Expr<R>>, paren: Token, arguments: Vec<Box<dyn Expr<R>>>) -> Self {
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
