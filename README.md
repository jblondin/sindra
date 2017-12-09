# Sindra

[![Build Status](https://travis-ci.org/jblondin/sindra.svg?branch=master)](https://travis-ci.org/jblondin/sindra)
[![Documentation](https://docs.rs/sindra/badge.svg)](https://docs.rs/sindra)

Sindra is a library for developing programming languages. It contains several useful structs, traits, methods and [PEG](https://github.com/jblondin/rust-peg) rules for implementing new programming languages.

## Usage

To use, add the following to your `Cargo.toml`:
```toml
[dependencies]
sindra = "0.1"
```

## Example
See the [piske](https://github.com/jblondin/piske) programming language for an example of using sindra. Additionally, some of the more programming-language-generic functionality currently implemented in the piske codebase will eventually be moved into the sindra library for ease of reuse.

## Features
Sindra currently contains the following features:
 - PEG rules (using the additional functionality in the forked version of the `peg` crate [here](https://github.com/jblondin/rust-peg)) for Rust-like integer, floating-point and string literal lexing.
 - Scope-handling traits and structures for managing scoped symbol tables and memory stores.
 - Type inference, promotion, and coercion traits and structs
 - Binary and unary operator traits
 - Framework for creation of an annotated heterogeneous abstract syntax tree
 - Logging structs for compiler error reporting

## Future work
There are several features that are planned for future implementation, including:
 - Span handling (keeping track of the original location in the source file for abstract syntax tree nodes)
 - Struct / class scoping
 - Framework and tools for creating the typical interpreter and compiler binaries for a given language
 - A more unified and easy-to-understand type inteference / coercion / promotion system
 - Reusable implementation of common language features like infix / prefix expression handling
