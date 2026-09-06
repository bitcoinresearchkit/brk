use std::fmt::{self, Debug};

use brk_error::Error;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use vecdb::PrintableIndex;

use super::{
    Date, Day1, Day3, EmptyAddrIndex, EmptyOutputIndex, Epoch, ExtendedEmptyAddrIndex,
    FundedAddrIndex, Halving, Height, Hour1, Hour4, Hour12, Minute10, Minute30, Month1, Month3,
    Month6, OpReturnIndex, P2AAddrIndex, P2MSOutputIndex, P2PK33AddrIndex, P2PK65AddrIndex,
    P2PKHAddrIndex, P2SHAddrIndex, P2TRAddrIndex, P2WPKHAddrIndex, P2WSHAddrIndex, Timestamp,
    TxInIndex, TxIndex, TxOutIndex, UnknownOutputIndex, Week1, Year1, Year10,
    hour1::HOUR1_INTERVAL, hour4::HOUR4_INTERVAL, hour12::HOUR12_INTERVAL,
    minute10::MINUTE10_INTERVAL, minute30::MINUTE30_INTERVAL, timestamp::INDEX_EPOCH,
};

/// Aggregation dimension for querying series. Includes time-based (date, week, month, year),
/// block-based (height, tx_index), and address/output type indexes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
#[schemars(example = Index::Day1)]
pub enum Index {
    Minute10,
    Minute30,
    Hour1,
    Hour4,
    Hour12,
    Day1,
    Day3,
    Week1,
    Month1,
    Month3,
    Month6,
    Year1,
    Year10,
    Halving,
    Epoch,
    Height,
    #[serde(rename = "tx_index")]
    TxIndex,
    #[serde(rename = "txin_index")]
    TxInIndex,
    #[serde(rename = "txout_index")]
    TxOutIndex,
    #[serde(rename = "empty_output_index")]
    EmptyOutputIndex,
    #[serde(rename = "op_return_index")]
    OpReturnIndex,
    #[serde(rename = "p2a_addr_index")]
    P2AAddrIndex,
    #[serde(rename = "p2ms_output_index")]
    P2MSOutputIndex,
    #[serde(rename = "p2pk33_addr_index")]
    P2PK33AddrIndex,
    #[serde(rename = "p2pk65_addr_index")]
    P2PK65AddrIndex,
    #[serde(rename = "p2pkh_addr_index")]
    P2PKHAddrIndex,
    #[serde(rename = "p2sh_addr_index")]
    P2SHAddrIndex,
    #[serde(rename = "p2tr_addr_index")]
    P2TRAddrIndex,
    #[serde(rename = "p2wpkh_addr_index")]
    P2WPKHAddrIndex,
    #[serde(rename = "p2wsh_addr_index")]
    P2WSHAddrIndex,
    #[serde(rename = "unknown_output_index")]
    UnknownOutputIndex,
    #[serde(rename = "funded_addr_index")]
    FundedAddrIndex,
    #[serde(rename = "empty_addr_index")]
    EmptyAddrIndex,
    #[serde(rename = "extended_empty_addr_index")]
    ExtendedEmptyAddrIndex,
}

/// How the trailing edge of an [`Index`] mutates over time. Drives the series
/// cache: bucketed indexes have a small fixed volatile tail, entity indexes
/// derive their stable count from a height→first-index mapping, and mutable
/// addr indexes have no immutable region (entries change retroactively).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CacheClass {
    /// Append-only with `margin` trailing entries volatile per ≥6-block reorg.
    Bucket { margin: usize },
    /// Append-only entity index (one number per tx / input / output / typed
    /// addr). Stable count must be looked up via the indexer's
    /// `first_X_index[tip - 6]` mapping.
    Entity,
    /// Retroactively mutable: no immutable region (`FundedAddrIndex`,
    /// `EmptyAddrIndex`).
    Mutable,
}

