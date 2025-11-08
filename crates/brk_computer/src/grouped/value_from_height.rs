use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Bitcoin, Dollars, Height, Sats, Version};
use vecdb::{CollectableVec, Database, EagerVec, Exit, StoredVec};

use crate::{
    Indexes,
    grouped::Source,
    indexes, price,
    traits::{ComputeFromBitcoin, ComputeFromSats},
};

use super::{ComputedVecsFromHeight, VecBuilderOptions};

#[derive(Clone, Traversable)]
pub struct ComputedValueVecsFromHeight {
    pub sats: ComputedVecsFromHeight<Sats>,
    pub bitcoin: ComputedVecsFromHeight<Bitcoin>,
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
        Ok(Self {
            sats: ComputedVecsFromHeight::forced_import(
                db,
                name,
                source,
                version + VERSION,
                indexes,
                options,
            )?,
            bitcoin: ComputedVecsFromHeight::forced_import(
                db,
                &format!("{name}_btc"),
                Source::Compute,
                version + VERSION,
                indexes,
                options,
            )?,
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
        starting_indexes: &Indexes,
        exit: &Exit,
        mut compute: F,
    ) -> Result<()>
    where
        F: FnMut(&mut EagerVec<Height, Sats>) -> Result<()>,
    {
        compute(self.sats.height.as_mut().unwrap())?;

        let height: Option<&StoredVec<Height, Sats>> = None;
        self.compute_rest(indexes, price, starting_indexes, exit, height)?;

        Ok(())
    }

    pub fn compute_rest(
        &mut self,
        indexes: &indexes::Vecs,
        price: Option<&price::Vecs>,
        starting_indexes: &Indexes,
        exit: &Exit,
        height: Option<&impl CollectableVec<Height, Sats>>,
    ) -> Result<()> {
        if let Some(height) = height {
            self.sats
                .compute_rest(indexes, starting_indexes, exit, Some(height))?;

            self.bitcoin
                .compute_all(indexes, starting_indexes, exit, |v| {
                    v.compute_from_sats(starting_indexes.height, height, exit)
                })?;
        } else {
            let height: Option<&StoredVec<Height, Sats>> = None;

            self.sats
                .compute_rest(indexes, starting_indexes, exit, height)?;

            self.bitcoin
                .compute_all(indexes, starting_indexes, exit, |v| {
                    v.compute_from_sats(
                        starting_indexes.height,
                        self.sats.height.as_ref().unwrap(),
                        exit,
                    )
                })?;
        }

        let height_to_bitcoin = self.bitcoin.height.as_ref().unwrap();
        let height_to_price_close = &price.as_ref().unwrap().chainindexes_to_price_close.height;

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
