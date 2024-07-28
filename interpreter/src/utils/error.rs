use crate::scanner::token::Token;

pub enum Error {
    ParseError(ParseError),
    RuntimeError(RuntimeError),
    ScanError,
}

impl From<ParseError> for Error {
    fn from(e: ParseError) -> Self {
        Error::ParseError(e)
    }
}

impl From<RuntimeError> for Error {
    fn from(e: RuntimeError) -> Self {
        Error::RuntimeError(e)
    }
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

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::ParseError(_) => write!(f, "Parse error"),
            Error::RuntimeError(e) => write!(f, "Runtime error: {} at {}", e.message(), e.token()),
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
