use brk_error::Result;
use brk_types::{BasisPointsSigned32, Indexes};
use vecdb::Exit;

use super::super::activity;
use super::Vecs;
use crate::supply;

impl Vecs {
    pub(crate) fn compute(
        &mut self,
        starting_indexes: &Indexes,
        supply: &supply::Vecs,
        activity: &activity::Vecs,
        exit: &Exit,
    ) -> Result<()> {
        self.inflation_rate.bps.height.compute_transform2(
            starting_indexes.height,
            &activity.liveliness.height,
            &supply.inflation_rate.bps.height,
            |(h, liveliness, inflation, ..)| {
                (
                    h,
                    BasisPointsSigned32::from(f64::from(liveliness) * f64::from(inflation)),
                )
            },
            exit,
        )?;

        self.tx_velocity_btc.height.compute_multiply(
            starting_indexes.height,
            &activity.activity_to_vaultedness_ratio.height,
            &supply.velocity.btc.height,
            exit,
        )?;

        self.tx_velocity_usd.height.compute_multiply(
            starting_indexes.height,
            &activity.activity_to_vaultedness_ratio.height,
            &supply.velocity.usd.height,
            exit,
        )?;

        Ok(())
    }
}
