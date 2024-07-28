use std::sync::atomic::AtomicBool;

pub mod ast;
pub mod interpreter;
pub mod parser;
pub mod resolver;
pub mod scanner;
pub mod utils;

pub static HAD_ERROR: AtomicBool = AtomicBool::new(false);
pub static HAD_RUNTIME_ERROR: AtomicBool = AtomicBool::new(false);
