use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Dollars, Height, StoredF32, Version};
use vecdb::{
    AnyStoredVec, AnyVec, Database, EagerVec, Exit, PcoVec, ReadableVec, Rw, StorageMode, VecIndex,
    WritableVec,
};

use crate::{ComputeIndexes, blocks, indexes};

use crate::internal::{ComputedFromHeightLast, Price};

use super::ComputedFromHeightStdDev;

#[derive(Traversable)]
pub struct ComputedFromHeightStdDevExtended<M: StorageMode = Rw> {
    #[traversable(flatten)]
    pub base: ComputedFromHeightStdDev<M>,

    pub zscore: ComputedFromHeightLast<StoredF32, M>,

    pub p0_5sd: ComputedFromHeightLast<StoredF32, M>,
    pub p1sd: ComputedFromHeightLast<StoredF32, M>,
    pub p1_5sd: ComputedFromHeightLast<StoredF32, M>,
    pub p2sd: ComputedFromHeightLast<StoredF32, M>,
    pub p2_5sd: ComputedFromHeightLast<StoredF32, M>,
    pub p3sd: ComputedFromHeightLast<StoredF32, M>,
    pub m0_5sd: ComputedFromHeightLast<StoredF32, M>,
    pub m1sd: ComputedFromHeightLast<StoredF32, M>,
    pub m1_5sd: ComputedFromHeightLast<StoredF32, M>,
    pub m2sd: ComputedFromHeightLast<StoredF32, M>,
    pub m2_5sd: ComputedFromHeightLast<StoredF32, M>,
    pub m3sd: ComputedFromHeightLast<StoredF32, M>,

    pub _0sd_usd: Price<ComputedFromHeightLast<Dollars, M>>,
    pub p0_5sd_usd: Price<ComputedFromHeightLast<Dollars, M>>,
    pub p1sd_usd: Price<ComputedFromHeightLast<Dollars, M>>,
    pub p1_5sd_usd: Price<ComputedFromHeightLast<Dollars, M>>,
    pub p2sd_usd: Price<ComputedFromHeightLast<Dollars, M>>,
    pub p2_5sd_usd: Price<ComputedFromHeightLast<Dollars, M>>,
    pub p3sd_usd: Price<ComputedFromHeightLast<Dollars, M>>,
    pub m0_5sd_usd: Price<ComputedFromHeightLast<Dollars, M>>,
    pub m1sd_usd: Price<ComputedFromHeightLast<Dollars, M>>,
    pub m1_5sd_usd: Price<ComputedFromHeightLast<Dollars, M>>,
    pub m2sd_usd: Price<ComputedFromHeightLast<Dollars, M>>,
    pub m2_5sd_usd: Price<ComputedFromHeightLast<Dollars, M>>,
    pub m3sd_usd: Price<ComputedFromHeightLast<Dollars, M>>,
}

impl ComputedFromHeightStdDevExtended {
    pub(crate) fn forced_import(
        db: &Database,
        name: &str,
        days: usize,
        parent_version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let version = parent_version + Version::TWO;

        macro_rules! import {
            ($suffix:expr) => {
                ComputedFromHeightLast::forced_import(
                    db,
                    &format!("{name}_{}", $suffix),
                    version,
                    indexes,
                )?
            };
        }

        macro_rules! import_usd {
            ($suffix:expr) => {
                Price::forced_import(db, &format!("{name}_{}", $suffix), version, indexes)?
            };
        }

        Ok(Self {
            base: ComputedFromHeightStdDev::forced_import(db, name, days, parent_version, indexes)?,
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
            _0sd_usd: import_usd!("0sd_usd"),
            p0_5sd_usd: import_usd!("p0_5sd_usd"),
            p1sd_usd: import_usd!("p1sd_usd"),
            p1_5sd_usd: import_usd!("p1_5sd_usd"),
            p2sd_usd: import_usd!("p2sd_usd"),
            p2_5sd_usd: import_usd!("p2_5sd_usd"),
            p3sd_usd: import_usd!("p3sd_usd"),
            m0_5sd_usd: import_usd!("m0_5sd_usd"),
            m1sd_usd: import_usd!("m1sd_usd"),
            m1_5sd_usd: import_usd!("m1_5sd_usd"),
            m2sd_usd: import_usd!("m2sd_usd"),
            m2_5sd_usd: import_usd!("m2_5sd_usd"),
            m3sd_usd: import_usd!("m3sd_usd"),
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

        for (offset, _ratio) in source_data.into_iter().enumerate() {
            let index = start + offset;
            let average = sma_data[offset];
            let sd = sd_data[offset];

            self.p0_5sd
                .height
                .truncate_push_at(index, average + StoredF32::from(0.5 * *sd))?;
            self.p1sd.height.truncate_push_at(index, average + sd)?;
            self.p1_5sd
                .height
                .truncate_push_at(index, average + StoredF32::from(1.5 * *sd))?;
            self.p2sd.height.truncate_push_at(index, average + 2 * sd)?;
            self.p2_5sd
                .height
                .truncate_push_at(index, average + StoredF32::from(2.5 * *sd))?;
            self.p3sd.height.truncate_push_at(index, average + 3 * sd)?;
            self.m0_5sd
                .height
                .truncate_push_at(index, average - StoredF32::from(0.5 * *sd))?;
            self.m1sd.height.truncate_push_at(index, average - sd)?;
            self.m1_5sd
                .height
                .truncate_push_at(index, average - StoredF32::from(1.5 * *sd))?;
            self.m2sd.height.truncate_push_at(index, average - 2 * sd)?;
            self.m2_5sd
                .height
                .truncate_push_at(index, average - StoredF32::from(2.5 * *sd))?;
            self.m3sd.height.truncate_push_at(index, average - 3 * sd)?;
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

    /// Compute USD price bands: usd_band = metric_price * band_ratio
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

        compute_band!(_0sd_usd, &self.base.sma.height);
        compute_band!(p0_5sd_usd, &self.p0_5sd.height);
        compute_band!(p1sd_usd, &self.p1sd.height);
        compute_band!(p1_5sd_usd, &self.p1_5sd.height);
        compute_band!(p2sd_usd, &self.p2sd.height);
        compute_band!(p2_5sd_usd, &self.p2_5sd.height);
        compute_band!(p3sd_usd, &self.p3sd.height);
        compute_band!(m0_5sd_usd, &self.m0_5sd.height);
        compute_band!(m1sd_usd, &self.m1sd.height);
        compute_band!(m1_5sd_usd, &self.m1_5sd.height);
        compute_band!(m2sd_usd, &self.m2sd.height);
        compute_band!(m2_5sd_usd, &self.m2_5sd.height);
        compute_band!(m3sd_usd, &self.m3sd.height);

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
