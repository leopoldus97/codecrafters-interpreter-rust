use crate::chunk::{Chunk, OpCode};

impl Chunk {
    pub fn disassemble(&self, name: &str) {
        println!("== {name} ==");

        let mut offset = 0;
        while offset < self.len() {
            offset = self.disassemble_instruction(offset);
        }
    }

    pub fn disassemble_instruction(&self, offset: usize) -> usize {
        print!("{offset:04} ");

        if offset > 0 && self.line(offset) == self.line(offset - 1) {
            print!("   | ");
        } else {
            print!("{:4} ", self.line(offset));
        }

        let instruction = self.code()[offset];

        match OpCode::try_from(instruction) {
            Ok(OpCode::OpReturn) => simple_instruction("OP_RETURN", offset),
            Ok(OpCode::OpConstant) => self.constant_instruction("OP_CONSTANT", offset),
            Err(byte) => {
                println!("Unknown opcode {byte}");
                offset + 1
            }
        }
    }

    fn constant_instruction(&self, name: &str, offset: usize) -> usize {
        if self.code().len() <= offset + 1 {
            println!("Error: Incomplete constant instruction at offset {offset}");
            return offset + 1;
        }

        let constant_idx = self.code()[offset + 1];
        let value = self
            .read_constant(constant_idx)
            .expect("Invalid constant index");
        println!("{name} {constant_idx:4} '{value}'");
        offset + 2
    }
}

fn simple_instruction(name: &str, offset: usize) -> usize {
    println!("{name}");
    offset + 1
}
