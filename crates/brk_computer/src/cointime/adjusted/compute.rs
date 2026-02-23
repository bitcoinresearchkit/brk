use brk_error::Result;
use brk_types::{StoredF32, StoredF64};
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
        self.cointime_adj_inflation_rate.height.compute_transform2(
            starting_indexes.height,
            &activity.activity_to_vaultedness_ratio.height,
            &supply.inflation.height,
            |(h, ratio, inflation, ..)| (h, StoredF32::from((*ratio) * f64::from(*inflation))),
            exit,
        )?;

        self.cointime_adj_tx_btc_velocity
            .height
            .compute_transform2(
                starting_indexes.height,
                &activity.activity_to_vaultedness_ratio.height,
                &supply.velocity.btc.height,
                |(h, ratio, vel, ..)| (h, StoredF64::from(*ratio * *vel)),
                exit,
            )?;

        self.cointime_adj_tx_usd_velocity
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
