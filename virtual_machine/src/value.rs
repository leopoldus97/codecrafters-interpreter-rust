use std::{
    fmt::{self, Display},
    ops::{Add, Div, Mul, Neg, Not, Sub},
};

use crate::object::Object;

pub enum ValueType {
    Bool,
    Nil,
    Number,
    Object,
}

impl Display for ValueType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ValueType::Bool => write!(f, "bool"),
            ValueType::Nil => write!(f, "nil"),
            ValueType::Number => write!(f, "number"),
            ValueType::Object => write!(f, "object"),
        }
    }
}

pub enum Value {
    Bool(bool),
    Nil,
    Number(f64),
    Object(Object),
}

impl Value {
    pub fn value_type(&self) -> ValueType {
        match self {
            Value::Bool(_) => ValueType::Bool,
            Value::Nil => ValueType::Nil,
            Value::Number(_) => ValueType::Number,
            Value::Object(_) => ValueType::Object,
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Bool(b) => write!(f, "{}", b),
            Value::Nil => write!(f, "nil"),
            Value::Number(n) => write!(f, "{}", n),
            Value::Object(o) => write!(f, "{}", o),
        }
    }
}

impl Clone for Value {
    fn clone(&self) -> Self {
        match self {
            Self::Bool(b) => Self::Bool(*b),
            Self::Nil => Self::Nil,
            Self::Number(n) => Self::Number(*n),
            Self::Object(o) => Self::Object(o.clone()),
        }
    }
}

impl Neg for Value {
    type Output = Option<Self>;

    fn neg(self) -> Self::Output {
        match self {
            Value::Number(n) => Some(Value::Number(-n)),
            _ => None,
        }
    }
}

impl Not for Value {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Value::Bool(b) => Value::Bool(!b),
            Value::Nil => Value::Bool(true),
            // Value::Number(0.0) => Value::Bool(true),
            _ => Value::Bool(false),
        }
    }
}

impl Add for Value {
    type Output = Option<Self>;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Number(a), Value::Number(b)) => Some(Value::Number(a + b)),
            (Value::Object(a), Value::Object(b)) => (a + b).map(Value::Object),
            _ => None,
        }
    }
}

impl Sub for Value {
    type Output = Option<Self>;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Number(a), Value::Number(b)) => Some(Value::Number(a - b)),
            _ => None,
        }
    }
}

impl Mul for Value {
    type Output = Option<Self>;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Number(a), Value::Number(b)) => Some(Value::Number(a * b)),
            _ => None,
        }
    }
}

impl Div for Value {
    type Output = Option<Self>;

    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Number(a), Value::Number(b)) => Some(Value::Number(a / b)),
            _ => None,
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Bool(a), Value::Bool(b)) => a == b,
            (Value::Nil, Value::Nil) => true,
            (Value::Number(a), Value::Number(b)) => a == b,
            (Value::Object(a), Value::Object(b)) => a == b,
            _ => false,
        }
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => a.partial_cmp(b),
            _ => None,
        }
    }
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Value::Bool(value)
    }
}

#[derive(Default)]
pub struct ValueArray {
    values: Vec<Value>,
}

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

impl ValueArray {
    pub fn write(&mut self, value: Value) {
        self.values.push(value);
    }

    pub fn get(&self, index: usize) -> Option<&Value> {
        if index >= self.values.len() {
            return None;
        }

        Some(&self.values[index])
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
}
