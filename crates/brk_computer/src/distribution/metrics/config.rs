use brk_cohort::Filter;
use brk_error::Result;
use brk_types::{BasisPoints16, BasisPoints32, BasisPointsSigned32, Cents, Height, Version};
use schemars::JsonSchema;
use vecdb::{BytesVec, BytesVecValue, Database, ImportableVec};

use crate::{
    indexes,
    internal::{
        AmountPerBlock, AmountPerBlockCumulative, AmountPerBlockWithSum24h, CentsType, ComputedPerBlock,
        ComputedPerBlockCumulative, ComputedPerBlockCumulativeSum, FiatPerBlockWithSum24h,
        PerBlockWithSum24h, PriceWithRatioExtendedPerBlock, PriceWithRatioPerBlock, RatioPerBlock, RollingWindow24hAmountPerBlock,
        RollingWindow24hFiatPerBlock, RollingWindow24hPerBlock,
        FiatPerBlock, FiatRollingDelta1m, FiatRollingDeltaExcept1m, NumericValue,
        PercentPerBlock, PercentRollingWindows, Price, RollingDelta1m, RollingDeltaExcept1m,
        RollingWindows, RollingWindowsFrom1w,
    },
};

/// Trait for types importable via `ImportConfig::import`.
pub(crate) trait ConfigImport: Sized {
    fn config_import(cfg: &ImportConfig, suffix: &str, offset: Version) -> Result<Self>;
}

/// Implement `ConfigImport` for types whose `forced_import` takes `(db, name, version, indexes)`.
macro_rules! impl_config_import {
    ($($type:ty),+ $(,)?) => {
        $(
            impl ConfigImport for $type {
                fn config_import(cfg: &ImportConfig, suffix: &str, offset: Version) -> Result<Self> {
                    Self::forced_import(cfg.db, &cfg.name(suffix), cfg.version + offset, cfg.indexes)
                }
            }
        )+
    };
}

// Non-generic types
impl_config_import!(
    AmountPerBlock,
    AmountPerBlockCumulative,
    RollingWindow24hAmountPerBlock,
    PriceWithRatioPerBlock,
    PriceWithRatioExtendedPerBlock,
    RatioPerBlock<BasisPoints32>,
    RatioPerBlock<BasisPointsSigned32>,
    PercentPerBlock<BasisPoints16>,
    PercentPerBlock<BasisPoints32>,
    PercentPerBlock<BasisPointsSigned32>,
    PercentRollingWindows<BasisPoints32>,
    Price<ComputedPerBlock<Cents>>,
);

