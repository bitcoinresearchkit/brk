//! Stateful in-memory holders. Each owns its `RwLock` and exposes a
//! behaviour-shaped API (insert, remove, evict, query).
//!
//! [`state::MempoolState`] aggregates five locked buckets:
//!
//! - [`tx_store::TxStore`] - full `Transaction` data for live txs.
//! - [`addr_tracker::AddrTracker`] - per-address mempool stats.
//! - [`entry_pool::EntryPool`] - slot-recycled [`TxEntry`](crate::TxEntry)
//!   storage indexed by [`entry_pool::TxIndex`].
//! - [`outpoint_spends::OutpointSpends`] - outpoint → spending mempool
//!   tx index, used to answer mempool-to-mempool outspend queries.
//! - [`tx_graveyard::TxGraveyard`] - recently-dropped txs as
//!   [`tx_graveyard::TxTombstone`]s, retained for reappearance
//!   detection and post-mine analytics.
//!
//! A sixth bucket, `info`, holds a `MempoolInfo` from `brk_types`,
//! so it has no file here.

pub mod addr_tracker;
pub mod entry_pool;
pub(crate) mod outpoint_spends;
pub mod state;
pub mod tx_graveyard;
pub mod tx_store;

pub use addr_tracker::AddrTracker;
pub use entry_pool::{EntryPool, TxIndex};
pub(crate) use outpoint_spends::OutpointSpends;
pub(crate) use state::LockedState;
pub use state::MempoolState;
pub use tx_graveyard::{TxGraveyard, TxTombstone};
pub use tx_store::TxStore;
