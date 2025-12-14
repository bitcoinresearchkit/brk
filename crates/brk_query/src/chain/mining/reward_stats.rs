use brk_error::Result;
use brk_types::{Height, RewardStats, Sats};
use vecdb::{IterableVec, VecIndex};

use crate::Query;

pub fn get_reward_stats(block_count: usize, query: &Query) -> Result<RewardStats> {
    let computer = query.computer();
    let current_height = query.get_height();

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
    let mut fee_iter = computer.chain.indexes_to_fee.sats.height.unwrap_sum().iter();
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
            total_reward += Sats::from(u64::from(*coinbase));
        }

        if let Some(fee) = fee_iter.get(h) {
            total_fee += Sats::from(u64::from(*fee));
        }

        if let Some(tx_count) = tx_count_iter.get(h) {
            total_tx += u64::from(*tx_count);
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
