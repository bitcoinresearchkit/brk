use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Cents, Height, StoredF32, Version};
use vecdb::{AnyStoredVec, AnyVec, Database, EagerVec, Exit, PcoVec, ReadableVec, Rw, StorageMode, VecIndex, WritableVec};

use crate::{
    ComputeIndexes, blocks, indexes,
    internal::{ComputedFromHeightStdDevExtended, Price, TDigest},
};

use super::super::ComputedFromHeight;

#[derive(Traversable)]
pub struct ComputedFromHeightRatioExtension<M: StorageMode = Rw> {
    pub ratio_1w_sma: ComputedFromHeight<StoredF32, M>,
    pub ratio_1m_sma: ComputedFromHeight<StoredF32, M>,
    pub ratio_pct99: ComputedFromHeight<StoredF32, M>,
    pub ratio_pct98: ComputedFromHeight<StoredF32, M>,
    pub ratio_pct95: ComputedFromHeight<StoredF32, M>,
    pub ratio_pct5: ComputedFromHeight<StoredF32, M>,
    pub ratio_pct2: ComputedFromHeight<StoredF32, M>,
    pub ratio_pct1: ComputedFromHeight<StoredF32, M>,
    pub ratio_pct99_price: Price<ComputedFromHeight<Cents, M>>,
    pub ratio_pct98_price: Price<ComputedFromHeight<Cents, M>>,
    pub ratio_pct95_price: Price<ComputedFromHeight<Cents, M>>,
    pub ratio_pct5_price: Price<ComputedFromHeight<Cents, M>>,
    pub ratio_pct2_price: Price<ComputedFromHeight<Cents, M>>,
    pub ratio_pct1_price: Price<ComputedFromHeight<Cents, M>>,

    pub ratio_sd: ComputedFromHeightStdDevExtended<M>,
    pub ratio_4y_sd: ComputedFromHeightStdDevExtended<M>,
    pub ratio_2y_sd: ComputedFromHeightStdDevExtended<M>,
    pub ratio_1y_sd: ComputedFromHeightStdDevExtended<M>,

    #[traversable(skip)]
    tdigest: TDigest,
}

const VERSION: Version = Version::new(4);

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
                ComputedFromHeight::forced_import(
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

        macro_rules! import_price {
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
            ratio_pct99_price: import_price!("ratio_pct99"),
            ratio_pct98_price: import_price!("ratio_pct98"),
            ratio_pct95_price: import_price!("ratio_pct95"),
            ratio_pct5_price: import_price!("ratio_pct5"),
            ratio_pct2_price: import_price!("ratio_pct2"),
            ratio_pct1_price: import_price!("ratio_pct1"),
            tdigest: TDigest::default(),
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
        let ratio_len = ratio_source.len();

        if ratio_len > start {
            let tdigest_count = self.tdigest.count() as usize;
            if tdigest_count != start {
                self.tdigest.reset();
                if start > 0 {
                    let historical = ratio_source.collect_range_at(0, start);
                    for &v in &historical {
                        self.tdigest.add(*v as f64);
                    }
                }
            }

            // Process new blocks [start, ratio_len)
            let new_ratios = ratio_source.collect_range_at(start, ratio_len);
            let mut pct_vecs: [&mut EagerVec<PcoVec<Height, StoredF32>>; 6] = [
                &mut self.ratio_pct1.height,
                &mut self.ratio_pct2.height,
                &mut self.ratio_pct5.height,
                &mut self.ratio_pct95.height,
                &mut self.ratio_pct98.height,
                &mut self.ratio_pct99.height,
            ];
            const PCTS: [f64; 6] = [0.01, 0.02, 0.05, 0.95, 0.98, 0.99];
            let mut out = [0.0f64; 6];

            for (offset, &ratio) in new_ratios.iter().enumerate() {
                self.tdigest.add(*ratio as f64);
                self.tdigest.quantiles(&PCTS, &mut out);
                let idx = start + offset;
                for (vec, &val) in pct_vecs.iter_mut().zip(out.iter()) {
                    vec.truncate_push_at(idx, StoredF32::from(val as f32))?;
                }
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

    /// Compute cents ratio bands: cents_band = metric_price_cents * ratio_percentile
    pub(crate) fn compute_cents_bands(
        &mut self,
        starting_indexes: &ComputeIndexes,
        metric_price: &impl ReadableVec<Height, Cents>,
        exit: &Exit,
    ) -> Result<()> {
        use crate::internal::PriceTimesRatioCents;

        macro_rules! compute_band {
            ($usd_field:ident, $band_source:expr) => {
                self.$usd_field
                    .cents
                    .compute_binary::<Cents, StoredF32, PriceTimesRatioCents>(
                        starting_indexes.height,
                        metric_price,
                        $band_source,
                        exit,
                    )?;
            };
        }

        compute_band!(ratio_pct99_price, &self.ratio_pct99.height);
        compute_band!(ratio_pct98_price, &self.ratio_pct98.height);
        compute_band!(ratio_pct95_price, &self.ratio_pct95.height);
        compute_band!(ratio_pct5_price, &self.ratio_pct5.height);
        compute_band!(ratio_pct2_price, &self.ratio_pct2.height);
        compute_band!(ratio_pct1_price, &self.ratio_pct1.height);

        // Stddev cents bands
        self.ratio_sd
            .compute_cents_bands(starting_indexes, metric_price, exit)?;
        self.ratio_4y_sd
            .compute_cents_bands(starting_indexes, metric_price, exit)?;
        self.ratio_2y_sd
            .compute_cents_bands(starting_indexes, metric_price, exit)?;
        self.ratio_1y_sd
            .compute_cents_bands(starting_indexes, metric_price, exit)?;

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
