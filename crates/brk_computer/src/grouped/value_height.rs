use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Bitcoin, Dollars, Height, Sats, Version};
use vecdb::{Database, EagerVec, Exit, ImportableVec, IterableCloneableVec, LazyVecFrom1, PcoVec};

use crate::{
    Indexes,
    grouped::{SatsToBitcoin, Source},
    price,
    traits::ComputeFromBitcoin,
    utils::OptionExt,
};

#[derive(Clone, Traversable)]
pub struct ComputedHeightValueVecs {
    pub sats: Option<EagerVec<PcoVec<Height, Sats>>>,
    pub bitcoin: LazyVecFrom1<Height, Bitcoin, Height, Sats>,
    pub dollars: Option<EagerVec<PcoVec<Height, Dollars>>>,
}

const VERSION: Version = Version::ZERO;

impl ComputedHeightValueVecs {
    pub fn forced_import(
        db: &Database,
        name: &str,
        source: Source<Height, Sats>,
        version: Version,
        compute_dollars: bool,
    ) -> Result<Self> {
        let sats = source
            .is_compute()
            .then(|| EagerVec::forced_import(db, name, version + VERSION + Version::ZERO).unwrap());

        let bitcoin = match &source {
            Source::Compute => LazyVecFrom1::transformed::<SatsToBitcoin>(
                &format!("{name}_btc"),
                version + VERSION + Version::ZERO,
                sats.as_ref().unwrap().boxed_clone(),
            ),
            Source::Vec(boxed) => LazyVecFrom1::transformed::<SatsToBitcoin>(
                &format!("{name}_btc"),
                version + VERSION + Version::ZERO,
                boxed.clone(),
            ),
            Source::None => {
                panic!("Source::None not supported for lazy bitcoin - use Source::Vec instead")
            }
        };

        Ok(Self {
            sats,
            bitcoin,
            dollars: compute_dollars.then(|| {
                EagerVec::forced_import(
                    db,
                    &format!("{name}_usd"),
                    version + VERSION + Version::ZERO,
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
        F: FnMut(&mut EagerVec<PcoVec<Height, Sats>>) -> Result<()>,
    {
        compute(self.sats.um())?;

        self.compute_rest(price, starting_indexes, exit)?;

        Ok(())
    }

    pub fn compute_rest(
        &mut self,
        price: Option<&price::Vecs>,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        if let Some(dollars) = self.dollars.as_mut() {
            dollars.compute_from_bitcoin(
                starting_indexes.height,
                &self.bitcoin,
                &price.u().chainindexes_to_price_close.height,
                exit,
            )?;
        }

        Ok(())
    }
}
