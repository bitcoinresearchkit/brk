use bitview_plugin_coinflow::Vecs as CoinflowVecs;
use bitview_plugin_cointime::Vecs as CointimeVecs;
use bitview_plugin_distribution::Vecs as DistributionVecs;
use bitview_plugin_indexer::Indexer;
use bitview_plugin_mappings::Vecs as MappingsVecs;
use bitview_traversable::Traversable;
use brk_error::Result;
use brk_exit::Exit;
use brk_types::Version;
use rayon::prelude::*;
use vecdb::{CacheBudget, Database, Rw, StorageMode};

use super::{Component, component, reference_prices::ReferencePrices};

#[derive(Traversable)]
pub struct Components<M: StorageMode = Rw> {
    /// Rarity Meter component using the all-chain realized price—the
    /// satoshi-weighted mean creation price of all unspent outputs—as its
    /// reference.
    pub realized_price: Component<M>,
    /// Rarity Meter component using the all-chain capitalized price—the mean
    /// creation price weighted by value invested at creation—as its reference.
    pub capitalized_price: Component<M>,
    /// Rarity Meter component using the satoshi-weighted mean creation price of
    /// UTXOs younger than 150 days as its reference.
    pub sth_realized_price: Component<M>,
    /// Rarity Meter component using the value-weighted mean creation price of
    /// UTXOs younger than 150 days as its reference.
    pub sth_capitalized_price: Component<M>,
    /// Rarity Meter component using the satoshi-weighted mean creation price of
    /// UTXOs at least 150 days old as its reference.
    pub lth_realized_price: Component<M>,
    /// Rarity Meter component using the value-weighted mean creation price of
    /// UTXOs at least 150 days old as its reference.
    pub lth_capitalized_price: Component<M>,
    /// Rarity Meter component using the satoshi-weighted mean creation price of
    /// UTXOs at least 180 days old as its reference.
    pub over_6m_realized_price: Component<M>,
    /// Rarity Meter component using the satoshi-weighted mean creation price of
    /// UTXOs at least 120 days old as its reference.
    pub over_4m_realized_price: Component<M>,
    /// Rarity Meter component using the satoshi-weighted mean creation price of
    /// UTXOs less than 120 days old as its reference.
    pub under_4m_realized_price: Component<M>,
    /// Rarity Meter component using the satoshi-weighted mean creation price of
    /// UTXOs less than 180 days old as its reference.
    pub under_6m_realized_price: Component<M>,
    /// Rarity Meter component using cointime vaulted price as its reference:
    /// realized price divided by one minus liveliness, where liveliness is
    /// cumulative coinblocks destroyed divided by cumulative coinblocks
    /// created.
    pub vaulted_price: Component<M>,
    /// Rarity Meter component using cointime active price as its reference:
    /// realized price divided by liveliness, where liveliness is cumulative
    /// coinblocks destroyed divided by cumulative coinblocks created.
    pub active_price: Component<M>,
    /// Rarity Meter component using cointime true market mean price as its
    /// reference: realized capitalization minus cumulative issuance-date
    /// subsidy value, divided by active supply; active supply is circulating
    /// supply multiplied by liveliness.
    pub true_market_mean_price: Component<M>,
    /// Rarity Meter component using cointime price as its reference: the
    /// cumulative sum of spot price multiplied by coinblocks destroyed, divided
    /// by cumulative coinblocks stored.
    pub cointime_price: Component<M>,
    /// Rarity Meter component using coinflow price as its reference: realized
    /// capitalization weighted by each UTXO age range's estimated eventual
    /// spending probability, divided by supply weighted by the same
    /// probability.
    pub coinflow_price: Component<M>,
}

