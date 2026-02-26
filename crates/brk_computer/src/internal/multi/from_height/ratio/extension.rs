use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Dollars, Height, StoredF32, Version};
use vecdb::{AnyStoredVec, AnyVec, Database, EagerVec, Exit, PcoVec, ReadableVec, Rw, StorageMode, VecIndex, WritableVec};

use crate::{
    ComputeIndexes, blocks, indexes,
    internal::{ComputedFromHeightStdDevExtended, Price},
    utils::get_percentile,
};

use super::super::ComputedFromHeightLast;

#[derive(Traversable)]
pub struct ComputedFromHeightRatioExtension<M: StorageMode = Rw> {
    pub ratio_1w_sma: ComputedFromHeightLast<StoredF32, M>,
    pub ratio_1m_sma: ComputedFromHeightLast<StoredF32, M>,
    pub ratio_pct99: ComputedFromHeightLast<StoredF32, M>,
    pub ratio_pct98: ComputedFromHeightLast<StoredF32, M>,
    pub ratio_pct95: ComputedFromHeightLast<StoredF32, M>,
    pub ratio_pct5: ComputedFromHeightLast<StoredF32, M>,
    pub ratio_pct2: ComputedFromHeightLast<StoredF32, M>,
    pub ratio_pct1: ComputedFromHeightLast<StoredF32, M>,
    pub ratio_pct99_usd: Price<ComputedFromHeightLast<Dollars, M>>,
    pub ratio_pct98_usd: Price<ComputedFromHeightLast<Dollars, M>>,
    pub ratio_pct95_usd: Price<ComputedFromHeightLast<Dollars, M>>,
    pub ratio_pct5_usd: Price<ComputedFromHeightLast<Dollars, M>>,
    pub ratio_pct2_usd: Price<ComputedFromHeightLast<Dollars, M>>,
    pub ratio_pct1_usd: Price<ComputedFromHeightLast<Dollars, M>>,

    pub ratio_sd: ComputedFromHeightStdDevExtended<M>,
    pub ratio_4y_sd: ComputedFromHeightStdDevExtended<M>,
    pub ratio_2y_sd: ComputedFromHeightStdDevExtended<M>,
    pub ratio_1y_sd: ComputedFromHeightStdDevExtended<M>,
}

const VERSION: Version = Version::TWO;

impl ComputedFromHeightRatioExtension {
    pub(crate) fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let v = version + VERSION;

        macro_rules! import {
            ($suffix:expr) => {
                ComputedFromHeightLast::forced_import(
                    db,
                    &format!("{name}_{}", $suffix),
                    v,
                    indexes,
                )?
            };
        }

        macro_rules! import_sd {
            ($suffix:expr, $days:expr) => {
                ComputedFromHeightStdDevExtended::forced_import(
                    db,
                    &format!("{name}_{}", $suffix),
                    $days,
                    v,
                    indexes,
                )?
            };
        }

        macro_rules! import_usd {
            ($suffix:expr) => {
                Price::forced_import(db, &format!("{name}_{}", $suffix), v, indexes)?
            };
        }

