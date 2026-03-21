use brk_error::Result;
use brk_types::{Indexes, Sats};
use vecdb::{Exit, VecIndex};

use super::Vecs;
use crate::{mining, prices, scripts};

impl Vecs {
    pub(crate) fn compute(
        &mut self,
        scripts: &scripts::Vecs,
        mining: &mining::Vecs,
        prices: &prices::Vecs,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        self.total.compute_with(
            starting_indexes.height,
            prices,
            exit,
            |sats| {
                Ok(sats.compute_transform2(
                    starting_indexes.height,
                    &scripts.value.op_return.block.sats,
                    &mining.rewards.unclaimed.block.sats,
                    |(h, op_return, unclaimed, ..)| {
                        let genesis = if h.to_usize() == 0 { Sats::FIFTY_BTC } else { Sats::ZERO };
                        (h, genesis + op_return + unclaimed)
                    },
                    exit,
                )?)
            },
        )?;

        Ok(())
    }
}
