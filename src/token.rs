use crate::token_type::TokenType;

pub struct Token {
    r#type: TokenType,
    lexeme: String,
    literal: Option<Literal>,
    _line: usize,
}

impl Token {
    pub fn new(r#type: TokenType, lexeme: String, literal: Option<Literal>, _line: usize) -> Self {
        Self {
            r#type,
            lexeme,
            literal,
            _line,
        }
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

pub enum Literal {
    Str(String),
    Num(f64),
    Bool(bool)
}

impl ToString for Literal {
    fn to_string(&self) -> String {
        match self {
            Literal::Str(s) => s.to_string(),
            Literal::Num(n) => n.to_string(),
            Literal::Bool(b) => b.to_string(),
        }
    }
}
