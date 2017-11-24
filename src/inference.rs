//! Traits for result type inference and type promotion.

use Type;

/// Trait for determining whether or not a promotion to a new type is available and necessary.
pub trait InferPromotion: ::std::marker::Sized {
    /// Returns `Some(...)` when a promotion is necessary, and `None` when either a promotion is
    /// not possible or when is not necessary.
    fn infer_promotion(&self, dest_ty: Self) -> Option<Self>;
}

/// Container for inferred operand / result types for unary operations.
pub struct BinaryOpTypes<T> {
    /// Operation result type.
    pub result: T,
    /// Required left operand type.
    pub left: T,
    /// Required right operand type.
    pub right: T,
}

/// Trait for determining the result type of a binary operation.
pub trait InferTypesBinary {
    /// The Type object being used for the operands.
    type Operand: Type;

    /// When the operation is possible, returns `Some(...)` containing the required operand
    /// coercions and result type.
    /// When the operation is not possible with the given operand types, returns `None`.
    fn infer_types(&self, left: Self::Operand, right: Self::Operand)
        -> Option<BinaryOpTypes<Self::Operand>>;
}

/// Container for inferred operand / result types for unary operations.
pub struct UnaryOpTypes<T> {
    /// Operation result type.
    pub result: T,
    /// Required operand type.
    pub operand: T,
}
/// Trait for determining the result type of a unary operation.
pub trait InferTypesUnary {
    /// The Type object being used as the operand.
    type Operand: Type;

    /// When the operation is posssible, returns `Some(...)` containing the required operand
    /// coercions and the result type.
    /// When the operation is not possible with the given operand type, return `None`.
    fn infer_types(&self, operand: Self::Operand) -> Option<UnaryOpTypes<Self::Operand>>;
}
