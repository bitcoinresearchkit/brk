use brk_error::Result;
use brk_types::Version;
use vecdb::Database;

use super::{ByIndicatorTimeframe, MacdChain, RsiChain, Vecs};
use crate::{
    indexes,
    internal::ComputedFromHeightLast,
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

        let rsi = ComputedFromHeightLast::forced_import(
            db,
            &format!("rsi_{tf}"),
            version,
            indexes,
        )?;

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

        let histogram = ComputedFromHeightLast::forced_import(
            db,
            &format!("macd_histogram_{tf}"),
            version,
            indexes,
        )?;

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
    ) -> Result<Self> {
        let v = version + VERSION;

        let nvt = ComputedFromHeightLast::forced_import(db, "nvt", v, indexes)?;

        let rsi = ByIndicatorTimeframe::try_new(|tf| RsiChain::forced_import(db, tf, v, indexes))?;
        let macd = ByIndicatorTimeframe::try_new(|tf| MacdChain::forced_import(db, tf, v, indexes))?;

        let stoch_k = ComputedFromHeightLast::forced_import(db, "stoch_k", v, indexes)?;
        let stoch_d = ComputedFromHeightLast::forced_import(db, "stoch_d", v, indexes)?;
        let gini = ComputedFromHeightLast::forced_import(db, "gini", v, indexes)?;

        let pi_cycle = ComputedFromHeightLast::forced_import(db, "pi_cycle", v, indexes)?;

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
