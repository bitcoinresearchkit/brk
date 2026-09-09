mod reverse_operands;

pub use reverse_operands::ReverseOperands;

/// Trait for binary transforms used by vector computations.
/// Zero-sized types implementing this get monomorphized (zero runtime cost).
pub trait BinaryTransform<In1, In2, Out = In1> {
    fn apply(lhs: In1, rhs: In2) -> Out;
}
