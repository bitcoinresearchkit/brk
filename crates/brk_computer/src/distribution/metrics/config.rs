use brk_cohort::Filter;
use brk_error::Result;
use brk_types::{BasisPoints16, BasisPoints32, BasisPointsSigned32, Cents, Height, Version};
use schemars::JsonSchema;
use vecdb::{BytesVec, BytesVecValue, Database, ImportableVec};

use crate::{
    indexes,
    internal::{
        AmountPerBlock, AmountPerBlockCumulative, AmountPerBlockCumulativeRolling, CentsType,
        FiatPerBlock, FiatPerBlockCumulativeWithSums, NumericValue, PerBlock,
        PerBlockCumulativeRolling, PercentPerBlock, PercentRollingWindows, Price,
        PriceWithRatioExtendedPerBlock, PriceWithRatioPerBlock, RatioPerBlock,
        RollingWindow24hPerBlock, RollingWindows, RollingWindowsFrom1w, WindowStartVec, Windows,
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
    PriceWithRatioPerBlock,
    PriceWithRatioExtendedPerBlock,
    RatioPerBlock<BasisPoints32>,
    RatioPerBlock<BasisPointsSigned32>,
    PercentPerBlock<BasisPoints16>,
    PercentPerBlock<BasisPoints32>,
    PercentPerBlock<BasisPointsSigned32>,
    PercentRollingWindows<BasisPoints32>,
    Price<PerBlock<Cents>>,
);

// Generic types (macro_rules can't parse generic bounds, so written out)
impl<T: NumericValue + JsonSchema> ConfigImport for PerBlock<T> {
    fn config_import(cfg: &ImportConfig, suffix: &str, offset: Version) -> Result<Self> {
        Self::forced_import(cfg.db, &cfg.name(suffix), cfg.version + offset, cfg.indexes)
    }
}
impl<T, C> ConfigImport for PerBlockCumulativeRolling<T, C>
where
    T: NumericValue + JsonSchema + Into<C>,
    C: NumericValue + JsonSchema,
{
    fn config_import(cfg: &ImportConfig, suffix: &str, offset: Version) -> Result<Self> {
        Self::forced_import(
            cfg.db,
            &cfg.name(suffix),
            cfg.version + offset,
            cfg.indexes,
            cfg.cached_starts,
        )
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
impl ConfigImport for AmountPerBlockCumulativeRolling {
    fn config_import(cfg: &ImportConfig, suffix: &str, offset: Version) -> Result<Self> {
        Self::forced_import(
            cfg.db,
            &cfg.name(suffix),
            cfg.version + offset,
            cfg.indexes,
            cfg.cached_starts,
        )
    }
}
impl<C: CentsType> ConfigImport for FiatPerBlockCumulativeWithSums<C> {
    fn config_import(cfg: &ImportConfig, suffix: &str, offset: Version) -> Result<Self> {
        Self::forced_import(
            cfg.db,
            &cfg.name(suffix),
            cfg.version + offset,
            cfg.indexes,
            cfg.cached_starts,
        )
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
    pub cached_starts: &'a Windows<&'a WindowStartVec>,
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
