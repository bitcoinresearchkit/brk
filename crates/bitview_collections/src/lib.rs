//! Typed shapes, field identities, and shape-preserving operations.
//! Concrete vectors and cache ownership belong to their consumers.

mod by_dca_cagr;
mod by_dca_class;
mod by_dca_period;
mod by_lookback_period;
mod by_percentile;
mod distribution_stats;
mod ohlc;
mod per_resolution;
mod percent;
mod rarity_percentiles;
mod resolution_fields;
mod windows;
mod windows_from_1w;
mod windows_to_1m;

pub use by_dca_cagr::{ByDcaCagr, DCA_CAGR_DAYS, DCA_CAGR_NAMES};
pub use by_dca_class::{ByDcaClass, DCA_CLASS_NAMES, DCA_CLASS_YEARS};
pub use by_dca_period::{ByDcaPeriod, DCA_PERIOD_DAYS, DCA_PERIOD_NAMES};
pub use by_lookback_period::{ByLookbackPeriod, LOOKBACK_PERIOD_DAYS, LOOKBACK_PERIOD_NAMES};
pub use by_percentile::ByPercentile;
pub use distribution_stats::DistributionStats;
pub use ohlc::Ohlc;
pub use per_resolution::PerResolution;
pub use percent::Percent;
pub use rarity_percentiles::RarityPercentiles;
pub use windows::{WindowId, Windows};
pub use windows_from_1w::{WindowFrom1wId, WindowsFrom1w};
pub use windows_to_1m::WindowsTo1m;
