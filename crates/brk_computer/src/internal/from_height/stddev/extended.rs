use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Cents, Height, StoredF32, Version};
use vecdb::{
    AnyStoredVec, AnyVec, Database, EagerVec, Exit, PcoVec, ReadableVec, Rw, StorageMode, VecIndex,
    WritableVec,
};

use crate::{ComputeIndexes, blocks, indexes};

use crate::internal::{ComputedFromHeight, Price};

use super::ComputedFromHeightStdDev;

#[derive(Traversable)]
pub struct ComputedFromHeightStdDevExtended<M: StorageMode = Rw> {
    #[traversable(flatten)]
    pub base: ComputedFromHeightStdDev<M>,

    pub zscore: ComputedFromHeight<StoredF32, M>,

    pub p0_5sd: ComputedFromHeight<StoredF32, M>,
    pub p1sd: ComputedFromHeight<StoredF32, M>,
    pub p1_5sd: ComputedFromHeight<StoredF32, M>,
    pub p2sd: ComputedFromHeight<StoredF32, M>,
    pub p2_5sd: ComputedFromHeight<StoredF32, M>,
    pub p3sd: ComputedFromHeight<StoredF32, M>,
    pub m0_5sd: ComputedFromHeight<StoredF32, M>,
    pub m1sd: ComputedFromHeight<StoredF32, M>,
    pub m1_5sd: ComputedFromHeight<StoredF32, M>,
    pub m2sd: ComputedFromHeight<StoredF32, M>,
    pub m2_5sd: ComputedFromHeight<StoredF32, M>,
    pub m3sd: ComputedFromHeight<StoredF32, M>,

    pub _0sd_price: Price<ComputedFromHeight<Cents, M>>,
    pub p0_5sd_price: Price<ComputedFromHeight<Cents, M>>,
    pub p1sd_price: Price<ComputedFromHeight<Cents, M>>,
    pub p1_5sd_price: Price<ComputedFromHeight<Cents, M>>,
    pub p2sd_price: Price<ComputedFromHeight<Cents, M>>,
    pub p2_5sd_price: Price<ComputedFromHeight<Cents, M>>,
    pub p3sd_price: Price<ComputedFromHeight<Cents, M>>,
    pub m0_5sd_price: Price<ComputedFromHeight<Cents, M>>,
    pub m1sd_price: Price<ComputedFromHeight<Cents, M>>,
    pub m1_5sd_price: Price<ComputedFromHeight<Cents, M>>,
    pub m2sd_price: Price<ComputedFromHeight<Cents, M>>,
    pub m2_5sd_price: Price<ComputedFromHeight<Cents, M>>,
    pub m3sd_price: Price<ComputedFromHeight<Cents, M>>,
}

impl ComputedFromHeightStdDevExtended {
    pub(crate) fn forced_import(
        db: &Database,
        name: &str,
        period: &str,
        days: usize,
        parent_version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let version = parent_version + Version::TWO;
        let p = super::period_suffix(period);

        macro_rules! import {
            ($suffix:expr) => {
                ComputedFromHeight::forced_import(
                    db,
                    &format!("{name}_{}{p}", $suffix),
                    version,
                    indexes,
                )?
            };
        }

        macro_rules! import_price {
            ($suffix:expr) => {
                Price::forced_import(db, &format!("{name}_{}{p}", $suffix), version, indexes)?
            };
        }

        Ok(Self {
            base: ComputedFromHeightStdDev::forced_import(db, name, period, days, parent_version, indexes)?,
            zscore: import!("zscore"),
            p0_5sd: import!("p0_5sd"),
            p1sd: import!("p1sd"),
            p1_5sd: import!("p1_5sd"),
            p2sd: import!("p2sd"),
            p2_5sd: import!("p2_5sd"),
            p3sd: import!("p3sd"),
            m0_5sd: import!("m0_5sd"),
            m1sd: import!("m1sd"),
            m1_5sd: import!("m1_5sd"),
            m2sd: import!("m2sd"),
            m2_5sd: import!("m2_5sd"),
            m3sd: import!("m3sd"),
            _0sd_price: import_price!("0sd"),
            p0_5sd_price: import_price!("p0_5sd"),
            p1sd_price: import_price!("p1sd"),
            p1_5sd_price: import_price!("p1_5sd"),
            p2sd_price: import_price!("p2sd"),
            p2_5sd_price: import_price!("p2_5sd"),
            p3sd_price: import_price!("p3sd"),
            m0_5sd_price: import_price!("m0_5sd"),
            m1sd_price: import_price!("m1sd"),
            m1_5sd_price: import_price!("m1_5sd"),
            m2sd_price: import_price!("m2sd"),
            m2_5sd_price: import_price!("m2_5sd"),
            m3sd_price: import_price!("m3sd"),
        })
    }

