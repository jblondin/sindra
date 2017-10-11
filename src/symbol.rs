//! Symbol type

use std::fmt;

use Type;
use Identifier;

/// Symbol object (for use in symbol tables).
#[derive(Clone, Debug, Hash, PartialEq)]
pub enum Symbol<T> where T: Type {
    /// Variable, denoted by specific identifier and type (optional since it may not always be
    /// known at some points of the compilation process)
    Variable(Identifier, Option<T>),
    /// Built-in type symbol
    BuiltinType(Identifier, T)
}
impl<T: Type> fmt::Display for Symbol<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> ::std::result::Result<(), fmt::Error> {
        match *self {
            Symbol::Variable(ref ident, Some(ref ty)) => {
                write!(f, "{}:{}", ident, ty)
            },
            Symbol::Variable(ref ident, None) => {
                write!(f, "{}:<null>", ident)
            },
            Symbol::BuiltinType(ref ident, ref ty) => {
                write!(f, "{}:{}", ident, ty)
            }
        }
    }
}
