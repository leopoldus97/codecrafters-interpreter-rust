use std::sync::atomic::AtomicBool;

pub mod error;
pub mod token;
pub mod token_type;
pub mod scanner;

pub static HAD_ERROR: AtomicBool = AtomicBool::new(false);