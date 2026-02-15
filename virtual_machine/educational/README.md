# Educational Implementations

This folder contains implementations from "Crafting Interpreters" that are educational but **not necessary for idiomatic Rust code**.

## Purpose

The book "Crafting Interpreters" is written with C and Java examples. Many of the low-level implementations (like hash tables, dynamic arrays, etc.) are necessary in C because it lacks a standard library with these data structures.

**In Rust, we have a rich standard library** with highly optimized, battle-tested data structures. For production Rust code, you should use:
- `std::collections::HashMap` instead of custom hash tables
- `Vec<T>` instead of custom dynamic arrays
- `std::collections::HashSet` for sets
- etc.

However, these implementations are still valuable for:
- **Understanding how data structures work internally**
- **Learning algorithm design and tradeoffs**
- **Reference for performance optimization concepts**
- **Educational purposes**

## Contents

### Chapter 20: Hash Tables
- Custom hash table with open addressing and linear probing
- FNV-1a hash function implementation
- Tombstone-based deletion
- String interning system

See `chapter_20_hash_tables/README.md` for details.

## When to Use Custom Implementations

Consider custom implementations when:
1. **Profiling shows** standard library is a bottleneck
2. You have **specific requirements** the standard library doesn't meet
3. You need **platform-specific optimizations**
4. You're building a **teaching tool** or **learning project**

## Idiomatic Rust Approach

For the Lox interpreter, prefer:

```rust
use std::collections::HashMap;

// For global variables
let mut globals: HashMap<String, Value> = HashMap::new();

// For local scopes
let mut locals: Vec<HashMap<String, Value>> = Vec::new();

// That's it! Simple and idiomatic.
```

## References

- [Crafting Interpreters Book](https://craftinginterpreters.com)
- [Rust std::collections](https://doc.rust-lang.org/std/collections/)
- [Rust Performance Book](https://nnethercote.github.io/perf-book/)

---

**Remember**: Premature optimization is the root of all evil. Use the standard library first, optimize later if profiling shows you need to.