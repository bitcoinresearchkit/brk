use brk_error::Result;
use brk_types::{Height, StoredF32};
use vecdb::{EagerVec, Exit, PcoVec, ReadableVec};

use super::super::returns;
use super::Vecs;

impl Vecs {
    pub(crate) fn compute(
        &mut self,
        returns: &returns::Vecs,
        starting_indexes_height: Height,
        exit: &Exit,
    ) -> Result<()> {
        // Sharpe ratios: returns / volatility
        for (out, ret, vol) in [
            (&mut self.sharpe_1w, &returns.price_returns._1w.height, &self.price_1w_volatility.height),
            (&mut self.sharpe_1m, &returns.price_returns._1m.height, &self.price_1m_volatility.height),
            (&mut self.sharpe_1y, &returns.price_returns._1y.height, &self.price_1y_volatility.height),
        ] {
            compute_ratio(&mut out.height, starting_indexes_height, ret, vol, exit)?;
        }

        // Sortino ratios: returns / downside volatility
        compute_ratio(&mut self.sortino_1w.height, starting_indexes_height, &returns.price_returns._1w.height, &returns.downside_1w_sd.sd.height, exit)?;
        compute_ratio(&mut self.sortino_1m.height, starting_indexes_height, &returns.price_returns._1m.height, &returns.downside_1m_sd.sd.height, exit)?;
        compute_ratio(&mut self.sortino_1y.height, starting_indexes_height, &returns.price_returns._1y.height, &returns.downside_1y_sd.sd.height, exit)?;

        Ok(())
    }
}

fn compute_ratio(
    out: &mut EagerVec<PcoVec<Height, StoredF32>>,
    starting_indexes_height: Height,
    ret: &impl ReadableVec<Height, StoredF32>,
    vol: &impl ReadableVec<Height, StoredF32>,
    exit: &Exit,
) -> Result<()> {
    out.compute_transform2(
        starting_indexes_height,
        ret,
        vol,
        |(h, ret, vol, ..)| {
            let ratio = if *vol == 0.0 { 0.0 } else { *ret / *vol };
            (h, StoredF32::from(ratio))
        },
        exit,
    )?;
    Ok(())
}
