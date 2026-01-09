use brk_error::Result;
use brk_types::{DateIndex, Dollars, Height};
use tracing::info;
use vecdb::{Exit, IterableVec};

use crate::{ComputeIndexes, indexes, price};

use super::super::cohorts::{AddressCohorts, UTXOCohorts};

/// Compute overlapping cohorts from component cohorts.
///
/// For example:
/// - ">=1d" UTXO cohort is computed from sum of age_range cohorts that match
/// - ">=1 BTC" address cohort is computed from sum of amount_range cohorts that match
pub fn compute_overlapping(
    utxo_cohorts: &mut UTXOCohorts,
    address_cohorts: &mut AddressCohorts,
    starting_indexes: &ComputeIndexes,
    exit: &Exit,
) -> Result<()> {
    info!("Computing overlapping cohorts...");

    utxo_cohorts.compute_overlapping_vecs(starting_indexes, exit)?;
    address_cohorts.compute_overlapping_vecs(starting_indexes, exit)?;

    Ok(())
}

/// First phase of post-processing: compute index transforms.
///
/// Converts height-indexed data to dateindex-indexed data and other transforms.
pub fn compute_rest_part1(
    utxo_cohorts: &mut UTXOCohorts,
    address_cohorts: &mut AddressCohorts,
    indexes: &indexes::Vecs,
    price: Option<&price::Vecs>,
    starting_indexes: &ComputeIndexes,
    exit: &Exit,
) -> Result<()> {
    info!("Computing rest part 1...");

    utxo_cohorts.compute_rest_part1(indexes, price, starting_indexes, exit)?;
    address_cohorts.compute_rest_part1(indexes, price, starting_indexes, exit)?;

    Ok(())
}

/// Second phase of post-processing: compute relative metrics.
///
/// Computes supply ratios, market cap ratios, etc. using total references.
#[allow(clippy::too_many_arguments)]
pub fn compute_rest_part2<HM, DM>(
    utxo_cohorts: &mut UTXOCohorts,
    address_cohorts: &mut AddressCohorts,
    indexes: &indexes::Vecs,
    price: Option<&price::Vecs>,
    starting_indexes: &ComputeIndexes,
    height_to_market_cap: Option<&HM>,
    dateindex_to_market_cap: Option<&DM>,
    exit: &Exit,
) -> Result<()>
where
    HM: IterableVec<Height, Dollars> + Sync,
    DM: IterableVec<DateIndex, Dollars> + Sync,
{
    info!("Computing rest part 2...");

    utxo_cohorts.compute_rest_part2(
        indexes,
        price,
        starting_indexes,
        height_to_market_cap,
        dateindex_to_market_cap,
        exit,
    )?;

    address_cohorts.compute_rest_part2(
        indexes,
        price,
        starting_indexes,
        height_to_market_cap,
        dateindex_to_market_cap,
        exit,
    )?;

    Ok(())
}
