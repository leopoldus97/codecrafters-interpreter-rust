# Lox Interpreter in Rust

A **Rust implementation of the Lox programming language** based on Robert Nystrom's book [**Crafting Interpreters**](https://craftinginterpreters.com/).

This project follows the two-part structure of the book, implementing both a tree-walking interpreter (Part II) and a bytecode virtual machine (Part III).

[![progress-banner](https://backend.codecrafters.io/progress/interpreter/266b3920-900f-480b-bc3e-37568227897e)](https://app.codecrafters.io/users/codecrafters-bot?r=2qF)

---

## Project Structure

This is a **Cargo workspace** with two independent crates, each corresponding to a different part of the book:

| Crate | Book Part | Status | Description |
|-------|-----------|--------|-------------|
| [`tree_walking/`](tree_walking/) | Part II: A Tree-Walk Interpreter | ✅ Complete | jlox-style interpreter using AST evaluation |
| [`virtual_machine/`](virtual_machine/) | Part III: A Bytecode Virtual Machine | 🚧 In Progress | clox-style VM with bytecode compilation |

---

## Part I: Tree-Walking Interpreter

> 📖 Based on **Part II** of *Crafting Interpreters* — "A Tree-Walk Interpreter"

The `tree_walking` crate implements a complete Lox interpreter that parses source code into an Abstract Syntax Tree (AST) and evaluates it directly. This approach prioritizes clarity and correctness over raw performance.

### Features

- ✅ Lexical scanning (tokenization)
- ✅ Recursive descent parsing
- ✅ Expression evaluation
- ✅ Statements and state
- ✅ Control flow (`if`, `while`, `for`)
- ✅ Functions and closures
- ✅ Variable resolution and binding
- ✅ Classes and instances
- ✅ Inheritance
- ✅ Interactive REPL

### Architecture

```
Source Code → Scanner → Tokens → Parser → AST → Resolver → Interpreter → Output
```

### Usage

```bash
# Run a Lox file
cargo run -p tree_walking -- <filename>.lox

# Start the REPL
cargo run -p tree_walking
```

### Example

```lox
// test.lox
class Greeter {
  init(name) {
    this.name = name;
  }

  greet() {
    print "Hello, " + this.name + "!";
  }
}

var greeter = Greeter("World");
greeter.greet();  // Output: Hello, World!
```

---

## Part II: Bytecode Virtual Machine

> 📖 Based on **Part III** of *Crafting Interpreters* — "A Bytecode Virtual Machine"

The `virtual_machine` crate implements a stack-based bytecode VM. Instead of walking an AST, this interpreter compiles Lox source code to bytecode instructions and executes them on a virtual machine — similar to how languages like Python and Lua work.

### Features (In Progress)

- ✅ Chunk-based bytecode representation
- ✅ Value types
- ✅ Bytecode disassembler/debugger
- ✅ Virtual machine stack
- ✅ Scanner (lexer)
- 🚧 Compiler (single-pass)
- 🚧 Variables and expressions
- 🔲 Control flow
- 🔲 Functions and closures
- 🔲 Classes and inheritance
- 🔲 Garbage collection

### Architecture

```
Source Code → Scanner → Tokens → Compiler → Bytecode → VM → Output
```

### Usage

```bash
# Run a Lox file
cargo run -p virtual_machine -- <filename>.lox

# Start the REPL
cargo run -p virtual_machine
```

---

## Building & Development

### Prerequisites

- Rust (stable toolchain)
- Cargo

### Build Commands

```bash
# Build the entire workspace
cargo build

# Build a specific crate
cargo build -p tree_walking
cargo build -p virtual_machine

# Run tests (when available)
cargo test

# Lint with Clippy
cargo clippy -- -D warnings

# Format code
cargo fmt
```

---

## About Crafting Interpreters

[*Crafting Interpreters*](https://craftinginterpreters.com/) by Robert Nystrom is a comprehensive guide to implementing programming languages. The book walks through building two complete interpreters for the Lox language:

1. **jlox** — A tree-walking interpreter written in Java (Part II)
2. **clox** — A bytecode virtual machine written in C (Part III)

This project reimplements both interpreters in Rust, taking advantage of Rust's strong type system, memory safety, and modern language features.

### Why Rust?

- **Memory safety** without garbage collection (perfect for implementing a VM)
- **Pattern matching** makes AST traversal elegant
- **Enums with data** model tokens and AST nodes naturally
- **Strong type system** catches bugs at compile time
- **Zero-cost abstractions** for high performance

---

## Resources

- 📖 [Crafting Interpreters](https://craftinginterpreters.com/) — The book this project is based on
- 🎯 [CodeCrafters Challenge](https://app.codecrafters.io/courses/interpreter/overview) — The challenge that inspired this implementation
- 🦀 [The Rust Programming Language](https://doc.rust-lang.org/book/) — Official Rust book

---

## License

This project is licensed under the MIT License — see the [LICENSE](LICENSE) file for details.

---

## Author

**Máté Kiss**
