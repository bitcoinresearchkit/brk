pub mod count;
pub mod difficulty;
pub mod halving;
pub mod interval;
pub mod lookback;
pub mod size;
pub mod weight;

mod compute;
mod import;

use brk_traversable::Traversable;
use vecdb::{Database, Rw, StorageMode};

pub use count::Vecs as CountVecs;
pub use difficulty::Vecs as DifficultyVecs;
pub use halving::Vecs as HalvingVecs;
pub use interval::Vecs as IntervalVecs;
pub use lookback::Vecs as LookbackVecs;
pub use size::Vecs as SizeVecs;
pub use weight::Vecs as WeightVecs;

pub const DB_NAME: &str = "blocks";

pub(crate) const TARGET_BLOCKS_PER_DAY_F64: f64 = 144.0;
pub(crate) const TARGET_BLOCKS_PER_DAY_F32: f32 = 144.0;
pub(crate) const TARGET_BLOCKS_PER_DAY: u64 = 144;
pub(crate) const TARGET_BLOCKS_PER_WEEK: u64 = 7 * TARGET_BLOCKS_PER_DAY;
pub(crate) const TARGET_BLOCKS_PER_MONTH: u64 = 30 * TARGET_BLOCKS_PER_DAY;
pub(crate) const TARGET_BLOCKS_PER_QUARTER: u64 = 3 * TARGET_BLOCKS_PER_MONTH;
pub(crate) const TARGET_BLOCKS_PER_SEMESTER: u64 = 2 * TARGET_BLOCKS_PER_QUARTER;
pub(crate) const TARGET_BLOCKS_PER_YEAR: u64 = 2 * TARGET_BLOCKS_PER_SEMESTER;
pub(crate) const ONE_TERA_HASH: f64 = 1_000_000_000_000.0;

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    #[traversable(skip)]
    pub db: Database,

    pub count: CountVecs<M>,
    pub lookback: LookbackVecs<M>,
    pub interval: IntervalVecs<M>,
    #[traversable(flatten)]
    pub size: SizeVecs<M>,
    #[traversable(flatten)]
    pub weight: WeightVecs<M>,
    pub difficulty: DifficultyVecs<M>,
    pub halving: HalvingVecs<M>,
}
