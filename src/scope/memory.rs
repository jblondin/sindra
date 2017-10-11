//! Memory space implementation and scope specification.

use std::fmt::{self, Display};
use std::collections::HashMap;

use scope::Scope;
use scope::symbol::SymbolTable;

use Identifier;

/// Symbol and associated memory (if exists) structure.
#[derive(Debug, Clone, PartialEq)]
pub struct SymbolMemory<Sym, Val> {
    symbol: Sym,
    memory: Option<Val>,
}
impl<Sym: Display, Val: Display> Display for SymbolMemory<Sym, Val> {
    fn fmt(&self, f: &mut fmt::Formatter) -> ::std::result::Result<(), fmt::Error> {
        match self.memory {
            Some(ref mem) => write!(f, "{} {{{}}}", self.symbol, mem),
            None => write!(f, "{}", self.symbol)
        }
    }
}

/// Scope specialization with both symbol (of type `Sym`) and memory (values of type `Val`)
/// information
pub type MemoryScope<Sym, Val> = Scope<HashMap<Identifier, SymbolMemory<Sym, Val>>>;

impl<Sym, Val> SymbolTable<Sym> for HashMap<Identifier, SymbolMemory<Sym, Val>> {
    fn symbol_set(&mut self, ident: Identifier, symbol: Sym) -> Option<Sym> {
        self.insert(ident, SymbolMemory { symbol: symbol, memory: None }).map(|sm| sm.symbol)
    }
    fn symbol_get(&self, ident: &Identifier) -> Option<&Sym> {
        self.get(ident).map(|&ref sm| &sm.symbol)
    }
}

/// Methods for accessing a value in a particular memory table.
////
/// These methods returns nested `Option`s: the outer `Option` is `Some(...)` if the identifier
/// exists at all in the table, and the inner `Option` is `Some(...)` if the identifier has
/// associated memory. This allows for the situation in which an identifier exists, but is
/// unassigned to a memory value.
pub trait MemoryTable<Val> {
    /// Accesses a reference to the value in  memory for a partiular identifier.
    fn memory(&self, ident: &Identifier) -> Option<&Option<Val>>;
    /// Accesses a mutable reference to the value in memory for a particular identifier.
    fn memory_mut(&mut self, ident: &Identifier) -> Option<&mut Option<Val>>;
}
impl<Sym, Val> MemoryTable<Val> for HashMap<Identifier, SymbolMemory<Sym, Val>> {
    fn memory(&self, ident: &Identifier) -> Option<&Option<Val>> {
        self.get(ident).map(|&ref sm| &sm.memory)
    }
    fn memory_mut(&mut self, ident: &Identifier) -> Option<&mut Option<Val>> {
        self.get_mut(ident).map(|&mut ref mut sm| &mut sm.memory)
    }
}

/// Methods for setting and getting values in a memory store. This is used instead of a
/// `MemoryTable` when a structure (like a scope) may search through multiple memory tables to
/// retrieve the value.
pub trait MemoryStore<Val> {
    /// Sets (or updates) the value in memory for an identifier. Returns the previous value (if
    /// exists). Typically, the identifier should already exist in the store, but may not have
    /// an existing value.
    ///
    /// # Failures
    /// Can fail if setting value for an identifier that doesn't otherwise exist in the store
    /// (unless this is allowed in a particular implementation).
    fn set(&mut self, ident: Identifier, value: Val) -> Result<Option<Val>, String>;
    /// Retrieves the value for a particular identifier, if exists.
    fn get(&self, ident: &Identifier) -> Option<Val>;
}
impl<Val: Clone, T> MemoryStore<Val> for Scope<T> where T: MemoryTable<Val> {
    fn set(&mut self, ident: Identifier, value: Val) -> Result<Option<Val>, String> {
        match self.item.memory_mut(&ident) {
            Some(&mut ref mut curr_mem) => {
                let ret = curr_mem.clone();
                *curr_mem = Some(value);
                Ok(ret)
            },
            None => {
                match self.parent {
                    Some(ref mut parent) => parent.borrow_mut().set(ident, value),
                    None => Err(format!("attempt to set memory for missing symbol: {}", ident))
                }
            }
        }
    }
    fn get(&self, ident: &Identifier) -> Option<Val> {
        match self.item.memory(ident) {
            Some(&ref curr_mem) => {
                curr_mem.clone()
            },
            None => {
                match self.parent {
                    Some(ref parent) => parent.borrow().get(ident),
                    None => None
                }
            }
        }
    }
}
