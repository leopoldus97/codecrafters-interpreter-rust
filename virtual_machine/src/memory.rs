use std::{
    alloc::{alloc, dealloc, realloc, Layout},
    ptr,
};

pub trait MemoryManagement<T> {
    fn new() -> Self;
    fn write(&mut self, value: T, line: usize);
    fn free(&self);
}

#[macro_export]
macro_rules! grow_capacity {
    ($capacity:expr) => {
        if $capacity < 8 {
            8
        } else {
            $capacity * 2
        }
    };
}

#[macro_export]
macro_rules! grow_array {
    ($type:ty, $pointer:expr, $old_count:expr, $new_count:expr) => {{
        let new_pointer = crate::memory::reallocate(
            $pointer,
            std::mem::size_of::<$type>() * $old_count,
            std::mem::size_of::<$type>() * $new_count,
        ) as *mut $type;

        new_pointer
    }};
}

#[macro_export]
macro_rules! free_array {
    ($type:ty, $pointer:expr, $old_count:expr) => {{
        let new_pointer =
            crate::memory::reallocate($pointer, std::mem::size_of::<$type>() * $old_count, 0)
                as *mut $type;

        new_pointer
    }};
}

pub fn reallocate<T>(pointer: *mut T, old_size: usize, new_size: usize) -> *mut u8 {
    let old_layout = Layout::array::<T>(old_size).unwrap_or_else(|_| Layout::new::<T>());

    if new_size == 0 {
        if old_size > 0 {
            unsafe {
                dealloc(pointer as *mut u8, old_layout);
            }
        }
        return ptr::null_mut();
    }

    let new_layout = Layout::array::<T>(new_size).unwrap_or_else(|_| Layout::new::<T>());

    // Handle fresh allocation (old_size == 0)
    if old_size == 0 {
        let result = unsafe { alloc(new_layout) };

        if result.is_null() {
            panic!("Failed to allocate memory!");
        }

        return result;
    }

    // Handle reallocation
    let result = unsafe { realloc(pointer as *mut u8, old_layout, new_size) };

    if result.is_null() {
        panic!("Failed to allocate memory!");
    }

    result
}
