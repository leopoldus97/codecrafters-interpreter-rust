use virtual_machine::{
    chunk::{Chunk, OpCode},
    vm::VM,
};

fn main() {
    let mut vm = VM::new();

    let mut chunk = Chunk::default();

    let constant = chunk.add_constant(1.2);
    chunk.write(OpCode::OpConstant as u8, 123);
    chunk.write(constant as u8, 123);

    let constant = chunk.add_constant(3.4);
    chunk.write(OpCode::OpConstant as u8, 123);
    chunk.write(constant as u8, 123);

    chunk.write(OpCode::OpAdd as u8, 123);

    let constant = chunk.add_constant(5.6);
    chunk.write(OpCode::OpConstant as u8, 123);
    chunk.write(constant as u8, 123);

    chunk.write(OpCode::OpDivide as u8, 123);
    chunk.write(OpCode::OpNegate as u8, 123);

    chunk.write(OpCode::OpReturn as u8, 123);

    chunk.disassemble("test chunk");
    vm.interpret(chunk);
}
