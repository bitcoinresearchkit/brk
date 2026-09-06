pub mod divide;
pub mod minus;
pub mod plus;
pub mod times;

pub use divide::Divide;
pub use minus::Minus;
pub use plus::Plus;
pub use times::Times;

/// Trait for binary transforms used by vector computations.
/// Zero-sized types implementing this get monomorphized (zero runtime cost).
pub trait BinaryTransform<In1, In2, Out = In1> {
    fn apply(lhs: In1, rhs: In2) -> Out;
}
