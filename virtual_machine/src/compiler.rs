use crate::{
    scanner::{Scanner, TokenType},
    vm::InterpretResult,
};

pub struct Compiler<'a> {
    pub scanner: Scanner<'a>,
}

impl<'a> Compiler<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            scanner: Scanner::new(source),
        }
    }

    pub fn compile(&mut self) -> InterpretResult {
        // Compilation logic goes here
        let mut line = 0;

        loop {
            let token = self.scanner.scan_token();
            if token.line != line {
                print!("{:4} ", token.line);
                line = token.line;
            } else {
                print!("   | ");
            }
            println!("{:?} '{}'", token.token_type, token.start);

            if matches!(token.token_type, TokenType::Eof) {
                break;
            }
        }

        InterpretResult::Ok
    }
}
