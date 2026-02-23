use brk_error::{Error, Result};
use brk_types::{BlockTimestamp, Date, Day1, Height, Timestamp};
use jiff::Timestamp as JiffTimestamp;
use vecdb::ReadableVec;

use crate::Query;

impl Query {
    pub fn block_by_timestamp(&self, timestamp: Timestamp) -> Result<BlockTimestamp> {
        let indexer = self.indexer();
        let computer = self.computer();

        let max_height = self.height();
        let max_height_usize: usize = max_height.into();

        if max_height_usize == 0 {
            return Err(Error::NotFound("No blocks indexed".into()));
        }

        let target = timestamp;
        let date = Date::from(target);
        let day1 = Day1::try_from(date).unwrap_or_default();

        // Get first height of the target date
        let first_height_of_day = computer
            .indexes
            .day1
            .first_height
            .collect_one(day1)
            .unwrap_or(Height::from(0usize));

        let start: usize = usize::from(first_height_of_day).min(max_height_usize);

        let timestamps = &indexer.vecs.blocks.timestamp;

        // Search forward from start to find the last block <= target timestamp
        let mut best_height = start;
        let mut best_ts = timestamps.collect_one_at(start).unwrap();

        for h in (start + 1)..=max_height_usize {
            let block_ts = timestamps.collect_one_at(h).unwrap();
            if block_ts <= target {
                best_height = h;
                best_ts = block_ts;
            } else {
                break;
            }
        }

        // Check one block before start in case we need to go backward
        if start > 0 && best_ts > target {
            let prev_ts = timestamps.collect_one_at(start - 1).unwrap();
            if prev_ts <= target {
                best_height = start - 1;
                best_ts = prev_ts;
            }
        }

        let height = Height::from(best_height);
        let blockhash = indexer.vecs.blocks.blockhash.reader().get(usize::from(height));

        // Convert timestamp to ISO 8601 format
        let ts_secs: i64 = (*best_ts).into();
        let iso_timestamp = JiffTimestamp::from_second(ts_secs)
            .map(|t| t.to_string())
            .unwrap_or_else(|_| best_ts.to_string());

        Ok(BlockTimestamp {
            height,
            hash: blockhash,
            timestamp: iso_timestamp,
        })
    }
}
