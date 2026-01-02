pub mod count;
pub mod difficulty;
pub mod halving;
pub mod interval;
pub mod mining;
pub mod rewards;
pub mod size;
pub mod time;
pub mod weight;

mod compute;
mod import;

use brk_traversable::Traversable;
use vecdb::Database;

pub use count::Vecs as CountVecs;
pub use difficulty::Vecs as DifficultyVecs;
pub use halving::Vecs as HalvingVecs;
pub use interval::Vecs as IntervalVecs;
pub use mining::Vecs as MiningVecs;
pub use rewards::Vecs as RewardsVecs;
pub use size::Vecs as SizeVecs;
pub use time::Vecs as TimeVecs;
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
pub(crate) const TARGET_BLOCKS_PER_DECADE: u64 = 10 * TARGET_BLOCKS_PER_YEAR;
pub(crate) const ONE_TERA_HASH: f64 = 1_000_000_000_000.0;

#[derive(Clone, Traversable)]
pub struct Vecs {
    #[traversable(skip)]
    pub(crate) db: Database,

    pub count: CountVecs,
    pub interval: IntervalVecs,
    pub size: SizeVecs,
    pub weight: WeightVecs,
    pub time: TimeVecs,
    pub mining: MiningVecs,
    pub rewards: RewardsVecs,
    pub difficulty: DifficultyVecs,
    pub halving: HalvingVecs,
}
