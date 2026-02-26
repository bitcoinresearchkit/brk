use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Dollars, Height, StoredF32, Version};
use vecdb::{
    AnyStoredVec, AnyVec, Database, EagerVec, Exit, PcoVec, ReadableVec, Rw, StorageMode, VecIndex,
    WritableVec,
};

use crate::{
    ComputeIndexes, blocks, indexes,
    internal::{ComputedFromHeightStdDev, Price, StandardDeviationVecsOptions},
    prices,
    utils::get_percentile,
};

use super::ComputedFromHeightLast;

#[derive(Traversable)]
pub struct ComputedFromHeightRatio<M: StorageMode = Rw> {
    pub price: Option<Price<ComputedFromHeightLast<Dollars, M>>>,

    pub ratio: ComputedFromHeightLast<StoredF32, M>,
    pub ratio_1w_sma: Option<ComputedFromHeightLast<StoredF32, M>>,
    pub ratio_1m_sma: Option<ComputedFromHeightLast<StoredF32, M>>,
    pub ratio_pct99: Option<ComputedFromHeightLast<StoredF32, M>>,
    pub ratio_pct98: Option<ComputedFromHeightLast<StoredF32, M>>,
    pub ratio_pct95: Option<ComputedFromHeightLast<StoredF32, M>>,
    pub ratio_pct5: Option<ComputedFromHeightLast<StoredF32, M>>,
    pub ratio_pct2: Option<ComputedFromHeightLast<StoredF32, M>>,
    pub ratio_pct1: Option<ComputedFromHeightLast<StoredF32, M>>,
    pub ratio_pct99_usd: Option<Price<ComputedFromHeightLast<Dollars, M>>>,
    pub ratio_pct98_usd: Option<Price<ComputedFromHeightLast<Dollars, M>>>,
    pub ratio_pct95_usd: Option<Price<ComputedFromHeightLast<Dollars, M>>>,
    pub ratio_pct5_usd: Option<Price<ComputedFromHeightLast<Dollars, M>>>,
    pub ratio_pct2_usd: Option<Price<ComputedFromHeightLast<Dollars, M>>>,
    pub ratio_pct1_usd: Option<Price<ComputedFromHeightLast<Dollars, M>>>,

    pub ratio_sd: Option<ComputedFromHeightStdDev<M>>,
    pub ratio_4y_sd: Option<ComputedFromHeightStdDev<M>>,
    pub ratio_2y_sd: Option<ComputedFromHeightStdDev<M>>,
    pub ratio_1y_sd: Option<ComputedFromHeightStdDev<M>>,
}

const VERSION: Version = Version::TWO;

impl ComputedFromHeightRatio {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn forced_import(
        db: &Database,
        name: &str,
        metric_price: Option<&ComputedFromHeightLast<Dollars>>,
        version: Version,
        indexes: &indexes::Vecs,
        extended: bool,
    ) -> Result<Self> {
        let v = version + VERSION;

        macro_rules! import {
            ($suffix:expr) => {
                ComputedFromHeightLast::forced_import(
                    db,
                    &format!("{name}_{}", $suffix),
                    v,
                    indexes,
                )
                .unwrap()
            };
        }

        // Only compute price internally when metric_price is None
        let price = metric_price
            .is_none()
            .then(|| Price::forced_import(db, name, v, indexes).unwrap());

        macro_rules! import_sd {
            ($suffix:expr, $days:expr) => {
                ComputedFromHeightStdDev::forced_import(
                    db,
                    &format!("{name}_{}", $suffix),
                    $days,
                    v,
                    indexes,
                    StandardDeviationVecsOptions::default().add_all(),
                )
                .unwrap()
            };
        }

        let ratio_pct99 = extended.then(|| import!("ratio_pct99"));
        let ratio_pct98 = extended.then(|| import!("ratio_pct98"));
        let ratio_pct95 = extended.then(|| import!("ratio_pct95"));
        let ratio_pct5 = extended.then(|| import!("ratio_pct5"));
        let ratio_pct2 = extended.then(|| import!("ratio_pct2"));
        let ratio_pct1 = extended.then(|| import!("ratio_pct1"));

        macro_rules! lazy_usd {
            ($ratio:expr, $suffix:expr) => {
                if !extended {
                    None
                } else {
                    $ratio.as_ref().map(|_| {
                        Price::forced_import(
                            db,
                            &format!("{name}_{}", $suffix),
                            v,
                            indexes,
                        )
                        .unwrap()
                    })
                }
            };
        }

        Ok(Self {
            ratio: import!("ratio"),
            ratio_1w_sma: extended.then(|| import!("ratio_1w_sma")),
            ratio_1m_sma: extended.then(|| import!("ratio_1m_sma")),
            ratio_sd: extended.then(|| import_sd!("ratio", usize::MAX)),
            ratio_1y_sd: extended.then(|| import_sd!("ratio_1y", 365)),
            ratio_2y_sd: extended.then(|| import_sd!("ratio_2y", 2 * 365)),
            ratio_4y_sd: extended.then(|| import_sd!("ratio_4y", 4 * 365)),
            ratio_pct99_usd: lazy_usd!(&ratio_pct99, "ratio_pct99_usd"),
            ratio_pct98_usd: lazy_usd!(&ratio_pct98, "ratio_pct98_usd"),
            ratio_pct95_usd: lazy_usd!(&ratio_pct95, "ratio_pct95_usd"),
            ratio_pct5_usd: lazy_usd!(&ratio_pct5, "ratio_pct5_usd"),
            ratio_pct2_usd: lazy_usd!(&ratio_pct2, "ratio_pct2_usd"),
            ratio_pct1_usd: lazy_usd!(&ratio_pct1, "ratio_pct1_usd"),
            price,
            ratio_pct99,
            ratio_pct98,
            ratio_pct95,
            ratio_pct5,
            ratio_pct2,
            ratio_pct1,
        })
    }

