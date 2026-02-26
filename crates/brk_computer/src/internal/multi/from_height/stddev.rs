use std::mem;

use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Dollars, Height, StoredF32, Version};
use vecdb::{
    AnyStoredVec, AnyVec, Database, EagerVec, Exit, PcoVec, ReadableVec, Rw, StorageMode, VecIndex,
    WritableVec,
};

use crate::{ComputeIndexes, blocks, indexes};

use crate::internal::{ComputedFromHeightLast, Price};

#[derive(Default)]
pub struct StandardDeviationVecsOptions {
    zscore: bool,
    bands: bool,
    price_bands: bool,
}

impl StandardDeviationVecsOptions {
    pub(crate) fn add_all(mut self) -> Self {
        self.zscore = true;
        self.bands = true;
        self.price_bands = true;
        self
    }

    pub(crate) fn zscore(&self) -> bool {
        self.zscore
    }

    pub(crate) fn bands(&self) -> bool {
        self.bands
    }

    pub(crate) fn price_bands(&self) -> bool {
        self.price_bands
    }
}

#[derive(Traversable)]
pub struct ComputedFromHeightStdDev<M: StorageMode = Rw> {
    days: usize,

    pub sma: Option<ComputedFromHeightLast<StoredF32, M>>,

    pub sd: ComputedFromHeightLast<StoredF32, M>,

    pub zscore: Option<ComputedFromHeightLast<StoredF32, M>>,

    pub p0_5sd: Option<ComputedFromHeightLast<StoredF32, M>>,
    pub p1sd: Option<ComputedFromHeightLast<StoredF32, M>>,
    pub p1_5sd: Option<ComputedFromHeightLast<StoredF32, M>>,
    pub p2sd: Option<ComputedFromHeightLast<StoredF32, M>>,
    pub p2_5sd: Option<ComputedFromHeightLast<StoredF32, M>>,
    pub p3sd: Option<ComputedFromHeightLast<StoredF32, M>>,
    pub m0_5sd: Option<ComputedFromHeightLast<StoredF32, M>>,
    pub m1sd: Option<ComputedFromHeightLast<StoredF32, M>>,
    pub m1_5sd: Option<ComputedFromHeightLast<StoredF32, M>>,
    pub m2sd: Option<ComputedFromHeightLast<StoredF32, M>>,
    pub m2_5sd: Option<ComputedFromHeightLast<StoredF32, M>>,
    pub m3sd: Option<ComputedFromHeightLast<StoredF32, M>>,

    pub _0sd_usd: Option<Price<ComputedFromHeightLast<Dollars, M>>>,
    pub p0_5sd_usd: Option<Price<ComputedFromHeightLast<Dollars, M>>>,
    pub p1sd_usd: Option<Price<ComputedFromHeightLast<Dollars, M>>>,
    pub p1_5sd_usd: Option<Price<ComputedFromHeightLast<Dollars, M>>>,
    pub p2sd_usd: Option<Price<ComputedFromHeightLast<Dollars, M>>>,
    pub p2_5sd_usd: Option<Price<ComputedFromHeightLast<Dollars, M>>>,
    pub p3sd_usd: Option<Price<ComputedFromHeightLast<Dollars, M>>>,
    pub m0_5sd_usd: Option<Price<ComputedFromHeightLast<Dollars, M>>>,
    pub m1sd_usd: Option<Price<ComputedFromHeightLast<Dollars, M>>>,
    pub m1_5sd_usd: Option<Price<ComputedFromHeightLast<Dollars, M>>>,
    pub m2sd_usd: Option<Price<ComputedFromHeightLast<Dollars, M>>>,
    pub m2_5sd_usd: Option<Price<ComputedFromHeightLast<Dollars, M>>>,
    pub m3sd_usd: Option<Price<ComputedFromHeightLast<Dollars, M>>>,
}

