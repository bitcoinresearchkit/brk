use brk_error::Result;

use bitview_traversable::Traversable;
use brk_types::{Bitcoin, Cents, Dollars, Sats, SatsSigned, Version};
use schemars::JsonSchema;
use vecdb::{Budgeted, Database, Rw, StorageMode, UnaryTransform};

use crate::{
    CachePolicy, CentsUnsignedToDollars, IndexSources, LazyPerBlock, NumericValue, PerBlock,
    SatsSignedToBitcoin, SatsToBitcoin,
};

/// Trait that associates a sats type with its transform to Bitcoin.
pub trait AmountType: NumericValue + JsonSchema {
    type ToBitcoin: UnaryTransform<Self, Bitcoin>;
}

impl AmountType for Sats {
    type ToBitcoin = SatsToBitcoin;
}

impl AmountType for SatsSigned {
    type ToBitcoin = SatsSignedToBitcoin;
}

/// The policy selects sats retention; the independent cents source stays budgeted.
#[derive(Traversable)]
pub struct ValuePerBlock<M: StorageMode = Rw, S: CachePolicy = Budgeted> {
    /// Reported in BTC; one BTC equals 100,000,000 satoshis.
    pub btc: LazyPerBlock<Bitcoin, Sats>,
    /// Reported in satoshis.
    pub sats: PerBlock<Sats, M, S>,
    /// Reported in US dollars.
    pub usd: LazyPerBlock<Dollars, Cents>,
    /// Reported in US cents; 100 cents equal one US dollar.
    pub cents: PerBlock<Cents, M>,
}

impl<S: CachePolicy> ValuePerBlock<Rw, S> {
    pub fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &IndexSources,
    ) -> Result<Self> {
        let sats = PerBlock::forced_import(db, &format!("{name}_sats"), version, indexes)?;

        let btc = LazyPerBlock::from_resolutions::<SatsToBitcoin>(name, version, &sats);

        let cents = PerBlock::forced_import(db, &format!("{name}_cents"), version, indexes)?;

        let usd = LazyPerBlock::from_resolutions::<CentsUnsignedToDollars>(
            &format!("{name}_usd"),
            version,
            &cents,
        );

        Ok(Self {
            btc,
            sats,
            usd,
            cents,
        })
    }
}
