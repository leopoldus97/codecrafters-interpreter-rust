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

#[derive(Clone, PartialEq, Debug)]
pub enum Object {
    Str(String),
    Num(f64),
    Bool(bool),
    Nil,
}

impl std::fmt::Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Object::Str(s) => write!(f, "{}", s),
            Object::Num(n) => write!(f, "{}", n),
            Object::Bool(b) => write!(f, "{}", b),
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
