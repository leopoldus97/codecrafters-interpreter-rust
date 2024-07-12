pub fn error(line: usize, message: String) {
    report(line, "".to_string(), message);
}

fn report(line: usize, location: String, message: String) {
    eprintln!("[line {}] Error{}: {}", line, location, message);
    crate::HAD_ERROR.store(true, std::sync::atomic::Ordering::SeqCst);
}