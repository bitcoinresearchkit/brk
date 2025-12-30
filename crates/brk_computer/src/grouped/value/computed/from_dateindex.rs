use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Bitcoin, DateIndex, Dollars, Sats, Version};
use vecdb::{CollectableVec, Database, EagerVec, Exit, IterableCloneableVec, PcoVec};

use crate::{
    Indexes,
    grouped::{ComputedVecsFromDateIndex, LazyVecsFromDateIndex, SatsToBitcoin},
    indexes, price,
    traits::ComputeFromBitcoin,
    utils::OptionExt,
};

use crate::grouped::{Source, VecBuilderOptions};

#[derive(Clone, Traversable)]
pub struct ComputedValueVecsFromDateIndex {
    pub sats: ComputedVecsFromDateIndex<Sats>,
    pub bitcoin: LazyVecsFromDateIndex<Bitcoin, Sats>,
    pub dollars: Option<ComputedVecsFromDateIndex<Dollars>>,
}

const VERSION: Version = Version::ZERO;

impl ComputedValueVecsFromDateIndex {
    #[allow(clippy::too_many_arguments)]
    pub fn forced_import(
        db: &Database,
        name: &str,
        source: Source<DateIndex, Sats>,
        version: Version,
        options: VecBuilderOptions,
        compute_dollars: bool,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let sats = ComputedVecsFromDateIndex::forced_import(
            db,
            name,
            source.clone(),
            version + VERSION,
            indexes,
            options,
        )?;

        let sats_source = source.vec().or(sats.dateindex.as_ref().map(|v| v.boxed_clone()));

        let bitcoin = LazyVecsFromDateIndex::from_computed::<SatsToBitcoin>(
            &format!("{name}_btc"),
            version + VERSION,
            sats_source,
            &sats,
        );

        Ok(Self {
            sats,
            bitcoin,
            dollars: compute_dollars.then(|| {
                ComputedVecsFromDateIndex::forced_import(
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
        price: Option<&price::Vecs>,
        starting_indexes: &Indexes,
        exit: &Exit,
        mut compute: F,
    ) -> Result<()>
    where
        F: FnMut(&mut EagerVec<PcoVec<DateIndex, Sats>>) -> Result<()>,
    {
        compute(self.sats.dateindex.um())?;

        let dateindex: Option<&PcoVec<DateIndex, Sats>> = None;
        self.compute_rest(price, starting_indexes, exit, dateindex)?;

        Ok(())
    }

    pub fn compute_rest(
        &mut self,
        price: Option<&price::Vecs>,
        starting_indexes: &Indexes,
        exit: &Exit,
        dateindex: Option<&impl CollectableVec<DateIndex, Sats>>,
    ) -> Result<()> {
        if let Some(dateindex) = dateindex {
            self.sats
                .compute_rest(starting_indexes, exit, Some(dateindex))?;
        } else {
            let dateindex: Option<&PcoVec<DateIndex, Sats>> = None;
            self.sats.compute_rest(starting_indexes, exit, dateindex)?;
        }

        let dateindex_to_bitcoin = self.bitcoin.dateindex.u();
        let dateindex_to_price_close = price
            .u()
            .timeindexes_to_price_close
            .dateindex
            .as_ref()
            .unwrap();

        if let Some(dollars) = self.dollars.as_mut() {
            dollars.compute_all(starting_indexes, exit, |v| {
                v.compute_from_bitcoin(
                    starting_indexes.dateindex,
                    dateindex_to_bitcoin,
                    dateindex_to_price_close,
                    exit,
                )
            })?;
        }

        Ok(())
    }
}
