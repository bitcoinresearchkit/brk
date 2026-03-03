use brk_error::Result;
use brk_types::{BasisPointsSigned32, StoredF64};
use vecdb::Exit;

use super::super::activity;
use super::Vecs;
use crate::{ComputeIndexes, supply};

impl Vecs {
    pub(crate) fn compute(
        &mut self,
        starting_indexes: &ComputeIndexes,
        supply: &supply::Vecs,
        activity: &activity::Vecs,
        exit: &Exit,
    ) -> Result<()> {
        self.cointime_adj_inflation_rate.bps.height.compute_transform2(
            starting_indexes.height,
            &activity.activity_to_vaultedness_ratio.height,
            &supply.inflation_rate.bps.height,
            |(h, ratio, inflation, ..)| (h, BasisPointsSigned32::from((*ratio) * f64::from(inflation))),
            exit,
        )?;

        self.cointime_adj_tx_velocity_btc
            .height
            .compute_transform2(
                starting_indexes.height,
                &activity.activity_to_vaultedness_ratio.height,
                &supply.velocity.btc.height,
                |(h, ratio, vel, ..)| (h, StoredF64::from(*ratio * *vel)),
                exit,
            )?;

        self.cointime_adj_tx_velocity_usd
            .height
            .compute_transform2(
                starting_indexes.height,
                &activity.activity_to_vaultedness_ratio.height,
                &supply.velocity.usd.height,
                |(h, ratio, vel, ..)| (h, StoredF64::from(*ratio * *vel)),
                exit,
            )?;

        Ok(())
    }
}
