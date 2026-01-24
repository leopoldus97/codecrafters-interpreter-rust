use crate::{
    chunk::{Chunk, OpCode},
    value::ValuePrint,
};

impl Chunk {
    pub fn disassemble(&self, name: &'static str) {
        print!("== {name} ==\n");

        let mut offset = 0;
        while offset < self.count {
            offset = self.disassemble_instruction(offset);
        }
    }

    pub fn disassemble_instruction(&self, offset: usize) -> usize {
        print!("{offset:04} ");

        if offset > 0 && unsafe { *self.lines.offset(offset as isize) }
            == unsafe { *self.lines.offset((offset - 1) as isize) }
        {
            print!("   | ");
        } else {
            print!("{:4} ", unsafe { *self.lines.offset(offset as isize) });
        }

        let instruction = unsafe {
            let inst = self.code.offset(offset as isize);
            *inst
        };

        match instruction {
            x if x == OpCode::OpReturn as u8 => simple_instruction("OP_RETURN", offset),
            x if x == OpCode::OpConstant as u8 => constant_instruction("OP_CONSTANT", self, offset),
            _ => {
                print!("Unknown opcode {instruction:?}\n");
                offset + 1
            }
        }
    }
}

fn simple_instruction(name: &'static str, offset: usize) -> usize {
    print!("{name}\n");
    offset + 1
}

fn constant_instruction(name: &'static str, chunk: &Chunk, offset: usize) -> usize {
    let constant = unsafe {
        let inst = chunk.code.offset((offset + 1) as isize);
        *inst
    };

    print!("{name} {constant:4} '");
    unsafe {
        let val = chunk.constants.values.offset(constant as isize);
        (*val).print();
    }
    print!("'\n");

    offset + 2
}
