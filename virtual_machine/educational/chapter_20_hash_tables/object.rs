use std::fmt::Display;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

/// FNV-1a hash function for strings
fn hash_string(key: &str) -> u32 {
    let mut hash: u32 = 2166136261;
    for byte in key.bytes() {
        hash ^= byte as u32;
        hash = hash.wrapping_mul(16777619);
    }
    hash
}

#[derive(Clone)]
pub struct ObjString {
    chars: Rc<String>,
    hash: u32,
}

impl ObjString {
    pub fn new(s: String) -> Self {
        let hash = hash_string(&s);
        Self {
            chars: Rc::new(s),
            hash,
        }
    }

    pub fn copy_str(s: &str) -> Self {
        Self::new(s.to_string())
    }

    pub fn as_str(&self) -> &str {
        &self.chars
    }

    pub fn len(&self) -> usize {
        self.chars.len()
    }

    pub fn is_empty(&self) -> bool {
        self.chars.is_empty()
    }

    pub fn hash(&self) -> u32 {
        self.hash
    }
}

impl Display for ObjString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.chars)
    }
}

impl PartialEq for ObjString {
    fn eq(&self, other: &Self) -> bool {
        // Fast pointer comparison for interned strings
        Rc::ptr_eq(&self.chars, &other.chars)
    }
}

impl Eq for ObjString {}

impl Hash for ObjString {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.hash.hash(state);
    }
}

#[derive(Clone)]
pub enum Object {
    String(ObjString),
}

impl Object {
    pub fn as_string(&self) -> Option<&ObjString> {
        match self {
            Object::String(s) => Some(s),
        }
    }
}

impl Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Object::String(s) => write!(f, "{}", s),
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
        Object::String(ObjString::copy_str(value))
    }
}

impl From<ObjString> for Object {
    fn from(value: ObjString) -> Self {
        Object::String(value)
    }
}
