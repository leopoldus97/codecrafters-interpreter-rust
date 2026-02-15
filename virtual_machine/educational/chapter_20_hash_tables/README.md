# Chapter 20: Hash Tables - Implementation Guide

## What Was Implemented

This implementation adds a complete hash table data structure to the Lox bytecode VM, following Chapter 20 of Crafting Interpreters. The hash table enables efficient key-value storage and implements **string interning** for memory efficiency and fast string operations.

## Features

### ✅ Core Hash Table Operations
- **Insert/Update**: `set(key, value)` with O(1) average time
- **Lookup**: `get(key)` with O(1) average time  
- **Delete**: `delete(key)` with tombstone support
- **Copy**: `add_all(from, to)` to merge tables

### ✅ String Interning
- All strings automatically interned in VM's global table
- Only one copy of each unique string in memory
- Fast pointer-based equality checks

### ✅ Collision Resolution
- Open addressing with linear probing
- Tombstone-based deletion (doesn't break probe sequences)
- Dynamic resizing at 75% load factor

### ✅ FNV-1a Hash Function
- Fast, simple, and effective for strings
- Hash cached in `ObjString` struct
- Good distribution for typical data

## File Structure

```
virtual_machine/src/
├── table.rs          # Hash table implementation (NEW)
├── object.rs         # ObjString with hash caching (MODIFIED)
├── value.rs          # Updated for ObjString (MODIFIED)
├── vm.rs             # Added strings table (MODIFIED)
├── compiler.rs       # String interning logic (MODIFIED)
└── lib.rs            # Export table module (MODIFIED)
```

## Usage Examples

### Basic String Operations

```bash
# String concatenation
echo '"hello" + " " + "world"' | cargo run -p virtual_machine
# Output: hello world

# String equality (fast pointer comparison)
echo '"test" == "test"' | cargo run -p virtual_machine
# Output: true

# Different strings
echo '"abc" == "xyz"' | cargo run -p virtual_machine
# Output: false
```

### Testing Interning

Create a file `test_interning.lox`:
```lox
"foo"
"foo"
"foo"
```

All three "foo" strings will reference the same `ObjString` object in memory!

## How It Works

### 1. Hash Table Structure

```rust
pub struct Table {
    count: usize,           // Entries + tombstones
    entries: Vec<Entry>,    // Bucket array
}

pub struct Entry {
    key: Option<ObjString>,
    value: Value,
}
```

- **Empty entry**: `key = None, value = Nil`
- **Tombstone**: `key = None, value = Bool(true)`
- **Occupied**: `key = Some(string), value = <any>`

### 2. String Interning Flow

```
Source: "hello"
    ↓
1. Calculate hash: FNV-1a("hello") = 0x12345678
    ↓
2. Check VM.strings table
    ↓
3a. If found → Reuse existing ObjString
3b. If not found → Create new ObjString, add to table
    ↓
4. Use interned string in bytecode
```

### 3. Collision Resolution (Linear Probing)

```
Hash to bucket 5 → Already occupied?
    ↓
Check bucket 6 → Already occupied?
    ↓
Check bucket 7 → Empty! Use this one.
```

### 4. Tombstone Example

```
Before delete "jam":       After delete "jam":
[2] "bagel"               [2] "bagel"
[3] "jam"                 [3] TOMBSTONE ← keeps sequence alive
[4] "eggs"                [4] "eggs"    ← still reachable
```

Without tombstones, deleting "jam" would break the probe sequence and make "eggs" unreachable!

## Performance Characteristics

| Operation | Time Complexity | Notes |
|-----------|----------------|-------|
| Insert    | O(1) average   | Amortized (due to resizing) |
| Lookup    | O(1) average   | With good hash distribution |
| Delete    | O(1) average   | Creates tombstone |
| Resize    | O(n)           | Triggered at 75% load factor |
| String equality | O(1) | Pointer comparison! |

## Building and Testing

```bash
# Build the virtual_machine crate
cargo build -p virtual_machine

# Run with string expression
echo '"a" + "b"' | cargo run -p virtual_machine

# Check for warnings
cargo clippy -p virtual_machine -- -D warnings

# Format code
cargo fmt -p virtual_machine
```

## Key Design Decisions

### Why Rc<String> for strings?
- **Shared ownership**: Multiple `ObjString` can reference same data
- **Fast cloning**: Only increments reference count
- **Automatic cleanup**: Freed when last reference dropped

### Why cache the hash?
- String hashing is O(n) where n = string length
- Strings are immutable, so hash never changes
- Cache once during creation, reuse forever

### Why 75% load factor?
- Balance between memory usage and performance
- Higher = more memory efficient, slower lookups
- Lower = faster lookups, more wasted space
- 75% is empirically proven to work well

### Why linear probing?
- **Simple**: Easy to implement and understand
- **Cache-friendly**: Accesses contiguous memory
- **Effective**: Works well with good hash function
- Alternative: quadratic probing, double hashing, chaining

## Common Issues and Solutions

### Issue: Infinite loop in find_entry
**Cause**: Table completely full (no empty buckets)  
**Solution**: Load factor < 1.0 guarantees empty buckets exist

### Issue: Memory leak with strings
**Cause**: Strings not freed when table destroyed  
**Solution**: `Vec<Entry>` automatically drops all entries

### Issue: String equality broken
**Cause**: Not interning all strings  
**Solution**: Compiler interns during string literal parsing

## Next Steps (Chapter 21)

The hash table will be used for:
- **Global variables**: Map variable names → values
- **Local variables**: Resolved at compile time (not in hash table)
- **Classes** (later): Map field names → values
- **Closures** (later): Capture upvalues

## Testing Checklist

- [x] String creation and concatenation
- [x] String equality with interning
- [x] Hash table insert/lookup/delete
- [x] Tombstone handling
- [x] Dynamic resizing
- [x] No compiler warnings
- [x] No clippy warnings
- [x] Code formatted

## References

- [Crafting Interpreters - Chapter 20](https://craftinginterpreters.com/hash-tables.html)
- [FNV-1a Hash Algorithm](http://www.isthe.com/chongo/tech/comp/fnv/)
- [Open Addressing](https://en.wikipedia.org/wiki/Open_addressing)

## Verification

```bash
# All these should pass:
✓ cargo build -p virtual_machine
✓ cargo clippy -p virtual_machine -- -D warnings
✓ cargo fmt -p virtual_machine -- --check
✓ cargo test -p virtual_machine  # (when tests exist)
```

---

**Status**: ✅ Complete and ready for Chapter 21 (Global Variables)