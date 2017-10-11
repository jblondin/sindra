//! Traits for performing binary and unary operations.

use Type;
use Value;

/// Logic for computing the result of a binary operation.
pub trait BinaryOperator<T: Type, V: Value> {
    /// Performs the operation with operands `left` and `right`, with output type `ty`.
    ///
    /// # Failures
    /// Generates a run-time failure when the two values cannot be combined to form the output type
    /// `ty`.
    fn op(&self, ty: T, left: &V, right: &V) -> Result<V, String>;
}

/// Logic for computing the result of a unary operation.
pub trait UnaryOperator<T: Type, V: Value> {
    /// Performs the oepration with the operand `operand`, with outpu type `ty`.type
    ///
    /// # Falures
    /// Generates a run-time failure with the value cannot form the output type `ty`.
    fn op(&self, ty: T, operand: &V) -> Result<V, String>;
}
