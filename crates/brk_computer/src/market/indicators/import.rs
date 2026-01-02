use brk_error::Result;
use brk_types::Version;
use vecdb::{Database, EagerVec, ImportableVec, IterableCloneableVec, LazyVecFrom2};

use super::{super::moving_average, Vecs};
use crate::{
    distribution, indexes,
    internal::{
        ComputedVecsFromDateIndex, DifferenceF32, LazyVecsFrom2FromDateIndex, Ratio32, RsiFormula,
        Source, VecBuilderOptions,
    },
    transactions,
};

impl Vecs {
    pub fn forced_import(
        db: &Database,
        version: Version,
        indexes: &indexes::Vecs,
        compute_dollars: bool,
        distribution: &distribution::Vecs,
        transactions: &transactions::Vecs,
        moving_average: &moving_average::Vecs,
    ) -> Result<Self> {
        let v0 = Version::ZERO;
        let last = || VecBuilderOptions::default().add_last();

        let indexes_to_nvt = distribution
            .utxo_cohorts
            .all
            .metrics
            .supply
            .indexes_to_supply
            .dollars
            .as_ref()
            .zip(transactions.volume.indexes_to_sent_sum.dollars.as_ref())
            .map(|(market_cap, volume)| {
                LazyVecsFrom2FromDateIndex::from_dateindex_and_height::<Ratio32>(
                    "nvt",
                    version + v0,
                    market_cap,
                    volume,
                )
            });

        let dateindex_to_rsi_gains = EagerVec::forced_import(db, "rsi_gains", version + v0)?;
        let dateindex_to_rsi_losses = EagerVec::forced_import(db, "rsi_losses", version + v0)?;
        let dateindex_to_rsi_avg_gain_14d =
            EagerVec::forced_import(db, "rsi_avg_gain_14d", version + v0)?;
        let dateindex_to_rsi_avg_loss_14d =
            EagerVec::forced_import(db, "rsi_avg_loss_14d", version + v0)?;
        let dateindex_to_rsi_14d = LazyVecFrom2::transformed::<RsiFormula>(
            "rsi_14d",
            version + v0,
            dateindex_to_rsi_avg_gain_14d.boxed_clone(),
            dateindex_to_rsi_avg_loss_14d.boxed_clone(),
        );

        let dateindex_to_macd_line = EagerVec::forced_import(db, "macd_line", version + v0)?;
        let dateindex_to_macd_signal = EagerVec::forced_import(db, "macd_signal", version + v0)?;
        let dateindex_to_macd_histogram = LazyVecFrom2::transformed::<DifferenceF32>(
            "macd_histogram",
            version + v0,
            dateindex_to_macd_line.boxed_clone(),
            dateindex_to_macd_signal.boxed_clone(),
        );

        let dateindex_to_rsi_14d_min = EagerVec::forced_import(db, "rsi_14d_min", version + v0)?;
        let dateindex_to_rsi_14d_max = EagerVec::forced_import(db, "rsi_14d_max", version + v0)?;
        let dateindex_to_stoch_rsi = EagerVec::forced_import(db, "stoch_rsi", version + v0)?;
        let dateindex_to_stoch_rsi_k = EagerVec::forced_import(db, "stoch_rsi_k", version + v0)?;
        let dateindex_to_stoch_rsi_d = EagerVec::forced_import(db, "stoch_rsi_d", version + v0)?;

        let dateindex_to_stoch_k = EagerVec::forced_import(db, "stoch_k", version + v0)?;
        let dateindex_to_stoch_d = EagerVec::forced_import(db, "stoch_d", version + v0)?;

        let dateindex_to_gini = EagerVec::forced_import(db, "gini", version + v0)?;

        // Pi Cycle Top: 111d SMA / (2 * 350d SMA) - signals top when > 1
        let dateindex_to_pi_cycle = moving_average
            .indexes_to_price_111d_sma
            .price
            .as_ref()
            .and_then(|sma_111| sma_111.dateindex.as_ref())
            .zip(moving_average.indexes_to_price_350d_sma_x2.dateindex.as_ref())
            .map(|(sma_111, sma_350_x2)| {
                LazyVecFrom2::transformed::<Ratio32>(
                    "pi_cycle",
                    version + v0,
                    sma_111.boxed_clone(),
                    sma_350_x2.boxed_clone(),
                )
            });

        Ok(Self {
            indexes_to_puell_multiple: compute_dollars
                .then(|| {
                    ComputedVecsFromDateIndex::forced_import(
                        db,
                        "puell_multiple",
                        Source::Compute,
                        version + v0,
                        indexes,
                        last(),
                    )
                })
                .transpose()?,
            indexes_to_nvt,
            dateindex_to_rsi_gains,
            dateindex_to_rsi_losses,
            dateindex_to_rsi_avg_gain_14d,
            dateindex_to_rsi_avg_loss_14d,
            dateindex_to_rsi_14d,
            dateindex_to_rsi_14d_min,
            dateindex_to_rsi_14d_max,
            dateindex_to_stoch_rsi,
            dateindex_to_stoch_rsi_k,
            dateindex_to_stoch_rsi_d,
            dateindex_to_stoch_k,
            dateindex_to_stoch_d,
            dateindex_to_pi_cycle,
            dateindex_to_macd_line,
            dateindex_to_macd_signal,
            dateindex_to_macd_histogram,
            dateindex_to_gini,
        })
    }
}
