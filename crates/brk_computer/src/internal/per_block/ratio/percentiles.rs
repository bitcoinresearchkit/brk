use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{BasisPoints32, Cents, Height, Indexes, StoredF32, Version};
use vecdb::{
    AnyStoredVec, AnyVec, Database, EagerVec, Exit, PcoVec, ReadableVec, Rw, StorageMode, VecIndex,
    WritableVec,
};

use crate::{
    indexes,
    internal::{Price, PriceTimesRatioBp32Cents, algo::ExpandingPercentiles},
};

use super::{super::PerBlock, RatioPerBlock};

#[derive(Traversable)]
pub struct RatioBand<M: StorageMode = Rw> {
    #[traversable(flatten)]
    pub ratio: RatioPerBlock<BasisPoints32, M>,
    pub price: Price<PerBlock<Cents, M>>,
}

#[derive(Traversable)]
pub struct RatioPerBlockPercentiles<M: StorageMode = Rw> {
    pub pct99_5: RatioBand<M>,
    pub pct99: RatioBand<M>,
    pub pct98: RatioBand<M>,
    pub pct95: RatioBand<M>,
    pub pct5: RatioBand<M>,
    pub pct2: RatioBand<M>,
    pub pct1: RatioBand<M>,
    pub pct0_5: RatioBand<M>,

    #[traversable(skip)]
    expanding_pct: ExpandingPercentiles,
}

const VERSION: Version = Version::new(6);

/// First height included in ratio percentile computation (first halving).
/// Earlier blocks lack meaningful market data and pollute the distribution.
const MIN_HEIGHT: usize = 210_000;

impl RatioPerBlockPercentiles {
    pub(crate) fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let v = version + VERSION;

        macro_rules! import_ratio {
            ($suffix:expr) => {
                RatioPerBlock::forced_import_raw(db, &format!("{name}_{}", $suffix), v, indexes)?
            };
        }

        macro_rules! import_price {
            ($suffix:expr) => {
                Price::forced_import(db, &format!("{name}_{}", $suffix), v, indexes)?
            };
        }

        macro_rules! import_band {
            ($pct:expr) => {
                RatioBand {
                    ratio: import_ratio!(concat!("ratio_", $pct)),
                    price: import_price!($pct),
                }
            };
        }

        Ok(Self {
            pct99_5: import_band!("pct99_5"),
            pct99: import_band!("pct99"),
            pct98: import_band!("pct98"),
            pct95: import_band!("pct95"),
            pct5: import_band!("pct5"),
            pct2: import_band!("pct2"),
            pct1: import_band!("pct1"),
            pct0_5: import_band!("pct0_5"),
            expanding_pct: ExpandingPercentiles::default(),
        })
    }

    pub(crate) fn compute(
        &mut self,
        starting_indexes: &Indexes,
        exit: &Exit,
        ratio_source: &impl ReadableVec<Height, StoredF32>,
        series_price: &impl ReadableVec<Height, Cents>,
    ) -> Result<()> {
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
            let expected_count = start.saturating_sub(MIN_HEIGHT);
            if self.expanding_pct.count() as usize != expected_count {
                self.expanding_pct.reset();
                if start > MIN_HEIGHT {
                    let historical = ratio_source.collect_range_at(MIN_HEIGHT, start);
                    self.expanding_pct.add_bulk(&historical);
                }
            }

            let new_ratios = ratio_source.collect_range_at(start, ratio_len);
            let mut pct_vecs: [&mut EagerVec<PcoVec<Height, BasisPoints32>>; 8] = [
                &mut self.pct0_5.ratio.bps.height,
                &mut self.pct1.ratio.bps.height,
                &mut self.pct2.ratio.bps.height,
                &mut self.pct5.ratio.bps.height,
                &mut self.pct95.ratio.bps.height,
                &mut self.pct98.ratio.bps.height,
                &mut self.pct99.ratio.bps.height,
                &mut self.pct99_5.ratio.bps.height,
            ];
            const PCTS: [f64; 8] = [0.005, 0.01, 0.02, 0.05, 0.95, 0.98, 0.99, 0.995];
            let mut out = [0u32; 8];

            for vec in pct_vecs.iter_mut() {
                vec.truncate_if_needed_at(start)?;
            }

            for (i, &ratio) in new_ratios.iter().enumerate() {
                if start + i >= MIN_HEIGHT {
                    self.expanding_pct.add(*ratio);
                }
                self.expanding_pct.quantiles(&PCTS, &mut out);
                for (vec, &val) in pct_vecs.iter_mut().zip(out.iter()) {
                    vec.push(BasisPoints32::from(val));
                }
            }
        }

        {
            let _lock = exit.lock();
            self.mut_pct_vecs().try_for_each(|v| v.write().map(|_| ()))?;
        }

        // Cents bands
        macro_rules! compute_band {
            ($band:ident) => {
                self.$band
                    .price
                    .cents
                    .compute_binary::<Cents, BasisPoints32, PriceTimesRatioBp32Cents>(
                        starting_indexes.height,
                        series_price,
                        &self.$band.ratio.bps.height,
                        exit,
                    )?;
            };
        }

        compute_band!(pct99_5);
        compute_band!(pct99);
        compute_band!(pct98);
        compute_band!(pct95);
        compute_band!(pct5);
        compute_band!(pct2);
        compute_band!(pct1);
        compute_band!(pct0_5);

        Ok(())
    }

    fn mut_pct_vecs(
        &mut self,
    ) -> impl Iterator<Item = &mut EagerVec<PcoVec<Height, BasisPoints32>>> {
        [
            &mut self.pct0_5.ratio.bps.height,
            &mut self.pct1.ratio.bps.height,
            &mut self.pct2.ratio.bps.height,
            &mut self.pct5.ratio.bps.height,
            &mut self.pct95.ratio.bps.height,
            &mut self.pct98.ratio.bps.height,
            &mut self.pct99.ratio.bps.height,
            &mut self.pct99_5.ratio.bps.height,
        ]
        .into_iter()
    }
}