impl ComputedFromHeightStdDev {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn forced_import(
        db: &Database,
        name: &str,
        days: usize,
        parent_version: Version,
        indexes: &indexes::Vecs,
        options: StandardDeviationVecsOptions,
    ) -> Result<Self> {
        let version = parent_version + Version::TWO;

        macro_rules! import {
            ($suffix:expr) => {
                ComputedFromHeightLast::forced_import(
                    db,
                    &format!("{name}_{}", $suffix),
                    version,
                    indexes,
                )
                .unwrap()
            };
        }

        let sma_vec = Some(import!("sma"));
        let p0_5sd = options.bands().then(|| import!("p0_5sd"));
        let p1sd = options.bands().then(|| import!("p1sd"));
        let p1_5sd = options.bands().then(|| import!("p1_5sd"));
        let p2sd = options.bands().then(|| import!("p2sd"));
        let p2_5sd = options.bands().then(|| import!("p2_5sd"));
        let p3sd = options.bands().then(|| import!("p3sd"));
        let m0_5sd = options.bands().then(|| import!("m0_5sd"));
        let m1sd = options.bands().then(|| import!("m1sd"));
        let m1_5sd = options.bands().then(|| import!("m1_5sd"));
        let m2sd = options.bands().then(|| import!("m2sd"));
        let m2_5sd = options.bands().then(|| import!("m2_5sd"));
        let m3sd = options.bands().then(|| import!("m3sd"));

        // Import USD price band vecs (computed eagerly at compute time)
        macro_rules! lazy_usd {
            ($band:expr, $suffix:expr) => {
                if !options.price_bands() {
                    None
                } else {
                    $band.as_ref().map(|_| {
                        Price::forced_import(
                            db,
                            &format!("{name}_{}", $suffix),
                            version,
                            indexes,
                        )
                        .unwrap()
                    })
                }
            };
        }

        Ok(Self {
            days,
            sd: import!("sd"),
            zscore: options.zscore().then(|| import!("zscore")),
            // Lazy USD vecs
            _0sd_usd: lazy_usd!(&sma_vec, "0sd_usd"),
            p0_5sd_usd: lazy_usd!(&p0_5sd, "p0_5sd_usd"),
            p1sd_usd: lazy_usd!(&p1sd, "p1sd_usd"),
            p1_5sd_usd: lazy_usd!(&p1_5sd, "p1_5sd_usd"),
            p2sd_usd: lazy_usd!(&p2sd, "p2sd_usd"),
            p2_5sd_usd: lazy_usd!(&p2_5sd, "p2_5sd_usd"),
            p3sd_usd: lazy_usd!(&p3sd, "p3sd_usd"),
            m0_5sd_usd: lazy_usd!(&m0_5sd, "m0_5sd_usd"),
            m1sd_usd: lazy_usd!(&m1sd, "m1sd_usd"),
            m1_5sd_usd: lazy_usd!(&m1_5sd, "m1_5sd_usd"),
            m2sd_usd: lazy_usd!(&m2sd, "m2sd_usd"),
            m2_5sd_usd: lazy_usd!(&m2_5sd, "m2_5sd_usd"),
            m3sd_usd: lazy_usd!(&m3sd, "m3sd_usd"),
            // Stored band sources
            sma: sma_vec,
            p0_5sd,
            p1sd,
            p1_5sd,
            p2sd,
            p2_5sd,
            p3sd,
            m0_5sd,
            m1sd,
            m1_5sd,
            m2sd,
            m2_5sd,
            m3sd,
        })
    }

