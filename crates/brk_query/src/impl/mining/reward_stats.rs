use brk_error::Result;
use brk_types::{Height, RewardStats, Sats};
use vecdb::{ReadableVec, VecIndex};

use crate::Query;

impl Query {
    pub fn reward_stats(&self, block_count: usize) -> Result<RewardStats> {
        let computer = self.computer();
        let current_height = self.height();

        let end_block = current_height;
        let start_block = Height::from(current_height.to_usize().saturating_sub(block_count - 1));

        let coinbase_vec = &computer.mining.rewards.coinbase.sats.height;
        let fee_vec = &computer.transactions.fees.fee.sum_cumulative.sum.0;
        let tx_count_vec = &computer.transactions.count.tx_count.height;

        let start = start_block.to_usize();
        let end = end_block.to_usize() + 1;

        let total_reward = coinbase_vec.fold_range_at(start, end, Sats::ZERO, |acc, v| acc + v);
        let total_fee = fee_vec.fold_range_at(start, end, Sats::ZERO, |acc, v| acc + v);
        let total_tx = tx_count_vec.fold_range_at(start, end, 0u64, |acc, v| acc + *v);

        Ok(RewardStats {
            start_block,
            end_block,
            total_reward,
            total_fee,
            total_tx,
        })
    }
}
