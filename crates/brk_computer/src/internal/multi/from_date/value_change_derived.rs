//! Lazy derived values for change (bitcoin from sats, period aggregations).

use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Bitcoin, DateIndex, Dollars, SatsSigned, Version};
use vecdb::{Database, Exit, IterableBoxedVec};

use crate::{
    ComputeIndexes, indexes,
    internal::{ComputedFromDateLast, LazyDateDerivedLast, LazyFromDateLast, SatsSignedToBitcoin},
    price,
    traits::ComputeFromBitcoin,
    utils::OptionExt,
};

const VERSION: Version = Version::ZERO;

/// Lazy derived values for change (bitcoin from sats, period aggregations).
#[derive(Clone, Traversable)]
pub struct LazyValueChangeDateDerived {
    pub sats: LazyDateDerivedLast<SatsSigned>,
    pub bitcoin: LazyFromDateLast<Bitcoin, SatsSigned>,
    pub dollars: Option<ComputedFromDateLast<Dollars>>,
}

impl LazyValueChangeDateDerived {
    pub fn from_source(
        db: &Database,
        name: &str,
        source: IterableBoxedVec<DateIndex, SatsSigned>,
        version: Version,
        compute_dollars: bool,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let sats =
            LazyDateDerivedLast::from_source(name, version + VERSION, source.clone(), indexes);

        let bitcoin = LazyFromDateLast::from_derived::<SatsSignedToBitcoin>(
            &format!("{name}_btc"),
            version + VERSION,
            source,
            &sats,
        );

        let dollars = compute_dollars
            .then(|| {
                ComputedFromDateLast::forced_import(
                    db,
                    &format!("{name}_usd"),
                    version + VERSION,
                    indexes,
                )
            })
            .transpose()?;

        Ok(Self {
            sats,
            bitcoin,
            dollars,
        })
    }

    pub fn compute_dollars_from_price(
        &mut self,
        price: Option<&price::Vecs>,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        if let Some(dollars) = self.dollars.as_mut() {
            let dateindex_to_bitcoin = &*self.bitcoin.dateindex;
            let dateindex_to_price_close = &price.u().usd.split.close.dateindex;

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
