use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Bitcoin, Dollars, Height, Sats, Version};
use vecdb::{CollectableVec, Database, EagerVec, Exit, Importable, PcoVec};

use crate::{
    Indexes,
    grouped::Source,
    price,
    traits::{ComputeFromBitcoin, ComputeFromSats},
};

#[derive(Clone, Traversable)]
pub struct ComputedHeightValueVecs {
    pub sats: Option<EagerVec<PcoVec<Height, Sats>>>,
    pub bitcoin: EagerVec<PcoVec<Height, Bitcoin>>,
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
        Ok(Self {
            sats: source.is_compute().then(|| {
                EagerVec::forced_import(db, name, version + VERSION + Version::ZERO).unwrap()
            }),
            bitcoin: EagerVec::forced_import(
                db,
                &format!("{name}_btc"),
                version + VERSION + Version::ZERO,
            )?,
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
        compute(self.sats.as_mut().unwrap())?;

        let height: Option<&PcoVec<Height, Sats>> = None;
        self.compute_rest(price, starting_indexes, exit, height)?;

        Ok(())
    }

    pub fn compute_rest(
        &mut self,
        price: Option<&price::Vecs>,
        starting_indexes: &Indexes,
        exit: &Exit,
        height: Option<&impl CollectableVec<Height, Sats>>,
    ) -> Result<()> {
        if let Some(height) = height {
            self.bitcoin
                .compute_from_sats(starting_indexes.height, height, exit)?;
        } else {
            self.bitcoin.compute_from_sats(
                starting_indexes.height,
                self.sats.as_ref().unwrap(),
                exit,
            )?;
        }

        let height_to_bitcoin = &self.bitcoin;
        let height_to_price_close = &price.as_ref().unwrap().chainindexes_to_price_close.height;

        if let Some(dollars) = self.dollars.as_mut() {
            dollars.compute_from_bitcoin(
                starting_indexes.height,
                height_to_bitcoin,
                height_to_price_close,
                exit,
            )?;
        }

        Ok(())
    }
}