    pub(crate) fn compute_all(
        &mut self,
        blocks: &blocks::Vecs,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
        source: &impl ReadableVec<Height, StoredF32>,
    ) -> Result<()> {
        self.base
            .compute_all(blocks, starting_indexes, exit, source)?;

        let sma_opt: Option<&EagerVec<PcoVec<Height, StoredF32>>> = None;
        self.compute_bands(starting_indexes, exit, sma_opt, source)
    }

    pub(crate) fn compute_bands(
        &mut self,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
        sma_opt: Option<&impl ReadableVec<Height, StoredF32>>,
        source: &impl ReadableVec<Height, StoredF32>,
    ) -> Result<()> {
        let source_version = source.version();

        self.mut_band_height_vecs()
            .try_for_each(|v| -> Result<()> {
                v.validate_computed_version_or_reset(source_version)?;
                Ok(())
            })?;

        let starting_height = self
            .mut_band_height_vecs()
            .map(|v| Height::from(v.len()))
            .min()
            .unwrap()
            .min(starting_indexes.height);

        let start = starting_height.to_usize();

        let source_len = source.len();
        let source_data = source.collect_range_at(start, source_len);

        let sma_len = sma_opt
            .map(|s| s.len())
            .unwrap_or(self.base.sma.height.len());
        let sma_data: Vec<StoredF32> = if let Some(sma) = sma_opt {
            sma.collect_range_at(start, sma_len)
        } else {
            self.base.sma.height.collect_range_at(start, sma_len)
        };
        let sd_data = self
            .base
            .sd
            .height
            .collect_range_at(start, self.base.sd.height.len());

        const MULTIPLIERS: [f32; 12] = [0.5, 1.0, 1.5, 2.0, 2.5, 3.0, -0.5, -1.0, -1.5, -2.0, -2.5, -3.0];
        let band_vecs: Vec<_> = self.mut_band_height_vecs().collect();
        for (vec, mult) in band_vecs.into_iter().zip(MULTIPLIERS) {
            for (offset, _) in source_data.iter().enumerate() {
                let index = start + offset;
                let average = sma_data[offset];
                let sd = sd_data[offset];
                vec.truncate_push_at(index, average + StoredF32::from(mult * *sd))?;
            }
        }

        {
            let _lock = exit.lock();
            self.mut_band_height_vecs().try_for_each(|v| v.flush())?;
        }

        if let Some(sma) = sma_opt {
            self.zscore.height.compute_zscore(
                starting_indexes.height,
                source,
                sma,
                &self.base.sd.height,
                exit,
            )?;
        } else {
            self.zscore.height.compute_zscore(
                starting_indexes.height,
                source,
                &self.base.sma.height,
                &self.base.sd.height,
                exit,
            )?;
        }

        Ok(())
    }

    /// Compute cents price bands: cents_band = metric_price_cents * band_ratio
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

        compute_band!(_0sd_price, &self.base.sma.height);
        compute_band!(p0_5sd_price, &self.p0_5sd.height);
        compute_band!(p1sd_price, &self.p1sd.height);
        compute_band!(p1_5sd_price, &self.p1_5sd.height);
        compute_band!(p2sd_price, &self.p2sd.height);
        compute_band!(p2_5sd_price, &self.p2_5sd.height);
        compute_band!(p3sd_price, &self.p3sd.height);
        compute_band!(m0_5sd_price, &self.m0_5sd.height);
        compute_band!(m1sd_price, &self.m1sd.height);
        compute_band!(m1_5sd_price, &self.m1_5sd.height);
        compute_band!(m2sd_price, &self.m2sd.height);
        compute_band!(m2_5sd_price, &self.m2_5sd.height);
        compute_band!(m3sd_price, &self.m3sd.height);

        Ok(())
    }

    fn mut_band_height_vecs(
        &mut self,
    ) -> impl Iterator<Item = &mut EagerVec<PcoVec<Height, StoredF32>>> {
        [
            &mut self.p0_5sd.height,
            &mut self.p1sd.height,
            &mut self.p1_5sd.height,
            &mut self.p2sd.height,
            &mut self.p2_5sd.height,
            &mut self.p3sd.height,
            &mut self.m0_5sd.height,
            &mut self.m1sd.height,
            &mut self.m1_5sd.height,
            &mut self.m2sd.height,
            &mut self.m2_5sd.height,
            &mut self.m3sd.height,
        ]
        .into_iter()
    }
}
