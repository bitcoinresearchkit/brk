pub mod halve;
pub mod ident;
pub mod negate;

pub use halve::Halve;
pub use ident::Ident;
pub use negate::Negate;

/// Trait for unary transforms applied lazily during iteration.
/// Zero-sized types implementing this get monomorphized (zero runtime cost).
pub trait UnaryTransform<In, Out = In> {
    fn apply(value: In) -> Out;
}
