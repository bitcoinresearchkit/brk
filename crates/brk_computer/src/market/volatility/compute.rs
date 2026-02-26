use brk_error::Result;
use brk_types::{Height, StoredF32};
use vecdb::Exit;

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
        self.sharpe_1w.height.compute_transform2(
            starting_indexes_height,
            &returns.price_returns._1w.height,
            &self.price_1w_volatility.height,
            |(h, ret, vol, ..)| {
                let ratio = if *vol == 0.0 { 0.0 } else { *ret / *vol };
                (h, StoredF32::from(ratio))
            },
            exit,
        )?;

        self.sharpe_1m.height.compute_transform2(
            starting_indexes_height,
            &returns.price_returns._1m.height,
            &self.price_1m_volatility.height,
            |(h, ret, vol, ..)| {
                let ratio = if *vol == 0.0 { 0.0 } else { *ret / *vol };
                (h, StoredF32::from(ratio))
            },
            exit,
        )?;

        self.sharpe_1y.height.compute_transform2(
            starting_indexes_height,
            &returns.price_returns._1y.height,
            &self.price_1y_volatility.height,
            |(h, ret, vol, ..)| {
                let ratio = if *vol == 0.0 { 0.0 } else { *ret / *vol };
                (h, StoredF32::from(ratio))
            },
            exit,
        )?;

        // Sortino ratios: returns / downside volatility
        self.sortino_1w.height.compute_transform2(
            starting_indexes_height,
            &returns.price_returns._1w.height,
            &returns.downside_1w_sd.sd.height,
            |(h, ret, vol, ..)| {
                let ratio = if *vol == 0.0 { 0.0 } else { *ret / *vol };
                (h, StoredF32::from(ratio))
            },
            exit,
        )?;

        self.sortino_1m.height.compute_transform2(
            starting_indexes_height,
            &returns.price_returns._1m.height,
            &returns.downside_1m_sd.sd.height,
            |(h, ret, vol, ..)| {
                let ratio = if *vol == 0.0 { 0.0 } else { *ret / *vol };
                (h, StoredF32::from(ratio))
            },
            exit,
        )?;

        self.sortino_1y.height.compute_transform2(
            starting_indexes_height,
            &returns.price_returns._1y.height,
            &returns.downside_1y_sd.sd.height,
            |(h, ret, vol, ..)| {
                let ratio = if *vol == 0.0 { 0.0 } else { *ret / *vol };
                (h, StoredF32::from(ratio))
            },
            exit,
        )?;

        Ok(())
    }
}
