use crate::utils::error::Error;

use super::stmt::{self, Stmt};

pub struct Block {
    pub statements: Vec<Box<dyn Stmt>>,
}

impl Block {
    pub fn new(statements: Vec<Box<dyn Stmt>>) -> Self {
        Self { statements }
    }

    pub fn statements(&self) -> &Vec<Box<dyn Stmt>> {
        &self.statements
    }
}

impl Stmt for Block {
    fn accept(&self, visitor: &mut dyn stmt::Visitor) -> Result<(), Error> {
        visitor.visit_block_stmt(self)
    }
    
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}