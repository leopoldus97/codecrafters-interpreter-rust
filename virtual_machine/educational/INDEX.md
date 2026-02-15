# Educational Implementations Index

This directory contains implementations from "Crafting Interpreters" that are **educational but not necessary for idiomatic Rust**.

## Purpose

The book is written for C and Java, where many low-level data structures must be built from scratch. In Rust, we have a rich standard library with battle-tested, optimized implementations of these data structures.

**Use this folder to learn how things work internally, but use `std::collections` in your actual code.**

## Contents

### Chapter 20: Hash Tables
**Location**: `chapter_20_hash_tables/`

**What it contains:**
- Custom hash table with open addressing and linear probing
- FNV-1a hash function implementation
- String interning system
- Tombstone-based deletion

**What you should use instead:**
```rust
use std::collections::HashMap;
let mut globals: HashMap<String, Value> = HashMap::new();
```

**Educational value:**
- ✅ Understanding collision resolution strategies
- ✅ Learning about load factors and dynamic resizing
- ✅ Appreciating why HashMap is complex internally
- ✅ Comparing different hash table designs

**Read first**: `chapter_20_hash_tables/WHY_NOT_NEEDED_IN_RUST.md`

---

## Philosophy

> "Premature optimization is the root of all evil." — Donald Knuth

### When to Use Standard Library (Always Start Here)

✅ **Use `std::collections::{HashMap, HashSet, Vec, VecDeque, BTreeMap}` when:**
- You're building an application (like Lox interpreter)
- Performance is "good enough" (profile first!)
- You want maintainable, idiomatic Rust
- You trust experts who built the standard library

### When to Consider Custom Implementations

🤔 **Consider custom data structures when:**
- Profiling proves std library is a bottleneck (rare!)
- You need specific guarantees std doesn't provide
- Platform-specific optimizations are critical
- You're building a teaching project (like this folder!)

### When to Actually Build Custom Implementations

⚠️ **Build custom implementations when:**
- All of the above conditions are met
- You've tried optimizing with std library first
- The performance gain justifies the maintenance cost
- You have benchmarks proving the improvement

## How to Use This Folder

### 1. Learn the Concepts
Read the implementations to understand data structure internals:
```bash
cd chapter_20_hash_tables/
cat README.md
cat IMPLEMENTATION.md
cat table.rs
```

### 2. Understand the Tradeoffs
Compare with Rust's standard library:
```bash
cat WHY_NOT_NEEDED_IN_RUST.md
# Then look at std source:
# https://github.com/rust-lang/rust/tree/master/library/std/src/collections
```

### 3. Appreciate the Standard Library
After reading these implementations, you'll better understand:
- Why `HashMap` is so well-designed
- The work that went into optimizing these data structures
- When to trust the standard library
- When optimization actually matters

### 4. Use Standard Library in Your Code
Don't cargo-cult these implementations into your actual project:
```rust
// ❌ Don't do this:
mod educational_hash_table;
use educational_hash_table::Table;

// ✅ Do this instead:
use std::collections::HashMap;
```

## Future Additions

This folder may grow to include other educational implementations:

- **Dynamic Arrays** (vs `Vec<T>`)
- **Bytecode Instructions** (vs `enum` with pattern matching)
- **Memory Allocators** (vs Rust's allocator)
- **Garbage Collection** (vs Rust's ownership system)

Each will include:
- Full implementation following the book
- Documentation explaining how it works
- Analysis of why Rust's approach is different/better
- Guidance on when (if ever) to use the custom version

## Quick Reference

| Chapter | Topic | Custom Implementation | Rust Standard Library |
|---------|-------|----------------------|----------------------|
| 14-15 | Dynamic Array | Educational only | `Vec<T>` |
| 20 | Hash Table | `chapter_20_hash_tables/` | `HashMap<K, V>` |
| Future | Garbage Collection | Maybe educational | Ownership system |

## Key Principles

1. **Standard library first** - Always start with `std`
2. **Profile before optimizing** - Measure, don't guess
3. **Maintain less code** - Less code = fewer bugs
4. **Learn, but don't cargo-cult** - Understand, then use std
5. **Idiomatic wins** - Write Rust that looks like Rust

## Resources

### Rust Standard Library
- [std::collections documentation](https://doc.rust-lang.org/std/collections/)
- [std::collections source code](https://github.com/rust-lang/rust/tree/master/library/std/src/collections)
- [HashMap internals](https://doc.rust-lang.org/std/collections/struct.HashMap.html)

### Performance
- [Rust Performance Book](https://nnethercote.github.io/perf-book/)
- [Criterion.rs](https://github.com/bheisler/criterion.rs) - Benchmarking
- [cargo-flamegraph](https://github.com/flamegraph-rs/flamegraph) - Profiling

### Learning
- [Crafting Interpreters Book](https://craftinginterpreters.com)
- [Too Many Lists](https://rust-unofficial.github.io/too-many-lists/) - Data structures in Rust
- [The Rust Book](https://doc.rust-lang.org/book/)

## Contributing

If you find these educational implementations helpful and want to add more:

1. Follow the same structure:
   - `README.md` - Overview and usage
   - `IMPLEMENTATION.md` - Technical details
   - `WHY_NOT_NEEDED_IN_RUST.md` - Idiomatic Rust alternative
   - `SUMMARY.md` - Quick reference
   - Source files - The actual implementation

2. Always emphasize:
   - Educational value
   - Why std library is better for production
   - When (if ever) custom implementation makes sense

3. Include benchmarks comparing with std library

---

## Summary

**This folder is a museum, not a toolkit.**

Learn from it, understand it, appreciate it. Then use `std::collections` in your actual code.

Happy learning! 🦀