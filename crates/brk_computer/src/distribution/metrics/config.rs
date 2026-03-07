use brk_cohort::Filter;
use brk_error::Result;
use brk_types::{
    BasisPoints16, BasisPoints32, BasisPointsSigned32, Cents, Height, Version,
};
use schemars::JsonSchema;
use vecdb::{BytesVec, BytesVecValue, Database, ImportableVec};

use crate::{
    indexes,
    internal::{
        CentsType, ComputedFromHeight, ComputedFromHeightCumulative,
        ComputedFromHeightCumulativeSum, ComputedFromHeightRatio, FiatFromHeight, NumericValue,
        PercentFromHeight, PercentRollingEmas1w1m, PercentRollingWindows, Price, RollingEmas1w1m,
        RollingEmas2w, RollingWindow24h, RollingWindows, RollingWindowsFrom1w,
        ValueFromHeight, ValueFromHeightChange, ValueFromHeightCumulative,
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
    ValueFromHeight,
    ValueFromHeightCumulative,
    ValueFromHeightChange,
    ComputedFromHeightRatio,
    RollingEmas2w,
    PercentFromHeight<BasisPoints16>,
    PercentFromHeight<BasisPoints32>,
    PercentFromHeight<BasisPointsSigned32>,
    PercentRollingWindows<BasisPoints32>,
    PercentRollingEmas1w1m<BasisPoints32>,
    Price<ComputedFromHeight<Cents>>,
);

// Generic types (macro_rules can't parse generic bounds, so written out)
impl<T: NumericValue + JsonSchema> ConfigImport for ComputedFromHeight<T> {
    fn config_import(cfg: &ImportConfig, suffix: &str, offset: Version) -> Result<Self> {
        Self::forced_import(cfg.db, &cfg.name(suffix), cfg.version + offset, cfg.indexes)
    }
}
impl<T: NumericValue + JsonSchema> ConfigImport for ComputedFromHeightCumulative<T> {
    fn config_import(cfg: &ImportConfig, suffix: &str, offset: Version) -> Result<Self> {
        Self::forced_import(cfg.db, &cfg.name(suffix), cfg.version + offset, cfg.indexes)
    }
}
impl<T: NumericValue + JsonSchema> ConfigImport for ComputedFromHeightCumulativeSum<T> {
    fn config_import(cfg: &ImportConfig, suffix: &str, offset: Version) -> Result<Self> {
        Self::forced_import(cfg.db, &cfg.name(suffix), cfg.version + offset, cfg.indexes)
    }
}
impl<T: NumericValue + JsonSchema> ConfigImport for RollingWindows<T> {
    fn config_import(cfg: &ImportConfig, suffix: &str, offset: Version) -> Result<Self> {
        Self::forced_import(cfg.db, &cfg.name(suffix), cfg.version + offset, cfg.indexes)
    }
}
impl<T: NumericValue + JsonSchema> ConfigImport for RollingWindow24h<T> {
    fn config_import(cfg: &ImportConfig, suffix: &str, offset: Version) -> Result<Self> {
        Self::forced_import(cfg.db, &cfg.name(suffix), cfg.version + offset, cfg.indexes)
    }
}
impl<T: NumericValue + JsonSchema> ConfigImport for RollingWindowsFrom1w<T> {
    fn config_import(cfg: &ImportConfig, suffix: &str, offset: Version) -> Result<Self> {
        Self::forced_import(cfg.db, &cfg.name(suffix), cfg.version + offset, cfg.indexes)
    }
}
impl<T: NumericValue + JsonSchema> ConfigImport for RollingEmas1w1m<T> {
    fn config_import(cfg: &ImportConfig, suffix: &str, offset: Version) -> Result<Self> {
        Self::forced_import(cfg.db, &cfg.name(suffix), cfg.version + offset, cfg.indexes)
    }
}
impl<C: CentsType> ConfigImport for FiatFromHeight<C> {
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

    pub(crate) fn import<T: ConfigImport>(
        &self,
        suffix: &str,
        offset: Version,
    ) -> Result<T> {
        T::config_import(self, suffix, offset)
    }
}
