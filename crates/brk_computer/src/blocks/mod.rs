pub mod count;
pub mod difficulty;
pub mod halving;
pub mod interval;
pub mod size;
pub mod time;
pub mod weight;

mod compute;
mod import;

use brk_traversable::Traversable;
use vecdb::{Database, Rw, StorageMode};

pub use count::Vecs as CountVecs;
pub use difficulty::Vecs as DifficultyVecs;
pub use halving::Vecs as HalvingVecs;
pub use interval::Vecs as IntervalVecs;
pub use size::Vecs as SizeVecs;
pub use time::Vecs as TimeVecs;
pub use weight::Vecs as WeightVecs;

pub const DB_NAME: &str = "blocks";

pub(crate) const TARGET_BLOCKS_PER_DAY_F64: f64 = 144.0;
pub(crate) const TARGET_BLOCKS_PER_DAY_F32: f32 = 144.0;
pub(crate) const TARGET_BLOCKS_PER_MINUTE1: u64 = 0;
pub(crate) const TARGET_BLOCKS_PER_MINUTE5: u64 = 0;
pub(crate) const TARGET_BLOCKS_PER_MINUTE10: u64 = 1;
pub(crate) const TARGET_BLOCKS_PER_MINUTE30: u64 = 3;
pub(crate) const TARGET_BLOCKS_PER_HOUR1: u64 = 6;
pub(crate) const TARGET_BLOCKS_PER_HOUR4: u64 = 24;
pub(crate) const TARGET_BLOCKS_PER_HOUR12: u64 = 72;
pub(crate) const TARGET_BLOCKS_PER_DAY: u64 = 144;
pub(crate) const TARGET_BLOCKS_PER_DAY3: u64 = 3 * TARGET_BLOCKS_PER_DAY;
pub(crate) const TARGET_BLOCKS_PER_WEEK: u64 = 7 * TARGET_BLOCKS_PER_DAY;
pub(crate) const TARGET_BLOCKS_PER_MONTH: u64 = 30 * TARGET_BLOCKS_PER_DAY;
pub(crate) const TARGET_BLOCKS_PER_QUARTER: u64 = 3 * TARGET_BLOCKS_PER_MONTH;
pub(crate) const TARGET_BLOCKS_PER_SEMESTER: u64 = 2 * TARGET_BLOCKS_PER_QUARTER;
pub(crate) const TARGET_BLOCKS_PER_YEAR: u64 = 2 * TARGET_BLOCKS_PER_SEMESTER;
pub(crate) const TARGET_BLOCKS_PER_DECADE: u64 = 10 * TARGET_BLOCKS_PER_YEAR;
pub(crate) const TARGET_BLOCKS_PER_HALVING: u64 = 210_000;
pub(crate) const ONE_TERA_HASH: f64 = 1_000_000_000_000.0;

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    #[traversable(skip)]
    pub(crate) db: Database,

    pub count: CountVecs<M>,
    pub interval: IntervalVecs<M>,
    #[traversable(flatten)]
    pub size: SizeVecs<M>,
    #[traversable(flatten)]
    pub weight: WeightVecs<M>,
    pub time: TimeVecs<M>,
    pub difficulty: DifficultyVecs<M>,
    pub halving: HalvingVecs<M>,
}
