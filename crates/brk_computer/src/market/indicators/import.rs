use brk_error::Result;
use brk_types::Version;
use vecdb::{Database, EagerVec, ImportableVec, IterableCloneableVec, LazyVecFrom2};

use super::{super::moving_average, Vecs};
use crate::{
    distribution, indexes,
    internal::{ComputedDateLast, DifferenceF32, LazyBinaryDateLast, Ratio32, RsiFormula},
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
        let nvt = distribution
            .utxo_cohorts
            .all
            .metrics
            .supply
            .total
            .dollars
            .as_ref()
            .zip(transactions.volume.sent_sum.dollars.as_ref())
            .map(|(market_cap, volume)| {
                LazyBinaryDateLast::from_block_last_and_height_sum::<Ratio32>(
                    "nvt", v, market_cap, volume,
                )
            });

        let rsi_gains = EagerVec::forced_import(db, "rsi_gains", v)?;
        let rsi_losses = EagerVec::forced_import(db, "rsi_losses", v)?;
        // v1: Changed from SMA to RMA (Wilder's smoothing)
        let rsi_average_gain_14d =
            EagerVec::forced_import(db, "rsi_average_gain_14d", v + Version::ONE)?;
        let rsi_average_loss_14d =
            EagerVec::forced_import(db, "rsi_average_loss_14d", v + Version::ONE)?;
        let rsi_14d = LazyVecFrom2::transformed::<RsiFormula>(
            "rsi_14d",
            v,
            rsi_average_gain_14d.boxed_clone(),
            rsi_average_loss_14d.boxed_clone(),
        );

        let macd_line = EagerVec::forced_import(db, "macd_line", v)?;
        let macd_signal = EagerVec::forced_import(db, "macd_signal", v)?;
        let macd_histogram = LazyVecFrom2::transformed::<DifferenceF32>(
            "macd_histogram",
            v,
            macd_line.boxed_clone(),
            macd_signal.boxed_clone(),
        );

        let rsi_14d_min = EagerVec::forced_import(db, "rsi_14d_min", v)?;
        let rsi_14d_max = EagerVec::forced_import(db, "rsi_14d_max", v)?;
        let stoch_rsi = EagerVec::forced_import(db, "stoch_rsi", v)?;
        let stoch_rsi_k = EagerVec::forced_import(db, "stoch_rsi_k", v)?;
        let stoch_rsi_d = EagerVec::forced_import(db, "stoch_rsi_d", v)?;

        let stoch_k = EagerVec::forced_import(db, "stoch_k", v)?;
        let stoch_d = EagerVec::forced_import(db, "stoch_d", v)?;

        let gini = EagerVec::forced_import(db, "gini", v)?;

        // Pi Cycle Top: 111d SMA / (2 * 350d SMA) - signals top when > 1
        let pi_cycle = moving_average.price_111d_sma.price.as_ref().map(|sma_111| {
            LazyVecFrom2::transformed::<Ratio32>(
                "pi_cycle",
                v,
                sma_111.dateindex.boxed_clone(),
                moving_average.price_350d_sma_x2.dateindex.boxed_clone(),
            )
        });

        Ok(Self {
            puell_multiple: compute_dollars
                .then(|| ComputedDateLast::forced_import(db, "puell_multiple", v, indexes))
                .transpose()?,
            nvt,
            rsi_gains,
            rsi_losses,
            rsi_average_gain_14d,
            rsi_average_loss_14d,
            rsi_14d,
            rsi_14d_min,
            rsi_14d_max,
            stoch_rsi,
            stoch_rsi_k,
            stoch_rsi_d,
            stoch_k,
            stoch_d,
            pi_cycle,
            macd_line,
            macd_signal,
            macd_histogram,
            gini,
        })
    }
}
