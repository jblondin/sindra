//! Value and associated traits for run-time in-memory values.
use std::fmt::{Debug, Display};
use Type;

/// Value trait (empty, just a collection of other traits). Used to specify the required traits
/// for any stored value in a programming language.
///
/// Typically, the Value trait is implemented by an enum which contains the possible values that
/// can occur. This is not strictly necessary, though.
pub trait Value: Clone + Debug + PartialEq + Display {}

/// Trait for type coercions for values.
pub trait Coerce<T: Type> {
    /// Coerce a value to target type, depending on the value of the option.
    fn coerce(self, dest_ty: Option<T>) -> Self;
}

/// Trait for type casting of values.
pub trait Cast<T: Type> {
    /// Cast a value to the target type (should return unchanged value if case is not possible).
    fn cast(self, dest_ty: T) -> Self;
}

/// Trait for extracting rust value (of type T) from Value type.
pub trait Extract<T>: Value {
    /// Extract the value, returning an `Err` if the Value is of the wrong variant.
    fn extract(&self) -> Result<T, String>;
}
