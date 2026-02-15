# Chapter 20: Hash Tables - Implementation Summary

This document describes the implementation of hash tables for the Lox bytecode VM as described in Chapter 20 of Crafting Interpreters.

## Overview

The hash table implementation provides:
1. **Efficient key-value storage** using open addressing with linear probing
2. **String interning** to ensure unique string representation in memory
3. **Tombstone-based deletion** to maintain probe sequences
4. **Dynamic resizing** with 75% load factor threshold

## Files Modified/Created

### New Files
- `virtual_machine/src/table.rs` - Complete hash table implementation

### Modified Files
- `virtual_machine/src/object.rs` - Refactored to `ObjString` with hash caching
- `virtual_machine/src/value.rs` - Updated for `ObjString` and pointer equality
- `virtual_machine/src/vm.rs` - Added `strings` table for interning
- `virtual_machine/src/compiler.rs` - Integrated string interning during compilation
- `virtual_machine/src/lib.rs` - Exported `table` module

## Key Components

### 1. Hash Table Structure (`table.rs`)

```rust
pub struct Table {
    count: usize,           // Number of entries (including tombstones)
    entries: Vec<Entry>,    // Bucket array
}

pub struct Entry {
    pub key: Option<ObjString>,
    pub value: Value,
}
```

**Design Decisions:**
- Uses `Vec<Entry>` instead of raw array for Rust safety
- `count` tracks entries + tombstones for load factor calculation
- Empty entry: `key = None, value = Nil`
- Tombstone: `key = None, value = Bool(true)`

### 2. Hash Function (FNV-1a)

Implemented in `object.rs`:
```rust
fn hash_string(key: &str) -> u32 {
    let mut hash: u32 = 2166136261;
    for byte in key.bytes() {
        hash ^= byte as u32;
        hash = hash.wrapping_mul(16777619);
    }
    hash
}
```

**Properties:**
- Fast and simple
- Good distribution for typical string data
- Deterministic (same input → same hash)
- Cached in `ObjString` struct

### 3. String Interning

**ObjString Structure:**
```rust
pub struct ObjString {
    chars: Rc<String>,  // Shared string data
    hash: u32,          // Cached hash code
}
```

**Interning Process:**
1. When creating a string, calculate hash using FNV-1a
2. Look up in VM's `strings` table using `find_string()`
3. If found, return existing `ObjString`
4. If not found, create new `ObjString` and add to table

**Benefits:**
- **Memory efficiency**: Only one copy of each unique string
- **Fast equality**: Pointer comparison via `Rc::ptr_eq()`
- **Fast hashing**: Hash calculated once and cached

### 4. Collision Resolution (Linear Probing)

**Algorithm:**
```rust
fn find_entry(entries: &[Entry], key: &ObjString) -> &Entry {
    let mut index = (key.hash() as usize) % entries.len();
    let mut tombstone: Option<usize> = None;
    
    loop {
        let entry = &entries[index];
        
        if entry.key.is_none() {
            if entry.is_empty() {
                // Return tombstone if we saw one, else this empty slot
                return tombstone.map_or(entry, |i| &entries[i]);
            } else {
                // Found tombstone, remember it
                if tombstone.is_none() {
                    tombstone = Some(index);
                }
            }
        } else if entry.key == key {
            return entry;  // Found the key
        }
        
        index = (index + 1) % entries.len();  // Linear probe
    }
}
```

**Key Features:**
- Start at `hash % capacity`
- Check each subsequent bucket (wrapping at end)
- Track first tombstone encountered
- Return tombstone for insertion (reuses deleted slots)
- Stop at empty (non-tombstone) bucket

### 5. Dynamic Resizing

**Trigger:** When `(count + 1) > capacity * 0.75`

**Process:**
1. Allocate new array (double the size, or 8 for initial)
2. Re-insert all non-tombstone entries
3. Recalculate `count` (excludes tombstones)
4. Free old array

**Why Rebuild?**
- Hash modulo changes with capacity
- Entries may map to different buckets
- Eliminates accumulated tombstones

### 6. Tombstone Handling

**What are tombstones?**
- Marker for deleted entries (`key=None, value=Bool(true)`)
- Prevents breaking probe sequences

**Example:**
```
Before delete:           After delete (no tombstone):
[0] empty                [0] empty
[1] empty                [1] empty
[2] "bagel"              [2] "bagel"
[3] "jam" (collided)     [3] empty  ← lookup stops here!
[4] "eggs" (collided)    [4] "eggs" ← now unreachable!
```

With tombstones:
```
[2] "bagel"
[3] TOMBSTONE  ← keeps sequence alive
[4] "eggs"     ← still reachable
```

