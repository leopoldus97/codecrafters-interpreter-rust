use std::rc::Rc;

use crate::{
    scanner::token::{Object, Token},
    utils::error::Error,
};

use super::Stmt;

#[derive(Clone)]
pub struct Function {
    name: Token,
    params: Vec<Token>,
    body: Rc<Vec<Rc<dyn Stmt>>>,
}

impl PartialEq for Function {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
            && self.params == other.params
            && Rc::ptr_eq(&self.body, &other.body)
    }
}

impl Function {
    pub fn new(name: Token, params: Vec<Token>, body: Vec<Rc<dyn Stmt>>) -> Self {
        Self {
            name,
            params,
            body: Rc::new(body),
        }
    }

    pub fn name(&self) -> &Token {
        &self.name
    }

    pub fn params(&self) -> &Vec<Token> {
        &self.params
    }

    pub fn body(&self) -> &Vec<Rc<dyn Stmt>> {
        &self.body
    }
}

impl Stmt for Function {
    fn accept(&self, visitor: &mut dyn super::Visitor) -> Result<Object, Error> {
        visitor.visit_function_stmt(self)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}