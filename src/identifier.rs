//! Identifier type.

use std::fmt;

use node::Annotated;

/// Identifier in a program. Used for lookups into symbol tables / memory.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Identifier(pub String);
impl Annotated for Identifier { type Annotation = usize; }


impl fmt::Display for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter) -> ::std::result::Result<(), fmt::Error> {
        write!(f, "{}", self.0)
    }
}
