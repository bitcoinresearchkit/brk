use brk_error::{Error, Result};
use brk_types::{
    Height, PoolBlockCounts, PoolBlockShares, PoolDetail, PoolDetailInfo, PoolInfo, PoolSlug,
    PoolStats, PoolsSummary, TimePeriod, pools,
};
use vecdb::{AnyVec, IterableVec, VecIndex};

use crate::Query;

impl Query {
    pub fn mining_pools(&self, time_period: TimePeriod) -> Result<PoolsSummary> {
        let computer = self.computer();
        let current_height = self.height();
        let end = current_height.to_usize();

        // No blocks indexed yet
        if computer.pools.height_to_pool.len() == 0 {
            return Ok(PoolsSummary {
                pools: vec![],
                block_count: 0,
                last_estimated_hashrate: 0,
            });
        }

        // Calculate start height based on time period
        let start = end.saturating_sub(time_period.block_count());

        let pools = pools();
        let mut pool_data: Vec<(&'static brk_types::Pool, u32)> = Vec::new();

        // For each pool, get cumulative count at end and start, subtract to get range count
        for (pool_id, pool_vecs) in &computer.pools.vecs {
            let mut cumulative = pool_vecs
                .indexes_to_blocks_mined
                .height_extra
                .unwrap_cumulative()
                .iter();

            let count_at_end: u32 = *cumulative.get(current_height).unwrap_or_default();

            let count_at_start: u32 = if start == 0 {
                0
            } else {
                *cumulative.get(Height::from(start - 1)).unwrap_or_default()
            };

            let block_count = count_at_end.saturating_sub(count_at_start);

            // Only include pools that mined at least one block in the period
            if block_count > 0 {
                pool_data.push((pools.get(*pool_id), block_count));
            }
        }

        // Sort by block count descending
        pool_data.sort_by(|a, b| b.1.cmp(&a.1));

        let total_blocks: u32 = pool_data.iter().map(|(_, count)| count).sum();

        // Build stats with ranks
        let pool_stats: Vec<PoolStats> = pool_data
            .into_iter()
            .enumerate()
            .map(|(idx, (pool, block_count))| {
                let share = if total_blocks > 0 {
                    block_count as f64 / total_blocks as f64
                } else {
                    0.0
                };
                PoolStats::new(pool, block_count, (idx + 1) as u32, share)
            })
            .collect();

        // TODO: Calculate actual hashrate from difficulty
        let last_estimated_hashrate = 0u128;

        Ok(PoolsSummary {
            pools: pool_stats,
            block_count: total_blocks,
            last_estimated_hashrate,
        })
    }

    pub fn all_pools(&self) -> Vec<PoolInfo> {
        pools().iter().map(PoolInfo::from).collect()
    }

    pub fn pool_detail(&self, slug: PoolSlug) -> Result<PoolDetail> {
        let computer = self.computer();
        let current_height = self.height();
        let end = current_height.to_usize();

        let pools_list = pools();
        let pool = pools_list.get(slug);

        // Get pool vecs for this specific pool
        let pool_vecs = computer
            .pools
            .vecs
            .get(&slug)
            .ok_or_else(|| Error::Str("Pool data not found"))?;

        let mut cumulative = pool_vecs
            .indexes_to_blocks_mined
            .height_extra
            .unwrap_cumulative()
            .iter();

        // Get total blocks (all time)
        let total_all: u32 = *cumulative.get(current_height).unwrap_or_default();

        // Get blocks for 24h (144 blocks)
        let start_24h = end.saturating_sub(144);
        let count_before_24h: u32 = if start_24h == 0 {
            0
        } else {
            *cumulative
                .get(Height::from(start_24h - 1))
                .unwrap_or_default()
        };
        let total_24h = total_all.saturating_sub(count_before_24h);

        // Get blocks for 1w (1008 blocks)
        let start_1w = end.saturating_sub(1008);
        let count_before_1w: u32 = if start_1w == 0 {
            0
        } else {
            *cumulative
                .get(Height::from(start_1w - 1))
                .unwrap_or_default()
        };
        let total_1w = total_all.saturating_sub(count_before_1w);

        // Calculate total network blocks for share calculation
        let network_blocks_all = (end + 1) as u32;
        let network_blocks_24h = (end - start_24h + 1) as u32;
        let network_blocks_1w = (end - start_1w + 1) as u32;

        let share_all = if network_blocks_all > 0 {
            total_all as f64 / network_blocks_all as f64
        } else {
            0.0
        };
        let share_24h = if network_blocks_24h > 0 {
            total_24h as f64 / network_blocks_24h as f64
        } else {
            0.0
        };
        let share_1w = if network_blocks_1w > 0 {
            total_1w as f64 / network_blocks_1w as f64
        } else {
            0.0
        };

        Ok(PoolDetail {
            pool: PoolDetailInfo::from(pool),
            block_count: PoolBlockCounts {
                all: total_all,
                day: total_24h,
                week: total_1w,
            },
            block_share: PoolBlockShares {
                all: share_all,
                day: share_24h,
                week: share_1w,
            },
            estimated_hashrate: 0, // TODO: Calculate from share and network hashrate
            reported_hashrate: None,
        })
    }
}