    pub(crate) fn forced_import_from_lazy(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
        extended: bool,
    ) -> Result<Self> {
        let v = version + VERSION;

        macro_rules! import {
            ($suffix:expr) => {
                ComputedFromHeightLast::forced_import(
                    db,
                    &format!("{name}_{}", $suffix),
                    v,
                    indexes,
                )
                .unwrap()
            };
        }

        macro_rules! import_sd {
            ($suffix:expr, $days:expr) => {
                ComputedFromHeightStdDev::forced_import_from_lazy(
                    db,
                    &format!("{name}_{}", $suffix),
                    $days,
                    v,
                    indexes,
                    StandardDeviationVecsOptions::default().add_all(),
                )
                .unwrap()
            };
        }

        let ratio_pct99 = extended.then(|| import!("ratio_pct99"));
        let ratio_pct98 = extended.then(|| import!("ratio_pct98"));
        let ratio_pct95 = extended.then(|| import!("ratio_pct95"));
        let ratio_pct5 = extended.then(|| import!("ratio_pct5"));
        let ratio_pct2 = extended.then(|| import!("ratio_pct2"));
        let ratio_pct1 = extended.then(|| import!("ratio_pct1"));

        macro_rules! lazy_usd {
            ($ratio:expr, $suffix:expr) => {
                $ratio.as_ref().map(|_| {
                    Price::forced_import(db, &format!("{name}_{}", $suffix), v, indexes)
                        .unwrap()
                })
            };
        }

        Ok(Self {
            ratio: import!("ratio"),
            ratio_1w_sma: extended.then(|| import!("ratio_1w_sma")),
            ratio_1m_sma: extended.then(|| import!("ratio_1m_sma")),
            ratio_sd: extended.then(|| import_sd!("ratio", usize::MAX)),
            ratio_1y_sd: extended.then(|| import_sd!("ratio_1y", 365)),
            ratio_2y_sd: extended.then(|| import_sd!("ratio_2y", 2 * 365)),
            ratio_4y_sd: extended.then(|| import_sd!("ratio_4y", 4 * 365)),
            ratio_pct99_usd: lazy_usd!(&ratio_pct99, "ratio_pct99_usd"),
            ratio_pct98_usd: lazy_usd!(&ratio_pct98, "ratio_pct98_usd"),
            ratio_pct95_usd: lazy_usd!(&ratio_pct95, "ratio_pct95_usd"),
            ratio_pct5_usd: lazy_usd!(&ratio_pct5, "ratio_pct5_usd"),
            ratio_pct2_usd: lazy_usd!(&ratio_pct2, "ratio_pct2_usd"),
            ratio_pct1_usd: lazy_usd!(&ratio_pct1, "ratio_pct1_usd"),
            price: None,
            ratio_pct99,
            ratio_pct98,
            ratio_pct95,
            ratio_pct5,
            ratio_pct2,
            ratio_pct1,
        })
    }

    /// Compute all: computes price at height level, then ratio + rest.
    pub(crate) fn compute_all<F>(
        &mut self,
        blocks: &blocks::Vecs,
        prices: &prices::Vecs,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
        mut compute: F,
    ) -> Result<()>
    where
        F: FnMut(&mut EagerVec<PcoVec<Height, Dollars>>) -> Result<()>,
    {
        compute(&mut self.price.as_mut().unwrap().usd.height)?;

        let price_opt: Option<&EagerVec<PcoVec<Height, Dollars>>> = None;
        self.compute_rest(blocks, prices, starting_indexes, exit, price_opt)
    }

