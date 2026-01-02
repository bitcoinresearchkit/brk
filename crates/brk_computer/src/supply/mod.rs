pub mod burned;
pub mod circulating;
pub mod inflation;
pub mod market_cap;
pub mod velocity;

mod compute;
mod import;
mod vecs;

pub use vecs::Vecs;

pub const DB_NAME: &str = "supply";
