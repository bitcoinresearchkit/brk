use brk_error::Result;

use bitview_traversable::Traversable;
use brk_exit::Exit;
use brk_types::{Cents, Height, Sats, Version};
use vecdb::{
    BinaryTransform, Budgeted, Database, ReadOnlyClone, ReadableCloneableVec, ReadableVec, Rw,
    StorageMode, VecIndex, VecValue,
};

use crate::{
    CachePolicy, IndexSources, LazyRollingAvgsAmountFromHeight, LazyRollingSumsAmountFromHeight,
    LazyValueBlock, RollingDistributionValuePerBlock, SatsToCents, ValuePerBlock, WindowStarts,
    Windows,
};

#[derive(Traversable)]
pub struct ValuePerBlockFull<M: StorageMode = Rw, S: CachePolicy = Budgeted> {
    /// Value for the represented block. At time-period indexes, the value is
    /// taken from the period's final block.
    pub block: LazyValueBlock,
    /// Cumulative value through the represented block. At time-period indexes,
    /// the value is taken at the period's final block.
    pub cumulative: ValuePerBlock<M, S>,
    pub sum: LazyRollingSumsAmountFromHeight,
    pub average: LazyRollingAvgsAmountFromHeight,
    #[traversable(flatten)]
    pub distribution: RollingDistributionValuePerBlock<M>,
}

const VERSION: Version = Version::TWO;

impl<S: CachePolicy> ValuePerBlockFull<Rw, S> {
    pub fn cumulative_sats_source(
        &self,
    ) -> &(impl ReadableVec<Height, Sats> + Clone + 'static + use<S>) {
        self.cumulative.sats.resolutions.height_source()
    }

    pub fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &IndexSources,
        window_starts: &Windows<&impl ReadableCloneableVec<Height, Height>>,
    ) -> Result<Self> {
        let full_version = version + VERSION;
        let rolling_version = full_version + Version::TWO;
        let cumulative_version = rolling_version + Version::ONE;
        let cumulative = ValuePerBlock::forced_import(
            db,
            &format!("{name}_cumulative"),
            cumulative_version,
            indexes,
        )?;
        let cumulative_sats = cumulative.sats.height.read_only_clone();
        let block = LazyValueBlock::from_cumulative_sources(
            name,
            cumulative_version,
            &cumulative_sats,
            &cumulative.cents.height,
        );
        let sum = LazyRollingSumsAmountFromHeight::new(
            &format!("{name}_sum"),
            rolling_version,
            &cumulative_sats,
            &cumulative.cents.height,
            window_starts,
            indexes,
        );
        let average = LazyRollingAvgsAmountFromHeight::new(
            &format!("{name}_average"),
            rolling_version,
            &cumulative_sats,
            &cumulative.cents.height,
            window_starts,
            indexes,
        );
        let distribution =
            RollingDistributionValuePerBlock::forced_import(db, name, full_version, indexes)?;

        Ok(Self {
            block,
            cumulative,
            sum,
            average,
            distribution,
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub fn compute_from_indexes<A, B>(
        &mut self,
        max_from: Height,
        windows: &WindowStarts<'_>,
        price_cents: &impl ReadableVec<Height, Cents>,
        first_indexes: &impl ReadableVec<Height, A>,
        indexes_count: &impl ReadableVec<Height, B>,
        source: &impl ReadableVec<A, Sats>,
        exit: &Exit,
    ) -> Result<()>
    where
        A: VecIndex + VecValue,
        B: VecValue,
        usize: From<B>,
    {
        self.cumulative.compute_sats_from_indexes(
            max_from,
            first_indexes,
            indexes_count,
            source,
            exit,
        )?;

        self.cumulative
            .cents
            .height
            .compute_cumulative_transformed_binary(
                max_from,
                &self.block.sats,
                price_cents,
                SatsToCents::apply,
                exit,
            )?;

        self.distribution
            .compute(max_from, windows, &self.block.sats, &self.block.cents, exit)
    }
}
