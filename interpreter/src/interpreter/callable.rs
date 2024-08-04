use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{
    ast::stmt,
    scanner::token::{Object, Token},
    utils::error::{Error, Runtime, RuntimeError},
};

use super::{environment::Environment, Interpreter};

pub trait Callable: std::fmt::Display {
    fn arity(&self) -> usize;
    fn call(&self, interpreter: &mut Interpreter, arguments: Vec<Object>) -> Object;
}

#[derive(Clone, PartialEq)]
pub enum Fun {
    Clock(clock::ClockFn),
    Function(Function),
}

impl Callable for Fun {
    fn arity(&self) -> usize {
        match self {
            Self::Clock(f) => f.arity(),
            Self::Function(f) => f.arity(),
        }
    }

    fn call(&self, interpreter: &mut Interpreter, arguments: Vec<Object>) -> Object {
        match self {
            Self::Clock(f) => f.call(interpreter, arguments),
            Self::Function(f) => f.call(interpreter, arguments),
        }
    }
}

impl std::fmt::Display for Fun {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Clock(fun) => write!(f, "{}", fun),
            Self::Function(fun) => write!(f, "{}", fun),
        }
    }
}

pub mod clock {
    use crate::scanner::token::Object;

    #[derive(Clone, PartialEq)]
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
            write!(f, "<fn native clock>")
        }
    }
}

#[derive(Clone, PartialEq)]
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

#[derive(Clone, PartialEq)]
pub struct Instance {
    klass: Class,
    fields: HashMap<String, Object>,
}

impl Instance {
    pub fn new(klass: Class) -> Self {
        let fields = HashMap::new();
        Self { klass, fields }
    }

    pub fn klass(&self) -> &Class {
        &self.klass
    }

    pub fn get(&self, name: &Token) -> Result<Object, Error> {
        if self.fields.contains_key(name.lexeme()) {
            Ok(self.fields.get(name.lexeme()).unwrap().to_owned())
        } else {
            Err(Error::Runtime(
                RuntimeError::new(
                    format!("Undefined property '{}'.", name.lexeme()),
                    name.to_owned(),
                )
                .into(),
            ))
        }
    }

    pub fn set(&mut self, name: &Token, value: Object) {
        self.fields.insert(name.lexeme().to_string(), value);
    }
}

impl std::fmt::Display for Instance {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} instance", self.klass.name())
    }
}

#[derive(Clone, PartialEq)]
pub struct Class {
    name: String,
}

impl Class {
    pub fn new(name: String) -> Self {
        Self { name }
    }

    pub fn name(&self) -> &String {
        &self.name
    }
}

impl Callable for Class {
    fn arity(&self) -> usize {
        0
    }

    fn call(&self, _: &mut Interpreter, _: Vec<Object>) -> Object {
        Object::Class(Instance::new(self.to_owned()))
    }
}

impl std::fmt::Display for Class {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<class {}>", self.name)
    }
}
