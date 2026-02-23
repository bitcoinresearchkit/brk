use brk_error::Result;
use brk_types::{Dollars, Height};
use tracing::info;
use vecdb::{Exit, ReadableVec};

use crate::{ComputeIndexes, blocks, prices};

use super::super::cohorts::{AddressCohorts, UTXOCohorts};

/// Compute overlapping cohorts from component cohorts.
///
/// For example:
/// - ">=1d" UTXO cohort is computed from sum of age_range cohorts that match
/// - ">=1 BTC" address cohort is computed from sum of amount_range cohorts that match
pub(crate) fn compute_overlapping(
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
/// Converts height-indexed data to day1-indexed data and other transforms.
pub(crate) fn compute_rest_part1(
    utxo_cohorts: &mut UTXOCohorts,
    address_cohorts: &mut AddressCohorts,
    blocks: &blocks::Vecs,
    prices: &prices::Vecs,
    starting_indexes: &ComputeIndexes,
    exit: &Exit,
) -> Result<()> {
    info!("Computing rest part 1...");

    utxo_cohorts.compute_rest_part1(blocks, prices, starting_indexes, exit)?;
    address_cohorts.compute_rest_part1(blocks, prices, starting_indexes, exit)?;

    Ok(())
}

/// Second phase of post-processing: compute relative metrics.
///
/// Computes supply ratios, market cap ratios, etc. using total references.
pub(crate) fn compute_rest_part2<HM>(
    utxo_cohorts: &mut UTXOCohorts,
    address_cohorts: &mut AddressCohorts,
    blocks: &blocks::Vecs,
    prices: &prices::Vecs,
    starting_indexes: &ComputeIndexes,
    height_to_market_cap: Option<&HM>,
    exit: &Exit,
) -> Result<()>
where
    HM: ReadableVec<Height, Dollars> + Sync,
{
    info!("Computing rest part 2...");

    utxo_cohorts.compute_rest_part2(
        blocks,
        prices,
        starting_indexes,
        height_to_market_cap,
        exit,
    )?;

    address_cohorts.compute_rest_part2(
        blocks,
        prices,
        starting_indexes,
        height_to_market_cap,
        exit,
    )?;

    Ok(())
}
