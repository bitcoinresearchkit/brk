mod chunk_input;
mod cluster;
mod cluster_chunk;
mod cluster_tx;
mod cluster_tx_index;
mod entry;
mod info;
mod linearize;

pub use chunk_input::ChunkInput;
pub use cluster::CpfpCluster;
pub use cluster_chunk::{CpfpClusterChunk, find_seed_chunk};
pub use cluster_tx::CpfpClusterTx;
pub use cluster_tx_index::CpfpClusterTxIndex;
pub use entry::CpfpEntry;
pub use info::CpfpInfo;
pub use linearize::linearize;

/// Bitcoin Core's default mempool ancestor/descendant chain cap, also
/// used by mempool.space-style truncation in CPFP walks.
pub const CPFP_CHAIN_LIMIT: usize = 25;