// Generic types (macro_rules can't parse generic bounds, so written out)
impl<T: NumericValue + JsonSchema> ConfigImport for ComputedPerBlock<T> {
    fn config_import(cfg: &ImportConfig, suffix: &str, offset: Version) -> Result<Self> {
        Self::forced_import(cfg.db, &cfg.name(suffix), cfg.version + offset, cfg.indexes)
    }
}
impl<T: NumericValue + JsonSchema> ConfigImport for ComputedPerBlockCumulative<T> {
    fn config_import(cfg: &ImportConfig, suffix: &str, offset: Version) -> Result<Self> {
        Self::forced_import(cfg.db, &cfg.name(suffix), cfg.version + offset, cfg.indexes)
    }
}
impl<T: NumericValue + JsonSchema> ConfigImport for ComputedPerBlockCumulativeSum<T> {
    fn config_import(cfg: &ImportConfig, suffix: &str, offset: Version) -> Result<Self> {
        Self::forced_import(cfg.db, &cfg.name(suffix), cfg.version + offset, cfg.indexes)
    }
}
impl<T: NumericValue + JsonSchema> ConfigImport for RollingWindows<T> {
    fn config_import(cfg: &ImportConfig, suffix: &str, offset: Version) -> Result<Self> {
        Self::forced_import(cfg.db, &cfg.name(suffix), cfg.version + offset, cfg.indexes)
    }
}
impl<T: NumericValue + JsonSchema> ConfigImport for RollingWindow24hPerBlock<T> {
    fn config_import(cfg: &ImportConfig, suffix: &str, offset: Version) -> Result<Self> {
        Self::forced_import(cfg.db, &cfg.name(suffix), cfg.version + offset, cfg.indexes)
    }
}
impl<T: NumericValue + JsonSchema> ConfigImport for PerBlockWithSum24h<T> {
    fn config_import(cfg: &ImportConfig, suffix: &str, offset: Version) -> Result<Self> {
        Ok(Self {
            raw: ComputedPerBlock::config_import(cfg, suffix, offset)?,
            sum: RollingWindow24hPerBlock::config_import(cfg, suffix, offset)?,
        })
    }
}
impl ConfigImport for AmountPerBlockWithSum24h {
    fn config_import(cfg: &ImportConfig, suffix: &str, offset: Version) -> Result<Self> {
        Ok(Self {
            raw: AmountPerBlock::config_import(cfg, suffix, offset)?,
            sum: RollingWindow24hAmountPerBlock::config_import(cfg, suffix, offset)?,
        })
    }
}
impl<C: CentsType> ConfigImport for RollingWindow24hFiatPerBlock<C> {
    fn config_import(cfg: &ImportConfig, suffix: &str, offset: Version) -> Result<Self> {
        Self::forced_import(cfg.db, &cfg.name(suffix), cfg.version + offset, cfg.indexes)
    }
}
impl<C: CentsType> ConfigImport for FiatPerBlockWithSum24h<C> {
    fn config_import(cfg: &ImportConfig, suffix: &str, offset: Version) -> Result<Self> {
        Ok(Self {
            raw: FiatPerBlock::config_import(cfg, suffix, offset)?,
            sum: RollingWindow24hFiatPerBlock::config_import(cfg, suffix, offset)?,
        })
    }
}
impl<T: NumericValue + JsonSchema> ConfigImport for RollingWindowsFrom1w<T> {
    fn config_import(cfg: &ImportConfig, suffix: &str, offset: Version) -> Result<Self> {
        Self::forced_import(cfg.db, &cfg.name(suffix), cfg.version + offset, cfg.indexes)
    }
}
impl<C: CentsType> ConfigImport for FiatPerBlock<C> {
    fn config_import(cfg: &ImportConfig, suffix: &str, offset: Version) -> Result<Self> {
        Self::forced_import(cfg.db, &cfg.name(suffix), cfg.version + offset, cfg.indexes)
    }
}
impl<S: NumericValue + JsonSchema, C: NumericValue + JsonSchema> ConfigImport
    for RollingDelta1m<S, C>
{
    fn config_import(cfg: &ImportConfig, suffix: &str, offset: Version) -> Result<Self> {
        Self::forced_import(cfg.db, &cfg.name(suffix), cfg.version + offset, cfg.indexes)
    }
}
impl<S: NumericValue + JsonSchema, C: NumericValue + JsonSchema> ConfigImport
    for RollingDeltaExcept1m<S, C>
{
    fn config_import(cfg: &ImportConfig, suffix: &str, offset: Version) -> Result<Self> {
        Self::forced_import(cfg.db, &cfg.name(suffix), cfg.version + offset, cfg.indexes)
    }
}
impl<S: NumericValue + JsonSchema, C: CentsType> ConfigImport for FiatRollingDelta1m<S, C> {
    fn config_import(cfg: &ImportConfig, suffix: &str, offset: Version) -> Result<Self> {
        Self::forced_import(cfg.db, &cfg.name(suffix), cfg.version + offset, cfg.indexes)
    }
}
impl<S: NumericValue + JsonSchema, C: CentsType> ConfigImport for FiatRollingDeltaExcept1m<S, C> {
    fn config_import(cfg: &ImportConfig, suffix: &str, offset: Version) -> Result<Self> {
        Self::forced_import(cfg.db, &cfg.name(suffix), cfg.version + offset, cfg.indexes)
    }
}
impl<T: BytesVecValue> ConfigImport for BytesVec<Height, T> {
    fn config_import(cfg: &ImportConfig, suffix: &str, offset: Version) -> Result<Self> {
        Ok(Self::forced_import(
            cfg.db,
            &cfg.name(suffix),
            cfg.version + offset,
        )?)
    }
}

#[derive(Clone, Copy)]
pub struct ImportConfig<'a> {
    pub db: &'a Database,
    pub filter: &'a Filter,
    pub full_name: &'a str,
    pub version: Version,
    pub indexes: &'a indexes::Vecs,
}

impl<'a> ImportConfig<'a> {
    pub(crate) fn name(&self, suffix: &str) -> String {
        if self.full_name.is_empty() {
            suffix.to_string()
        } else if suffix.is_empty() {
            self.full_name.to_string()
        } else {
            format!("{}_{suffix}", self.full_name)
        }
    }

    pub(crate) fn import<T: ConfigImport>(&self, suffix: &str, offset: Version) -> Result<T> {
        T::config_import(self, suffix, offset)
    }
}
