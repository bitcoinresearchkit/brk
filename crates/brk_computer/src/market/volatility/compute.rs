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
            compute_divided(
                &mut out.height,
                starting_indexes_height,
                ret,
                vol,
                1.0,
                exit,
            )?;
        }

        // Sortino ratios: returns / downside volatility (sd * sqrt(days))
        for (out, ret, sd, sqrt_days) in [
            (
                &mut self.price_sortino_1w,
                &returns.price_return._1w.ratio.height,
                &returns.price_downside_24h_sd_1w.sd.height,
                7.0_f32.sqrt(),
            ),
            (
                &mut self.price_sortino_1m,
                &returns.price_return._1m.ratio.height,
                &returns.price_downside_24h_sd_1m.sd.height,
                30.0_f32.sqrt(),
            ),
            (
                &mut self.price_sortino_1y,
                &returns.price_return._1y.ratio.height,
                &returns.price_downside_24h_sd_1y.sd.height,
                365.0_f32.sqrt(),
            ),
        ] {
            compute_divided(
                &mut out.height,
                starting_indexes_height,
                ret,
                sd,
                sqrt_days,
                exit,
            )?;
        }

        Ok(())
    }
}

fn compute_divided(
    out: &mut EagerVec<PcoVec<Height, StoredF32>>,
    starting_indexes_height: Height,
    ret: &impl ReadableVec<Height, StoredF32>,
    divisor: &impl ReadableVec<Height, StoredF32>,
    divisor_scale: f32,
    exit: &Exit,
) -> Result<()> {
    out.compute_transform2(
        starting_indexes_height,
        ret,
        divisor,
        |(h, ret, div, ..)| {
            let denom = (*div) * divisor_scale;
            let ratio = if denom == 0.0 { 0.0 } else { (*ret) / denom };
            (h, StoredF32::from(ratio))
        },
        exit,
    )?;
    Ok(())
}
