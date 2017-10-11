//! Scope types.
//!
//! Provides implementation of general scope handling, and specific scope implementations for
//! symbol table and memory space.

pub mod symbol;
pub use self::symbol::*;
pub mod memory;
pub use self::memory::*;

use std::cell::RefCell;
use std::rc::Rc;

/// General scope object with underlying data structure `T`.
///
/// Scoping is handled by a linked list of `Scope` objects from the child `Scope` object (which is
/// the parent of no other scope) to the root (global) `Scope` object, which has no parent.
#[derive(Debug, Clone, PartialEq)]
pub struct Scope<T> {
    /// Underlying scoped data structure.
    pub item: T,
    /// Optional pointer to the parent of this scope.
    ///
    /// This should be `Some(...)` for every `Scope` object except the root node.
    ///
    /// This is maintained as an `Rc<RefCell<...>>` since a single scope can be the parent of
    /// multiple other scopes.
    pub parent: Option<Rc<RefCell<Scope<T>>>>,
}
impl<T: Default> Scope<T> {
    fn new_with_parent(parent: Option<Rc<RefCell<Scope<T>>>>) -> Scope<T> {
        Scope {
            item: T::default(),
            parent: parent,
        }
    }
}
impl<T: Default> Default for Scope<T> {
    fn default() -> Scope<T> {
        Scope {
            item: T::default(),
            parent: None,
        }
    }
}

/// General stack trait for an object which contains its own stack management.
///
/// Handles typical `push` and `pop` operations.
pub trait Stack: Sized {
    /// Push the current object down the stack, and create and return the new object (which
    /// maintains a stack pointer to the previous object on the stack).
    fn push(&self) -> Self;
    /// Pop the current object off the stack, and return the next object on the stack (if exists).
    fn pop(&self) -> Option<Self>;
}
impl<T: Default> Stack for Rc<RefCell<Scope<T>>> {
    fn push(&self) -> Rc<RefCell<Scope<T>>> {
        Rc::new(RefCell::new(Scope::<T>::new_with_parent(Some(Rc::clone(self)))))
    }
    fn pop(&self) -> Option<Rc<RefCell<Scope<T>>>> {
        match self.borrow().parent {
            Some(ref scope) => Some(Rc::clone(scope)),
            None => None
        }
    }
}

/// Trait for any object which has an associated scope.
///
/// Provides get / set access for object's scope.
pub trait Scoped {
    /// Type of the scope.
    type Scope: Default;

    /// Accesses an `Rc` pointer to the scope (if exists).
    fn scope(&self) -> Option<Rc<RefCell<Self::Scope>>>;
    /// Ses the `Rc` pointer to the scope, or `None` if no scope should exist.
    fn set_scope(&mut self, Option<Rc<RefCell<Self::Scope>>>);
}