**Implementation:**
- `delete()`: Replace entry with tombstone
- `find_entry()`: Skip tombstones, continue probing
- `set()`: Reuse tombstones for new entries
- `adjust_capacity()`: Discard tombstones during resize

### 7. Load Factor Management

**Definition:** `load_factor = count / capacity`

**Threshold:** 0.75 (75%)

**Why count tombstones?**
- Tombstones slow down lookups (must skip over them)
- If ignored, could fill array with tombstones → infinite loop
- Including them ensures empty buckets always exist

**Trade-off:**
- Higher load = better memory usage, slower lookups
- Lower load = faster lookups, more memory waste
- 0.75 is empirically good balance

## API

### Core Operations

```rust
// Create new table
let mut table = Table::new();

// Insert/update entry
let is_new = table.set(key, value);  // true if new, false if updated

// Lookup entry
if let Some(value) = table.get(&key) {
    // Found
}

// Delete entry
let deleted = table.delete(&key);  // true if found and deleted

// Copy all entries
table1.add_all(&mut table2);

// Find interned string
if let Some(existing) = table.find_string(chars, hash) {
    // Reuse existing
}
```

## Integration with VM

### VM Structure
```rust
pub struct VM {
    // ... other fields
    strings: Table,  // All interned strings
}
```

### Compiler Integration
```rust
impl Compiler {
    fn string(&mut self) {
        let str_content = /* extract from token */;
        let hash = ObjString::copy_str(str_content).hash();
        
        // Check if already interned
        let interned = if let Some(existing) = self.strings.find_string(str_content, hash) {
            existing  // Reuse
        } else {
            let new_string = ObjString::copy_str(str_content);
            self.strings.set(new_string.clone(), Value::Nil);
            new_string  // New
        };
        
        self.emit_constant(Value::Object(Object::String(interned)));
    }
}
```

## Performance Characteristics

| Operation | Average Case | Worst Case | Notes |
|-----------|--------------|------------|-------|
| `set()` | O(1) | O(n) | Amortized due to resizing |
| `get()` | O(1) | O(n) | With good hash function |
| `delete()` | O(1) | O(n) | Creates tombstone |
| Resize | O(n) | O(n) | Rebuilds entire table |

**Worst case** occurs when:
- All keys hash to same bucket (poor hash function)
- Table nearly full (high load factor)
- Many tombstones (degrades to linear search)

**In practice:** With FNV-1a and 0.75 load factor, operations are effectively O(1)

## Testing

```bash
# Build
cargo build -p virtual_machine

# Test string interning
echo '"hello" + " " + "world"' | cargo run -p virtual_machine
# Output: hello world

# Test string equality (fast pointer comparison)
echo '"test" == "test"' | cargo run -p virtual_machine
# Output: true

# Run clippy
cargo clippy -p virtual_machine -- -D warnings

# Format code
cargo fmt -p virtual_machine
```

## Differences from C Implementation

### Rust-Specific Adaptations

1. **Memory Management**
   - C: Manual `malloc`/`free`
   - Rust: `Vec<Entry>` with automatic cleanup

2. **String Storage**
   - C: Raw `char*` pointer
   - Rust: `Rc<String>` for shared ownership

3. **Null Handling**
   - C: `NULL` pointers
   - Rust: `Option<ObjString>` for type safety

4. **Pointer Comparison**
   - C: `key == other_key`
   - Rust: `Rc::ptr_eq(&self.chars, &other.chars)`

5. **Array Indexing**
   - C: Direct pointer arithmetic
   - Rust: Bounds-checked indexing

### Design Benefits in Rust

- **No use-after-free**: Ownership prevents dangling references
- **No null pointer errors**: `Option` forces explicit handling
- **No buffer overflows**: Bounds checking on all array access
- **Thread safety**: `Rc` prevents data races (though VM is single-threaded)

## Future Enhancements (Not in Chapter 20)

1. **Generic keys**: Support numbers, booleans as keys
2. **Better hashing**: Use SipHash for cryptographic security
3. **Metrics**: Track collisions, probe lengths, resize frequency
4. **Memory pool**: Reduce allocations for small strings
5. **Robin Hood hashing**: Reduce variance in probe lengths

## References

- [Crafting Interpreters - Chapter 20](https://craftinginterpreters.com/hash-tables.html)
- [FNV Hash](http://www.isthe.com/chongo/tech/comp/fnv/)
- [Open Addressing](https://en.wikipedia.org/wiki/Open_addressing)
- [String Interning](https://en.wikipedia.org/wiki/String_interning)

## Verification

All code compiles without warnings:
```bash
✓ cargo build -p virtual_machine
✓ cargo clippy -p virtual_machine -- -D warnings
✓ cargo fmt -p virtual_machine -- --check
```

The implementation is complete and ready for Chapter 21 (Global Variables).