impl Index {
    pub const fn all() -> [Self; 34] {
        [
            Self::Minute10,
            Self::Minute30,
            Self::Hour1,
            Self::Hour4,
            Self::Hour12,
            Self::Day1,
            Self::Day3,
            Self::Week1,
            Self::Month1,
            Self::Month3,
            Self::Month6,
            Self::Year1,
            Self::Year10,
            Self::Halving,
            Self::Epoch,
            Self::Height,
            Self::TxIndex,
            Self::TxInIndex,
            Self::TxOutIndex,
            Self::EmptyOutputIndex,
            Self::OpReturnIndex,
            Self::P2AAddrIndex,
            Self::P2MSOutputIndex,
            Self::P2PK33AddrIndex,
            Self::P2PK65AddrIndex,
            Self::P2PKHAddrIndex,
            Self::P2SHAddrIndex,
            Self::P2TRAddrIndex,
            Self::P2WPKHAddrIndex,
            Self::P2WSHAddrIndex,
            Self::UnknownOutputIndex,
            Self::FundedAddrIndex,
            Self::EmptyAddrIndex,
            Self::ExtendedEmptyAddrIndex,
        ]
    }

    pub fn possible_values(&self) -> &'static [&'static str] {
        match self {
            Self::Minute10 => Minute10::to_possible_strings(),
            Self::Minute30 => Minute30::to_possible_strings(),
            Self::Hour1 => Hour1::to_possible_strings(),
            Self::Hour4 => Hour4::to_possible_strings(),
            Self::Hour12 => Hour12::to_possible_strings(),
            Self::Day1 => Day1::to_possible_strings(),
            Self::Day3 => Day3::to_possible_strings(),
            Self::Week1 => Week1::to_possible_strings(),
            Self::Month1 => Month1::to_possible_strings(),
            Self::Month3 => Month3::to_possible_strings(),
            Self::Month6 => Month6::to_possible_strings(),
            Self::Year1 => Year1::to_possible_strings(),
            Self::Year10 => Year10::to_possible_strings(),
            Self::Halving => Halving::to_possible_strings(),
            Self::Epoch => Epoch::to_possible_strings(),
            Self::Height => Height::to_possible_strings(),
            Self::TxIndex => TxIndex::to_possible_strings(),
            Self::TxInIndex => TxInIndex::to_possible_strings(),
            Self::TxOutIndex => TxOutIndex::to_possible_strings(),
            Self::EmptyOutputIndex => EmptyOutputIndex::to_possible_strings(),
            Self::OpReturnIndex => OpReturnIndex::to_possible_strings(),
            Self::P2AAddrIndex => P2AAddrIndex::to_possible_strings(),
            Self::P2MSOutputIndex => P2MSOutputIndex::to_possible_strings(),
            Self::P2PK33AddrIndex => P2PK33AddrIndex::to_possible_strings(),
            Self::P2PK65AddrIndex => P2PK65AddrIndex::to_possible_strings(),
            Self::P2PKHAddrIndex => P2PKHAddrIndex::to_possible_strings(),
            Self::P2SHAddrIndex => P2SHAddrIndex::to_possible_strings(),
            Self::P2TRAddrIndex => P2TRAddrIndex::to_possible_strings(),
            Self::P2WPKHAddrIndex => P2WPKHAddrIndex::to_possible_strings(),
            Self::P2WSHAddrIndex => P2WSHAddrIndex::to_possible_strings(),
            Self::UnknownOutputIndex => UnknownOutputIndex::to_possible_strings(),
            Self::FundedAddrIndex => FundedAddrIndex::to_possible_strings(),
            Self::EmptyAddrIndex => EmptyAddrIndex::to_possible_strings(),
            Self::ExtendedEmptyAddrIndex => ExtendedEmptyAddrIndex::to_possible_strings(),
        }
    }

    pub fn all_possible_values() -> Vec<&'static str> {
        Self::all()
            .into_iter()
            .flat_map(|i| i.possible_values().iter().cloned())
            .collect::<Vec<_>>()
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::Minute10 => <Minute10 as PrintableIndex>::to_string(),
            Self::Minute30 => <Minute30 as PrintableIndex>::to_string(),
            Self::Hour1 => <Hour1 as PrintableIndex>::to_string(),
            Self::Hour4 => <Hour4 as PrintableIndex>::to_string(),
            Self::Hour12 => <Hour12 as PrintableIndex>::to_string(),
            Self::Day1 => <Day1 as PrintableIndex>::to_string(),
            Self::Day3 => <Day3 as PrintableIndex>::to_string(),
            Self::Week1 => <Week1 as PrintableIndex>::to_string(),
            Self::Month1 => <Month1 as PrintableIndex>::to_string(),
            Self::Month3 => <Month3 as PrintableIndex>::to_string(),
            Self::Month6 => <Month6 as PrintableIndex>::to_string(),
            Self::Year1 => <Year1 as PrintableIndex>::to_string(),
            Self::Year10 => <Year10 as PrintableIndex>::to_string(),
            Self::Halving => <Halving as PrintableIndex>::to_string(),
            Self::Epoch => <Epoch as PrintableIndex>::to_string(),
            Self::Height => <Height as PrintableIndex>::to_string(),
            Self::TxIndex => <TxIndex as PrintableIndex>::to_string(),
            Self::TxInIndex => <TxInIndex as PrintableIndex>::to_string(),
            Self::TxOutIndex => <TxOutIndex as PrintableIndex>::to_string(),
            Self::EmptyOutputIndex => <EmptyOutputIndex as PrintableIndex>::to_string(),
            Self::OpReturnIndex => <OpReturnIndex as PrintableIndex>::to_string(),
            Self::P2AAddrIndex => <P2AAddrIndex as PrintableIndex>::to_string(),
            Self::P2MSOutputIndex => <P2MSOutputIndex as PrintableIndex>::to_string(),
            Self::P2PK33AddrIndex => <P2PK33AddrIndex as PrintableIndex>::to_string(),
            Self::P2PK65AddrIndex => <P2PK65AddrIndex as PrintableIndex>::to_string(),
            Self::P2PKHAddrIndex => <P2PKHAddrIndex as PrintableIndex>::to_string(),
            Self::P2SHAddrIndex => <P2SHAddrIndex as PrintableIndex>::to_string(),
            Self::P2TRAddrIndex => <P2TRAddrIndex as PrintableIndex>::to_string(),
            Self::P2WPKHAddrIndex => <P2WPKHAddrIndex as PrintableIndex>::to_string(),
            Self::P2WSHAddrIndex => <P2WSHAddrIndex as PrintableIndex>::to_string(),
            Self::UnknownOutputIndex => <UnknownOutputIndex as PrintableIndex>::to_string(),
            Self::FundedAddrIndex => <FundedAddrIndex as PrintableIndex>::to_string(),
            Self::EmptyAddrIndex => <EmptyAddrIndex as PrintableIndex>::to_string(),
            Self::ExtendedEmptyAddrIndex => <ExtendedEmptyAddrIndex as PrintableIndex>::to_string(),
        }
    }

    /// Classifies how the trailing edge of an index mutates, so the series cache
    /// can pick the right stable-count strategy.
    pub const fn cache_class(&self) -> CacheClass {
        match self {
            Self::Minute10 => CacheClass::Bucket { margin: 8 },
            Self::Minute30 => CacheClass::Bucket { margin: 3 },
            Self::Hour1 | Self::Hour4 | Self::Hour12 => CacheClass::Bucket { margin: 2 },
            Self::Day1 | Self::Day3 | Self::Week1 => CacheClass::Bucket { margin: 2 },
            Self::Month1 | Self::Month3 | Self::Month6 => CacheClass::Bucket { margin: 1 },
            Self::Year1 | Self::Year10 | Self::Halving | Self::Epoch => {
                CacheClass::Bucket { margin: 1 }
            }
            Self::Height => CacheClass::Bucket { margin: 6 },
            Self::TxIndex
            | Self::TxInIndex
            | Self::TxOutIndex
            | Self::EmptyOutputIndex
            | Self::OpReturnIndex
            | Self::P2AAddrIndex
            | Self::P2MSOutputIndex
            | Self::P2PK33AddrIndex
            | Self::P2PK65AddrIndex
            | Self::P2PKHAddrIndex
            | Self::P2SHAddrIndex
            | Self::P2TRAddrIndex
            | Self::P2WPKHAddrIndex
            | Self::P2WSHAddrIndex
            | Self::UnknownOutputIndex => CacheClass::Entity,
            Self::FundedAddrIndex | Self::EmptyAddrIndex | Self::ExtendedEmptyAddrIndex => {
                CacheClass::Mutable
            }
        }
    }

    /// Returns true if this index type is date-based.
    pub const fn is_date_based(&self) -> bool {
        matches!(
            self,
            Self::Minute10
                | Self::Minute30
                | Self::Hour1
                | Self::Hour4
                | Self::Hour12
                | Self::Day1
                | Self::Day3
                | Self::Week1
                | Self::Month1
                | Self::Month3
                | Self::Month6
                | Self::Year1
                | Self::Year10
        )
    }

    /// Convert an index value to a timestamp for time-based indexes.
    /// Returns None for non-time-based indexes.
    pub fn index_to_timestamp(&self, i: usize) -> Option<Timestamp> {
        let interval = match self {
            Self::Minute10 => MINUTE10_INTERVAL,
            Self::Minute30 => MINUTE30_INTERVAL,
            Self::Hour1 => HOUR1_INTERVAL,
            Self::Hour4 => HOUR4_INTERVAL,
            Self::Hour12 => HOUR12_INTERVAL,
            Self::Day3 => return Some(Day3::from(i).to_timestamp()),
            _ => return self.index_to_date(i).map(|d| d.into()),
        };
        Some(Timestamp::new(INDEX_EPOCH + i as u32 * interval))
    }

    /// Convert a date to an index value for day-precision or coarser indexes.
    /// Returns None for sub-daily indexes (use `timestamp_to_index` instead),
    /// non-date-based indexes, or dates before genesis.
    pub fn date_to_index(&self, date: Date) -> Option<usize> {
        if date < Date::INDEX_ZERO {
            return None;
        }
        match self {
            Self::Day1 => Day1::try_from(date).ok().map(usize::from),
            Self::Day3 => Day3::try_from(date).ok().map(usize::from),
            Self::Week1 => Week1::try_from(date).ok().map(usize::from),
            Self::Month1 => Month1::try_from(date).ok().map(usize::from),
            Self::Month3 => Month3::try_from(date).ok().map(usize::from),
            Self::Month6 => Month6::try_from(date).ok().map(usize::from),
            Self::Year1 => Year1::try_from(date).ok().map(usize::from),
            Self::Year10 => Year10::try_from(date).ok().map(usize::from),
            _ => None,
        }
    }

    /// Convert a timestamp to an index value for any date-based index.
    /// Works for both sub-daily (minute, hour) and daily+ indexes.
    /// Returns None for non-date-based indexes.
    pub fn timestamp_to_index(&self, ts: Timestamp) -> Option<usize> {
        let interval = match self {
            Self::Minute10 => MINUTE10_INTERVAL,
            Self::Minute30 => MINUTE30_INTERVAL,
            Self::Hour1 => HOUR1_INTERVAL,
            Self::Hour4 => HOUR4_INTERVAL,
            Self::Hour12 => HOUR12_INTERVAL,
            Self::Day3 => return Some(usize::from(Day3::from_timestamp(ts))),
            _ => return self.date_to_index(Date::from(ts)),
        };
        Some((*ts).saturating_sub(INDEX_EPOCH) as usize / interval as usize)
    }

    /// Convert an index value to a date for day-precision or coarser indexes.
    /// Returns None for sub-daily indexes (use `index_to_timestamp` instead)
    /// and non-date-based indexes.
    pub fn index_to_date(&self, i: usize) -> Option<Date> {
        match self {
            Self::Day1 => Some(Date::from(Day1::from(i))),
            Self::Week1 => Some(Date::from(Week1::from(i))),
            Self::Month1 => Some(Date::from(Month1::from(i))),
            Self::Year1 => Some(Date::from(Year1::from(i))),
            Self::Month3 => Some(Date::from(Month3::from(i))),
            Self::Month6 => Some(Date::from(Month6::from(i))),
            Self::Year10 => Some(Date::from(Year10::from(i))),
            _ => None,
        }
    }
}

impl TryFrom<&str> for Index {
    type Error = Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let v = value.to_lowercase();
        let v = v.as_str();
        for idx in Self::all() {
            if idx.possible_values().contains(&v) {
                return Ok(idx);
            }
        }
        Err(Error::Parse(format!("Invalid index: {value}")))
    }
}

impl fmt::Display for Index {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl<'de> Deserialize<'de> for Index {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let str = String::deserialize(deserializer)?;
        Index::try_from(str.as_str()).map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
#[path = "../tests/unit/index.rs"]
mod tests;
