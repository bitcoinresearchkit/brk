use brk_error::Result;
use brk_types::Version;
use vecdb::Database;

use super::{MacdChain, RsiChain, Vecs};
use crate::{
    indexes,
    internal::{
        ComputedFromHeight, ComputedFromHeightRatio,
        PercentFromHeight, Windows,
    },
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
                ComputedFromHeight::forced_import(
                    db,
                    &format!("rsi_{}_{}", $name, tf),
                    version,
                    indexes,
                )?
            };
        }

        macro_rules! percent_import {
            ($name:expr) => {
                PercentFromHeight::forced_import_bp16(
                    db,
                    &format!("rsi_{}_{}", $name, tf),
                    version,
                    indexes,
                )?
            };
        }

        let average_gain = import!("average_gain");
        let average_loss = import!("average_loss");

        let rsi = PercentFromHeight::forced_import_bp16(
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
            rsi_min: percent_import!("min"),
            rsi_max: percent_import!("max"),
            stoch_rsi: percent_import!("stoch"),
            stoch_rsi_k: percent_import!("stoch_k"),
            stoch_rsi_d: percent_import!("stoch_d"),
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
        let line = ComputedFromHeight::forced_import(
            db,
            &format!("macd_line_{tf}"),
            version,
            indexes,
        )?;
        let signal = ComputedFromHeight::forced_import(
            db,
            &format!("macd_signal_{tf}"),
            version,
            indexes,
        )?;

        let histogram = ComputedFromHeight::forced_import(
            db,
            &format!("macd_histogram_{tf}"),
            version,
            indexes,
        )?;

        Ok(Self {
            ema_fast: ComputedFromHeight::forced_import(
                db,
                &format!("macd_ema_fast_{tf}"),
                version,
                indexes,
            )?,
            ema_slow: ComputedFromHeight::forced_import(
                db,
                &format!("macd_ema_slow_{tf}"),
                version,
                indexes,
            )?,
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

        let nvt = ComputedFromHeightRatio::forced_import_raw(db, "nvt", v, indexes)?;

        let rsi = Windows::try_from_fn(|tf| RsiChain::forced_import(db, tf, v, indexes))?;
        let macd = Windows::try_from_fn(|tf| MacdChain::forced_import(db, tf, v, indexes))?;

        let stoch_k = PercentFromHeight::forced_import_bp16(db, "stoch_k", v, indexes)?;
        let stoch_d = PercentFromHeight::forced_import_bp16(db, "stoch_d", v, indexes)?;
        let gini = PercentFromHeight::forced_import_bp16(db, "gini", v, indexes)?;

        let pi_cycle = ComputedFromHeightRatio::forced_import_raw(db, "pi_cycle", v, indexes)?;

        Ok(Self {
            puell_multiple: ComputedFromHeightRatio::forced_import_raw(db, "puell_multiple", v, indexes)?,
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
