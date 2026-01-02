use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Bitcoin, Close, Dollars, Height, Sats, Version};
use vecdb::{
    Database, EagerVec, ImportableVec, IterableBoxedVec, IterableCloneableVec, LazyVecFrom1,
    LazyVecFrom2, PcoVec,
};

use crate::internal::{ClosePriceTimesSats, SatsToBitcoin, Source};

#[derive(Clone, Traversable)]
pub struct ComputedHeightValueVecs {
    pub sats: Option<EagerVec<PcoVec<Height, Sats>>>,
    pub bitcoin: LazyVecFrom1<Height, Bitcoin, Height, Sats>,
    pub dollars: Option<LazyVecFrom2<Height, Dollars, Height, Close<Dollars>, Height, Sats>>,
}

const VERSION: Version = Version::ZERO;

impl ComputedHeightValueVecs {
    pub fn forced_import(
        db: &Database,
        name: &str,
        source: Source<Height, Sats>,
        version: Version,
        price_source: Option<IterableBoxedVec<Height, Close<Dollars>>>,
    ) -> Result<Self> {
        let sats = source
            .is_compute()
            .then(|| EagerVec::forced_import(db, name, version + VERSION + Version::ZERO).unwrap());

        let sats_source: IterableBoxedVec<Height, Sats> = source
            .vec()
            .unwrap_or_else(|| sats.as_ref().unwrap().boxed_clone());

        let bitcoin = LazyVecFrom1::transformed::<SatsToBitcoin>(
            &format!("{name}_btc"),
            version + VERSION + Version::ZERO,
            sats_source.clone(),
        );

        let dollars = price_source.map(|price| {
            LazyVecFrom2::transformed::<ClosePriceTimesSats>(
                &format!("{name}_usd"),
                version + VERSION + Version::ZERO,
                price,
                sats_source.clone(),
            )
        });

        Ok(Self {
            sats,
            bitcoin,
            dollars,
        })
    }
}
