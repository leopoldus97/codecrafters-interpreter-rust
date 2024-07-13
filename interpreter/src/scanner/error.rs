use crate::utils::report;

pub fn error(line: usize, message: String) {
    report(line, "".to_string(), message);
}
