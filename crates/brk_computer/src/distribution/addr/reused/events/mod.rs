//! Per-block reused-address event tracking. Holds both the output-side
//! ("an output landed on a previously-used address") and input-side
//! ("an input spent from an address in the reused set") event counters.
//! See [`vecs::ReusedAddrEventsVecs`] for the full description of each
//! metric.

mod state;
mod vecs;

pub use state::AddrTypeToReusedAddrEventCount;
pub use vecs::ReusedAddrEventsVecs;
