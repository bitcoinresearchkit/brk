use brk_error::Result;
use brk_indexer::Indexer;
use brk_types::Dollars;
use vecdb::Exit;

use super::{
    super::{moving_average, returns},
    Vecs, macd, rsi,
};
use crate::{
    blocks,
    internal::{RatioDollarsBp32, WindowsTo1m},
    price,
};

impl Vecs {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn compute(
        &mut self,
        indexer: &Indexer,
        returns: &returns::Vecs,
        prices: &price::Vecs,
        blocks: &blocks::Vecs,
        moving_average: &moving_average::Vecs,
        exit: &Exit,
    ) -> Result<()> {
        let starting_height = indexer.safe_lengths().height;
        let daily_returns = &returns.periods._24h.ratio.height;
        for (rsi_chain, &m) in self
            .rsi
            .as_mut_array()
            .into_iter()
            .zip(&WindowsTo1m::<()>::DAYS)
        {
            rsi::compute(
                rsi_chain,
                indexer,
                blocks,
                daily_returns,
                14 * m,
                3 * m,
                exit,
            )?;
        }

        for (macd_chain, &m) in self
            .macd
            .as_mut_array()
            .into_iter()
            .zip(&WindowsTo1m::<()>::DAYS)
        {
            macd::compute(
                macd_chain,
                indexer,
                blocks,
                prices,
                12 * m,
                26 * m,
                9 * m,
                exit,
            )?;
        }

        self.pi_cycle
            .bps
            .compute_binary::<Dollars, Dollars, RatioDollarsBp32>(
                starting_height,
                &moving_average.sma._111d.usd.height,
                &moving_average.sma._350d_x2.usd.height,
                exit,
            )?;

        Ok(())
    }
}
