//! Type-related traits

use std::fmt::{Debug, Display};
use Value;

/// Specifies a Type object in your programming language.
pub trait Type: Clone + Debug + PartialEq + Display {
    /// Generates the name of the type, for display purposes
    fn name(&self) -> &str;
}

/// Finds the Type for an object implementing the `Value` trait.
pub trait FindType: Value {
    /// The type that implements the `Type` trait associated with this value.
    type Output: Type;

    /// Discovers the type of this value.
    fn ty(&self) -> Self::Output;
}

/// Methods for accessing Type information for an object.
pub trait Typed<T: Type> {
    /// Retrieve the Type (if set).
    fn ty(&self) -> Option<T>;
    /// Set (or unset if passing `None`) the Type.
    fn set_type(&mut self, ty: Option<T>);
    /// Retrieve the promotion Type (if set). Defaults to `ty()` if not implemented.
    fn promote_type(&self) -> Option<T> { self.ty() }
    /// Set (or unset if passing `None`) the promotion Type. Default to `set_type()` if not
    /// implemented.
    fn set_promote_type(&mut self, ty: Option<T>) { self.set_type(ty) }
}
