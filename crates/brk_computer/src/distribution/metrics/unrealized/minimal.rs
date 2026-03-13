use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{BasisPointsSigned32, Cents, Height, Version};
use vecdb::{Exit, ReadableVec, Rw, StorageMode};

use crate::internal::RatioPerBlock;

use crate::distribution::metrics::ImportConfig;

#[derive(Traversable)]
pub struct UnrealizedMinimal<M: StorageMode = Rw> {
    pub nupl: RatioPerBlock<BasisPointsSigned32, M>,
}

impl UnrealizedMinimal {
    pub(crate) fn forced_import(cfg: &ImportConfig) -> Result<Self> {
        Ok(Self {
            nupl: RatioPerBlock::forced_import_raw(
                cfg.db,
                &cfg.name("nupl"),
                cfg.version + Version::ONE,
                cfg.indexes,
            )?,
        })
    }

    pub(crate) fn compute(
        &mut self,
        max_from: Height,
        spot_price: &impl ReadableVec<Height, Cents>,
        realized_price: &impl ReadableVec<Height, Cents>,
        exit: &Exit,
    ) -> Result<()> {
        self.nupl.bps.height.compute_transform2(
            max_from,
            spot_price,
            realized_price,
            |(i, price, realized_price, ..)| {
                let p = price.as_u128();
                if p == 0 {
                    (i, BasisPointsSigned32::ZERO)
                } else {
                    let rp = realized_price.as_u128();
                    let nupl_bps = ((p as i128 - rp as i128) * 10000) / p as i128;
                    (i, BasisPointsSigned32::from(nupl_bps as i32))
                }
            },
            exit,
        )?;
        Ok(())
    }
}