    pub(crate) fn forced_import_from_lazy(
        db: &Database,
        name: &str,
        days: usize,
        parent_version: Version,
        indexes: &indexes::Vecs,
        options: StandardDeviationVecsOptions,
    ) -> Result<Self> {
        let version = parent_version + Version::TWO;

        macro_rules! import {
            ($suffix:expr) => {
                ComputedFromHeightLast::forced_import(
                    db,
                    &format!("{name}_{}", $suffix),
                    version,
                    indexes,
                )
                .unwrap()
            };
        }

        let sma_vec = Some(import!("sma"));
        let p0_5sd = options.bands().then(|| import!("p0_5sd"));
        let p1sd = options.bands().then(|| import!("p1sd"));
        let p1_5sd = options.bands().then(|| import!("p1_5sd"));
        let p2sd = options.bands().then(|| import!("p2sd"));
        let p2_5sd = options.bands().then(|| import!("p2_5sd"));
        let p3sd = options.bands().then(|| import!("p3sd"));
        let m0_5sd = options.bands().then(|| import!("m0_5sd"));
        let m1sd = options.bands().then(|| import!("m1sd"));
        let m1_5sd = options.bands().then(|| import!("m1_5sd"));
        let m2sd = options.bands().then(|| import!("m2sd"));
        let m2_5sd = options.bands().then(|| import!("m2_5sd"));
        let m3sd = options.bands().then(|| import!("m3sd"));

        // For lazy metric price, use from_lazy_block_last_and_block_last.
        macro_rules! lazy_usd {
            ($band:expr, $suffix:expr) => {
                if !options.price_bands() {
                    None
                } else {
                    $band.as_ref().map(|_| {
                        Price::forced_import(
                            db,
                            &format!("{name}_{}", $suffix),
                            version,
                            indexes,
                        )
                        .unwrap()
                    })
                }
            };
        }

        Ok(Self {
            days,
            sd: import!("sd"),
            zscore: options.zscore().then(|| import!("zscore")),
            _0sd_usd: lazy_usd!(&sma_vec, "0sd_usd"),
            p0_5sd_usd: lazy_usd!(&p0_5sd, "p0_5sd_usd"),
            p1sd_usd: lazy_usd!(&p1sd, "p1sd_usd"),
            p1_5sd_usd: lazy_usd!(&p1_5sd, "p1_5sd_usd"),
            p2sd_usd: lazy_usd!(&p2sd, "p2sd_usd"),
            p2_5sd_usd: lazy_usd!(&p2_5sd, "p2_5sd_usd"),
            p3sd_usd: lazy_usd!(&p3sd, "p3sd_usd"),
            m0_5sd_usd: lazy_usd!(&m0_5sd, "m0_5sd_usd"),
            m1sd_usd: lazy_usd!(&m1sd, "m1sd_usd"),
            m1_5sd_usd: lazy_usd!(&m1_5sd, "m1_5sd_usd"),
            m2sd_usd: lazy_usd!(&m2sd, "m2sd_usd"),
            m2_5sd_usd: lazy_usd!(&m2_5sd, "m2_5sd_usd"),
            m3sd_usd: lazy_usd!(&m3sd, "m3sd_usd"),
            sma: sma_vec,
            p0_5sd,
            p1sd,
            p1_5sd,
            p2sd,
            p2_5sd,
            p3sd,
            m0_5sd,
            m1sd,
            m1_5sd,
            m2sd,
            m2_5sd,
            m3sd,
        })
    }

    pub(crate) fn compute_all(
        &mut self,
        blocks: &blocks::Vecs,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
        source: &impl ReadableVec<Height, StoredF32>,
    ) -> Result<()> {
        // 1. Compute SMA using the appropriate lookback vec (or full-history SMA)
        if self.days != usize::MAX {
            let window_starts = blocks.count.start_vec(self.days);
            self.sma.as_mut().unwrap().height.compute_rolling_average(
                starting_indexes.height,
                window_starts,
                source,
                exit,
            )?;
        } else {
            // Full history SMA (days == usize::MAX)
            self.sma.as_mut().unwrap().height.compute_sma_(
                starting_indexes.height,
                source,
                self.days,
                exit,
                None,
            )?;
        }

        let sma_opt: Option<&EagerVec<PcoVec<Height, StoredF32>>> = None;
        self.compute_rest(blocks, starting_indexes, exit, sma_opt, source)
    }

