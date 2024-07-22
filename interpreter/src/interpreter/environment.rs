use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{
    scanner::token::{Object, Token},
    utils::error::{Error, RuntimeError},
};

#[derive(Clone, PartialEq)]
pub struct Environment {
    pub values: HashMap<String, Object>,
    pub enclosing: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub fn new(enclosing: Option<Rc<RefCell<Environment>>>) -> Self {
        Self {
            values: HashMap::new(),
            enclosing,
        }
    }

    pub fn define(&mut self, name: String, value: Object) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: &Token) -> Result<Object, Error> {
        if self.values.contains_key(name.lexeme()) {
            return Ok(self.values.get(name.lexeme()).unwrap().to_owned());
        }

        if self.enclosing.is_some() {
            return self.enclosing.as_ref().unwrap().borrow().get(name);
        }

        let error = RuntimeError::new(
            format!("Undefined variable '{}'.", name.lexeme()),
            name.to_owned(),
        );
        Err(Error::RuntimeError(error))
    }

    pub fn assign(&mut self, name: &Token, value: Object) -> Result<(), Error> {
        if self.values.contains_key(name.lexeme()) {
            self.values.insert(name.lexeme().to_string(), value);
            return Ok(());
        }

        if self.enclosing.is_some() {
            return self
                .enclosing
                .as_ref()
                .unwrap()
                .borrow_mut()
                .assign(name, value);
        }

        let error = RuntimeError::new(
            format!("Undefined variable '{}'.", name.lexeme()),
            name.to_owned(),
        );
        Err(Error::RuntimeError(error))
    }
}
