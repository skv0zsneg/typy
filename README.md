# TyPy

_**Ty**ped **Py**thon_

> ⚠️ Under Development

A programming language inspired by Python. The goal is to build clean and maintainable code using the best of the Python ecosystem. The language is:

1. **Statically Typed** — catches type errors at compile time
2. **Simple** — minimal syntax and semantics
3. **Small** — focused feature set with predictable behavior

## Features

- Static type system with `int` and `bool` types
- Python-like syntax with indentation-based blocks
- Conditional statements (`if` / `elif` / `else`)
- Arithmetic and comparison operators
- Stack-based bytecode virtual machine
- Interactive REPL for quick experimentation
- File execution with filename validation

## Quick Start

### Execute a File

Create a file `example.tp` with snake_case naming:

```python
a: int = 10
b: int = 20
result: int = a * b
```

Run it with:

```bash
$ typy example.tp
200
```

TyPy enforces filename must have the `.tp` extension.

### Interactive REPL

We also support an interactive REPL:

```bash
$ typy
=== TyPy (v 0.1.0) ===
>>> a: int = 10
10
>>> b: int = 20
20
>>> b * a
200
```

## Contributing

### Architecture

The language is written in Rust using a stack-based virtual machine. All code goes through the following pipeline:

```mermaid
graph TD
    A(Tokenizer) -->|CST| B(Parser)
    B -->|AST| C(Type Checker)
    C -->|Type Validated AST| D(Compiler)
    D -->|Bytecode| E(Virtual Machine)
```

### Making PR

Before submitting a PR, ensure your code passes:

1. Formatted `cargo fmt --all -- --check`
2. Linted `cargo clippy --locked --all-targets --all-features -- -D warnings`
3. Tested `cargo test --locked --all-features --verbose`
4. Built `cargo build --locked --verbose`

You can use `just` for quick check `$ just check-all`
