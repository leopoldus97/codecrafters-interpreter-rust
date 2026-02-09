use crate::value::{Value, ValueArray};

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpCode {
    OpReturn = 0,
    OpConstant = 1,
    OpNegate = 2,
    OpAdd = 3,
    OpSubtract = 4,
    OpMultiply = 5,
    OpDivide = 6,
    OpNil = 7,
    OpTrue = 8,
    OpFalse = 9,
    OpNot = 10,
    OpEqual = 11,
    OpGreater = 12,
    OpLess = 13,
}

impl TryFrom<u8> for OpCode {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(OpCode::OpReturn),
            1 => Ok(OpCode::OpConstant),
            2 => Ok(OpCode::OpNegate),
            3 => Ok(OpCode::OpAdd),
            4 => Ok(OpCode::OpSubtract),
            5 => Ok(OpCode::OpMultiply),
            6 => Ok(OpCode::OpDivide),
            7 => Ok(OpCode::OpNil),
            8 => Ok(OpCode::OpTrue),
            9 => Ok(OpCode::OpFalse),
            10 => Ok(OpCode::OpNot),
            11 => Ok(OpCode::OpEqual),
            12 => Ok(OpCode::OpGreater),
            13 => Ok(OpCode::OpLess),
            _ => Err(value),
        }
    }
}

#[derive(Default)]
pub struct Chunk {
    code: Vec<u8>,
    lines: Vec<usize>,
    constants: ValueArray,
}

impl Chunk {
    pub fn write(&mut self, byte: u8, line: usize) {
        self.code.push(byte);
        self.lines.push(line);
    }

    pub fn add_constant(&mut self, value: Value) -> usize {
        self.constants.write(value);
        self.constants.len() - 1
    }

    pub fn code(&self) -> &[u8] {
        &self.code
    }

    pub fn line(&self, offset: usize) -> usize {
        if offset >= self.lines.len() {
            return 0;
        }

        self.lines[offset]
    }

    pub fn read_constant(&self, index: u8) -> Option<Value> {
        self.constants.get(index as usize).cloned()
    }

    pub fn len(&self) -> usize {
        self.code.len()
    }

    pub fn is_empty(&self) -> bool {
        self.code.is_empty()
    }
}
