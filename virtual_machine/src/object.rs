use std::{fmt::Display, ops::Add};

// type Object = Rc<RefCell<dyn Any>>;
#[derive(Clone)]
pub enum Object {
    String(String),
}

impl Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Object::String(s) => write!(f, "{}", s),
        }
    }
}

impl Add for Object {
    type Output = Option<Self>;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Object::String(s1), Object::String(s2)) => Some(Object::String(s1 + &s2)),
        }
    }
}

impl PartialEq for Object {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Object::String(s1), Object::String(s2)) => s1 == s2,
        }
    }
}

impl From<&str> for Object {
    fn from(value: &str) -> Self {
        Object::String(value.to_string())
    }
}