    pub(crate) fn compute_rest(
        &mut self,
        blocks: &blocks::Vecs,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
        sma_opt: Option<&impl ReadableVec<Height, StoredF32>>,
        source: &impl ReadableVec<Height, StoredF32>,
    ) -> Result<()> {
        let sma = sma_opt
            .unwrap_or_else(|| unsafe { mem::transmute(&self.sma.as_ref().unwrap().height) });

        let source_version = source.version();

        self.mut_stateful_height_vecs()
            .try_for_each(|v| -> Result<()> {
                v.validate_computed_version_or_reset(source_version)?;
                Ok(())
            })?;

        let starting_height = self
            .mut_stateful_height_vecs()
            .map(|v| Height::from(v.len()))
            .min()
            .unwrap()
            .min(starting_indexes.height);

        // Reconstruct running statistics up to starting point.
        // We accumulate one data point per day boundary, tracking sum and sum_sq
        // for O(1) per-height SD computation (instead of O(n) sorted-array scan).
        let day_start = &blocks.count.height_24h_ago;
        let start = starting_height.to_usize();

        let mut n: usize = 0;
        let mut welford_sum: f64 = 0.0;
        let mut welford_sum_sq: f64 = 0.0;
        if start > 0 {
            let day_start_hist = day_start.collect_range_at(0, start);
            let source_hist = source.collect_range_at(0, start);
            let mut last_ds = Height::from(0_usize);
            for h in 0..start {
                let cur_ds = day_start_hist[h];
                if h == 0 || cur_ds != last_ds {
                    let val = *source_hist[h] as f64;
                    n += 1;
                    welford_sum += val;
                    welford_sum_sq += val * val;
                    last_ds = cur_ds;
                }
            }
        }

        macro_rules! band_ref {
            ($field:ident) => {
                self.$field.as_mut().map(|c| &mut c.height)
            };
        }
        let mut p0_5sd = band_ref!(p0_5sd);
        let mut p1sd = band_ref!(p1sd);
        let mut p1_5sd = band_ref!(p1_5sd);
        let mut p2sd = band_ref!(p2sd);
        let mut p2_5sd = band_ref!(p2_5sd);
        let mut p3sd = band_ref!(p3sd);
        let mut m0_5sd = band_ref!(m0_5sd);
        let mut m1sd = band_ref!(m1sd);
        let mut m1_5sd = band_ref!(m1_5sd);
        let mut m2sd = band_ref!(m2sd);
        let mut m2_5sd = band_ref!(m2_5sd);
        let mut m3sd = band_ref!(m3sd);

        let source_len = source.len();
        let source_data = source.collect_range_at(start, source_len);
        let sma_data = sma.collect_range_at(start, sma.len());
        let mut last_day_start = if start > 0 {
            day_start
                .collect_one_at(start - 1)
                .unwrap_or(Height::from(0_usize))
        } else {
            Height::from(0_usize)
        };

        let day_start_data = day_start.collect_range_at(start, source_len);

        for (offset, ratio) in source_data.into_iter().enumerate() {
            let index = start + offset;
            // Update running statistics on day boundaries
            let cur_day_start = day_start_data[offset];
            if index == 0 || cur_day_start != last_day_start {
                let val = *ratio as f64;
                n += 1;
                welford_sum += val;
                welford_sum_sq += val * val;
                last_day_start = cur_day_start;
            }

            let average = sma_data[offset];
            let avg_f64 = *average as f64;

            // SD = sqrt((sum_sq/n - 2*avg*sum/n + avg^2))
            // This is the population SD of all daily values relative to the current SMA
            let sd = if n > 0 {
                let nf = n as f64;
                let variance =
                    welford_sum_sq / nf - 2.0 * avg_f64 * welford_sum / nf + avg_f64 * avg_f64;
                StoredF32::from(variance.max(0.0).sqrt() as f32)
            } else {
                StoredF32::from(0.0_f32)
            };

            self.sd.height.truncate_push_at(index, sd)?;
            if let Some(v) = p0_5sd.as_mut() {
                v.truncate_push_at(index, average + StoredF32::from(0.5 * *sd))?
            }
            if let Some(v) = p1sd.as_mut() {
                v.truncate_push_at(index, average + sd)?
            }
            if let Some(v) = p1_5sd.as_mut() {
                v.truncate_push_at(index, average + StoredF32::from(1.5 * *sd))?
            }
            if let Some(v) = p2sd.as_mut() {
                v.truncate_push_at(index, average + 2 * sd)?
            }
            if let Some(v) = p2_5sd.as_mut() {
                v.truncate_push_at(index, average + StoredF32::from(2.5 * *sd))?
            }
            if let Some(v) = p3sd.as_mut() {
                v.truncate_push_at(index, average + 3 * sd)?
            }
            if let Some(v) = m0_5sd.as_mut() {
                v.truncate_push_at(index, average - StoredF32::from(0.5 * *sd))?
            }
            if let Some(v) = m1sd.as_mut() {
                v.truncate_push_at(index, average - sd)?
            }
            if let Some(v) = m1_5sd.as_mut() {
                v.truncate_push_at(index, average - StoredF32::from(1.5 * *sd))?
            }
            if let Some(v) = m2sd.as_mut() {
                v.truncate_push_at(index, average - 2 * sd)?
            }
            if let Some(v) = m2_5sd.as_mut() {
                v.truncate_push_at(index, average - StoredF32::from(2.5 * *sd))?
            }
            if let Some(v) = m3sd.as_mut() {
                v.truncate_push_at(index, average - 3 * sd)?
            }
        }

        {
            let _lock = exit.lock();
            self.mut_stateful_height_vecs()
                .try_for_each(|v| v.flush())?;
        }

        if let Some(zscore) = self.zscore.as_mut() {
            zscore.height.compute_zscore(
                starting_indexes.height,
                source,
                sma,
                &self.sd.height,
                exit,
            )?;
        }

        Ok(())
    }

