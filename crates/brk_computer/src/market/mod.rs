pub mod ath;
mod compute;
pub mod dca;
mod import;
pub mod indicators;
pub mod lookback;
pub mod moving_average;
pub mod range;
pub mod returns;
pub mod volatility;

use brk_traversable::Traversable;
use vecdb::{Database, Rw, StorageMode};

pub use ath::Vecs as AthVecs;
pub use dca::Vecs as DcaVecs;
pub use indicators::Vecs as IndicatorsVecs;
pub use lookback::Vecs as LookbackVecs;
pub use moving_average::Vecs as MovingAverageVecs;
pub use range::Vecs as RangeVecs;
pub use returns::Vecs as ReturnsVecs;
pub use volatility::Vecs as VolatilityVecs;

pub const DB_NAME: &str = "market";

/// Main market metrics struct composed of sub-modules
#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    #[traversable(skip)]
    pub(crate) db: Database,
    pub ath: AthVecs<M>,
    pub lookback: LookbackVecs<M>,
    pub returns: ReturnsVecs<M>,
    pub volatility: VolatilityVecs<M>,
    pub range: RangeVecs<M>,
    pub moving_average: MovingAverageVecs<M>,
    pub dca: DcaVecs<M>,
    pub indicators: IndicatorsVecs<M>,
}
