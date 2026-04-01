use brk_error::{Error, Result};
use brk_types::{
    BlockInfoV1, Height, PoolBlockCounts, PoolBlockShares, PoolDetail, PoolDetailInfo,
    PoolHashrateEntry, PoolInfo, PoolSlug, PoolStats, PoolsSummary, TimePeriod, pools,
};
use vecdb::{AnyVec, ReadableVec, VecIndex};

use crate::Query;

impl Query {
    pub fn mining_pools(&self, time_period: TimePeriod) -> Result<PoolsSummary> {
        let computer = self.computer();
        let current_height = self.height();
        let end = current_height.to_usize();

        // No blocks indexed yet
        if computer.pools.pool.len() == 0 {
            return Ok(PoolsSummary {
                pools: vec![],
                block_count: 0,
                last_estimated_hashrate: 0,
            });
        }

        // Calculate start height based on time period
        let start = end.saturating_sub(time_period.block_count());

        let pools = pools();
        let mut pool_data: Vec<(&'static brk_types::Pool, u64)> = Vec::new();

        // For each pool, get cumulative count at end and start, subtract to get range count
        for (pool_id, cumulative) in computer
            .pools
            .major
            .iter()
            .map(|(id, v)| (id, &v.blocks_mined.cumulative.height))
            .chain(
                computer
                    .pools
                    .minor
                    .iter()
                    .map(|(id, v)| (id, &v.blocks_mined.cumulative.height)),
            )
        {
            let count_at_end: u64 = *cumulative.collect_one(current_height).unwrap_or_default();

            let count_at_start: u64 = if start == 0 {
                0
            } else {
                *cumulative
                    .collect_one(Height::from(start - 1))
                    .unwrap_or_default()
            };

            let block_count = count_at_end.saturating_sub(count_at_start);

            if block_count > 0 {
                pool_data.push((pools.get(*pool_id), block_count));
            }
        }

        // Sort by block count descending
        pool_data.sort_by(|a, b| b.1.cmp(&a.1));

        let total_blocks: u64 = pool_data.iter().map(|(_, count)| count).sum();

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

        // Get cumulative blocks for this pool (works for both major and minor)
        let cumulative = computer
            .pools
            .major
            .get(&slug)
            .map(|v| &v.blocks_mined.cumulative.height)
            .or_else(|| {
                computer
                    .pools
                    .minor
                    .get(&slug)
                    .map(|v| &v.blocks_mined.cumulative.height)
            })
            .ok_or_else(|| Error::NotFound("Pool data not found".into()))?;

        // Get total blocks (all time)
        let total_all: u64 = *cumulative.collect_one(current_height).unwrap_or_default();

        // Get blocks for 24h (144 blocks)
        let start_24h = end.saturating_sub(144);
        let count_before_24h: u64 = if start_24h == 0 {
            0
        } else {
            *cumulative
                .collect_one(Height::from(start_24h - 1))
                .unwrap_or_default()
        };
        let total_24h = total_all.saturating_sub(count_before_24h);

        // Get blocks for 1w (1008 blocks)
        let start_1w = end.saturating_sub(1008);
        let count_before_1w: u64 = if start_1w == 0 {
            0
        } else {
            *cumulative
                .collect_one(Height::from(start_1w - 1))
                .unwrap_or_default()
        };
        let total_1w = total_all.saturating_sub(count_before_1w);

        // Calculate total network blocks for share calculation
        let network_blocks_all = (end + 1) as u64;
        let network_blocks_24h = (end - start_24h + 1) as u64;
        let network_blocks_1w = (end - start_1w + 1) as u64;

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

    pub fn pool_blocks(
        &self,
        slug: PoolSlug,
        start_height: Option<Height>,
    ) -> Result<Vec<BlockInfoV1>> {
        let computer = self.computer();
        let max_height = self.height().to_usize();
        let start = start_height.map(|h| h.to_usize()).unwrap_or(max_height);

        // BytesVec reader gives O(1) mmap reads — efficient for backward scan
        let reader = computer.pools.pool.reader();
        let end = start.min(reader.len().saturating_sub(1));

        let mut heights = Vec::with_capacity(10);
        for h in (0..=end).rev() {
            if reader.get(h) == slug {
                heights.push(h);
                if heights.len() >= 10 {
                    break;
                }
            }
        }

        let mut blocks = Vec::with_capacity(heights.len());
        for h in heights {
            if let Ok(mut v) = self.blocks_v1_range(h, h + 1) {
                blocks.append(&mut v);
            }
        }
        Ok(blocks)
    }

    pub fn pool_hashrate(&self, slug: PoolSlug) -> Result<Vec<PoolHashrateEntry>> {
        let pools_list = pools();
        let pool = pools_list.get(slug);
        let entries = self.compute_pool_hashrate_entries(slug, 0)?;
        Ok(entries
            .into_iter()
            .map(|(ts, hr, share)| PoolHashrateEntry {
                timestamp: ts,
                avg_hashrate: hr,
                share,
                pool_name: pool.name.to_string(),
            })
            .collect())
    }

    pub fn pools_hashrate(
        &self,
        time_period: Option<TimePeriod>,
    ) -> Result<Vec<PoolHashrateEntry>> {
        let current_height = self.height().to_usize();
        let start = match time_period {
            Some(tp) => current_height.saturating_sub(tp.block_count()),
            None => 0,
        };
        let pools_list = pools();
        let mut entries = Vec::new();

        for pool in pools_list.iter() {
            if let Ok(pool_entries) = self.compute_pool_hashrate_entries(pool.slug, start) {
                for (ts, hr, share) in pool_entries {
                    if share > 0.0 {
                        entries.push(PoolHashrateEntry {
                            timestamp: ts,
                            avg_hashrate: hr,
                            share,
                            pool_name: pool.name.to_string(),
                        });
                    }
                }
            }
        }

        Ok(entries)
    }

    /// Compute (timestamp, hashrate, share) tuples for a pool from `start_height`.
    fn compute_pool_hashrate_entries(
        &self,
        slug: PoolSlug,
        start_height: usize,
    ) -> Result<Vec<(brk_types::Timestamp, u128, f64)>> {
        let computer = self.computer();
        let indexer = self.indexer();
        let end = self.height().to_usize() + 1;
        let start = start_height;

        let dominance_bps = computer
            .pools
            .major
            .get(&slug)
            .map(|v| &v.base.dominance.bps.height)
            .or_else(|| {
                computer
                    .pools
                    .minor
                    .get(&slug)
                    .map(|v| &v.dominance.bps.height)
            })
            .ok_or_else(|| Error::NotFound("Pool not found".into()))?;

        let total = end - start;
        let step = (total / 200).max(1);

        // Batch read everything for the range
        let timestamps = indexer.vecs.blocks.timestamp.collect_range_at(start, end);
        let bps_values = dominance_bps.collect_range_at(start, end);
        let day1_values = computer.indexes.height.day1.collect_range_at(start, end);
        let hashrate_vec = &computer.mining.hashrate.rate.base.day1;

        // Pre-read all needed hashrates by collecting unique day1 values
        let max_day = day1_values.iter().map(|d| d.to_usize()).max().unwrap_or(0);
        let min_day = day1_values.iter().map(|d| d.to_usize()).min().unwrap_or(0);
        let hashrates = hashrate_vec.collect_range_dyn(min_day, max_day + 1);

        Ok((0..total)
            .step_by(step)
            .filter_map(|i| {
                let bps = *bps_values[i];
                let share = bps as f64 / 10000.0;
                let day_idx = day1_values[i].to_usize() - min_day;
                let network_hr = f64::from(*hashrates.get(day_idx)?.as_ref()?);
                Some((timestamps[i], (network_hr * share) as u128, share))
            })
            .collect())
    }
}
