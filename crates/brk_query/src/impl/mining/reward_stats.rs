use brk_error::Result;
use brk_types::{Height, RewardStats, Sats};
use vecdb::{IterableVec, VecIndex};

use crate::Query;

impl Query {
    pub fn reward_stats(&self, block_count: usize) -> Result<RewardStats> {
        let computer = self.computer();
        let current_height = self.height();

        let end_block = current_height;
        let start_block = Height::from(current_height.to_usize().saturating_sub(block_count - 1));

        let mut coinbase_iter = computer
            .chain
            .indexes_to_coinbase
            .sats
            .height
            .as_ref()
            .unwrap()
            .iter();
        let mut fee_iter = computer
            .chain
            .indexes_to_fee
            .sats
            .height
            .unwrap_sum()
            .iter();
        let mut tx_count_iter = computer
            .chain
            .indexes_to_tx_count
            .height
            .as_ref()
            .unwrap()
            .iter();

        let mut total_reward = Sats::ZERO;
        let mut total_fee = Sats::ZERO;
        let mut total_tx: u64 = 0;

        for height in start_block.to_usize()..=end_block.to_usize() {
            let h = Height::from(height);

            if let Some(coinbase) = coinbase_iter.get(h) {
                total_reward += coinbase;
            }

            if let Some(fee) = fee_iter.get(h) {
                total_fee += fee;
            }

            if let Some(tx_count) = tx_count_iter.get(h) {
                total_tx += *tx_count;
            }
        }

        Ok(RewardStats {
            start_block,
            end_block,
            total_reward,
            total_fee,
            total_tx,
        })
    }
}
