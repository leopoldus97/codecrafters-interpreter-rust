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
            match self.values.get(name.lexeme()) {
                Some(value) => return Ok(value.to_owned()),
                None => {
                    let error = RuntimeError::new(
                        format!("Undefined variable '{}'.", name.lexeme()),
                        name.to_owned(),
                    );
                    return Err(Error::RuntimeError(error));
                }
            }
        }

        if self.enclosing.is_some() {
            match self.enclosing.as_ref() {
                Some(enclosing) => return enclosing.borrow().get(name),
                None => {
                    let error = RuntimeError::new(
                        format!("Undefined variable '{}'.", name.lexeme()),
                        name.to_owned(),
                    );
                    return Err(Error::RuntimeError(error));
                }
            }
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
            match self.enclosing.as_ref() {
                Some(enclosing) => return enclosing.borrow_mut().assign(name, value),
                None => {
                    let error = RuntimeError::new(
                        format!("Undefined variable '{}'.", name.lexeme()),
                        name.to_owned(),
                    );
                    return Err(Error::RuntimeError(error));
                }
            }
        }

        let error = RuntimeError::new(
            format!("Undefined variable '{}'.", name.lexeme()),
            name.to_owned(),
        );
        Err(Error::RuntimeError(error))
    }
}