        Ok(Self {
            ratio_1w_sma: import!("ratio_1w_sma"),
            ratio_1m_sma: import!("ratio_1m_sma"),
            ratio_sd: import_sd!("ratio", usize::MAX),
            ratio_1y_sd: import_sd!("ratio_1y", 365),
            ratio_2y_sd: import_sd!("ratio_2y", 2 * 365),
            ratio_4y_sd: import_sd!("ratio_4y", 4 * 365),
            ratio_pct99: import!("ratio_pct99"),
            ratio_pct98: import!("ratio_pct98"),
            ratio_pct95: import!("ratio_pct95"),
            ratio_pct5: import!("ratio_pct5"),
            ratio_pct2: import!("ratio_pct2"),
            ratio_pct1: import!("ratio_pct1"),
            ratio_pct99_usd: import_usd!("ratio_pct99_usd"),
            ratio_pct98_usd: import_usd!("ratio_pct98_usd"),
            ratio_pct95_usd: import_usd!("ratio_pct95_usd"),
            ratio_pct5_usd: import_usd!("ratio_pct5_usd"),
            ratio_pct2_usd: import_usd!("ratio_pct2_usd"),
            ratio_pct1_usd: import_usd!("ratio_pct1_usd"),
        })
    }

    /// Compute extended ratio metrics from an externally-provided ratio source.
    pub(crate) fn compute_rest(
        &mut self,
        blocks: &blocks::Vecs,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
        ratio_source: &impl ReadableVec<Height, StoredF32>,
    ) -> Result<()> {
        // SMA using lookback vecs
        self.ratio_1w_sma.height.compute_rolling_average(
            starting_indexes.height,
            &blocks.count.height_1w_ago,
            ratio_source,
            exit,
        )?;

        self.ratio_1m_sma.height.compute_rolling_average(
            starting_indexes.height,
            &blocks.count.height_1m_ago,
            ratio_source,
            exit,
        )?;

        // Percentiles: insert into sorted array on day boundaries
        let ratio_version = ratio_source.version();
        self.mut_ratio_vecs()
            .try_for_each(|v| -> Result<()> {
                v.validate_computed_version_or_reset(ratio_version)?;
                Ok(())
            })?;

        let starting_height = self
            .mut_ratio_vecs()
            .map(|v| Height::from(v.len()))
            .min()
            .unwrap()
            .min(starting_indexes.height);

        let start = starting_height.to_usize();
        let day_start = &blocks.count.height_24h_ago;

        // Collect sorted history up to starting point (one per day boundary)
        let mut sorted = {
            let ratio_data = ratio_source.collect_range_at(0, start);
            let day_start_hist = day_start.collect_range_at(0, start);
            let mut sorted: Vec<StoredF32> = Vec::new();
            let mut last_day_start = Height::from(0_usize);
            for (h, ratio) in ratio_data.into_iter().enumerate() {
                let cur_day_start = day_start_hist[h];
                if h == 0 || cur_day_start != last_day_start {
                    sorted.push(ratio);
                    last_day_start = cur_day_start;
                }
            }
            sorted.sort_unstable();
            sorted
        };

        let pct1_vec = &mut self.ratio_pct1.height;
        let pct2_vec = &mut self.ratio_pct2.height;
        let pct5_vec = &mut self.ratio_pct5.height;
        let pct95_vec = &mut self.ratio_pct95.height;
        let pct98_vec = &mut self.ratio_pct98.height;
        let pct99_vec = &mut self.ratio_pct99.height;

        let ratio_len = ratio_source.len();
        let ratio_data = ratio_source.collect_range_at(start, ratio_len);
        let mut last_day_start = if start > 0 {
            day_start
                .collect_one_at(start - 1)
                .unwrap_or(Height::from(0_usize))
        } else {
            Height::from(0_usize)
        };

        let day_start_data = day_start.collect_range_at(start, ratio_len);

        for (offset, ratio) in ratio_data.into_iter().enumerate() {
            let index = start + offset;

            let cur_day_start = day_start_data[offset];
            if index == 0 || cur_day_start != last_day_start {
                let pos = sorted.binary_search(&ratio).unwrap_or_else(|p| p);
                sorted.insert(pos, ratio);
                last_day_start = cur_day_start;
            }

            if sorted.is_empty() {
                pct1_vec.truncate_push_at(index, StoredF32::NAN)?;
                pct2_vec.truncate_push_at(index, StoredF32::NAN)?;
                pct5_vec.truncate_push_at(index, StoredF32::NAN)?;
                pct95_vec.truncate_push_at(index, StoredF32::NAN)?;
                pct98_vec.truncate_push_at(index, StoredF32::NAN)?;
                pct99_vec.truncate_push_at(index, StoredF32::NAN)?;
            } else {
                pct1_vec.truncate_push_at(index, get_percentile(&sorted, 0.01))?;
                pct2_vec.truncate_push_at(index, get_percentile(&sorted, 0.02))?;
                pct5_vec.truncate_push_at(index, get_percentile(&sorted, 0.05))?;
                pct95_vec.truncate_push_at(index, get_percentile(&sorted, 0.95))?;
                pct98_vec.truncate_push_at(index, get_percentile(&sorted, 0.98))?;
                pct99_vec.truncate_push_at(index, get_percentile(&sorted, 0.99))?;
            }
        }

        {
            let _lock = exit.lock();
            self.mut_ratio_vecs()
                .try_for_each(|v| v.flush())?;
        }

        // Compute stddev at height level
        self.ratio_sd
            .compute_all(blocks, starting_indexes, exit, ratio_source)?;
        self.ratio_4y_sd
            .compute_all(blocks, starting_indexes, exit, ratio_source)?;
        self.ratio_2y_sd
            .compute_all(blocks, starting_indexes, exit, ratio_source)?;
        self.ratio_1y_sd
            .compute_all(blocks, starting_indexes, exit, ratio_source)?;

        Ok(())
    }

    /// Compute USD ratio bands: usd_band = metric_price * ratio_percentile
    pub(crate) fn compute_usd_bands(
        &mut self,
        starting_indexes: &ComputeIndexes,
        metric_price: &impl ReadableVec<Height, Dollars>,
        exit: &Exit,
    ) -> Result<()> {
        use crate::internal::PriceTimesRatio;

        macro_rules! compute_band {
            ($usd_field:ident, $band_source:expr) => {
                self.$usd_field
                    .usd
                    .compute_binary::<Dollars, StoredF32, PriceTimesRatio>(
                        starting_indexes.height,
                        metric_price,
                        $band_source,
                        exit,
                    )?;
            };
        }

        compute_band!(ratio_pct99_usd, &self.ratio_pct99.height);
        compute_band!(ratio_pct98_usd, &self.ratio_pct98.height);
        compute_band!(ratio_pct95_usd, &self.ratio_pct95.height);
        compute_band!(ratio_pct5_usd, &self.ratio_pct5.height);
        compute_band!(ratio_pct2_usd, &self.ratio_pct2.height);
        compute_band!(ratio_pct1_usd, &self.ratio_pct1.height);

        // Stddev USD bands
        self.ratio_sd
            .compute_usd_bands(starting_indexes, metric_price, exit)?;
        self.ratio_4y_sd
            .compute_usd_bands(starting_indexes, metric_price, exit)?;
        self.ratio_2y_sd
            .compute_usd_bands(starting_indexes, metric_price, exit)?;
        self.ratio_1y_sd
            .compute_usd_bands(starting_indexes, metric_price, exit)?;

        Ok(())
    }

    fn mut_ratio_vecs(
        &mut self,
    ) -> impl Iterator<Item = &mut EagerVec<PcoVec<Height, StoredF32>>> {
        [
            &mut self.ratio_pct1.height,
            &mut self.ratio_pct2.height,
            &mut self.ratio_pct5.height,
            &mut self.ratio_pct95.height,
            &mut self.ratio_pct98.height,
            &mut self.ratio_pct99.height,
        ]
        .into_iter()
    }
}