    /// Compute ratio and derived metrics from an externally-provided or internal price.
    pub(crate) fn compute_rest(
        &mut self,
        blocks: &blocks::Vecs,
        prices: &prices::Vecs,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
        price_opt: Option<&impl ReadableVec<Height, Dollars>>,
    ) -> Result<()> {
        let close_price = &prices.usd.price;

        let price = price_opt.unwrap_or_else(|| unsafe {
            std::mem::transmute(&self.price.as_ref().unwrap().usd.height)
        });

        // Compute ratio = close_price / metric_price at height level
        self.ratio.height.compute_transform2(
            starting_indexes.height,
            close_price,
            price,
            |(i, close, price, ..)| {
                if price == Dollars::ZERO {
                    (i, StoredF32::from(1.0))
                } else {
                    (i, StoredF32::from(close / price))
                }
            },
            exit,
        )?;

        if self.ratio_1w_sma.is_none() {
            return Ok(());
        }

        // SMA using lookback vecs
        self.ratio_1w_sma
            .as_mut()
            .unwrap()
            .height
            .compute_rolling_average(
                starting_indexes.height,
                &blocks.count.height_1w_ago,
                &self.ratio.height,
                exit,
            )?;

        self.ratio_1m_sma
            .as_mut()
            .unwrap()
            .height
            .compute_rolling_average(
                starting_indexes.height,
                &blocks.count.height_1m_ago,
                &self.ratio.height,
                exit,
            )?;

        // Percentiles: insert into sorted array on day boundaries
        let ratio_version = self.ratio.height.version();
        self.mut_ratio_vecs()
            .iter_mut()
            .try_for_each(|v| -> Result<()> {
                v.validate_computed_version_or_reset(ratio_version)?;
                Ok(())
            })?;

        let starting_height = self
            .mut_ratio_vecs()
            .iter()
            .map(|v| Height::from(v.len()))
            .min()
            .unwrap()
            .min(starting_indexes.height);

        let start = starting_height.to_usize();
        let day_start = &blocks.count.height_24h_ago;

        // Collect sorted history up to starting point (one per day boundary)
        let mut sorted = {
            let ratio_data = self.ratio.height.collect_range_at(0, start);
            let day_start_hist = day_start.collect_range_at(0, start);
            let mut sorted: Vec<StoredF32> = Vec::new();
            let mut last_day_start = Height::from(0_usize);
            for (h, ratio) in ratio_data.into_iter().enumerate() {
                let cur_day_start = day_start_hist[h];
                if h == 0 || cur_day_start != last_day_start {
                    let pos = sorted.binary_search(&ratio).unwrap_or_else(|p| p);
                    sorted.insert(pos, ratio);
                    last_day_start = cur_day_start;
                }
            }
            sorted
        };

        let pct1_vec = &mut self.ratio_pct1.as_mut().unwrap().height;
        let pct2_vec = &mut self.ratio_pct2.as_mut().unwrap().height;
        let pct5_vec = &mut self.ratio_pct5.as_mut().unwrap().height;
        let pct95_vec = &mut self.ratio_pct95.as_mut().unwrap().height;
        let pct98_vec = &mut self.ratio_pct98.as_mut().unwrap().height;
        let pct99_vec = &mut self.ratio_pct99.as_mut().unwrap().height;

        let ratio_len = self.ratio.height.len();
        let ratio_data = self.ratio.height.collect_range_at(start, ratio_len);
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

            // Insert into sorted history on day boundaries
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
                .into_iter()
                .try_for_each(|v| v.flush())?;
        }

        // Compute stddev at height level
        macro_rules! compute_sd {
            ($($field:ident),*) => {
                $(self.$field.as_mut().unwrap().compute_all(
                    blocks, starting_indexes, exit, &self.ratio.height,
                )?;)*
            };
        }
        compute_sd!(ratio_sd, ratio_4y_sd, ratio_2y_sd, ratio_1y_sd);

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

        compute_band!(ratio_pct99_usd, ratio_pct99);
        compute_band!(ratio_pct98_usd, ratio_pct98);
        compute_band!(ratio_pct95_usd, ratio_pct95);
        compute_band!(ratio_pct5_usd, ratio_pct5);
        compute_band!(ratio_pct2_usd, ratio_pct2);
        compute_band!(ratio_pct1_usd, ratio_pct1);

        // Stddev USD bands
        macro_rules! compute_sd_usd {
            ($($field:ident),*) => {
                $(if let Some(sd) = self.$field.as_mut() {
                    sd.compute_usd_bands(starting_indexes, metric_price, exit)?;
                })*
            };
        }
        compute_sd_usd!(ratio_sd, ratio_4y_sd, ratio_2y_sd, ratio_1y_sd);

        Ok(())
    }

    fn mut_ratio_vecs(&mut self) -> Vec<&mut EagerVec<PcoVec<Height, StoredF32>>> {
        macro_rules! collect_vecs {
            ($($field:ident),*) => {{
                let mut vecs = Vec::with_capacity(6);
                $(if let Some(v) = self.$field.as_mut() { vecs.push(&mut v.height); })*
                vecs
            }};
        }
        collect_vecs!(
            ratio_pct1,
            ratio_pct2,
            ratio_pct5,
            ratio_pct95,
            ratio_pct98,
            ratio_pct99
        )
    }
}
