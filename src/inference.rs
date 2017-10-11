//! Traits for result type inference and type promotion.

use Type;

/// Trait for determining whether or not a promotion to a new type is available and necessary.
pub trait InferPromotion: ::std::marker::Sized {
    /// Returns `Some(...)` when a promotion is necessary, and `None` when either a promotion is
    /// not possible or when is not necessary.
    fn infer_promotion(&self, dest_ty: Self) -> Option<Self>;
}

/// Trait for determining the result type of a binary operation.
pub trait InferResultBinary {
    /// The Type object being used for the operands.
    type Operand: Type;

    /// Returns `Some(...)` containing the resulting Type when the operation is possible, and `None`
    /// when the operation is not possible with the given operands.
    fn infer_result_type(&self, left: Self::Operand, right: Self::Operand) -> Option<Self::Operand>;
}

/// Trait for determining the result type of a unary operation.
pub trait InferResultUnary {
    /// The Type object being used as the operand.
    type Operand: Type;

    /// Returns `Some(...)` containing the resulting Type when the operation is possible, and `None`
    /// when the operation is not possible with the given operand.
    fn infer_result_type(&self, operand: Self::Operand) -> Option<Self::Operand>;
}
