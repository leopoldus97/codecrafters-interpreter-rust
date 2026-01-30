use crate::{chunk::{Chunk, OpCode}, value::Value};

const DEBUG_TRACE_EXECUTION: bool = true;
const STACK_MAX: usize = 256;

pub enum InterpretResult {
    Ok,
    CompileError,
    RuntimeError,
}

pub struct VM {
    pub chunk: Chunk,
    pub ip: usize,
    pub stack: Vec<Value>,
    pub stack_top: usize,
}

impl VM {
    pub fn new() -> Self {
        Self {
            chunk: Chunk::default(),
            ip: 0,
            stack: Vec::with_capacity(STACK_MAX),
            stack_top: 0,
        }
    }

    #[inline]
    fn read_byte(&mut self) -> Option<u8> {
        let byte = *self.chunk.code().get(self.ip)?;
        self.ip += 1;
        Some(byte)
    }

    #[inline]
    fn read_constant(&mut self) -> Option<Value> {
        let index = self.read_byte()?;
        self.chunk.read_constant(index)
    }

    pub fn interpret(&mut self, chunk: Chunk) {
        self.chunk = chunk;
        self.ip = 0;
        self.run();
    }

    pub fn push(&mut self, value: Value) {
        self.stack.push(value);
        self.stack_top += 1;
    }

    pub fn pop(&mut self) -> Option<Value> {
        self.stack_top -= 1;
        self.stack.pop()
    }

    fn run(&mut self) -> Option<InterpretResult> {
        loop {
            if DEBUG_TRACE_EXECUTION {
                print!("          ");
                for slot in &self.stack[..self.stack_top] {
                    print!("[ {slot} ]");
                }
                println!();

                self.chunk.disassemble_instruction(self.ip);
            }
            let _instruction = match self.read_byte() {
                x if x == Some(OpCode::OpConstant as u8) => {
                    let constant = self.read_constant().unwrap();
                    self.push(constant);
                    println!("{constant}");
                    None
                }
                x if x == Some(OpCode::OpNegate as u8) => {
                    let value = self.pop().unwrap();
                    self.push(-value);
                    println!("{}", -value);
                    None
                }
                x if x == Some(OpCode::OpAdd as u8) => {
                    let b = self.pop().unwrap();
                    let a = self.pop().unwrap();
                    self.push(a + b);
                    println!("{}", a + b);
                    None
                }
                x if x == Some(OpCode::OpSubtract as u8) => {
                    let b = self.pop().unwrap();
                    let a = self.pop().unwrap();
                    self.push(a - b);
                    println!("{}", a - b);
                    None
                }
                x if x == Some(OpCode::OpMultiply as u8) => {
                    let b = self.pop().unwrap();
                    let a = self.pop().unwrap();
                    self.push(a * b);
                    println!("{}", a * b);
                    None
                }
                x if x == Some(OpCode::OpDivide as u8) => {
                    let b = self.pop().unwrap();
                    let a = self.pop().unwrap();
                    self.push(a / b);
                    println!("{}", a / b);
                    None
                }
                x if x == Some(OpCode::OpReturn as u8) => {
                    println!("{}", self.pop().unwrap());
                    Some(InterpretResult::Ok)
                }
                _ => None
            };
        }
    }
}
