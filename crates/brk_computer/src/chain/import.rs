use std::path::Path;

use brk_error::Result;
use brk_indexer::Indexer;
use brk_traversable::Traversable;
use brk_types::{StoredBool, StoredU64, TxIndex, Version, Weight};
use vecdb::{
    Database, EagerVec, ImportableVec, IterableCloneableVec, LazyVecFrom1, LazyVecFrom2, PAGE_SIZE,
    VecIndex,
};

use crate::{
    grouped::{
        ComputedValueVecsFromHeight, ComputedValueVecsFromTxindex, ComputedVecsFromDateIndex,
        ComputedVecsFromHeight, ComputedVecsFromTxindex, Source, VecBuilderOptions,
    },
    indexes, price,
};

use super::{
    TARGET_BLOCKS_PER_DAY, TARGET_BLOCKS_PER_DECADE, TARGET_BLOCKS_PER_MONTH,
    TARGET_BLOCKS_PER_QUARTER, TARGET_BLOCKS_PER_SEMESTER, TARGET_BLOCKS_PER_WEEK,
    TARGET_BLOCKS_PER_YEAR, Vecs,
};

impl Vecs {
    pub fn forced_import(
        parent_path: &Path,
        parent_version: Version,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        price: Option<&price::Vecs>,
    ) -> Result<Self> {
        let db = Database::open(&parent_path.join("chain"))?;
        db.set_min_len(PAGE_SIZE * 50_000_000)?;

        let version = parent_version + Version::ZERO;

        let compute_dollars = price.is_some();
        let v0 = Version::ZERO;
        let v2 = Version::TWO;
        let v4 = Version::new(4);
        let v5 = Version::new(5);

        macro_rules! eager {
            ($name:expr) => {
                EagerVec::forced_import(&db, $name, version + v0)?
            };
            ($name:expr, $v:expr) => {
                EagerVec::forced_import(&db, $name, version + $v)?
            };
        }
        macro_rules! computed_h {
            ($name:expr, $source:expr, $opts:expr) => {
                ComputedVecsFromHeight::forced_import(
                    &db,
                    $name,
                    $source,
                    version + v0,
                    indexes,
                    $opts,
                )?
            };
            ($name:expr, $source:expr, $v:expr, $opts:expr) => {
                ComputedVecsFromHeight::forced_import(
                    &db,
                    $name,
                    $source,
                    version + $v,
                    indexes,
                    $opts,
                )?
            };
        }
        macro_rules! computed_di {
            ($name:expr, $opts:expr) => {
                ComputedVecsFromDateIndex::forced_import(
                    &db,
                    $name,
                    Source::Compute,
                    version + v0,
                    indexes,
                    $opts,
                )?
            };
            ($name:expr, $v:expr, $opts:expr) => {
                ComputedVecsFromDateIndex::forced_import(
                    &db,
                    $name,
                    Source::Compute,
                    version + $v,
                    indexes,
                    $opts,
                )?
            };
        }
        macro_rules! computed_tx {
            ($name:expr, $source:expr, $opts:expr) => {
                ComputedVecsFromTxindex::forced_import(
                    &db,
                    $name,
                    $source,
                    version + v0,
                    indexes,
                    $opts,
                )?
            };
        }
        let last = || VecBuilderOptions::default().add_last();
        let sum = || VecBuilderOptions::default().add_sum();
        let sum_cum = || VecBuilderOptions::default().add_sum().add_cumulative();
        let stats = || {
            VecBuilderOptions::default()
                .add_average()
                .add_minmax()
                .add_percentiles()
        };
        let full_stats = || {
            VecBuilderOptions::default()
                .add_average()
                .add_minmax()
                .add_percentiles()
                .add_sum()
                .add_cumulative()
        };

        let txindex_to_weight = LazyVecFrom2::init(
            "weight",
            version + Version::ZERO,
            indexer.vecs.tx.txindex_to_base_size.boxed_clone(),
            indexer.vecs.tx.txindex_to_total_size.boxed_clone(),
            |index: TxIndex, txindex_to_base_size_iter, txindex_to_total_size_iter| {
                let index = index.to_usize();
                txindex_to_base_size_iter.get_at(index).map(|base_size| {
                    let total_size = txindex_to_total_size_iter.get_at_unwrap(index);

                    // This is the exact definition of a weight unit, as defined by BIP-141 (quote above).
                    let wu = usize::from(base_size) * 3 + usize::from(total_size);

                    Weight::from(bitcoin::Weight::from_wu_usize(wu))
                })
            },
        );

        let txindex_to_vsize = LazyVecFrom1::init(
            "vsize",
            version + Version::ZERO,
            txindex_to_weight.boxed_clone(),
            |index: TxIndex, iter| iter.get(index).map(brk_types::VSize::from),
        );

        let txindex_to_is_coinbase = LazyVecFrom2::init(
            "is_coinbase",
            version + Version::ZERO,
            indexer.vecs.tx.txindex_to_height.boxed_clone(),
            indexer.vecs.tx.height_to_first_txindex.boxed_clone(),
            |index: TxIndex, txindex_to_height_iter, height_to_first_txindex_iter| {
                txindex_to_height_iter.get(index).map(|height| {
                    let txindex = height_to_first_txindex_iter.get_unwrap(height);
                    StoredBool::from(index == txindex)
                })
            },
        );

        let txindex_to_input_value = eager!("input_value");
        let txindex_to_output_value = eager!("output_value");
        let txindex_to_fee = eager!("fee");
        let txindex_to_fee_rate = eager!("fee_rate");

        let dateindex_to_block_count_target = LazyVecFrom1::init(
            "block_count_target",
            version + Version::ZERO,
            indexes.dateindex_to_dateindex.boxed_clone(),
            |_, _| Some(StoredU64::from(TARGET_BLOCKS_PER_DAY)),
        );
        let weekindex_to_block_count_target = LazyVecFrom1::init(
            "block_count_target",
            version + Version::ZERO,
            indexes.weekindex_to_weekindex.boxed_clone(),
            |_, _| Some(StoredU64::from(TARGET_BLOCKS_PER_WEEK)),
        );
        let monthindex_to_block_count_target = LazyVecFrom1::init(
            "block_count_target",
            version + Version::ZERO,
            indexes.monthindex_to_monthindex.boxed_clone(),
            |_, _| Some(StoredU64::from(TARGET_BLOCKS_PER_MONTH)),
        );
        let quarterindex_to_block_count_target = LazyVecFrom1::init(
            "block_count_target",
            version + Version::ZERO,
            indexes.quarterindex_to_quarterindex.boxed_clone(),
            |_, _| Some(StoredU64::from(TARGET_BLOCKS_PER_QUARTER)),
        );
        let semesterindex_to_block_count_target = LazyVecFrom1::init(
            "block_count_target",
            version + Version::ZERO,
            indexes.semesterindex_to_semesterindex.boxed_clone(),
            |_, _| Some(StoredU64::from(TARGET_BLOCKS_PER_SEMESTER)),
        );
        let yearindex_to_block_count_target = LazyVecFrom1::init(
            "block_count_target",
            version + Version::ZERO,
            indexes.yearindex_to_yearindex.boxed_clone(),
            |_, _| Some(StoredU64::from(TARGET_BLOCKS_PER_YEAR)),
        );
        let decadeindex_to_block_count_target = LazyVecFrom1::init(
            "block_count_target",
            version + Version::ZERO,
            indexes.decadeindex_to_decadeindex.boxed_clone(),
            |_, _| Some(StoredU64::from(TARGET_BLOCKS_PER_DECADE)),
        );

        let this = Self {
            dateindex_to_block_count_target,
            weekindex_to_block_count_target,
            monthindex_to_block_count_target,
            quarterindex_to_block_count_target,
            semesterindex_to_block_count_target,
            yearindex_to_block_count_target,
            decadeindex_to_block_count_target,
            height_to_interval: eager!("interval"),
            timeindexes_to_timestamp: computed_di!(
                "timestamp",
                VecBuilderOptions::default().add_first()
            ),
            indexes_to_block_interval: computed_h!("block_interval", Source::None, stats()),
            indexes_to_block_count: computed_h!("block_count", Source::Compute, sum_cum()),
            indexes_to_1w_block_count: computed_di!("1w_block_count", last()),
            indexes_to_1m_block_count: computed_di!("1m_block_count", last()),
            indexes_to_1y_block_count: computed_di!("1y_block_count", last()),
            indexes_to_block_weight: computed_h!("block_weight", Source::None, full_stats()),
            indexes_to_block_size: computed_h!("block_size", Source::None, full_stats()),
            height_to_vbytes: eager!("vbytes"),
            height_to_24h_block_count: eager!("24h_block_count"),
            height_to_24h_coinbase_sum: eager!("24h_coinbase_sum"),
            height_to_24h_coinbase_usd_sum: eager!("24h_coinbase_usd_sum"),
            indexes_to_block_vbytes: computed_h!("block_vbytes", Source::None, full_stats()),
            difficultyepoch_to_timestamp: eager!("timestamp"),
            halvingepoch_to_timestamp: eager!("timestamp"),

            dateindex_to_fee_dominance: eager!("fee_dominance"),
            dateindex_to_subsidy_dominance: eager!("subsidy_dominance"),
            indexes_to_difficulty: computed_h!("difficulty", Source::None, last()),
            indexes_to_difficultyepoch: computed_di!("difficultyepoch", last()),
            indexes_to_halvingepoch: computed_di!("halvingepoch", last()),
            indexes_to_tx_count: computed_h!("tx_count", Source::Compute, full_stats()),
            indexes_to_input_count: computed_tx!("input_count", Source::None, full_stats()),
            indexes_to_output_count: computed_tx!("output_count", Source::None, full_stats()),
            indexes_to_tx_v1: computed_h!("tx_v1", Source::Compute, sum_cum()),
            indexes_to_tx_v2: computed_h!("tx_v2", Source::Compute, sum_cum()),
            indexes_to_tx_v3: computed_h!("tx_v3", Source::Compute, sum_cum()),
            indexes_to_sent_sum: ComputedValueVecsFromHeight::forced_import(
                &db,
                "sent_sum",
                Source::Compute,
                version + Version::ZERO,
                VecBuilderOptions::default().add_sum(),
                compute_dollars,
                indexes,
            )?,
            indexes_to_fee: ComputedValueVecsFromTxindex::forced_import(
                &db,
                "fee",
                indexer,
                indexes,
                Source::Vec(txindex_to_fee.boxed_clone()),
                version + Version::ZERO,
                price,
                VecBuilderOptions::default()
                    .add_sum()
                    .add_cumulative()
                    .add_percentiles()
                    .add_minmax()
                    .add_average(),
            )?,
            indexes_to_fee_rate: computed_tx!("fee_rate", Source::None, stats()),
            indexes_to_tx_vsize: computed_tx!("tx_vsize", Source::None, stats()),
            indexes_to_tx_weight: computed_tx!("tx_weight", Source::None, stats()),
            indexes_to_subsidy: ComputedValueVecsFromHeight::forced_import(
                &db,
                "subsidy",
                Source::Compute,
                version + Version::ZERO,
                VecBuilderOptions::default()
                    .add_percentiles()
                    .add_sum()
                    .add_cumulative()
                    .add_minmax()
                    .add_average(),
                compute_dollars,
                indexes,
            )?,
            indexes_to_coinbase: ComputedValueVecsFromHeight::forced_import(
                &db,
                "coinbase",
                Source::Compute,
                version + Version::ZERO,
                VecBuilderOptions::default()
                    .add_sum()
                    .add_cumulative()
                    .add_percentiles()
                    .add_minmax()
                    .add_average(),
                compute_dollars,
                indexes,
            )?,
            indexes_to_unclaimed_rewards: ComputedValueVecsFromHeight::forced_import(
                &db,
                "unclaimed_rewards",
                Source::Compute,
                version + Version::ZERO,
                VecBuilderOptions::default().add_sum().add_cumulative(),
                compute_dollars,
                indexes,
            )?,
            indexes_to_p2a_count: computed_h!("p2a_count", Source::Compute, full_stats()),
            indexes_to_p2ms_count: computed_h!("p2ms_count", Source::Compute, full_stats()),
            indexes_to_p2pk33_count: computed_h!("p2pk33_count", Source::Compute, full_stats()),
            indexes_to_p2pk65_count: computed_h!("p2pk65_count", Source::Compute, full_stats()),
            indexes_to_p2pkh_count: computed_h!("p2pkh_count", Source::Compute, full_stats()),
            indexes_to_p2sh_count: computed_h!("p2sh_count", Source::Compute, full_stats()),
            indexes_to_p2tr_count: computed_h!("p2tr_count", Source::Compute, full_stats()),
            indexes_to_p2wpkh_count: computed_h!("p2wpkh_count", Source::Compute, full_stats()),
            indexes_to_p2wsh_count: computed_h!("p2wsh_count", Source::Compute, full_stats()),
            indexes_to_opreturn_count: computed_h!("opreturn_count", Source::Compute, full_stats()),
            indexes_to_unknownoutput_count: computed_h!(
                "unknownoutput_count",
                Source::Compute,
                full_stats()
            ),
            indexes_to_emptyoutput_count: computed_h!(
                "emptyoutput_count",
                Source::Compute,
                full_stats()
            ),
            indexes_to_exact_utxo_count: computed_h!("exact_utxo_count", Source::Compute, last()),
            indexes_to_subsidy_usd_1y_sma: compute_dollars
                .then(|| {
                    ComputedVecsFromDateIndex::forced_import(
                        &db,
                        "subsidy_usd_1y_sma",
                        Source::Compute,
                        version + v0,
                        indexes,
                        last(),
                    )
                })
                .transpose()?,
            indexes_to_puell_multiple: compute_dollars
                .then(|| {
                    ComputedVecsFromDateIndex::forced_import(
                        &db,
                        "puell_multiple",
                        Source::Compute,
                        version + v0,
                        indexes,
                        last(),
                    )
                })
                .transpose()?,
            indexes_to_hash_rate: computed_h!("hash_rate", Source::Compute, v5, last()),
            indexes_to_hash_rate_1w_sma: computed_di!("hash_rate_1w_sma", last()),
            indexes_to_hash_rate_1m_sma: computed_di!("hash_rate_1m_sma", last()),
            indexes_to_hash_rate_2m_sma: computed_di!("hash_rate_2m_sma", last()),
            indexes_to_hash_rate_1y_sma: computed_di!("hash_rate_1y_sma", last()),
            indexes_to_difficulty_as_hash: computed_h!(
                "difficulty_as_hash",
                Source::Compute,
                last()
            ),
            indexes_to_difficulty_adjustment: computed_h!(
                "difficulty_adjustment",
                Source::Compute,
                sum()
            ),
            indexes_to_blocks_before_next_difficulty_adjustment: computed_h!(
                "blocks_before_next_difficulty_adjustment",
                Source::Compute,
                v2,
                last()
            ),
            indexes_to_days_before_next_difficulty_adjustment: computed_h!(
                "days_before_next_difficulty_adjustment",
                Source::Compute,
                v2,
                last()
            ),
            indexes_to_blocks_before_next_halving: computed_h!(
                "blocks_before_next_halving",
                Source::Compute,
                v2,
                last()
            ),
            indexes_to_days_before_next_halving: computed_h!(
                "days_before_next_halving",
                Source::Compute,
                v2,
                last()
            ),
            indexes_to_hash_price_ths: computed_h!("hash_price_ths", Source::Compute, v4, last()),
            indexes_to_hash_price_phs: computed_h!("hash_price_phs", Source::Compute, v4, last()),
            indexes_to_hash_value_ths: computed_h!("hash_value_ths", Source::Compute, v4, last()),
            indexes_to_hash_value_phs: computed_h!("hash_value_phs", Source::Compute, v4, last()),
            indexes_to_hash_price_ths_min: computed_h!(
                "hash_price_ths_min",
                Source::Compute,
                v4,
                last()
            ),
            indexes_to_hash_price_phs_min: computed_h!(
                "hash_price_phs_min",
                Source::Compute,
                v4,
                last()
            ),
            indexes_to_hash_price_rebound: computed_h!(
                "hash_price_rebound",
                Source::Compute,
                v4,
                last()
            ),
            indexes_to_hash_value_ths_min: computed_h!(
                "hash_value_ths_min",
                Source::Compute,
                v4,
                last()
            ),
            indexes_to_hash_value_phs_min: computed_h!(
                "hash_value_phs_min",
                Source::Compute,
                v4,
                last()
            ),
            indexes_to_hash_value_rebound: computed_h!(
                "hash_value_rebound",
                Source::Compute,
                v4,
                last()
            ),
            indexes_to_inflation_rate: computed_di!("inflation_rate", last()),
            indexes_to_annualized_volume: computed_di!("annualized_volume", last()),
            indexes_to_annualized_volume_btc: computed_di!("annualized_volume_btc", last()),
            indexes_to_annualized_volume_usd: computed_di!("annualized_volume_usd", last()),
            indexes_to_tx_btc_velocity: computed_di!("tx_btc_velocity", last()),
            indexes_to_tx_usd_velocity: computed_di!("tx_usd_velocity", last()),
            indexes_to_tx_per_sec: computed_di!("tx_per_sec", v2, last()),
            indexes_to_outputs_per_sec: computed_di!("outputs_per_sec", v2, last()),
            indexes_to_inputs_per_sec: computed_di!("inputs_per_sec", v2, last()),

            txindex_to_is_coinbase,
            txindex_to_input_value,
            txindex_to_output_value,
            txindex_to_fee,
            txindex_to_fee_rate,
            txindex_to_vsize,
            txindex_to_weight,

            db,
        };

        this.db.retain_regions(
            this.iter_any_exportable()
                .flat_map(|v| v.region_names())
                .collect(),
        )?;

        this.db.compact()?;

        Ok(this)
    }
}
