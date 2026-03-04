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
            (
                &mut self.price_sharpe_1w,
                &returns.price_return._1w.ratio.height,
                &self.price_volatility_1w.height,
            ),
            (
                &mut self.price_sharpe_1m,
                &returns.price_return._1m.ratio.height,
                &self.price_volatility_1m.height,
            ),
            (
                &mut self.price_sharpe_1y,
                &returns.price_return._1y.ratio.height,
                &self.price_volatility_1y.height,
            ),
        ] {
            compute_ratio(&mut out.height, starting_indexes_height, ret, vol, exit)?;
        }

        // Sortino ratios: returns / downside volatility (sd * sqrt(days))
        compute_sortino(
            &mut self.price_sortino_1w.height,
            starting_indexes_height,
            &returns.price_return._1w.ratio.height,
            &returns.price_downside_24h_sd_1w.sd.height,
            7.0_f32.sqrt(),
            exit,
        )?;
        compute_sortino(
            &mut self.price_sortino_1m.height,
            starting_indexes_height,
            &returns.price_return._1m.ratio.height,
            &returns.price_downside_24h_sd_1m.sd.height,
            30.0_f32.sqrt(),
            exit,
        )?;
        compute_sortino(
            &mut self.price_sortino_1y.height,
            starting_indexes_height,
            &returns.price_return._1y.ratio.height,
            &returns.price_downside_24h_sd_1y.sd.height,
            365.0_f32.sqrt(),
            exit,
        )?;

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

fn compute_sortino(
    out: &mut EagerVec<PcoVec<Height, StoredF32>>,
    starting_indexes_height: Height,
    ret: &impl ReadableVec<Height, StoredF32>,
    sd: &impl ReadableVec<Height, StoredF32>,
    sqrt_days: f32,
    exit: &Exit,
) -> Result<()> {
    out.compute_transform2(
        starting_indexes_height,
        ret,
        sd,
        |(h, ret, sd, ..)| {
            let downside_vol = (*sd) * sqrt_days;
            let ratio = if downside_vol == 0.0 {
                0.0
            } else {
                (*ret) / downside_vol
            };
            (h, StoredF32::from(ratio))
        },
        exit,
    )?;
    Ok(())
}
