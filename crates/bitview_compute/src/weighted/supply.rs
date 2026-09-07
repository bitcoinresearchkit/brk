use brk_types::{BoundedRatio, Cents, Height, Sats, Version};
use vecdb::{CachedBoxedVec, ReadableBoxedVec};

use crate::{IndexSources, LazyIndexedVec, LazySpotValuePerBlock, WeightedCohortState};

/// A lazy weighted stock from shared age-cohort inputs. Retains the historical
/// independent flooring of the weighted and complementary sides.
pub fn lazy_weighted_supply<const COMPLEMENT: bool>(
    name: &str,
    version: Version,
    supply: ReadableBoxedVec<Height, Sats>,
    weight: CachedBoxedVec<Height, BoundedRatio>,
    indexes: &IndexSources,
    spot: &CachedBoxedVec<Height, Cents>,
) -> LazySpotValuePerBlock {
    let source = LazyIndexedVec::new(
        &format!("{name}_sats"),
        version,
        supply,
        weight,
        |_, supply, weight| {
            let (weighted, complement) = WeightedCohortState::split_supply(supply, weight);
            if COMPLEMENT { complement } else { weighted }
        },
    );
    LazySpotValuePerBlock::from_sats_source(name, version, source, indexes, spot)
}
