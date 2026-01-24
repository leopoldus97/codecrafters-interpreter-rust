
use std::fmt;

pub type Value = f64;

impl fmt::Display for ValueArray {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        for (i, v) in self.values.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{v}")?;
        }
        write!(f, "]")
    }
}

#[derive(Default)]
pub struct ValueArray {
    values: Vec<Value>,
}

impl ValueArray {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn write(&mut self, value: Value) {
        self.values.push(value);
    }

    pub fn get(&self, index: usize) -> Value {
        self.values[index]
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
}
