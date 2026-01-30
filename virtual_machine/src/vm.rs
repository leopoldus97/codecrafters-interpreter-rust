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

    pub fn interpret(&mut self, chunk: Chunk) -> InterpretResult {
        self.chunk = chunk;
        self.ip = 0;
        self.run()
    }

    pub fn push(&mut self, value: Value) {
        self.stack.push(value);
        self.stack_top += 1;
    }

    pub fn pop(&mut self) -> Option<Value> {
        self.stack_top -= 1;
        self.stack.pop()
    }

    fn run(&mut self) -> InterpretResult {
        loop {
            if DEBUG_TRACE_EXECUTION {
                print!("          ");
                for slot in &self.stack[..self.stack_top] {
                    print!("[ {slot} ]");
                }
                println!();

                self.chunk.disassemble_instruction(self.ip);
            }
            
            let byte = match self.read_byte() {
                Some(byte) => byte,
                None => return InterpretResult::RuntimeError,
            };

            match OpCode::try_from(byte) {
                Ok(OpCode::OpConstant) => {
                    let constant = match self.read_constant() {
                        Some(constant) => constant,
                        None => return InterpretResult::RuntimeError,
                    };
                    self.push(constant);
                }
                Ok(OpCode::OpNegate) => {
                    let value = match self.pop() {
                        Some(value) => value,
                        None => return InterpretResult::RuntimeError,
                    };
                    self.push(-value);
                }
                Ok(OpCode::OpAdd) => {
                    let b = match self.pop() {
                        Some(value) => value,
                        None => return InterpretResult::RuntimeError,
                    };
                    let a = match self.pop() {
                        Some(value) => value,
                        None => return InterpretResult::RuntimeError,
                    };
                    self.push(a + b);
                }
                Ok(OpCode::OpSubtract) => {
                    let b = match self.pop() {
                        Some(value) => value,
                        None => return InterpretResult::RuntimeError,
                    };
                    let a = match self.pop() {
                        Some(value) => value,
                        None => return InterpretResult::RuntimeError,
                    };
                    self.push(a - b);
                }
                Ok(OpCode::OpMultiply) => {
                    let b = match self.pop() {
                        Some(value) => value,
                        None => return InterpretResult::RuntimeError,
                    };
                    let a = match self.pop() {
                        Some(value) => value,
                        None => return InterpretResult::RuntimeError,
                    };
                    self.push(a * b);
                }
                Ok(OpCode::OpDivide) => {
                    let b = match self.pop() {
                        Some(value) => value,
                        None => return InterpretResult::RuntimeError,
                    };
                    let a = match self.pop() {
                        Some(value) => value,
                        None => return InterpretResult::RuntimeError,
                    };
                    self.push(a / b);
                }
                Ok(OpCode::OpReturn) => {
                    let value = match self.pop() {
                        Some(value) => value,
                        None => return InterpretResult::RuntimeError,
                    };
                    println!("{}", value);
                    return InterpretResult::Ok;
                }
                Err(_) => return InterpretResult::RuntimeError,
            }
        }
    }
}
