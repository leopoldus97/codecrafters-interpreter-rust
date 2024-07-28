use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::stmt,
    scanner::token::Object,
    utils::error::{Error, Runtime},
};

use super::{environment::Environment, Interpreter};

pub trait Callable: std::fmt::Display {
    fn arity(&self) -> usize;
    fn call(&self, interpreter: &mut Interpreter, arguments: Vec<Object>) -> Object;
}

pub mod clock {
    use crate::scanner::token::Object;

    pub struct ClockFn;

    impl ClockFn {
        pub fn new() -> Self {
            Self
        }
    }

    impl Default for ClockFn {
        fn default() -> Self {
            Self::new()
        }
    }

    impl super::Callable for ClockFn {
        fn arity(&self) -> usize {
            0
        }

        fn call(&self, _: &mut super::Interpreter, _: Vec<Object>) -> Object {
            Object::Num(
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs_f64(),
            )
        }
    }

    impl std::fmt::Display for ClockFn {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "<fn clock>")
        }
    }
}

pub struct Function {
    declaration: stmt::function::Function,
    closure: Rc<RefCell<Environment>>,
}

impl Function {
    pub fn new(declaration: stmt::function::Function, closure: Rc<RefCell<Environment>>) -> Self {
        Self {
            declaration,
            closure,
        }
    }

    pub fn declaration(&self) -> &stmt::function::Function {
        &self.declaration
    }

    pub fn closure(&self) -> Rc<RefCell<Environment>> {
        Rc::clone(&self.closure)
    }
}

impl Callable for Function {
    fn arity(&self) -> usize {
        self.declaration.params().len()
    }

    fn call(&self, interpreter: &mut Interpreter, arguments: Vec<Object>) -> Object {
        let mut environment = Environment::new(Some(Rc::clone(&self.closure)));

        for (index, param) in self.declaration.params().iter().enumerate() {
            let name = param.lexeme().to_string();
            let value = match arguments.get(index) {
                Some(value) => value.to_owned(),
                None => Object::Nil,
            };

            environment.define(name, value);
        }

        let environment = Rc::new(RefCell::new(environment));

        if let Err(Error::Runtime(Runtime::Return(r))) =
            interpreter.execute_block(self.declaration.body(), environment)
        {
            return r.value().to_owned();
        }

        Object::Nil
    }
}

impl std::fmt::Display for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<fn {}>", self.declaration.name().lexeme())
    }
}
