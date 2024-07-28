use crate::{ast::expr::Expr, scanner::token::Object, utils::error::Error};

use super::Stmt;

pub struct While {
    condition: Box<dyn Expr<Object>>,
    body: Box<dyn Stmt>,
}

impl While {
    pub fn new(condition: Box<dyn Expr<Object>>, body: Box<dyn Stmt>) -> Self {
        Self { condition, body }
    }

    pub fn condition(&self) -> &dyn Expr<Object> {
        self.condition.as_ref()
    }

    pub fn body(&self) -> &dyn Stmt {
        self.body.as_ref()
    }
}

impl Stmt for While {
    fn accept(&self, visitor: &mut dyn super::Visitor) -> Result<(), Error> {
        visitor.visit_while_stmt(self)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
