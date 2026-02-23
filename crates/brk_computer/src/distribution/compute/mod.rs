pub mod aggregates;
mod block_loop;
mod context;
mod readers;
mod recover;
mod write;

pub(crate) use block_loop::process_blocks;
pub(crate) use context::{ComputeContext, PriceRangeMax};
pub(crate) use readers::{
    TxInReaders, TxOutData, TxOutReaders, VecsReaders, build_txinindex_to_txindex,
    build_txoutindex_to_txindex,
};
pub(crate) use recover::{StartMode, determine_start_mode, recover_state, reset_state};

/// Flush checkpoint interval (every N blocks).
pub const FLUSH_INTERVAL: usize = 10_000;

// BIP30 duplicate coinbase heights (special case handling)
pub const BIP30_DUPLICATE_HEIGHT_1: u32 = 91_842;
pub const BIP30_DUPLICATE_HEIGHT_2: u32 = 91_880;
pub const BIP30_ORIGINAL_HEIGHT_1: u32 = 91_812;
pub const BIP30_ORIGINAL_HEIGHT_2: u32 = 91_722;
