use std::sync::atomic::Ordering;

use crate::{scanner::token::Token, HAD_RUNTIME_ERROR};

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

pub fn runtime_error(error: RuntimeError) {
    eprintln!("{}\n[line {}]", error.message(), error.token());
    HAD_RUNTIME_ERROR.store(true, Ordering::SeqCst);
}