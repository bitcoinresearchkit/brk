use brk_cohort::Filter;
use brk_error::Result;
use brk_types::{BasisPoints16, BasisPointsSigned16, Cents, Height, Version};
use schemars::JsonSchema;
use vecdb::{BytesVec, BytesVecValue, Database, ImportableVec};

use crate::{
    indexes,
    internal::{
        CentsType, ComputedFromHeight, ComputedFromHeightCumulative,
        ComputedFromHeightCumulativeSum, ComputedFromHeightRatio, FiatFromHeight, NumericValue,
        PercentFromHeight, PercentRollingEmas1w1m, PercentRollingWindows, Price, RollingEmas1w1m,
        RollingEmas2w, RollingWindows, ValueFromHeight, ValueFromHeightChange,
        ValueFromHeightCumulative,
    },
};

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

    pub(crate) fn import_computed<T: NumericValue + JsonSchema>(
        &self,
        suffix: &str,
        offset: Version,
    ) -> Result<ComputedFromHeight<T>> {
        ComputedFromHeight::forced_import(
            self.db,
            &self.name(suffix),
            self.version + offset,
            self.indexes,
        )
    }

    pub(crate) fn import_cumulative<T: NumericValue + JsonSchema>(
        &self,
        suffix: &str,
        offset: Version,
    ) -> Result<ComputedFromHeightCumulative<T>> {
        ComputedFromHeightCumulative::forced_import(
            self.db,
            &self.name(suffix),
            self.version + offset,
            self.indexes,
        )
    }

    pub(crate) fn import_cumulative_sum<T: NumericValue + JsonSchema>(
        &self,
        suffix: &str,
        offset: Version,
    ) -> Result<ComputedFromHeightCumulativeSum<T>> {
        ComputedFromHeightCumulativeSum::forced_import(
            self.db,
            &self.name(suffix),
            self.version + offset,
            self.indexes,
        )
    }

    pub(crate) fn import_percent_bp16(
        &self,
        suffix: &str,
        offset: Version,
    ) -> Result<PercentFromHeight<BasisPoints16>> {
        PercentFromHeight::forced_import(
            self.db,
            &self.name(suffix),
            self.version + offset,
            self.indexes,
        )
    }

    pub(crate) fn import_percent_bps16(
        &self,
        suffix: &str,
        offset: Version,
    ) -> Result<PercentFromHeight<BasisPointsSigned16>> {
        PercentFromHeight::forced_import(
            self.db,
            &self.name(suffix),
            self.version + offset,
            self.indexes,
        )
    }

    pub(crate) fn import_fiat<C: CentsType>(
        &self,
        suffix: &str,
        offset: Version,
    ) -> Result<FiatFromHeight<C>> {
        FiatFromHeight::forced_import(
            self.db,
            &self.name(suffix),
            self.version + offset,
            self.indexes,
        )
    }

    pub(crate) fn import_value(&self, suffix: &str, offset: Version) -> Result<ValueFromHeight> {
        ValueFromHeight::forced_import(
            self.db,
            &self.name(suffix),
            self.version + offset,
            self.indexes,
        )
    }

    pub(crate) fn import_value_cumulative(
        &self,
        suffix: &str,
        offset: Version,
    ) -> Result<ValueFromHeightCumulative> {
        ValueFromHeightCumulative::forced_import(
            self.db,
            &self.name(suffix),
            self.version + offset,
            self.indexes,
        )
    }

    pub(crate) fn import_value_change(
        &self,
        suffix: &str,
        offset: Version,
    ) -> Result<ValueFromHeightChange> {
        ValueFromHeightChange::forced_import(
            self.db,
            &self.name(suffix),
            self.version + offset,
            self.indexes,
        )
    }

    pub(crate) fn import_price(
        &self,
        suffix: &str,
        offset: Version,
    ) -> Result<Price<ComputedFromHeight<Cents>>> {
        Price::forced_import(
            self.db,
            &self.name(suffix),
            self.version + offset,
            self.indexes,
        )
    }

    pub(crate) fn import_ratio(
        &self,
        suffix: &str,
        offset: Version,
    ) -> Result<ComputedFromHeightRatio> {
        ComputedFromHeightRatio::forced_import(
            self.db,
            &self.name(suffix),
            self.version + offset,
            self.indexes,
        )
    }

    pub(crate) fn import_bytes<T: BytesVecValue>(
        &self,
        suffix: &str,
        offset: Version,
    ) -> Result<BytesVec<Height, T>> {
        Ok(BytesVec::forced_import(
            self.db,
            &self.name(suffix),
            self.version + offset,
        )?)
    }

    pub(crate) fn import_rolling<T: NumericValue + JsonSchema>(
        &self,
        suffix: &str,
        offset: Version,
    ) -> Result<RollingWindows<T>> {
        RollingWindows::forced_import(
            self.db,
            &self.name(suffix),
            self.version + offset,
            self.indexes,
        )
    }

    pub(crate) fn import_percent_rolling_bp16(
        &self,
        suffix: &str,
        offset: Version,
    ) -> Result<PercentRollingWindows<BasisPoints16>> {
        PercentRollingWindows::forced_import(
            self.db,
            &self.name(suffix),
            self.version + offset,
            self.indexes,
        )
    }

    pub(crate) fn import_emas_1w_1m<T: NumericValue + JsonSchema>(
        &self,
        suffix: &str,
        offset: Version,
    ) -> Result<RollingEmas1w1m<T>> {
        RollingEmas1w1m::forced_import(
            self.db,
            &self.name(suffix),
            self.version + offset,
            self.indexes,
        )
    }

    pub(crate) fn import_percent_emas_1w_1m_bp16(
        &self,
        suffix: &str,
        offset: Version,
    ) -> Result<PercentRollingEmas1w1m<BasisPoints16>> {
        PercentRollingEmas1w1m::forced_import(
            self.db,
            &self.name(suffix),
            self.version + offset,
            self.indexes,
        )
    }

    pub(crate) fn import_emas_2w(&self, suffix: &str, offset: Version) -> Result<RollingEmas2w> {
        RollingEmas2w::forced_import(
            self.db,
            &self.name(suffix),
            self.version + offset,
            self.indexes,
        )
    }
}
