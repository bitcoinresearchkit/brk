use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{BasisPoints32, Cents, Height, Indexes, StoredF32, Version};
use vecdb::{
    AnyStoredVec, AnyVec, Database, EagerVec, Exit, PcoVec, ReadableVec, Rw, StorageMode, VecIndex,
    WritableVec,
};

use crate::{
    blocks, indexes,
    internal::{ExpandingPercentiles, Price, PriceTimesRatioBp32Cents},
};

use super::{super::ComputedFromHeight, ComputedFromHeightRatio};

#[derive(Traversable)]
pub struct ComputedFromHeightRatioPercentiles<M: StorageMode = Rw> {
    pub ratio_sma_1w: ComputedFromHeightRatio<M>,
    pub ratio_sma_1m: ComputedFromHeightRatio<M>,
    pub ratio_pct99: ComputedFromHeightRatio<M>,
    pub ratio_pct98: ComputedFromHeightRatio<M>,
    pub ratio_pct95: ComputedFromHeightRatio<M>,
    pub ratio_pct5: ComputedFromHeightRatio<M>,
    pub ratio_pct2: ComputedFromHeightRatio<M>,
    pub ratio_pct1: ComputedFromHeightRatio<M>,
    pub ratio_pct99_price: Price<ComputedFromHeight<Cents, M>>,
    pub ratio_pct98_price: Price<ComputedFromHeight<Cents, M>>,
    pub ratio_pct95_price: Price<ComputedFromHeight<Cents, M>>,
    pub ratio_pct5_price: Price<ComputedFromHeight<Cents, M>>,
    pub ratio_pct2_price: Price<ComputedFromHeight<Cents, M>>,
    pub ratio_pct1_price: Price<ComputedFromHeight<Cents, M>>,

    #[traversable(skip)]
    expanding_pct: ExpandingPercentiles,
}

const VERSION: Version = Version::new(4);

impl ComputedFromHeightRatioPercentiles {
    pub(crate) fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let v = version + VERSION;

        macro_rules! import_ratio {
            ($suffix:expr) => {
                ComputedFromHeightRatio::forced_import_raw(
                    db,
                    &format!("{name}_{}", $suffix),
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
            ratio_sma_1w: import_ratio!("ratio_sma_1w"),
            ratio_sma_1m: import_ratio!("ratio_sma_1m"),
            ratio_pct99: import_ratio!("ratio_pct99"),
            ratio_pct98: import_ratio!("ratio_pct98"),
            ratio_pct95: import_ratio!("ratio_pct95"),
            ratio_pct5: import_ratio!("ratio_pct5"),
            ratio_pct2: import_ratio!("ratio_pct2"),
            ratio_pct1: import_ratio!("ratio_pct1"),
            ratio_pct99_price: import_price!("ratio_pct99"),
            ratio_pct98_price: import_price!("ratio_pct98"),
            ratio_pct95_price: import_price!("ratio_pct95"),
            ratio_pct5_price: import_price!("ratio_pct5"),
            ratio_pct2_price: import_price!("ratio_pct2"),
            ratio_pct1_price: import_price!("ratio_pct1"),
            expanding_pct: ExpandingPercentiles::default(),
        })
    }

    pub(crate) fn compute(
        &mut self,
        blocks: &blocks::Vecs,
        starting_indexes: &Indexes,
        exit: &Exit,
        ratio_source: &impl ReadableVec<Height, StoredF32>,
        metric_price: &impl ReadableVec<Height, Cents>,
    ) -> Result<()> {
        self.ratio_sma_1w.bps.height.compute_rolling_average(
            starting_indexes.height,
            &blocks.lookback.height_1w_ago,
            ratio_source,
            exit,
        )?;

        self.ratio_sma_1m.bps.height.compute_rolling_average(
            starting_indexes.height,
            &blocks.lookback.height_1m_ago,
            ratio_source,
            exit,
        )?;

        let ratio_version = ratio_source.version();
        self.mut_pct_vecs().try_for_each(|v| -> Result<()> {
            v.validate_computed_version_or_reset(ratio_version)?;
            Ok(())
        })?;

        let starting_height = self
            .mut_pct_vecs()
            .map(|v| Height::from(v.len()))
            .min()
            .unwrap()
            .min(starting_indexes.height);

        let start = starting_height.to_usize();
        let ratio_len = ratio_source.len();

        if ratio_len > start {
            let pct_count = self.expanding_pct.count() as usize;
            if pct_count != start {
                self.expanding_pct.reset();
                if start > 0 {
                    let historical = ratio_source.collect_range_at(0, start);
                    self.expanding_pct.add_bulk(&historical);
                }
            }

            let new_ratios = ratio_source.collect_range_at(start, ratio_len);
            let mut pct_vecs: [&mut EagerVec<PcoVec<Height, BasisPoints32>>; 6] = [
                &mut self.ratio_pct1.bps.height,
                &mut self.ratio_pct2.bps.height,
                &mut self.ratio_pct5.bps.height,
                &mut self.ratio_pct95.bps.height,
                &mut self.ratio_pct98.bps.height,
                &mut self.ratio_pct99.bps.height,
            ];
            const PCTS: [f64; 6] = [0.01, 0.02, 0.05, 0.95, 0.98, 0.99];
            let mut out = [0u32; 6];

            for (offset, &ratio) in new_ratios.iter().enumerate() {
                self.expanding_pct.add(*ratio);
                self.expanding_pct.quantiles(&PCTS, &mut out);
                let idx = start + offset;
                for (vec, &val) in pct_vecs.iter_mut().zip(out.iter()) {
                    vec.truncate_push_at(idx, BasisPoints32::from(val))?;
                }
            }
        }

        {
            let _lock = exit.lock();
            self.mut_pct_vecs().try_for_each(|v| v.flush())?;
        }

        // Cents bands
        macro_rules! compute_band {
            ($usd_field:ident, $band_source:expr) => {
                self.$usd_field
                    .cents
                    .compute_binary::<Cents, BasisPoints32, PriceTimesRatioBp32Cents>(
                        starting_indexes.height,
                        metric_price,
                        $band_source,
                        exit,
                    )?;
            };
        }

        compute_band!(ratio_pct99_price, &self.ratio_pct99.bps.height);
        compute_band!(ratio_pct98_price, &self.ratio_pct98.bps.height);
        compute_band!(ratio_pct95_price, &self.ratio_pct95.bps.height);
        compute_band!(ratio_pct5_price, &self.ratio_pct5.bps.height);
        compute_band!(ratio_pct2_price, &self.ratio_pct2.bps.height);
        compute_band!(ratio_pct1_price, &self.ratio_pct1.bps.height);

        Ok(())
    }

    fn mut_pct_vecs(
        &mut self,
    ) -> impl Iterator<Item = &mut EagerVec<PcoVec<Height, BasisPoints32>>> {
        [
            &mut self.ratio_pct1.bps.height,
            &mut self.ratio_pct2.bps.height,
            &mut self.ratio_pct5.bps.height,
            &mut self.ratio_pct95.bps.height,
            &mut self.ratio_pct98.bps.height,
            &mut self.ratio_pct99.bps.height,
        ]
        .into_iter()
    }
}
