use crate::{
    chunk::{Chunk, OpCode},
    compiler::Compiler,
    value::Value,
};

const DEBUG_TRACE_EXECUTION: bool = true;
const STACK_MAX: usize = 256;

#[macro_export]
macro_rules! runtime_error {
    ($vm:expr, $($arg:tt)*) => {{
        // First line: message
        eprintln!($($arg)*);

        // Stack trace / line info
        let instruction = $vm.ip.saturating_sub(1);
        let line = $vm.chunk.line(instruction);
        eprintln!("[line {}] in script", line);

        $vm.reset_stack();
    }};
}

#[derive(PartialEq)]
pub enum InterpretResult {
    Ok,
    CompileError,
    RuntimeError,
}

pub struct VM {
    chunk: Chunk,
    ip: usize,
    stack: Vec<Value>,
    stack_top: usize,
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

    pub fn interpret(&mut self, source: &str) -> InterpretResult {
        let mut chunk = Chunk::default();

        let mut compiler = Compiler::new(source);

        if !compiler.compile(&mut chunk) {
            return InterpretResult::CompileError;
        }

        self.chunk = chunk;
        self.ip = 0;
        self.stack.truncate(0);
        self.stack_top = 0;

        self.run()
    }

    pub fn push(&mut self, value: Value) {
        if self.stack_top >= STACK_MAX {
            panic!("Stack overflow: attempted to push past STACK_MAX ({STACK_MAX})");
        }
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

    pub fn peek(&self, distance: usize) -> Option<&Value> {
        if distance >= self.stack_top {
            return None;
        }
        self.stack.get(self.stack_top - 1 - distance)
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
                    if let Some(value) = self.pop()
                        && let Some(value_neg) = -value
                    {
                        self.push(value_neg);
                    } else {
                        runtime_error!(self, "Operand must be a number.");
                        return InterpretResult::RuntimeError;
                    }
                }
                Ok(OpCode::OpAdd) => {
                    if self.binary_op(|a, b| a + b).is_none() {
                        runtime_error!(self, "Operands must be numbers.");
                        return InterpretResult::RuntimeError;
                    }
                }
                Ok(OpCode::OpSubtract) => {
                    if self.binary_op(|a, b| a - b).is_none() {
                        runtime_error!(self, "Operands must be numbers.");
                        return InterpretResult::RuntimeError;
                    }
                }
                Ok(OpCode::OpMultiply) => {
                    if self.binary_op(|a, b| a * b).is_none() {
                        runtime_error!(self, "Operands must be numbers.");
                        return InterpretResult::RuntimeError;
                    }
                }
                Ok(OpCode::OpDivide) => {
                    if self.binary_op(|a, b| a / b).is_none() {
                        runtime_error!(self, "Operands must be numbers.");
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
                Ok(OpCode::OpNil) => self.push(Value::Nil),
                Ok(OpCode::OpTrue) => self.push(Value::Bool(true)),
                Ok(OpCode::OpFalse) => self.push(Value::Bool(false)),
                Ok(OpCode::OpNot) => match self.pop() {
                    Some(value) => self.push(!value),
                    None => {
                        runtime_error!(self, "Missing value.");
                        return InterpretResult::RuntimeError;
                    }
                },
                Ok(OpCode::OpEqual) => {
                    if self.binary_op(|a, b| Some((a == b).into())).is_none() {
                        runtime_error!(self, "Equal operand is not supported for this type.");
                        return InterpretResult::RuntimeError;
                    }
                }
                Ok(OpCode::OpGreater) => {
                    if self.binary_op(|a, b| Some((a > b).into())).is_none() {
                        runtime_error!(self, "Greater operand is not supported for this type.");
                        return InterpretResult::RuntimeError;
                    }
                }
                Ok(OpCode::OpLess) => {
                    if self.binary_op(|a, b| Some((a < b).into())).is_none() {
                        runtime_error!(self, "Less operand is not supported for this type.");
                        return InterpretResult::RuntimeError;
                    }
                }
                Err(_) => return InterpretResult::RuntimeError,
            }
        }
    }

    fn binary_op<F>(&mut self, op: F) -> Option<()>
    where
        F: Fn(Value, Value) -> Option<Value>,
    {
        let b = self.pop()?;
        let a = self.pop()?;
        self.push(op(a, b)?);

        Some(())
    }

    fn reset_stack(&mut self) {
        self.stack.clear();
        self.stack_top = 0;
    }
}

impl Default for VM {
    fn default() -> Self {
        Self::new()
    }
}
