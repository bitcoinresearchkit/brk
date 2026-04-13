//! Exposed address supply (sats) tracking — running sum of balances held by
//! addresses currently in the funded exposed set, per address type plus an
//! aggregated `all`. See the parent [`super`] module for the definition of
//! "exposed" and how it varies by address type.

mod state;
mod vecs;

pub use state::AddrTypeToExposedAddrSupply;
pub use vecs::ExposedAddrSupplyVecs;
