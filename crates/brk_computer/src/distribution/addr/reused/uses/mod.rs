//! Per-block reused-address-use tracking. See [`vecs::ReusedAddrUsesVecs`]
//! for the full description of the metric.

mod state;
mod vecs;

pub use state::AddrTypeToReusedAddrUseCount;
pub use vecs::ReusedAddrUsesVecs;
