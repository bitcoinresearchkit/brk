use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Cents, Height, Indexes, StoredF32, Version};
use vecdb::{
    AnyStoredVec, AnyVec, Database, EagerVec, Exit, PcoVec, ReadableVec, Rw, StorageMode, VecIndex,
    WritableVec,
};

use crate::{
    blocks, indexes,
    internal::{PerBlock, Price, PriceTimesRatioCents, per_block::stddev::period_suffix},
};

#[derive(Traversable)]
pub struct StdDevBand<M: StorageMode = Rw> {
    #[traversable(flatten)]
    pub ratio: PerBlock<StoredF32, M>,
    pub price: Price<PerBlock<Cents, M>>,
}

#[derive(Traversable)]
pub struct StdDevPerBlockExtended<M: StorageMode = Rw> {
    days: usize,
    pub sd: PerBlock<StoredF32, M>,
    pub zscore: PerBlock<StoredF32, M>,

    pub _0sd: Price<PerBlock<Cents, M>>,
    pub p0_5sd: StdDevBand<M>,
    pub p1sd: StdDevBand<M>,
    pub p1_5sd: StdDevBand<M>,
    pub p2sd: StdDevBand<M>,
    pub p2_5sd: StdDevBand<M>,
    pub p3sd: StdDevBand<M>,
    pub m0_5sd: StdDevBand<M>,
    pub m1sd: StdDevBand<M>,
    pub m1_5sd: StdDevBand<M>,
    pub m2sd: StdDevBand<M>,
    pub m2_5sd: StdDevBand<M>,
    pub m3sd: StdDevBand<M>,
}

impl StdDevPerBlockExtended {
    pub(crate) fn forced_import(
        db: &Database,
        name: &str,
        period: &str,
        days: usize,
        parent_version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let version = parent_version + Version::TWO;
        let p = period_suffix(period);

        macro_rules! import {
            ($suffix:expr) => {
                PerBlock::forced_import(db, &format!("{name}_{}{p}", $suffix), version, indexes)?
            };
        }

        macro_rules! import_price {
            ($suffix:expr) => {
                Price::forced_import(db, &format!("{name}_{}{p}", $suffix), version, indexes)?
            };
        }

        macro_rules! import_band {
            ($suffix:expr) => {{
                StdDevBand {
                    ratio: import!(concat!("ratio_", $suffix)),
                    price: import_price!($suffix),
                }
            }};
        }

        Ok(Self {
            days,
            sd: import!("ratio_sd"),
            zscore: import!("ratio_zscore"),
            _0sd: import_price!("0sd"),
            p0_5sd: import_band!("p0_5sd"),
            p1sd: import_band!("p1sd"),
            p1_5sd: import_band!("p1_5sd"),
            p2sd: import_band!("p2sd"),
            p2_5sd: import_band!("p2_5sd"),
            p3sd: import_band!("p3sd"),
            m0_5sd: import_band!("m0_5sd"),
            m1sd: import_band!("m1sd"),
            m1_5sd: import_band!("m1_5sd"),
            m2sd: import_band!("m2sd"),
            m2_5sd: import_band!("m2_5sd"),
            m3sd: import_band!("m3sd"),
        })
    }

    pub(crate) fn compute_all(
        &mut self,
        blocks: &blocks::Vecs,
        starting_indexes: &Indexes,
        exit: &Exit,
        source: &impl ReadableVec<Height, StoredF32>,
        sma: &impl ReadableVec<Height, StoredF32>,
    ) -> Result<()> {
        if self.days == usize::MAX {
            self.sd
                .height
                .compute_expanding_sd(starting_indexes.height, source, sma, exit)?;
        } else {
            let window_starts = blocks.lookback.start_vec(self.days);
            self.sd.height.compute_rolling_sd(
                starting_indexes.height,
                window_starts,
                source,
                sma,
                exit,
            )?;
        }

        self.compute_bands(starting_indexes, exit, sma, source)
    }

    fn compute_bands(
        &mut self,
        starting_indexes: &Indexes,
        exit: &Exit,
        sma: &impl ReadableVec<Height, StoredF32>,
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

        let sma_data = sma.collect_range_at(start, sma.len());
        let sd_data = self.sd.height.collect_range_at(start, self.sd.height.len());

        const MULTIPLIERS: [f32; 12] = [
            0.5, 1.0, 1.5, 2.0, 2.5, 3.0, -0.5, -1.0, -1.5, -2.0, -2.5, -3.0,
        ];
        for (vec, mult) in self.mut_band_height_vecs().zip(MULTIPLIERS) {
            vec.truncate_if_needed_at(start)?;
            for (offset, _) in source_data.iter().enumerate() {
                let average = sma_data[offset];
                let sd = sd_data[offset];
                vec.push(average + StoredF32::from(mult * *sd));
            }
        }

        {
            let _lock = exit.lock();
            self.mut_band_height_vecs().try_for_each(|v| v.flush())?;
        }

        self.zscore.height.compute_zscore(
            starting_indexes.height,
            source,
            sma,
            &self.sd.height,
            exit,
        )?;

        Ok(())
    }

    pub(crate) fn compute_cents_bands(
        &mut self,
        starting_indexes: &Indexes,
        metric_price: &impl ReadableVec<Height, Cents>,
        sma: &impl ReadableVec<Height, StoredF32>,
        exit: &Exit,
    ) -> Result<()> {
        macro_rules! compute_band_price {
            ($price:expr, $band_source:expr) => {
                $price
                    .cents
                    .compute_binary::<Cents, StoredF32, PriceTimesRatioCents>(
                        starting_indexes.height,
                        metric_price,
                        $band_source,
                        exit,
                    )?;
            };
        }

        compute_band_price!(&mut self._0sd, sma);
        compute_band_price!(&mut self.p0_5sd.price, &self.p0_5sd.ratio.height);
        compute_band_price!(&mut self.p1sd.price, &self.p1sd.ratio.height);
        compute_band_price!(&mut self.p1_5sd.price, &self.p1_5sd.ratio.height);
        compute_band_price!(&mut self.p2sd.price, &self.p2sd.ratio.height);
        compute_band_price!(&mut self.p2_5sd.price, &self.p2_5sd.ratio.height);
        compute_band_price!(&mut self.p3sd.price, &self.p3sd.ratio.height);
        compute_band_price!(&mut self.m0_5sd.price, &self.m0_5sd.ratio.height);
        compute_band_price!(&mut self.m1sd.price, &self.m1sd.ratio.height);
        compute_band_price!(&mut self.m1_5sd.price, &self.m1_5sd.ratio.height);
        compute_band_price!(&mut self.m2sd.price, &self.m2sd.ratio.height);
        compute_band_price!(&mut self.m2_5sd.price, &self.m2_5sd.ratio.height);
        compute_band_price!(&mut self.m3sd.price, &self.m3sd.ratio.height);

        Ok(())
    }

    fn mut_band_height_vecs(
        &mut self,
    ) -> impl Iterator<Item = &mut EagerVec<PcoVec<Height, StoredF32>>> {
        [
            &mut self.p0_5sd.ratio.height,
            &mut self.p1sd.ratio.height,
            &mut self.p1_5sd.ratio.height,
            &mut self.p2sd.ratio.height,
            &mut self.p2_5sd.ratio.height,
            &mut self.p3sd.ratio.height,
            &mut self.m0_5sd.ratio.height,
            &mut self.m1sd.ratio.height,
            &mut self.m1_5sd.ratio.height,
            &mut self.m2sd.ratio.height,
            &mut self.m2_5sd.ratio.height,
            &mut self.m3sd.ratio.height,
        ]
        .into_iter()
    }
}
