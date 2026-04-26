//! State held inside the mempool, plus the value types stored in it.
//!
//! [`state::MempoolState`] aggregates four locked buckets:
//!
//! - [`tx_store::TxStore`] - full `Transaction` data for live txs.
//! - [`addr_tracker::AddrTracker`] - per-address mempool stats.
//! - [`entry_pool::EntryPool`] - slot-recycled `Entry` storage indexed
//!   by [`tx_index::TxIndex`].
//! - [`tx_graveyard::TxGraveyard`] - recently-dropped txs as
//!   [`tombstone::Tombstone`]s, retained for reappearance detection
//!   and post-mine analytics.
//!
//! A fifth bucket, `info`, holds a `MempoolInfo` from `brk_types`,
//! so it has no file here.

pub mod addr_tracker;
pub mod entry;
pub mod entry_pool;
pub mod state;
pub mod tombstone;
pub mod tx_graveyard;
pub mod tx_index;
pub mod tx_store;

pub use addr_tracker::AddrTracker;
pub use entry::Entry;
pub use entry_pool::EntryPool;
pub use state::MempoolState;
pub use tombstone::Tombstone;
pub use tx_graveyard::TxGraveyard;
pub use tx_index::TxIndex;
pub use tx_store::TxStore;
