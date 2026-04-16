//! Exposed address supply (sats) tracking — running sum of balances held by
//! addresses currently in the funded exposed set, per address type plus an
//! aggregated `all`. See the parent [`super`] module for the definition of
//! "exposed" and how it varies by address type.

mod share;
mod state;
mod vecs;

pub use share::ExposedSupplyShareVecs;
pub use state::AddrTypeToExposedSupply;
pub use vecs::ExposedAddrSupplyVecs;
