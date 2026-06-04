use brk_error::Result;
use brk_indexer::Indexer;
use brk_types::Sats;
use vecdb::{Exit, VecIndex};

use super::Vecs;
use crate::{mining, outputs, price};

impl Vecs {
    pub(crate) fn compute(
        &mut self,
        indexer: &Indexer,
        outputs: &outputs::Vecs,
        mining: &mining::Vecs,
        prices: &price::Vecs,
        exit: &Exit,
    ) -> Result<()> {
        let starting_height = indexer.safe_lengths().height;

        self.total
            .compute_with(starting_height, prices, exit, |sats| {
                Ok(sats.compute_transform2(
                    starting_height,
                    &outputs.value.op_return.block.sats,
                    &mining.rewards.unclaimed.block.sats,
                    |(h, op_return, unclaimed, ..)| {
                        let genesis = if h.to_usize() == 0 {
                            Sats::FIFTY_BTC
                        } else {
                            Sats::ZERO
                        };
                        (h, genesis + op_return + unclaimed)
                    },
                    exit,
                )?)
            })?;

        Ok(())
    }
}
