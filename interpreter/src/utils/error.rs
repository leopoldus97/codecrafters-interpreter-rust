use crate::scanner::token::Token;

#[derive(Debug)]
pub enum Error {
    ParseError(ParseError),
    RuntimeError(RuntimeError),
    ScanError,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::ParseError(_) => write!(f, "Parse error"),
            Error::RuntimeError(e) => write!(f, "Runtime error: {}", e.message()),
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

impl Into<Error> for ParseError {
    fn into(self) -> Error {
        Error::ParseError(self)
    }
}

#[derive(Debug)]
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

impl Into<Error> for RuntimeError {
    fn into(self) -> Error {
        Error::RuntimeError(self)
    }
}