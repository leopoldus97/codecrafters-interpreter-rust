use std::sync::atomic::AtomicBool;

pub mod ast;
pub mod scanner;
pub mod utils;

pub static HAD_ERROR: AtomicBool = AtomicBool::new(false);