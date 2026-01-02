use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Bitcoin, Dollars, Height, Sats, Version};
use vecdb::{CollectableVec, Database, EagerVec, Exit, IterableCloneableVec, PcoVec};

use crate::{
    ComputeIndexes,
    internal::{LazyVecsFromHeight, SatsToBitcoin, Source},
    indexes, price,
    traits::ComputeFromBitcoin,
    utils::OptionExt,
};

use crate::internal::{ComputedVecsFromHeight, VecBuilderOptions};

#[derive(Clone, Traversable)]
pub struct ComputedValueVecsFromHeight {
    pub sats: ComputedVecsFromHeight<Sats>,
    pub bitcoin: LazyVecsFromHeight<Bitcoin, Sats>,
    pub dollars: Option<ComputedVecsFromHeight<Dollars>>,
}

const VERSION: Version = Version::ZERO;

impl ComputedValueVecsFromHeight {
    #[allow(clippy::too_many_arguments)]
    pub fn forced_import(
        db: &Database,
        name: &str,
        source: Source<Height, Sats>,
        version: Version,
        options: VecBuilderOptions,
        compute_dollars: bool,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let sats = ComputedVecsFromHeight::forced_import(
            db,
            name,
            source.clone(),
            version + VERSION,
            indexes,
            options,
        )?;

        let sats_source = source
            .vec()
            .unwrap_or_else(|| sats.height.as_ref().unwrap().boxed_clone());

        let bitcoin = LazyVecsFromHeight::from_computed::<SatsToBitcoin>(
            &format!("{name}_btc"),
            version + VERSION,
            sats_source,
            &sats,
        );

        Ok(Self {
            sats,
            bitcoin,
            dollars: compute_dollars.then(|| {
                ComputedVecsFromHeight::forced_import(
                    db,
                    &format!("{name}_usd"),
                    Source::Compute,
                    version + VERSION,
                    indexes,
                    options,
                )
                .unwrap()
            }),
        })
    }

    pub fn compute_all<F>(
        &mut self,
        indexes: &indexes::Vecs,
        price: Option<&price::Vecs>,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
        mut compute: F,
    ) -> Result<()>
    where
        F: FnMut(&mut EagerVec<PcoVec<Height, Sats>>) -> Result<()>,
    {
        compute(self.sats.height.um())?;

        let height: Option<&PcoVec<Height, Sats>> = None;
        self.compute_rest(indexes, price, starting_indexes, exit, height)?;

        Ok(())
    }

    pub fn compute_rest(
        &mut self,
        indexes: &indexes::Vecs,
        price: Option<&price::Vecs>,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
        height: Option<&impl CollectableVec<Height, Sats>>,
    ) -> Result<()> {
        if let Some(height) = height {
            self.sats
                .compute_rest(indexes, starting_indexes, exit, Some(height))?;
        } else {
            let height: Option<&PcoVec<Height, Sats>> = None;
            self.sats
                .compute_rest(indexes, starting_indexes, exit, height)?;
        }

        let height_to_bitcoin = &self.bitcoin.height;
        let height_to_price_close = &price.u().usd.chainindexes_to_price_close.height;

        if let Some(dollars) = self.dollars.as_mut() {
            dollars.compute_all(indexes, starting_indexes, exit, |v| {
                v.compute_from_bitcoin(
                    starting_indexes.height,
                    height_to_bitcoin,
                    height_to_price_close,
                    exit,
                )
            })?;
        }

        Ok(())
    }
}
