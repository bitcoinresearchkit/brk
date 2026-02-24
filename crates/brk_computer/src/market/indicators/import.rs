use brk_error::Result;
use brk_types::Version;
use vecdb::{Database, ReadableCloneableVec, LazyVecFrom2};

use super::{ByIndicatorTimeframe, MacdChain, RsiChain, Vecs};
use crate::{
    distribution, indexes,
    internal::{ComputedFromHeightLast, DifferenceF32, LazyBinaryFromHeightLast, Ratio32, RsiFormula},
    transactions,
};

const VERSION: Version = Version::ONE;

impl RsiChain {
    fn forced_import(
        db: &Database,
        tf: &str,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        macro_rules! import {
            ($name:expr) => {
                ComputedFromHeightLast::forced_import(
                    db,
                    &format!("rsi_{}_{}", $name, tf),
                    version,
                    indexes,
                )?
            };
        }

        let average_gain = import!("avg_gain");
        let average_loss = import!("avg_loss");

        let rsi = LazyVecFrom2::transformed::<RsiFormula>(
            &format!("rsi_{tf}"),
            version,
            average_gain.height.read_only_boxed_clone(),
            average_loss.height.read_only_boxed_clone(),
        );

        Ok(Self {
            gains: import!("gains"),
            losses: import!("losses"),
            average_gain,
            average_loss,
            rsi,
            rsi_min: import!("rsi_min"),
            rsi_max: import!("rsi_max"),
            stoch_rsi: import!("stoch_rsi"),
            stoch_rsi_k: import!("stoch_rsi_k"),
            stoch_rsi_d: import!("stoch_rsi_d"),
        })
    }
}

impl MacdChain {
    fn forced_import(
        db: &Database,
        tf: &str,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let line = ComputedFromHeightLast::forced_import(
            db,
            &format!("macd_line_{tf}"),
            version,
            indexes,
        )?;
        let signal = ComputedFromHeightLast::forced_import(
            db,
            &format!("macd_signal_{tf}"),
            version,
            indexes,
        )?;

        let histogram = LazyVecFrom2::transformed::<DifferenceF32>(
            &format!("macd_histogram_{tf}"),
            version,
            line.height.read_only_boxed_clone(),
            signal.height.read_only_boxed_clone(),
        );

        Ok(Self {
            line,
            signal,
            histogram,
        })
    }
}

impl Vecs {
    pub(crate) fn forced_import(
        db: &Database,
        version: Version,
        indexes: &indexes::Vecs,
        distribution: &distribution::Vecs,
        transactions: &transactions::Vecs,
        moving_average: &super::super::moving_average::Vecs,
    ) -> Result<Self> {
        let v = version + VERSION;

        let nvt = LazyBinaryFromHeightLast::from_both_lazy_binary_computed_block_last::<
            Ratio32,
            _,
            _,
            _,
            _,
        >(
            "nvt",
            v,
            &distribution.utxo_cohorts.all.metrics.supply.total.usd,
            &transactions.volume.sent_sum.usd,
        );

        let rsi = ByIndicatorTimeframe::try_new(|tf| RsiChain::forced_import(db, tf, v, indexes))?;
        let macd = ByIndicatorTimeframe::try_new(|tf| MacdChain::forced_import(db, tf, v, indexes))?;

        let stoch_k = ComputedFromHeightLast::forced_import(db, "stoch_k", v, indexes)?;
        let stoch_d = ComputedFromHeightLast::forced_import(db, "stoch_d", v, indexes)?;
        let gini = ComputedFromHeightLast::forced_import(db, "gini", v, indexes)?;

        let pi_cycle = LazyBinaryFromHeightLast::from_block_last_and_lazy_block_last::<Ratio32, _>(
            "pi_cycle",
            v,
            &moving_average.price_111d_sma.price.as_ref().unwrap().usd,
            &moving_average.price_350d_sma_x2.usd,
        );

        Ok(Self {
            puell_multiple: ComputedFromHeightLast::forced_import(db, "puell_multiple", v, indexes)?,
            nvt,
            rsi,
            stoch_k,
            stoch_d,
            pi_cycle,
            macd,
            gini,
        })
    }
}