#[allow(clippy::too_many_arguments)]
pub fn forced_import(
    cache: &'static CacheBudget,
    db: &Database,
    version: Version,
    mappings: &MappingsVecs,
    distribution: &DistributionVecs,
    reference_prices: &ReferencePrices,
    cointime: &CointimeVecs,
    coinflow: &CoinflowVecs,
) -> Result<Components> {
    let utxos = &distribution.cohorts;
    let realized_price = &utxos.realized.price.cohorts;
    let capitalized_price = &utxos.realized.capitalized_price.series;

    macro_rules! import {
        ($name:expr, $source:expr) => {
            component::forced_import(cache, db, $name, version, mappings, &$source.cents.height)?
        };
    }

    Ok(Components {
        realized_price: import!("realized_price", realized_price.all),
        capitalized_price: import!("capitalized_price", capitalized_price.all),
        sth_realized_price: import!("sth_realized_price", realized_price.term.short),
        sth_capitalized_price: import!("sth_capitalized_price", capitalized_price.sth),
        lth_realized_price: import!("lth_realized_price", realized_price.term.long),
        lth_capitalized_price: import!("lth_capitalized_price", capitalized_price.lth),
        over_6m_realized_price: import!("over_6m_realized_price", reference_prices.over_6m),
        over_4m_realized_price: import!("over_4m_realized_price", reference_prices.over_4m),
        under_4m_realized_price: import!("under_4m_realized_price", reference_prices.under_4m),
        under_6m_realized_price: import!("under_6m_realized_price", reference_prices.under_6m),
        vaulted_price: import!("vaulted_price", cointime.prices.vaulted),
        active_price: import!("active_price", cointime.prices.active),
        true_market_mean_price: import!("true_market_mean_price", cointime.prices.true_market_mean),
        cointime_price: import!("cointime_price", cointime.prices.cointime),
        coinflow_price: import!("coinflow_price", coinflow.all.price),
    })
}

pub fn compute(
    components: &mut Components,
    indexer: &Indexer,
    distribution: &DistributionVecs,
    reference_prices: &ReferencePrices,
    cointime: &CointimeVecs,
    coinflow: &CoinflowVecs,
    exit: &Exit,
) -> Result<()> {
    let starting_lengths = indexer.safe_lengths();
    let utxos = &distribution.cohorts;
    let realized_price = &utxos.realized.price.cohorts;
    let capitalized_price = &utxos.realized.capitalized_price.series;

    let jobs = [
        (
            &mut components.realized_price,
            &realized_price.all.relative.ratio.height,
        ),
        (
            &mut components.capitalized_price,
            &capitalized_price.all.relative.ratio.height,
        ),
        (
            &mut components.sth_realized_price,
            &realized_price.term.short.relative.ratio.height,
        ),
        (
            &mut components.sth_capitalized_price,
            &capitalized_price.sth.relative.ratio.height,
        ),
        (
            &mut components.lth_realized_price,
            &realized_price.term.long.relative.ratio.height,
        ),
        (
            &mut components.lth_capitalized_price,
            &capitalized_price.lth.relative.ratio.height,
        ),
        (
            &mut components.over_6m_realized_price,
            &reference_prices.over_6m.relative.ratio.height,
        ),
        (
            &mut components.over_4m_realized_price,
            &reference_prices.over_4m.relative.ratio.height,
        ),
        (
            &mut components.under_4m_realized_price,
            &reference_prices.under_4m.relative.ratio.height,
        ),
        (
            &mut components.under_6m_realized_price,
            &reference_prices.under_6m.relative.ratio.height,
        ),
        (
            &mut components.vaulted_price,
            &cointime.prices.vaulted.relative.ratio.height,
        ),
        (
            &mut components.active_price,
            &cointime.prices.active.relative.ratio.height,
        ),
        (
            &mut components.true_market_mean_price,
            &cointime.prices.true_market_mean.relative.ratio.height,
        ),
        (
            &mut components.cointime_price,
            &cointime.prices.cointime.relative.ratio.height,
        ),
        (
            &mut components.coinflow_price,
            &coinflow.all.price.relative.ratio.height,
        ),
    ];
    let has_work = jobs
        .iter()
        .any(|(component, source)| component.needs_compute(starting_lengths.height, *source));
    let compute =
        |(component, source)| component::compute(component, &starting_lengths, source, exit);

    if has_work {
        jobs.into_par_iter().try_for_each(compute)
    } else {
        jobs.into_iter().try_for_each(compute)
    }
}
