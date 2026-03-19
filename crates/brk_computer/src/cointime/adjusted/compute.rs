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
            &activity.ratio.height,
            &supply.inflation_rate.bps.height,
            |(h, a2vr, inflation, ..)| {
                (
                    h,
                    BasisPointsSigned32::from(f64::from(a2vr) * f64::from(inflation)),
                )
            },
            exit,
        )?;

        self.tx_velocity_native.height.compute_multiply(
            starting_indexes.height,
            &activity.ratio.height,
            &supply.velocity.native.height,
            exit,
        )?;

        self.tx_velocity_fiat.height.compute_multiply(
            starting_indexes.height,
            &activity.ratio.height,
            &supply.velocity.fiat.height,
            exit,
        )?;

        Ok(())
    }
}
