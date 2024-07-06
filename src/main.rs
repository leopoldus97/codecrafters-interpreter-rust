use std::collections::HashMap;
use std::{env, process};
use std::fmt::Display;
use std::fs;
use std::io::{self, Write};

use regex::Regex;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        writeln!(io::stderr(), "Usage: {} tokenize <filename>", args[0]).unwrap();
        return;
    }

    let command = &args[1];
    let filename = &args[2];
    let mut exit_code = 0;

    match command.as_str() {
        "tokenize" => {
            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
                String::new()
            });

            if !file_contents.is_empty() {
                for (line_number, mut line) in file_contents.lines().enumerate() {
                    while let Some(result) = eat_string(&mut line, line_number + 1) {
                        match result {
                            Ok(token) => println!("{}", token),
                            Err(e) => {
                                writeln!(io::stderr(), "{}", e).unwrap();
                                exit_code = e.get_exit_code();
                            },                      
                        }
                    }
                }
            }

            let eof = Token {
                token_type: TokenType::Eof,
                lexeme: "".to_string(),
                literal: None,
            };

            println!("{}", eof);
        }
        _ => {
            writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
            return;
        }
    }
    process::exit(exit_code);
}

#[derive(thiserror::Error, Debug)]
enum TokenizerError {
    // #[error("Failed to read file {0}")]
    // FileReadError(String),
    #[error("[line {0}] Error: Unexpected character: {1}")]
    UnexpectedCharacterError(usize, char),
}

impl TokenizerError {
    fn get_exit_code(&self) -> i32 {
        match self {
            TokenizerError::UnexpectedCharacterError(_, _) => 65,
        }
    }
}

#[derive(thiserror::Error, Debug)]
enum Error {
    TokenizerError(#[from] TokenizerError),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::TokenizerError(e) => write!(f, "{}", e),
        }
    }
}

enum TokenType {
    Keyword(String),
    Punctuation(String),
    String,
    Number,
    Identifier,
    Eof,
}

impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenType::Keyword(k) => write!(f, "{}", k),
            TokenType::Punctuation(p) => write!(f, "{}", p),
            TokenType::String => write!(f, "STRING"),
            TokenType::Number => write!(f, "NUMBER"),
            TokenType::Identifier => write!(f, "IDENTIFIER"),
            TokenType::Eof => write!(f, "EOF"),
        }
    }
}

struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: Option<String>,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let literal = match &self.literal {
            Some(l) => l,
            None => "null",
        };
        write!(f, "{} {} {}", self.token_type, self.lexeme, literal)
    }
}

fn eat_string(line: &mut &str, line_number: usize) -> Option<Result<Token, TokenizerError>> {
    let stripped_line = line.trim_start();

    if stripped_line.is_empty() {
        return None;
    }

    let token: Option<Token> = if let Some(token) = get_keyword(stripped_line) {
        Some(token)
    } else if let Some(token) = get_punctuation(stripped_line) {
        Some(token)
    } else if let Some(token) = get_string(stripped_line) {
        Some(token)
    } else if let Some(token) = get_number(stripped_line) {
        Some(token)
    } else if let Some(token) = get_identifier(stripped_line) {
        Some(token)
    } else {
        None
    };

    if let Some(token) = token {
        *line = &stripped_line[token.lexeme.len()..];
        Some(Ok(token))
    } else {
        *line = &stripped_line[1..];
        Some(Err(TokenizerError::UnexpectedCharacterError(
            line_number,
            stripped_line.chars().next().unwrap(),
        )))
    }
}

fn get_keyword(line: &str) -> Option<Token> {
    let re = Regex::new(r"^[a-zA-Z]+").unwrap();

    if !re.is_match(line) {
        return None;
    }

    let token = re.find(line).unwrap();
    let token = token.as_str();

    let keywords = HashMap::from([
        ("and", "AND"),
        ("class", "CLASS"),
        ("else", "ELSE"),
        ("false", "FALSE"),
        ("for", "FOR"),
        ("fun", "FUN"),
        ("if", "IF"),
        ("nil", "NIL"),
        ("or", "OR"),
        ("return", "RETURN"),
        ("super", "SUPER"),
        ("this", "THIS"),
        ("true", "TRUE"),
        ("var", "VAR"),
        ("while", "WHILE"),
    ]);
    keywords.get(token).map(|keyword| Token {
        token_type: TokenType::Keyword(keyword.to_string()),
        lexeme: token.to_string(),
        literal: None,
    })
}

fn get_punctuation(line: &str) -> Option<Token> {
    let re = Regex::new(r"^[(){};,\+\-\*!=<>/\.]").unwrap();

    if !re.is_match(line) {
        return None;
    }

    let token = re.find(line).unwrap();
    let token = token.as_str();

    let punctuations = HashMap::from([
        ("(", "LEFT_PAREN"),
        (")", "RIGHT_PAREN"),
        ("{", "LEFT_BRACE"),
        ("}", "RIGHT_BRACE"),
        (";", "SEMICOLON"),
        (",", "COMMA"),
        ("+", "PLUS"),
        ("-", "MINUS"),
        ("*", "STAR"),
        ("!=", "BANG_EQUAL"),
        ("==", "EQUAL_EQUAL"),
        ("<=", "LESS_EQUAL"),
        (">=", "GREATER_EQUAL"),
        ("!", "BANG"),
        ("=", "EQUAL"),
        ("<", "LESS"),
        (">", "GREATER"),
        ("/", "SLASH"),
        (".", "DOT"),
    ]);
    punctuations.get(token).map(|name| Token {
        token_type: TokenType::Punctuation(name.to_string()),
        lexeme: token.to_string(),
        literal: None,
    })
}

fn get_string(line: &str) -> Option<Token> {
    let re = Regex::new(r#"^"[^"]*""#).unwrap();
    if let Some(token) = re.find(line) {
        let token = token.as_str();

        Some(Token {
            token_type: TokenType::String,
            lexeme: token.to_string(),
            literal: Some(token[1..token.len() - 1].to_string()),
        })
    } else {
        None
    }
}

fn get_number(line: &str) -> Option<Token> {
    let re = Regex::new(r"^[0-9\.]+").unwrap();
    if let Some(token) = re.find(line) {
        let token = token.as_str();

        Some(Token {
            token_type: TokenType::Number,
            lexeme: token.to_string(),
            literal: Some(token.to_string()),
        })
    } else {
        None
    }
}

fn get_identifier(line: &str) -> Option<Token> {
    let re = Regex::new(r"^[a-zA-Z0-9_]+").unwrap();
    if let Some(token) = re.find(line) {
        let token = token.as_str();

        Some(Token {
            token_type: TokenType::Identifier,
            lexeme: token.to_string(),
            literal: None,
        })
    } else {
        None
    }
}

// fn get_unknown_character(line: &str, line_number: usize) -> Result<(), TokenizerError> {
//     let re = Regex::new(r#"^[^(){};,\+\-\*!=<>/\."a-zA-Z0-9]"#).unwrap();
//     if let Some(token) = re.find(line) {
//         let token = token.as_str();

//         Err(TokenizerError::UnexpectedCharacterError(
//             line_number,
//             token.chars().next().unwrap(),
//         ))
//     } else {
//         Ok(())
//     }
// }
