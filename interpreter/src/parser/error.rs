use crate::{
    scanner::{token::Token, token_type::TokenType},
    utils::report,
};

pub(super) struct ParseError {}

pub(super) fn error(token: &Token, message: String) {
    if token.token_type() == &TokenType::Eof {
        report(token.line(), " at end".to_string(), message);
    } else {
        report(token.line(), format!(" at '{}'", token.lexeme()), message);
    }
}
