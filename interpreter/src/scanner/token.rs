use crate::interpreter::callable::Callable;

use super::token_type::TokenType;

#[derive(Clone, PartialEq)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: Object,
    line: usize,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, literal: Object, line: usize) -> Self {
        Self {
            token_type,
            lexeme,
            literal,
            line,
        }
    }

    pub fn token_type(&self) -> &TokenType {
        &self.token_type
    }

    pub fn lexeme(&self) -> &String {
        &self.lexeme
    }

    pub fn literal(&self) -> &Object {
        &self.literal
    }

    pub fn line(&self) -> usize {
        self.line
    }
}

impl Default for Token {
    fn default() -> Self {
        Self {
            token_type: TokenType::Eof,
            lexeme: String::new(),
            literal: Object::Nil,
            line: 0,
        }
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} {} {}", self.token_type, self.lexeme, self.literal)
    }
}

impl Clone for Box<dyn Callable> {
    fn clone(&self) -> Self {
        self.to_owned()
    }
}

impl PartialEq for Box<dyn Callable> {
    fn eq(&self, other: &Self) -> bool {
        self.to_string() == other.to_string() && self.arity() == other.arity()
    }
}

#[derive(Clone, PartialEq)]
pub enum Object {
    Str(String),
    Num(f64),
    Bool(bool),
    Callable(Box<dyn Callable>),
    Nil,
}

impl std::fmt::Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Object::Str(s) => write!(f, "{}", s),
            Object::Num(n) => write!(f, "{}", n),
            Object::Bool(b) => write!(f, "{}", b),
            Object::Callable(c) => write!(f, "{}", c),
            Object::Nil => write!(f, "nil"),
        }
    }
}

impl Object {
    pub fn parse(&self) -> Result<f64, Box<dyn std::error::Error>> {
        match self {
            Object::Num(n) => Ok(*n),
            _ => Err("Object is not a number".into()),
        }
    }

    pub fn is_truthy(&self) -> bool {
        match self {
            Object::Bool(b) => *b,
            Object::Nil => false,
            // Object::Str(s) if s == "" => false,
            // Object::Num(n) if *n == 0.0 => false,
            _ => true,
        }
    }
}