    /// Compute USD price bands: usd_band = metric_price * band_ratio
    pub(crate) fn compute_usd_bands(
        &mut self,
        starting_indexes: &ComputeIndexes,
        metric_price: &impl ReadableVec<Height, Dollars>,
        exit: &Exit,
    ) -> Result<()> {
        use crate::internal::PriceTimesRatio;

        macro_rules! compute_band {
            ($usd_field:ident, $band_field:ident) => {
                if let Some(usd) = self.$usd_field.as_mut() {
                    if let Some(band) = self.$band_field.as_ref() {
                        usd.usd
                            .compute_binary::<Dollars, StoredF32, PriceTimesRatio>(
                                starting_indexes.height,
                                metric_price,
                                &band.height,
                                exit,
                            )?;
                    }
                }
            };
        }

        compute_band!(_0sd_usd, sma);
        compute_band!(p0_5sd_usd, p0_5sd);
        compute_band!(p1sd_usd, p1sd);
        compute_band!(p1_5sd_usd, p1_5sd);
        compute_band!(p2sd_usd, p2sd);
        compute_band!(p2_5sd_usd, p2_5sd);
        compute_band!(p3sd_usd, p3sd);
        compute_band!(m0_5sd_usd, m0_5sd);
        compute_band!(m1sd_usd, m1sd);
        compute_band!(m1_5sd_usd, m1_5sd);
        compute_band!(m2sd_usd, m2sd);
        compute_band!(m2_5sd_usd, m2_5sd);
        compute_band!(m3sd_usd, m3sd);

        Ok(())
    }

    fn mut_stateful_computed(
        &mut self,
    ) -> impl Iterator<Item = &mut ComputedFromHeightLast<StoredF32>> {
        [
            Some(&mut self.sd),
            self.p0_5sd.as_mut(),
            self.p1sd.as_mut(),
            self.p1_5sd.as_mut(),
            self.p2sd.as_mut(),
            self.p2_5sd.as_mut(),
            self.p3sd.as_mut(),
            self.m0_5sd.as_mut(),
            self.m1sd.as_mut(),
            self.m1_5sd.as_mut(),
            self.m2sd.as_mut(),
            self.m2_5sd.as_mut(),
            self.m3sd.as_mut(),
        ]
        .into_iter()
        .flatten()
    }

    fn mut_stateful_height_vecs(
        &mut self,
    ) -> impl Iterator<Item = &mut EagerVec<PcoVec<Height, StoredF32>>> {
        self.mut_stateful_computed().map(|c| &mut c.height)
    }
}
