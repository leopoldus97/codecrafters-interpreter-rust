use std::rc::Rc;

use crate::{scanner::token::Object, utils::error::Error};

use super::Stmt;

pub struct Block {
    statements: Vec<Rc<dyn Stmt>>,
}

impl Block {
    pub fn new(statements: Vec<Rc<dyn Stmt>>) -> Self {
        Self { statements }
    }

    pub fn statements(&self) -> &Vec<Rc<dyn Stmt>> {
        &self.statements
    }
}

impl Stmt for Block {
    fn accept(&self, visitor: &mut dyn super::Visitor) -> Result<Object, Box<Error>> {
        visitor.visit_block_stmt(self)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
