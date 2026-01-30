# Lox Interpreter in Rust - Copilot Instructions

## Repository Summary

A **Rust implementation of the Lox programming language** for the CodeCrafters "Build Your Own Interpreter" challenge. This is a **Cargo workspace** with two crates:

| Crate | Version | Status | Description |
|-------|---------|--------|-------------|
| `tree_walking/` | 0.9.0 | ✅ Complete | Tree-walk interpreter (fully functional) |
| `virtual_machine/` | 0.1.0 | ⚠️ WIP | Bytecode VM (has compilation errors, in development) |

**Languages/Tools**: Rust (stable toolchain), Cargo workspace  
**Dependencies**: None outside standard library

---

## Build & Validation Commands

### CRITICAL: The `virtual_machine` crate currently has compilation errors

**Always target `tree_walking` specifically when building/testing:**

```bash
# ✅ WORKING - Build tree_walking only
cargo build -p tree_walking

# ✅ WORKING - Lint tree_walking only  
cargo clippy -p tree_walking -- -D warnings

# ✅ WORKING - Format check (entire workspace)
cargo fmt -- --check

# ✅ WORKING - Run interpreter with a .lox file
cargo run -p tree_walking -- test.lox

# ⚠️ FAILS - Full workspace build (vm has errors)
cargo build  # Will fail due to virtual_machine errors

# ⚠️ FAILS - Full workspace test
cargo test   # Will fail due to virtual_machine errors
```

### CI Pipeline Checks (`.github/workflows/pull_request.yml`)

The PR workflow runs these checks on `ubuntu-latest` with stable Rust:
1. `cargo fetch` - Install dependencies
2. `cargo build --verbose` - Build all (will fail if vm not fixed)
3. `cargo clippy -- -D warnings` - Lint with warnings as errors
4. `cargo fmt -- --check` - Format check (no auto-fix)

**Before submitting changes**, always run:
```bash
cargo fmt                                    # Auto-format code
cargo clippy -p tree_walking -- -D warnings  # Lint check
cargo build -p tree_walking                  # Verify build
```

### Pre-commit/Pre-push Hooks

Git hooks are in `.github/hooks/` (enable with `git config core.hooksPath .github/hooks`):
- **pre-commit**: Runs `cargo fmt --all`
- **pre-push**: Runs `cargo clippy`, `cargo test`, `cargo fmt -- --check`

---

## Project Layout

```
codecrafters-interpreter-rust/
├── Cargo.toml                    # Workspace manifest (resolver = "2")
├── tree_walking/                 # ✅ COMPLETE interpreter
│   ├── Cargo.toml
│   └── src/
│       ├── main.rs               # Entry point (REPL + file execution)
│       ├── lib.rs                # Exports + HAD_ERROR/HAD_RUNTIME_ERROR flags
│       ├── scanner/              # Lexer: source → tokens
│       │   ├── mod.rs, token.rs, token_type.rs, error.rs
│       ├── parser/               # Parser: tokens → AST
│       │   ├── mod.rs, error.rs
│       ├── resolver/             # Semantic analysis: variable binding
│       │   └── mod.rs
│       ├── interpreter/          # Execution: AST → output
│       │   ├── mod.rs, callable.rs, environment.rs, error.rs
│       ├── ast/                  # AST node definitions
│       │   ├── expr/             # 13 expression types
│       │   └── stmt/             # 10 statement types
│       └── utils/                # Error types, reporting
├── virtual_machine/              # ⚠️ WIP - has compile errors
│   ├── Cargo.toml
│   └── src/
│       ├── main.rs, lib.rs
│       ├── chunk.rs              # OpCode enum, bytecode storage
│       ├── value.rs              # Value type (f64)
│       ├── debug.rs              # Disassembler
│       └── vm.rs                 # VM (broken - compile errors)
├── test*.lox                     # Test files for each chapter
├── .github/
│   ├── workflows/                # CI: pull_request.yml, auto_label_branch.yml
│   └── hooks/                    # Git hooks: pre-commit, pre-push
└── .vscode/                      # VS Code tasks and launch configs
```

---

## Key Architecture Patterns

### Tree-Walking Interpreter Pipeline
```
Source Code → Scanner → Tokens → Parser → AST → Resolver → Interpreter → Output
```

### Visitor Pattern (AST)
- All expressions implement `Expr` trait with `accept()` method
- All statements implement `Stmt` trait with `accept()` method
- Parser, Resolver, and Interpreter all implement visitor methods

### Memory Management
- `Rc<RefCell<>>` for shared mutable state (environments, callables)
- `Rc<dyn Trait>` for polymorphic AST nodes
- `HashMap` with `ExprKey` wrapper for variable resolution tracking

### Error Handling
- Global atomic flags: `HAD_ERROR`, `HAD_RUNTIME_ERROR` (use `Ordering::SeqCst`)
- Exit codes: 64 (usage), 65 (data error), 70 (runtime), 74 (I/O)
- `Return` error type used for early function returns (not actual errors)

---

## Adding New Features

### New Expression Type (tree_walking)
1. Create `tree_walking/src/ast/expr/<name>.rs`
2. Implement `Expr` trait with `accept()` method
3. Export in `ast/expr/mod.rs`
4. Add parsing logic in `parser/mod.rs`
5. Add resolution logic in `resolver/mod.rs`  
6. Add interpretation logic in `interpreter/mod.rs`

### New Statement Type (tree_walking)
Same pattern as expressions, but in `ast/stmt/` directory.

### New OpCode (virtual_machine)
1. Add variant to `OpCode` enum in `chunk.rs`
2. Update `TryFrom<u8>` implementation
3. Add disassembly case in `debug.rs`
4. Add execution case in `vm.rs` (once vm.rs compiles)

---

## Known Issues

1. **virtual_machine/src/vm.rs has compile errors** - The VM is incomplete. Do not attempt to build the full workspace until vm.rs is fixed.

2. **Format differences may exist** - Always run `cargo fmt` before committing.

3. **Clippy warnings are errors in CI** - Use `cargo clippy -- -D warnings` to catch issues locally.

---

## Trust These Instructions

These instructions have been validated against the actual codebase. If you need to explore:
- Check `tree_walking/src/main.rs` for the execution pipeline
- Check `tree_walking/src/lib.rs` for module exports and global flags
- Check `.github/workflows/pull_request.yml` for exact CI steps

Only search the codebase if these instructions are incomplete or found to be incorrect.
