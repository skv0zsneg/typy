# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog 1.1.0], and this project adheres to
[Semantic Versioning 2.0.0].

## [Unreleased]

### Added

- Support for `elif` branches in conditional statements, including parsing,
  static type checking, bytecode compilation, and integration coverage.

## [0.1.1] - 2026-08-21

### Added

- `if`/`else` conditional statements, including nested blocks.
- Multi-line input support in the REPL for indented blocks.
- `justfile` commands for common development tasks.
- Contributor documentation with formatting, linting, testing, and build
  checks.

### Changed

- Refactored statement and expression handling across the tokenizer, parser,
  type checker, compiler, and virtual machine.

### Fixed

- REPL handling of block input.

## [0.1.0] - 2026-08-15

### Added

- Initial public release of the TyPy interpreter.
- Tokenization, parsing, static type checking, bytecode compilation, and a
  stack-based virtual machine.
- Integer and boolean values, variables, arithmetic, and comparison
  expressions.
- Command-line configuration, REPL, automated tests, and continuous
  integration.
