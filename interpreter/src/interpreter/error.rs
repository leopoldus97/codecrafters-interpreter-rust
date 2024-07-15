use std::sync::atomic::Ordering;

use crate::{utils::error::RuntimeError, HAD_RUNTIME_ERROR};

pub fn runtime_error(error: RuntimeError) {
    eprintln!("{}\n[line {}]", error.message(), error.token());
    HAD_RUNTIME_ERROR.store(true, Ordering::SeqCst);
}
