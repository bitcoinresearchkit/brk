mod _type;
mod eager;
mod lazy;

pub use _type::*;
pub use eager::*;
pub use lazy::*;

#[derive(Debug, Clone, Copy)]
enum Mode {
    Lazy,
    Eager,
}
