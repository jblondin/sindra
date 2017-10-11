//! Collection of rules to use with `rustpeg` parser specification files.

pub mod int {
    //! Integer parsing rule.
    //!
    //! Parses an Rust-style integer specified in either decimal, binary (e.g. 0b0101),
    //! octal (e.g. 0o754), or hexadecimal (e.g. 0x1AF3) format.

    // allow missing docs in generated code
    #![allow(missing_docs)]
    include!(concat!(env!("OUT_DIR"), "/int.rs"));
}
pub mod float {
    //! Floating point parsing rule.
    //!
    //! Parses a Rust-style floating point number, with optional exponent.

    // allow missing docs in generated code
    #![allow(missing_docs)]
    include!(concat!(env!("OUT_DIR"), "/float.rs"));
}
pub mod string;

/// Removes underscores from a string. Used in number parsing (where underscores are allowed
/// as grouping symbols but do not affect the parsed value).
pub fn remove_underscores(input: &str) -> String {
    let mut s = String::new();
    for c in input.chars().filter(|&c| c != '_') { s.push(c); }
    s
}
