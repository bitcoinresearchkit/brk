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
use vecdb::Database;

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
#[derive(Clone, Traversable)]
pub struct Vecs {
    #[traversable(skip)]
    pub(crate) db: Database,
    pub ath: AthVecs,
    pub lookback: LookbackVecs,
    pub returns: ReturnsVecs,
    pub volatility: VolatilityVecs,
    pub range: RangeVecs,
    pub moving_average: MovingAverageVecs,
    pub dca: DcaVecs,
    pub indicators: IndicatorsVecs,
}
