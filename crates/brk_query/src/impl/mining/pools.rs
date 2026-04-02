use brk_error::{Error, Result};
use brk_types::{
    BlockInfoV1, Day1, Height, Pool, PoolBlockCounts, PoolBlockShares, PoolDetail,
    PoolDetailInfo, PoolHashrateEntry, PoolInfo, PoolSlug, PoolStats, PoolsSummary, StoredF64,
    StoredU64, TimePeriod, pools,
};
use vecdb::{AnyVec, ReadableVec, VecIndex};

use crate::Query;

/// 7-day lookback for share computation (matching mempool.space)
const LOOKBACK_DAYS: usize = 7;
/// Weekly sample interval (matching mempool.space's 604800s interval)
const SAMPLE_WEEKLY: usize = 7;

/// Pre-read shared data for hashrate computation.
struct HashrateSharedData {
    start_day: usize,
    end_day: usize,
    daily_hashrate: Vec<Option<StoredF64>>,
    first_heights: Vec<Height>,
}

impl Query {
    pub fn mining_pools(&self, time_period: TimePeriod) -> Result<PoolsSummary> {
        let computer = self.computer();
        let current_height = self.height();

        // No blocks indexed yet
        if computer.pools.pool.len() == 0 {
            return Ok(PoolsSummary {
                pools: vec![],
                block_count: 0,
                last_estimated_hashrate: 0,
                last_estimated_hashrate3d: 0,
                last_estimated_hashrate1w: 0,
            });
        }

        // Use timestamp-based lookback for accurate time boundaries
        let lookback = &computer.blocks.lookback;
        let start = match time_period {
            TimePeriod::Day => lookback.cached_window_starts.0._24h.collect_one(current_height),
            TimePeriod::ThreeDays => lookback._3d.collect_one(current_height),
            TimePeriod::Week => lookback.cached_window_starts.0._1w.collect_one(current_height),
            TimePeriod::Month => lookback.cached_window_starts.0._1m.collect_one(current_height),
            TimePeriod::ThreeMonths => lookback._3m.collect_one(current_height),
            TimePeriod::SixMonths => lookback._6m.collect_one(current_height),
            TimePeriod::Year => lookback.cached_window_starts.0._1y.collect_one(current_height),
            TimePeriod::TwoYears => lookback._2y.collect_one(current_height),
            TimePeriod::ThreeYears => lookback._3y.collect_one(current_height),
        }
        .unwrap_or_default()
        .to_usize();

        let pools = pools();
        let mut pool_data: Vec<(&'static Pool, u64)> = Vec::new();

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

        let hashrate_at = |height: Height| -> u128 {
            let day = computer.indexes.height.day1.collect_one(height).unwrap_or_default();
            computer
                .mining
                .hashrate
                .rate
                .base
                .day1
                .collect_one(day)
                .flatten()
                .map(|v| *v as u128)
                .unwrap_or(0)
        };

        let lookback = &computer.blocks.lookback;
        let last_estimated_hashrate = hashrate_at(current_height);
        let last_estimated_hashrate3d =
            hashrate_at(lookback._3d.collect_one(current_height).unwrap_or_default());
        let last_estimated_hashrate1w =
            hashrate_at(lookback._1w.collect_one(current_height).unwrap_or_default());

        Ok(PoolsSummary {
            pools: pool_stats,
            block_count: total_blocks,
            last_estimated_hashrate,
            last_estimated_hashrate3d,
            last_estimated_hashrate1w,
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

        // Use timestamp-based lookback for accurate time boundaries
        let lookback = &computer.blocks.lookback;
        let start_24h = lookback
            .cached_window_starts
            .0
            ._24h
            .collect_one(current_height)
            .unwrap_or_default()
            .to_usize();
        let count_before_24h: u64 = if start_24h == 0 {
            0
        } else {
            *cumulative
                .collect_one(Height::from(start_24h - 1))
                .unwrap_or_default()
        };
        let total_24h = total_all.saturating_sub(count_before_24h);

        let start_1w = lookback
            .cached_window_starts
            .0
            ._1w
            .collect_one(current_height)
            .unwrap_or_default()
            .to_usize();
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

        const POOL_BLOCKS_LIMIT: usize = 100;
        let mut heights = Vec::with_capacity(POOL_BLOCKS_LIMIT);
        for h in (0..=end).rev() {
            if reader.get(h) == slug {
                heights.push(h);
                if heights.len() >= POOL_BLOCKS_LIMIT {
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
        let pool_name = pools().get(slug).name.to_string();
        let shared = self.hashrate_shared_data(0)?;
        let pool_cum = self.pool_daily_cumulative(slug, shared.start_day, shared.end_day)?;
        Ok(Self::compute_hashrate_entries(
            &shared, &pool_cum, &pool_name, SAMPLE_WEEKLY,
        ))
    }

    pub fn pools_hashrate(
        &self,
        time_period: Option<TimePeriod>,
    ) -> Result<Vec<PoolHashrateEntry>> {
        let start_height = match time_period {
            Some(tp) => {
                let lookback = &self.computer().blocks.lookback;
                let current_height = self.height();
                match tp {
                    TimePeriod::Day => lookback.cached_window_starts.0._24h.collect_one(current_height),
                    TimePeriod::ThreeDays => lookback._3d.collect_one(current_height),
                    TimePeriod::Week => lookback.cached_window_starts.0._1w.collect_one(current_height),
                    TimePeriod::Month => lookback.cached_window_starts.0._1m.collect_one(current_height),
                    TimePeriod::ThreeMonths => lookback._3m.collect_one(current_height),
                    TimePeriod::SixMonths => lookback._6m.collect_one(current_height),
                    TimePeriod::Year => lookback.cached_window_starts.0._1y.collect_one(current_height),
                    TimePeriod::TwoYears => lookback._2y.collect_one(current_height),
                    TimePeriod::ThreeYears => lookback._3y.collect_one(current_height),
                }
                .unwrap_or_default()
                .to_usize()
            }
            None => 0,
        };

        let shared = self.hashrate_shared_data(start_height)?;
        let pools_list = pools();
        let mut entries = Vec::new();

        for pool in pools_list.iter() {
            let Ok(pool_cum) =
                self.pool_daily_cumulative(pool.slug, shared.start_day, shared.end_day)
            else {
                continue;
            };
            entries.extend(Self::compute_hashrate_entries(
                &shared,
                &pool_cum,
                &pool.name,
                SAMPLE_WEEKLY,
            ));
        }

        Ok(entries)
    }

    /// Shared data needed for hashrate computation (read once, reuse across pools).
    fn hashrate_shared_data(&self, start_height: usize) -> Result<HashrateSharedData> {
        let computer = self.computer();
        let current_height = self.height();
        let start_day = computer
            .indexes
            .height
            .day1
            .collect_one_at(start_height)
            .unwrap_or_default()
            .to_usize();
        let end_day = computer
            .indexes
            .height
            .day1
            .collect_one(current_height)
            .unwrap_or_default()
            .to_usize()
            + 1;
        let daily_hashrate = computer
            .mining
            .hashrate
            .rate
            .base
            .day1
            .collect_range_at(start_day, end_day);
        let first_heights = computer
            .indexes
            .day1
            .first_height
            .collect_range_at(start_day, end_day);

        Ok(HashrateSharedData {
            start_day,
            end_day,
            daily_hashrate,
            first_heights,
        })
    }

    /// Read daily cumulative blocks mined for a pool.
    fn pool_daily_cumulative(
        &self,
        slug: PoolSlug,
        start_day: usize,
        end_day: usize,
    ) -> Result<Vec<Option<StoredU64>>> {
        let computer = self.computer();
        computer
            .pools
            .major
            .get(&slug)
            .map(|v| v.base.blocks_mined.cumulative.day1.collect_range_at(start_day, end_day))
            .or_else(|| {
                computer
                    .pools
                    .minor
                    .get(&slug)
                    .map(|v| v.blocks_mined.cumulative.day1.collect_range_at(start_day, end_day))
            })
            .ok_or_else(|| Error::NotFound("Pool not found".into()))
    }

    /// Compute hashrate entries from daily cumulative blocks + shared data.
    /// Uses 7-day windowed share: pool_blocks_in_week / total_blocks_in_week.
    fn compute_hashrate_entries(
        shared: &HashrateSharedData,
        pool_cum: &[Option<StoredU64>],
        pool_name: &str,
        sample_days: usize,
    ) -> Vec<PoolHashrateEntry> {
        let total = pool_cum.len();
        if total <= LOOKBACK_DAYS {
            return vec![];
        }

        let mut entries = Vec::new();
        let mut i = LOOKBACK_DAYS;
        while i < total {
            if let (Some(cum_now), Some(cum_prev)) =
                (pool_cum[i], pool_cum[i - LOOKBACK_DAYS])
            {
                let pool_blocks = (*cum_now).saturating_sub(*cum_prev);
                if pool_blocks > 0 {
                    let h_now = shared.first_heights[i].to_usize();
                    let h_prev = shared.first_heights[i - LOOKBACK_DAYS].to_usize();
                    let total_blocks = h_now.saturating_sub(h_prev);

                    if total_blocks > 0 {
                        if let Some(hr) = shared.daily_hashrate[i].as_ref() {
                            let network_hr = f64::from(**hr);
                            let share = pool_blocks as f64 / total_blocks as f64;
                            let day = Day1::from(shared.start_day + i);
                            entries.push(PoolHashrateEntry {
                                timestamp: day.to_timestamp(),
                                avg_hashrate: (network_hr * share) as u128,
                                share,
                                pool_name: pool_name.to_string(),
                            });
                        }
                    }
                }
            }
            i += sample_days;
        }

        entries
    }
}
