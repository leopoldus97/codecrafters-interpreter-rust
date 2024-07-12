use super::token_type::TokenType;

pub struct Token {
    r#type: TokenType,
    lexeme: String,
    literal: Option<Object>,
    _line: usize,
}

impl Token {
    pub fn new(r#type: TokenType, lexeme: String, literal: Option<Object>, _line: usize) -> Self {
        Self {
            r#type,
            lexeme,
            literal,
            _line,
        }
    }

    pub fn r#type(&self) -> &TokenType {
        &self.r#type
    }

    pub fn lexeme(&self) -> &String {
        &self.lexeme
    }

    pub fn literal(&self) -> &Option<Object> {
        &self.literal
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let literal = match &self.literal {
            Some(literal) => literal.to_string(),
            None => "null".to_string(),
        };

        write!(f, "{} {} {}", self.r#type, self.lexeme, literal)
    }
}

pub enum Object {
    Str(String),
    Num(f64),
    Bool(bool)
}

impl ToString for Object {
    fn to_string(&self) -> String {
        match self {
            Object::Str(s) => s.to_string(),
            Object::Num(n) => n.to_string(),
            Object::Bool(b) => b.to_string(),
        }
    }
}
