use crate::{compiler::Compiler, value::Value};

const _DEBUG_TRACE_EXECUTION: bool = true;
const STACK_MAX: usize = 256;

#[derive(PartialEq)]
pub enum InterpretResult {
    Ok,
    CompileError,
    RuntimeError,
}

pub struct VM {
    stack: Vec<Value>,
    stack_top: usize,
}

impl VM {
    pub fn new() -> Self {
        Self {
            stack: Vec::with_capacity(STACK_MAX),
            stack_top: 0,
        }
    }

    pub fn interpret(&mut self, source: &str) -> InterpretResult {
        let mut compiler = Compiler::new(source);
        compiler.compile()
    }

    pub fn push(&mut self, value: Value) {
        if self.stack_top >= STACK_MAX {
            panic!(
                "Stack overflow: attempted to push beyond STACK_MAX ({})",
                STACK_MAX
            );
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
}

impl Default for VM {
    fn default() -> Self {
        Self::new()
    }
}
