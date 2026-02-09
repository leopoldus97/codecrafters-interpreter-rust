use crate::utils::report::report;

pub fn error(line: usize, message: String) {
    report(line, "".to_string(), message);
}
