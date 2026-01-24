use crate::{free_array, grow_array, grow_capacity, value::{Value, ValueArray}};

pub enum OpCode {
    OpReturn,
    OpConstant,
}

impl PartialEq for OpCode {
    fn eq(&self, other: &Self) -> bool {
        core::mem::discriminant(self) == core::mem::discriminant(other)
    }
}

pub struct Chunk {
    pub count: usize,
    pub capacity: usize,
    pub code: *mut u8,
    pub lines: *mut usize,
    pub constants: ValueArray,
}

impl Chunk {
    pub fn new() -> Self {
        Chunk {
            count: 0,
            capacity: 0,
            code: std::ptr::null_mut(),
            lines: std::ptr::null_mut(),
            constants: ValueArray::new(),
        }
    }

    pub fn write(&mut self, value: u8, line: usize) {
        if self.capacity < self.count + 1 {
            let old_capacity = self.capacity;
            self.capacity = grow_capacity!(old_capacity);
            self.code = grow_array!(u8, self.code, old_capacity, self.capacity);
            self.lines = grow_array!(usize, self.lines, old_capacity, self.capacity);
        }

        unsafe {
            *self.code.add(self.count) = value;
            *self.lines.add(self.count) = line;
            self.count += 1;
        }
    }

    pub fn free(&self) {
        free_array!(u8, self.code, self.capacity);
        Chunk::new();
    }

    pub fn add_constant(&mut self, value: Value) -> usize {
        self.constants.write(value);
        self.constants.count - 1
    }
}