use crate::{scanner::token::Token, utils::error::Error};

use super::expr::{self, Expr};

pub struct Assign<R> {
    pub name: Token,
    pub value: Box<dyn Expr<R>>,
}

impl<R> Assign<R> {
    pub fn new(name: Token, value: Box<dyn Expr<R>>) -> Self {
        Self { name, value }
    }

    pub fn name(&self) -> &Token {
        &self.name
    }

    pub fn value(&self) -> &dyn Expr<R> {
        self.value.as_ref()
    }
}

impl<R: 'static> Expr<R> for Assign<R> {
    fn accept(&self, visitor: &mut dyn expr::Visitor<R>) -> Result<R, Error> {
        visitor.visit_assign_expr(self)
    }
}