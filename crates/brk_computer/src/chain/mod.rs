pub mod block;
pub mod coinbase;
mod compute;
pub mod epoch;
mod import;
pub mod mining;
pub mod output_type;
pub mod transaction;
pub mod volume;

use brk_traversable::Traversable;
use vecdb::Database;

pub use block::Vecs as BlockVecs;
pub use coinbase::Vecs as CoinbaseVecs;
pub use epoch::Vecs as EpochVecs;
pub use mining::Vecs as MiningVecs;
pub use output_type::Vecs as OutputTypeVecs;
pub use transaction::Vecs as TransactionVecs;
pub use volume::Vecs as VolumeVecs;

pub const DB_NAME: &str = "chain";

pub(crate) const TARGET_BLOCKS_PER_DAY_F64: f64 = 144.0;
pub(crate) const TARGET_BLOCKS_PER_DAY_F32: f32 = 144.0;
pub(crate) const TARGET_BLOCKS_PER_DAY: u64 = 144;
pub(crate) const TARGET_BLOCKS_PER_WEEK: u64 = 7 * TARGET_BLOCKS_PER_DAY;
pub(crate) const TARGET_BLOCKS_PER_MONTH: u64 = 30 * TARGET_BLOCKS_PER_DAY;
pub(crate) const TARGET_BLOCKS_PER_QUARTER: u64 = 3 * TARGET_BLOCKS_PER_MONTH;
pub(crate) const TARGET_BLOCKS_PER_SEMESTER: u64 = 2 * TARGET_BLOCKS_PER_QUARTER;
pub(crate) const TARGET_BLOCKS_PER_YEAR: u64 = 2 * TARGET_BLOCKS_PER_SEMESTER;
pub(crate) const TARGET_BLOCKS_PER_DECADE: u64 = 10 * TARGET_BLOCKS_PER_YEAR;
pub(crate) const ONE_TERA_HASH: f64 = 1_000_000_000_000.0;

/// Main chain metrics struct composed of sub-modules
#[derive(Clone, Traversable)]
pub struct Vecs {
    #[traversable(skip)]
    pub(crate) db: Database,
    pub block: BlockVecs,
    pub epoch: EpochVecs,
    pub mining: MiningVecs,
    pub coinbase: CoinbaseVecs,
    pub transaction: TransactionVecs,
    pub output_type: OutputTypeVecs,
    pub volume: VolumeVecs,
}
