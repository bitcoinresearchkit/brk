//! Value type for Derived Last pattern from DateIndex (when source is external).

use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Bitcoin, DateIndex, Dollars, Sats, Version};
use vecdb::{Database, Exit, IterableBoxedVec};

use crate::{
    ComputeIndexes, indexes,
    internal::{ComputedDateLast, DerivedDateLast, LazyDateLast, SatsToBitcoin},
    price,
    traits::ComputeFromBitcoin,
    utils::OptionExt,
};

#[derive(Clone, Traversable)]
pub struct ValueDerivedDateLast {
    pub sats: DerivedDateLast<Sats>,
    pub bitcoin: LazyDateLast<Bitcoin, Sats>,
    pub dollars: Option<ComputedDateLast<Dollars>>,
}

const VERSION: Version = Version::ZERO;

impl ValueDerivedDateLast {
    pub fn from_source(
        db: &Database,
        name: &str,
        source: IterableBoxedVec<DateIndex, Sats>,
        version: Version,
        compute_dollars: bool,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let sats = DerivedDateLast::from_source(name, version + VERSION, source.clone(), indexes);

        let bitcoin = LazyDateLast::from_derived::<SatsToBitcoin>(
            &format!("{name}_btc"),
            version + VERSION,
            source,
            &sats,
        );

        let dollars = compute_dollars.then(|| {
            ComputedDateLast::forced_import(db, &format!("{name}_usd"), version + VERSION, indexes)
                .unwrap()
        });

        Ok(Self {
            sats,
            bitcoin,
            dollars,
        })
    }

    pub fn compute_rest(
        &mut self,
        price: Option<&price::Vecs>,
        _starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        let dateindex_to_bitcoin = &*self.bitcoin.dateindex;
        let dateindex_to_price_close = &price.u().usd.timeindexes_to_price_close.dateindex;

        if let Some(dollars) = self.dollars.as_mut() {
            dollars.compute_all(_starting_indexes, exit, |v| {
                v.compute_from_bitcoin(
                    _starting_indexes.dateindex,
                    dateindex_to_bitcoin,
                    dateindex_to_price_close,
                    exit,
                )
            })?;
        }

        Ok(())
    }
}
