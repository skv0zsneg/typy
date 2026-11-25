# TyPy

**Understandable to people. Verifiable by compilers. Inspectable by agents.**

> Status: early research and educational project. The language and tooling described here are planned, not yet implemented.

TyPy is a small, statically typed programming language with Python-inspired syntax, implemented in Rust. It will compile to bytecode executed by a portable, resource-bounded `no_std` virtual machine. A deterministic compiler-side semantic model will expose symbols, types, dependencies and supported contracts to developers and AI agents to help them understand architecture and assess changes.

## Goals

- Learn computer science by building data structures, parsers, type systems, compilers and virtual machines.
- Keep execution portable and predictable with caller-owned memory and no required heap allocation.
- Investigate whether compiler-backed semantic information helps agents make safer changes. The compiler itself must not depend on an LLM.

## Architecture

```text
Source -> Lexer -> Parser -> AST -> Name Resolution -> Type Checker
                                            |                |
                                      Semantic Model      Bytecode
                                            |                |
                                      CLI / Agent API     no_std VM
                                                             |
                                                      Embedded target
```

## Roadmap

Follow the [GitHub issues](https://github.com/skv0zsneg/typy/issues) for the staged roadmap, dependencies, learning objectives and acceptance criteria.

1. Foundation and stack-based VM.
2. Lexer, parser, AST, type checker and compiler.
3. Semantic model and agent inspection tools.
4. Contracts, architecture rules and change analysis.
5. Embedded integration and reproducible AI research.

## Engineering principles

- Start with `no_std`, no `alloc`, and `forbid(unsafe_code)` in the VM.
- Use checked operations and explicit errors; test malformed inputs.
- Practice TDD, document invariants and measure resource usage.
- Treat compiler facts as deterministic and distinguish them from prose and unproven claims.
- Evaluate new ideas through focused experiments rather than restarting the project.

## First milestone

Execute `PUSH_I32 10; PUSH_I32 20; ADD_I32; PUSH_I32 2; MUL_I32; HALT` and return `60` using caller-provided memory and an instruction budget.

## References

- [Crafting Interpreters](https://craftinginterpreters.com/contents.html)
- [The Rust Programming Language](https://doc.rust-lang.org/book/)
- [The Embedded Rust Book](https://doc.rust-lang.org/stable/embedded-book/)
- [CPython](https://github.com/python/cpython)

## License

A license has not yet been selected. Public visibility alone does not grant an open-source license.
