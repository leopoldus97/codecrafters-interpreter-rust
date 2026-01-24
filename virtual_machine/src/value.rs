pub type Value = f64;

pub trait ValuePrint {
    fn print(&self);
}

impl ValuePrint for Value {
    fn print(&self) {
        print!("{self}");
    }
}

pub struct ValueArray {
    pub capacity: usize,
    pub count: usize,
    pub values: *mut Value,
}

impl ValueArray {
    pub fn new() -> Self {
        ValueArray {
            capacity: 0,
            count: 0,
            values: std::ptr::null_mut(),
        }
    }

    pub fn write(&mut self, value: Value) {
        if self.capacity < self.count + 1 {
            let old_capacity = self.capacity;
            self.capacity = crate::grow_capacity!(old_capacity);
            self.values = crate::grow_array!(Value, self.values, old_capacity, self.capacity);
        }

        unsafe {
            *self.values.add(self.count) = value;
            self.count += 1;
        }
    }

    pub fn free(&self) {
        crate::free_array!(Value, self.values, self.capacity);
        ValueArray::new();
    }
}