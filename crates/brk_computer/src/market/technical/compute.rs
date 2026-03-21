use brk_error::Result;
use brk_types::{Dollars, Indexes};
use vecdb::Exit;

use super::{
    super::{moving_average, returns},
    Vecs, macd, rsi,
};
use crate::{blocks, internal::{RatioDollarsBp32, WindowsTo1m}, prices};

impl Vecs {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn compute(
        &mut self,
        returns: &returns::Vecs,
        prices: &prices::Vecs,
        blocks: &blocks::Vecs,
        moving_average: &moving_average::Vecs,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        let daily_returns = &returns.periods._24h.ratio.height;
        for (rsi_chain, &m) in self.rsi.as_mut_array().into_iter().zip(&WindowsTo1m::<()>::DAYS) {
            rsi::compute(
                rsi_chain,
                blocks,
                daily_returns,
                14 * m,
                3 * m,
                starting_indexes,
                exit,
            )?;
        }

        for (macd_chain, &m) in self.macd.as_mut_array().into_iter().zip(&WindowsTo1m::<()>::DAYS) {
            macd::compute(
                macd_chain,
                blocks,
                prices,
                12 * m,
                26 * m,
                9 * m,
                starting_indexes,
                exit,
            )?;
        }

        self.pi_cycle
            .bps
            .compute_binary::<Dollars, Dollars, RatioDollarsBp32>(
                starting_indexes.height,
                &moving_average.sma._111d.usd.height,
                &moving_average.sma._350d_x2.usd.height,
                exit,
            )?;

        Ok(())
    }
}
