use crate::scanner::token::{Object, Token};

pub enum Error {
    ParseError(ParseError),
    Runtime(Runtime),
    ScanError,
}

impl From<ParseError> for Error {
    fn from(e: ParseError) -> Self {
        Error::ParseError(e)
    }
}

impl From<Runtime> for Error {
    fn from(e: Runtime) -> Self {
        Error::Runtime(e)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::ParseError(_) => write!(f, "Parse error"),
            Error::Runtime(e) => match e {
                Runtime::RuntimeError(e) => {
                    write!(f, "Runtime error: {} at {}", e.message(), e.token())
                }
                Runtime::Return(e) => write!(f, "Return: {}", e.value()),
            },
            Error::ScanError => write!(f, "Scan error"),
        }
    }
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::ParseError(_) => write!(f, "Parse error"),
            Error::Runtime(e) => match e {
                Runtime::RuntimeError(e) => {
                    write!(f, "Runtime error: {} at {}", e.message(), e.token())
                }
                Runtime::Return(e) => write!(f, "Return: {}", e.value()),
            },
            Error::ScanError => write!(f, "Scan error"),
        }
    }
}

#[derive(Debug)]
pub struct ParseError {}

impl ParseError {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for ParseError {
    fn default() -> Self {
        Self::new()
    }
}

pub enum Runtime {
    RuntimeError(RuntimeError),
    Return(Return),
}

impl From<RuntimeError> for Runtime {
    fn from(e: RuntimeError) -> Self {
        Runtime::RuntimeError(e)
    }
}

impl From<Return> for Runtime {
    fn from(e: Return) -> Self {
        Runtime::Return(e)
    }
}

pub struct RuntimeError {
    message: String,
    token: Token,
}

impl RuntimeError {
    pub fn new(message: String, token: Token) -> Self {
        Self { message, token }
    }

    pub fn message(&self) -> &String {
        &self.message
    }

    pub fn token(&self) -> &Token {
        &self.token
    }
}

impl Default for RuntimeError {
    fn default() -> Self {
        Self::new(String::new(), Token::default())
    }
}

pub struct Return {
    value: Object,
}

impl Return {
    pub fn new(value: Object) -> Self {
        Self { value }
    }

    pub fn value(&self) -> &Object {
        &self.value
    }
}
