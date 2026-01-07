use brk_error::Result;
use brk_types::Version;
use vecdb::{Database, EagerVec, ImportableVec, IterableCloneableVec, LazyVecFrom2};

use super::{super::moving_average, Vecs};
use crate::{
    distribution, indexes,
    internal::{BinaryDateLast, ComputedDateLast, DifferenceF32, Ratio32, RsiFormula},
    transactions,
};

const VERSION: Version = Version::ZERO;

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
        let v = version + VERSION;

        // NVT = Market Cap (KISS DateIndex) / Volume (Height)
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
                // KISS: market_cap is ComputedVecsDateLast, volume is ComputedBlockSum
                BinaryDateLast::from_dateindex_last_and_height_sum::<Ratio32>(
                    "nvt", v, market_cap, volume,
                )
            });

        let dateindex_to_rsi_gains = EagerVec::forced_import(db, "rsi_gains", v)?;
        let dateindex_to_rsi_losses = EagerVec::forced_import(db, "rsi_losses", v)?;
        // v1: Changed from SMA to RMA (Wilder's smoothing)
        let dateindex_to_rsi_average_gain_14d =
            EagerVec::forced_import(db, "rsi_average_gain_14d", v + Version::ONE)?;
        let dateindex_to_rsi_average_loss_14d =
            EagerVec::forced_import(db, "rsi_average_loss_14d", v + Version::ONE)?;
        let dateindex_to_rsi_14d = LazyVecFrom2::transformed::<RsiFormula>(
            "rsi_14d",
            v,
            dateindex_to_rsi_average_gain_14d.boxed_clone(),
            dateindex_to_rsi_average_loss_14d.boxed_clone(),
        );

        let dateindex_to_macd_line = EagerVec::forced_import(db, "macd_line", v)?;
        let dateindex_to_macd_signal = EagerVec::forced_import(db, "macd_signal", v)?;
        let dateindex_to_macd_histogram = LazyVecFrom2::transformed::<DifferenceF32>(
            "macd_histogram",
            v,
            dateindex_to_macd_line.boxed_clone(),
            dateindex_to_macd_signal.boxed_clone(),
        );

        let dateindex_to_rsi_14d_min = EagerVec::forced_import(db, "rsi_14d_min", v)?;
        let dateindex_to_rsi_14d_max = EagerVec::forced_import(db, "rsi_14d_max", v)?;
        let dateindex_to_stoch_rsi = EagerVec::forced_import(db, "stoch_rsi", v)?;
        let dateindex_to_stoch_rsi_k = EagerVec::forced_import(db, "stoch_rsi_k", v)?;
        let dateindex_to_stoch_rsi_d = EagerVec::forced_import(db, "stoch_rsi_d", v)?;

        let dateindex_to_stoch_k = EagerVec::forced_import(db, "stoch_k", v)?;
        let dateindex_to_stoch_d = EagerVec::forced_import(db, "stoch_d", v)?;

        let dateindex_to_gini = EagerVec::forced_import(db, "gini", v)?;

        // Pi Cycle Top: 111d SMA / (2 * 350d SMA) - signals top when > 1
        let dateindex_to_pi_cycle =
            moving_average
                .indexes_to_price_111d_sma
                .price
                .as_ref()
                .map(|sma_111| {
                    LazyVecFrom2::transformed::<Ratio32>(
                        "pi_cycle",
                        v,
                        sma_111.dateindex.boxed_clone(),
                        moving_average
                            .indexes_to_price_350d_sma_x2
                            .dateindex
                            .boxed_clone(),
                    )
                });

        Ok(Self {
            indexes_to_puell_multiple: compute_dollars
                .then(|| ComputedDateLast::forced_import(db, "puell_multiple", v, indexes))
                .transpose()?,
            indexes_to_nvt,
            dateindex_to_rsi_gains,
            dateindex_to_rsi_losses,
            dateindex_to_rsi_average_gain_14d,
            dateindex_to_rsi_average_loss_14d,
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
