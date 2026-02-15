# Chapter 20: Hash Tables - Educational Implementation Summary

## What's In This Folder

This folder contains a **complete from-scratch hash table implementation** following Chapter 20 of "Crafting Interpreters". It's here for **educational purposes** but is **not used in the actual virtual_machine crate**.

## Files

- `table.rs` - Full hash table implementation with:
  - Open addressing with linear probing
  - Tombstone-based deletion
  - Dynamic resizing at 75% load factor
  - FNV-1a hash function

- `object.rs` - Modified object system with:
  - `ObjString` struct with cached hash
  - `Rc<String>` for shared ownership
  - Pointer-based equality for interned strings

- `IMPLEMENTATION.md` - Detailed technical documentation
- `README.md` - Usage guide and examples
- `WHY_NOT_NEEDED_IN_RUST.md` - **READ THIS!** Explains why you don't need this

## Why This Exists But Isn't Used

### In the Book (C):
```c
// C has no standard hash table, must build your own
Table globals;
initTable(&globals);
tableSet(&globals, key, value);
```

### In Idiomatic Rust:
```rust
// Rust has HashMap in standard library
use std::collections::HashMap;

let mut globals: HashMap<String, Value> = HashMap::new();
globals.insert(key, value);
```

**The Rust standard library's `HashMap` is:**
- ✅ Highly optimized (by experts with PhDs)
- ✅ Battle-tested across millions of projects
- ✅ Generic and flexible
- ✅ The idiomatic choice

## What You Should Actually Use

For the Lox interpreter, use standard Rust collections:

```rust
use std::collections::HashMap;

pub struct VM {
    // For global variables (Chapter 21)
    globals: HashMap<String, Value>,
    
    // For local scopes (later)
    locals: Vec<HashMap<String, Value>>,
}
```

That's it! No custom hash table needed.

## Educational Value

This implementation is still valuable for learning:

### 1. **Understanding Hash Table Internals**
- How hashing works
- Collision resolution strategies
- Load factor and resizing
- Tombstone deletion

### 2. **Algorithm Design Tradeoffs**
- Open addressing vs. chaining
- Linear probing vs. quadratic/double hashing
- Memory usage vs. lookup speed

### 3. **Low-Level Optimization Techniques**
- Cache-friendly data structures
- Reducing pointer indirection
- Hash function design

### 4. **Why Standard Library Choices Matter**
- Appreciate the work that went into `HashMap`
- Understand when custom implementations make sense
- Learn to profile before optimizing

## When to Use Custom Data Structures in Rust

✅ **Use custom implementation when:**
- Profiling proves standard library is a bottleneck
- You have specific requirements std doesn't meet
- Platform-specific optimizations needed
- You're building a teaching/learning project

❌ **Don't use custom implementation when:**
- "It might be faster" (without profiling)
- "The book does it" (different language constraints)
- "I want to learn" (that's what this folder is for!)
- You haven't tried the standard library first

## Performance Reality Check

```rust
// Modern Rust HashMap performance for Lox interpreter:
// - Insert: ~50 nanoseconds
// - Lookup: ~30 nanoseconds
// - 10,000 variables: ~0.5 milliseconds total overhead
//
// User won't notice. Don't optimize prematurely.
```

## How to Use This Educational Code

### 1. Read and Learn
```bash
# Study the implementation
cat table.rs
cat WHY_NOT_NEEDED_IN_RUST.md
```

### 2. Compare with std::HashMap
```bash
# Look at Rust's implementation
# https://github.com/rust-lang/rust/blob/master/library/std/src/collections/hash/map.rs
```

### 3. Benchmark (Optional)
```rust
// Create your own benchmarks to see the difference
// (Spoiler: HashMap is usually faster or same speed)
```

### 4. Move On
```bash
# Use HashMap in your actual code
# Continue with Chapter 21
```

## Key Takeaways

1. **C needs custom hash tables** → No standard library
2. **Rust has HashMap** → Use it
3. **Learning is valuable** → Understand the internals
4. **Pragmatism wins** → Use standard library in production
5. **Profile first** → Optimize only proven bottlenecks

## Next Steps

✅ **Continue with Chapter 21 (Global Variables)**

Use this approach:
```rust
// In vm.rs
use std::collections::HashMap;

pub struct VM {
    globals: HashMap<String, Value>,
    // ... rest of fields
}

impl VM {
    pub fn define_global(&mut self, name: String, value: Value) {
        self.globals.insert(name, value);
    }
    
    pub fn get_global(&self, name: &str) -> Option<&Value> {
        self.globals.get(name)
    }
}
```

That's the idiomatic Rust way!

## References

- [Crafting Interpreters - Chapter 20](https://craftinginterpreters.com/hash-tables.html)
- [Rust HashMap docs](https://doc.rust-lang.org/std/collections/struct.HashMap.html)
- [Rust Performance Book](https://nnethercote.github.io/perf-book/)
- [Why Premature Optimization is Bad](https://wiki.c2.com/?PrematureOptimization)

---

**Remember**: The best code is code you don't have to write or maintain. Use the standard library!