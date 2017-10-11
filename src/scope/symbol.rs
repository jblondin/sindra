//! Symbol table implementation and scope specification.

use std::collections::HashMap;

use scope::Scope;
use Identifier;

/// Scope specialization for a symbol table using symbol type `Sym`.
pub type SymbolScope<Sym> = Scope<HashMap<Identifier, Sym>>;

/// Methods for setting and retrieving symbols of type `Sym` from a symbol table.
pub trait SymbolTable<Sym> {
    /// Adds (or updates) a symbol in the symbol table using the specific identifier. Returns the
    /// previous symbol set using that identifier (if exists).
    fn symbol_set(&mut self, ident: Identifier, symbol: Sym) -> Option<Sym>;
    /// Retrieves the symbol for an identifier, or `None` if no symbol exists for that identifier.
    fn symbol_get(&self, ident: &Identifier) -> Option<&Sym>;
}

/// Methods for defining and resolving symbols of type `Sym`. Similar to `SymbolTable`, but
/// intended for use when a search (possibly through multiple scopes) is required to find a
/// particular symbol.
pub trait SymbolStore<Sym> {
    /// Defines (or updates) a symbol in this symbol store. Returns previous symbol if this
    /// identifier has been previously declared.
    fn define(&mut self, ident: Identifier, symbol: Sym) -> Option<Sym>;
    /// Resolves a symbol in this symbol store, or `None` if no symbol exists for the identifier.
    fn resolve(&self, ident: &Identifier) -> Option<Sym>;
}

impl<Sym> SymbolTable<Sym> for HashMap<Identifier, Sym> {
    fn symbol_set(&mut self, ident: Identifier, symbol: Sym) -> Option<Sym> {
        self.insert(ident, symbol)
    }
    fn symbol_get(&self, ident: &Identifier) -> Option<&Sym> { self.get(ident) }
}
impl<Sym: Clone, Tbl> SymbolStore<Sym> for Scope<Tbl> where Tbl: SymbolTable<Sym> {
    fn define(&mut self, ident: Identifier, symbol: Sym) -> Option<Sym> {
        self.item.symbol_set(ident, symbol)
    }
    fn resolve(&self, ident: &Identifier) -> Option<Sym> {
        match self.item.symbol_get(ident) {
            Some(&ref symbol) => Some(symbol.clone()),
            None => {
                match self.parent {
                    Some(ref parent) => {
                        parent.borrow().resolve(ident)
                    },
                    None => None
                }
            }
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use std::fmt;
//     use std::borrow::{Borrow, BorrowMut};

//     use Identifier;
//     use Symbol;
//     use Type;
//     use super::{SymbolScope, SymbolStore};
//     use annotate::Annotated;

//     #[derive(Clone)]
//     struct Annotation<Sym> {
//         scope: SymbolScope<Sym>,
//     }
//     impl<Sym> Default for Annotation<Sym> {
//         fn default() -> Annotation<Sym> {
//             Annotation {
//                 scope: SymbolScope::default()
//             }
//         }
//     }
//     impl<Sym: fmt::Display + fmt::Debug> fmt::Display for Annotation<Sym> {
//         fn fmt(&self, f: &mut fmt::Formatter) -> ::std::result::Result<(), fmt::Error> {
//             write!(f, "{:?}", self.scope)
//         }
//     }
//     impl<Sym: Clone> SymbolStore<Sym> for Annotation<Sym> {
//         fn define(&mut self, ident: Identifier, symbol: Sym) -> Option<Sym> {
//             self.scope.borrow_mut().define(ident, symbol)
//         }
//         fn resolve(&self, ident: &Identifier) -> Option<Sym> {
//             self.scope.borrow().resolve(ident)
//         }
//     }
//     #[derive(Clone, Debug, PartialEq)]
//     struct PType;
//     impl fmt::Display for PType {
//         fn fmt(&self, f: &mut fmt::Formatter) -> ::std::result::Result<(), fmt::Error> {
//             write!(f, "<type>")
//         }
//     }
//     impl Type for PType {
//         fn name(&self) -> &str { "null" }
//     }

//     type Ast<T> = Annotated<T, Annotation<Symbol<PType>>>;

//     #[test]
//     fn test_define() {
//         let elem = "hello".to_string();
//         let mut foo = Ast::<String>::new(elem);
//         foo.define(Identifier("ident".to_string()),
//             Symbol::Variable(Identifier("ident".to_string()), None));
//         println!("{}", foo);
//     }
// }
