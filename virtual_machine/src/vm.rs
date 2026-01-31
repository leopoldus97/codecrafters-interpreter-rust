use crate::{
    chunk::{Chunk, OpCode},
    compiler::Compiler,
    value::Value,
};

const _DEBUG_TRACE_EXECUTION: bool = true;
const STACK_MAX: usize = 256;

#[derive(PartialEq)]
pub enum InterpretResult {
    Ok,
    CompileError,
    RuntimeError,
}

pub struct VM {
    _chunk: Chunk,
    _ip: usize,
    stack: Vec<Value>,
    stack_top: usize,
}

impl VM {
    pub fn new() -> Self {
        Self {
            _chunk: Chunk::default(),
            _ip: 0,
            stack: Vec::with_capacity(STACK_MAX),
            stack_top: 0,
        }
    }

    #[inline]
    fn _read_byte(&mut self) -> Option<u8> {
        let byte = *self._chunk.code().get(self._ip)?;
        self._ip += 1;
        Some(byte)
    }

    #[inline]
    fn _read_constant(&mut self) -> Option<Value> {
        let index = self._read_byte()?;
        self._chunk.read_constant(index)
    }

    pub fn interpret(&mut self, source: &str) -> InterpretResult {
        let mut compiler = Compiler::new(source);
        compiler.compile()
    }

    pub fn push(&mut self, value: Value) {
        self.stack.push(value);
        self.stack_top += 1;
    }

    pub fn pop(&mut self) -> Option<Value> {
        if self.stack_top == 0 {
            return None;
        }
        self.stack_top -= 1;
        self.stack.pop()
    }

    fn _run(&mut self) -> InterpretResult {
        loop {
            if _DEBUG_TRACE_EXECUTION {
                print!("          ");
                for slot in &self.stack[..self.stack_top] {
                    print!("[ {slot} ]");
                }
                println!();

                self._chunk.disassemble_instruction(self._ip);
            }

            let byte = match self._read_byte() {
                Some(byte) => byte,
                None => return InterpretResult::RuntimeError,
            };

            match OpCode::try_from(byte) {
                Ok(OpCode::OpConstant) => {
                    let constant = match self._read_constant() {
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
                    if self._binary_op(|a, b| a + b).is_none() {
                        return InterpretResult::RuntimeError;
                    }
                }
                Ok(OpCode::OpSubtract) => {
                    if self._binary_op(|a, b| a - b).is_none() {
                        return InterpretResult::RuntimeError;
                    }
                }
                Ok(OpCode::OpMultiply) => {
                    if self._binary_op(|a, b| a * b).is_none() {
                        return InterpretResult::RuntimeError;
                    }
                }
                Ok(OpCode::OpDivide) => {
                    if self._binary_op(|a, b| a / b).is_none() {
                        return InterpretResult::RuntimeError;
                    }
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

    fn _binary_op<F>(&mut self, op: F) -> Option<()>
    where
        F: Fn(f64, f64) -> f64,
    {
        let b = self.pop()?;
        let a = self.pop()?;
        self.push(op(a, b));

        Some(())
    }
}

impl Default for VM {
    fn default() -> Self {
        Self::new()
    }
}
