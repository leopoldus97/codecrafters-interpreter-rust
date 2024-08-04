use std::rc::Rc;

use crate::{ast::expr::Expr, scanner::token::Object, utils::error::Error};

use super::Stmt;

pub struct If {
    condition: Rc<dyn Expr>,
    then_branch: Rc<dyn Stmt>,
    else_branch: Option<Rc<dyn Stmt>>,
}

impl If {
    pub fn new(
        condition: Rc<dyn Expr>,
        then_branch: Rc<dyn Stmt>,
        else_branch: Option<Rc<dyn Stmt>>,
    ) -> Self {
        Self {
            condition,
            then_branch,
            else_branch,
        }
    }

    pub fn condition(&self) -> &dyn Expr {
        self.condition.as_ref()
    }

    pub fn then_branch(&self) -> &dyn Stmt {
        self.then_branch.as_ref()
    }

    pub fn else_branch(&self) -> Option<&dyn Stmt> {
        self.else_branch.as_ref().map(|stmt| stmt.as_ref())
    }
}

impl Stmt for If {
    fn accept(&self, visitor: &mut dyn super::Visitor) -> Result<Object, Error> {
        visitor.visit_if_stmt(self)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
