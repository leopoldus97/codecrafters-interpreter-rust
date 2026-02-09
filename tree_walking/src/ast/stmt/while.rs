use std::rc::Rc;

use crate::{ast::expr::Expr, scanner::token::Object, utils::error::Error};

use super::Stmt;

pub struct While {
    condition: Rc<dyn Expr>,
    body: Rc<dyn Stmt>,
}

impl While {
    pub fn new(condition: Rc<dyn Expr>, body: Rc<dyn Stmt>) -> Self {
        Self { condition, body }
    }

    pub fn condition(&self) -> &dyn Expr {
        self.condition.as_ref()
    }

    pub fn body(&self) -> &dyn Stmt {
        self.body.as_ref()
    }
}

impl Stmt for While {
    fn accept(&self, visitor: &mut dyn super::Visitor) -> Result<Object, Box<Error>> {
        visitor.visit_while_stmt(self)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
