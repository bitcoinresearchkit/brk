use std::fmt::{self, Debug};

use brk_error::Error;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use vecdb::PrintableIndex;

use crate::PairOutputIndex;

use super::{
    Date, DateIndex, DecadeIndex, DifficultyEpoch, EmptyAddressIndex, EmptyOutputIndex, HalvingEpoch,
    Height, LoadedAddressIndex, MonthIndex, OpReturnIndex, P2AAddressIndex, P2MSOutputIndex,
    P2PK33AddressIndex, P2PK65AddressIndex, P2PKHAddressIndex, P2SHAddressIndex, P2TRAddressIndex,
    P2WPKHAddressIndex, P2WSHAddressIndex, QuarterIndex, SemesterIndex, TxInIndex, TxIndex,
    TxOutIndex, UnknownOutputIndex, WeekIndex, YearIndex,
};

/// Aggregation dimension for querying metrics. Includes time-based (date, week, month, year),
/// block-based (height, txindex), and address/output type indexes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
#[schemars(example = Index::DateIndex)]
pub enum Index {
    DateIndex,
    DecadeIndex,
    DifficultyEpoch,
    EmptyOutputIndex,
    HalvingEpoch,
    Height,
    TxInIndex,
    MonthIndex,
    OpReturnIndex,
    TxOutIndex,
    P2AAddressIndex,
    P2MSOutputIndex,
    P2PK33AddressIndex,
    P2PK65AddressIndex,
    P2PKHAddressIndex,
    P2SHAddressIndex,
    P2TRAddressIndex,
    P2WPKHAddressIndex,
    P2WSHAddressIndex,
    QuarterIndex,
    SemesterIndex,
    TxIndex,
    UnknownOutputIndex,
    WeekIndex,
    YearIndex,
    LoadedAddressIndex,
    EmptyAddressIndex,
    PairOutputIndex,
}

impl Index {
    pub const fn all() -> [Self; 28] {
        [
            Self::DateIndex,
            Self::DecadeIndex,
            Self::DifficultyEpoch,
            Self::EmptyOutputIndex,
            Self::HalvingEpoch,
            Self::Height,
            Self::TxInIndex,
            Self::MonthIndex,
            Self::OpReturnIndex,
            Self::TxOutIndex,
            Self::P2AAddressIndex,
            Self::P2MSOutputIndex,
            Self::P2PK33AddressIndex,
            Self::P2PK65AddressIndex,
            Self::P2PKHAddressIndex,
            Self::P2SHAddressIndex,
            Self::P2TRAddressIndex,
            Self::P2WPKHAddressIndex,
            Self::P2WSHAddressIndex,
            Self::QuarterIndex,
            Self::SemesterIndex,
            Self::TxIndex,
            Self::UnknownOutputIndex,
            Self::WeekIndex,
            Self::YearIndex,
            Self::LoadedAddressIndex,
            Self::EmptyAddressIndex,
            Self::PairOutputIndex,
        ]
    }

    pub fn possible_values(&self) -> &'static [&'static str] {
        match self {
            Self::DateIndex => DateIndex::to_possible_strings(),
            Self::DecadeIndex => DecadeIndex::to_possible_strings(),
            Self::DifficultyEpoch => DifficultyEpoch::to_possible_strings(),
            Self::EmptyOutputIndex => EmptyOutputIndex::to_possible_strings(),
            Self::HalvingEpoch => HalvingEpoch::to_possible_strings(),
            Self::Height => Height::to_possible_strings(),
            Self::TxInIndex => TxInIndex::to_possible_strings(),
            Self::MonthIndex => MonthIndex::to_possible_strings(),
            Self::OpReturnIndex => OpReturnIndex::to_possible_strings(),
            Self::TxOutIndex => TxOutIndex::to_possible_strings(),
            Self::P2AAddressIndex => P2AAddressIndex::to_possible_strings(),
            Self::P2MSOutputIndex => P2MSOutputIndex::to_possible_strings(),
            Self::P2PK33AddressIndex => P2PK33AddressIndex::to_possible_strings(),
            Self::P2PK65AddressIndex => P2PK65AddressIndex::to_possible_strings(),
            Self::P2PKHAddressIndex => P2PKHAddressIndex::to_possible_strings(),
            Self::P2SHAddressIndex => P2SHAddressIndex::to_possible_strings(),
            Self::P2TRAddressIndex => P2TRAddressIndex::to_possible_strings(),
            Self::P2WPKHAddressIndex => P2WPKHAddressIndex::to_possible_strings(),
            Self::P2WSHAddressIndex => P2WSHAddressIndex::to_possible_strings(),
            Self::QuarterIndex => QuarterIndex::to_possible_strings(),
            Self::SemesterIndex => SemesterIndex::to_possible_strings(),
            Self::TxIndex => TxIndex::to_possible_strings(),
            Self::UnknownOutputIndex => UnknownOutputIndex::to_possible_strings(),
            Self::WeekIndex => WeekIndex::to_possible_strings(),
            Self::YearIndex => YearIndex::to_possible_strings(),
            Self::LoadedAddressIndex => LoadedAddressIndex::to_possible_strings(),
            Self::EmptyAddressIndex => EmptyAddressIndex::to_possible_strings(),
            Self::PairOutputIndex => PairOutputIndex::to_possible_strings(),
        }
    }

    pub fn all_possible_values() -> Vec<&'static str> {
        Self::all()
            .into_iter()
            .flat_map(|i| i.possible_values().iter().cloned())
            .collect::<Vec<_>>()
    }

    pub fn serialize_short(&self) -> &'static str {
        self.possible_values()
            .iter()
            .find(|str| str.len() > 1)
            .unwrap()
    }

    pub fn serialize_long(&self) -> &'static str {
        self.possible_values().last().unwrap()
    }

    /// Returns the query cost multiplier for this index type.
    /// Used for rate limiting to account for expensive lazy computations.
    pub const fn cost_multiplier(&self) -> usize {
        match self {
            Self::DifficultyEpoch => 60,
            _ => 1,
        }
    }

    /// Returns true if this index type is date-based.
    pub const fn is_date_based(&self) -> bool {
        matches!(
            self,
            Self::DateIndex
                | Self::WeekIndex
                | Self::MonthIndex
                | Self::YearIndex
                | Self::QuarterIndex
                | Self::SemesterIndex
                | Self::DecadeIndex
        )
    }

    /// Convert an index value to a date for date-based indexes.
    /// Returns None for non-date-based indexes.
    pub fn index_to_date(&self, i: usize) -> Option<Date> {
        match self {
            Self::DateIndex => Some(Date::from(DateIndex::from(i))),
            Self::WeekIndex => Some(Date::from(WeekIndex::from(i))),
            Self::MonthIndex => Some(Date::from(MonthIndex::from(i))),
            Self::YearIndex => Some(Date::from(YearIndex::from(i))),
            Self::QuarterIndex => Some(Date::from(QuarterIndex::from(i))),
            Self::SemesterIndex => Some(Date::from(SemesterIndex::from(i))),
            Self::DecadeIndex => Some(Date::from(DecadeIndex::from(i))),
            _ => None,
        }
    }
}

impl TryFrom<&str> for Index {
    type Error = Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(match value.to_lowercase().as_str() {
            v if (Self::DateIndex).possible_values().contains(&v) => Self::DateIndex,
            v if (Self::DecadeIndex).possible_values().contains(&v) => Self::DecadeIndex,
            v if (Self::DifficultyEpoch).possible_values().contains(&v) => Self::DifficultyEpoch,
            v if (Self::EmptyOutputIndex).possible_values().contains(&v) => Self::EmptyOutputIndex,
            v if (Self::HalvingEpoch).possible_values().contains(&v) => Self::HalvingEpoch,
            v if (Self::Height).possible_values().contains(&v) => Self::Height,
            v if (Self::TxInIndex).possible_values().contains(&v) => Self::TxInIndex,
            v if (Self::MonthIndex).possible_values().contains(&v) => Self::MonthIndex,
            v if (Self::OpReturnIndex).possible_values().contains(&v) => Self::OpReturnIndex,
            v if (Self::TxOutIndex).possible_values().contains(&v) => Self::TxOutIndex,
            v if (Self::P2AAddressIndex).possible_values().contains(&v) => Self::P2AAddressIndex,
            v if (Self::P2MSOutputIndex).possible_values().contains(&v) => Self::P2MSOutputIndex,
            v if (Self::P2PK33AddressIndex).possible_values().contains(&v) => {
                Self::P2PK33AddressIndex
            }
            v if (Self::P2PK65AddressIndex).possible_values().contains(&v) => {
                Self::P2PK65AddressIndex
            }
            v if (Self::P2PKHAddressIndex).possible_values().contains(&v) => {
                Self::P2PKHAddressIndex
            }
            v if (Self::P2SHAddressIndex).possible_values().contains(&v) => Self::P2SHAddressIndex,
            v if (Self::P2TRAddressIndex).possible_values().contains(&v) => Self::P2TRAddressIndex,
            v if (Self::P2WPKHAddressIndex).possible_values().contains(&v) => {
                Self::P2WPKHAddressIndex
            }
            v if (Self::P2WSHAddressIndex).possible_values().contains(&v) => {
                Self::P2WSHAddressIndex
            }
            v if (Self::QuarterIndex).possible_values().contains(&v) => Self::QuarterIndex,
            v if (Self::SemesterIndex).possible_values().contains(&v) => Self::SemesterIndex,
            v if (Self::TxIndex).possible_values().contains(&v) => Self::TxIndex,
            v if (Self::WeekIndex).possible_values().contains(&v) => Self::WeekIndex,
            v if (Self::YearIndex).possible_values().contains(&v) => Self::YearIndex,
            v if (Self::UnknownOutputIndex).possible_values().contains(&v) => {
                Self::UnknownOutputIndex
            }
            v if (Self::LoadedAddressIndex).possible_values().contains(&v) => {
                Self::LoadedAddressIndex
            }
            v if (Self::EmptyAddressIndex).possible_values().contains(&v) => {
                Self::EmptyAddressIndex
            }
            v if (Self::PairOutputIndex).possible_values().contains(&v) => Self::PairOutputIndex,
            _ => return Err(Error::Parse(format!("Invalid index: {value}"))),
        })
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
        if let Ok(index) = Index::try_from(str.as_str()) {
            // dbg!(index);
            Ok(index)
        } else {
            Err(serde::de::Error::custom("Bad index"))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_date_based_dateindex() {
        assert!(Index::DateIndex.is_date_based());
    }

    #[test]
    fn test_is_date_based_weekindex() {
        assert!(Index::WeekIndex.is_date_based());
    }

    #[test]
    fn test_is_date_based_monthindex() {
        assert!(Index::MonthIndex.is_date_based());
    }

    #[test]
    fn test_is_date_based_yearindex() {
        assert!(Index::YearIndex.is_date_based());
    }

    #[test]
    fn test_is_date_based_quarterindex() {
        assert!(Index::QuarterIndex.is_date_based());
    }

    #[test]
    fn test_is_date_based_semesterindex() {
        assert!(Index::SemesterIndex.is_date_based());
    }

    #[test]
    fn test_is_date_based_decadeindex() {
        assert!(Index::DecadeIndex.is_date_based());
    }

    #[test]
    fn test_is_not_date_based_height() {
        assert!(!Index::Height.is_date_based());
    }

    #[test]
    fn test_is_not_date_based_txindex() {
        assert!(!Index::TxIndex.is_date_based());
    }

    #[test]
    fn test_index_to_date_dateindex_zero() {
        let date = Index::DateIndex.index_to_date(0).unwrap();
        assert_eq!(date, Date::from(DateIndex::from(0_usize)));
    }

    #[test]
    fn test_index_to_date_dateindex_one() {
        let date = Index::DateIndex.index_to_date(1).unwrap();
        assert_eq!(date, Date::from(DateIndex::from(1_usize)));
    }

    #[test]
    fn test_index_to_date_weekindex() {
        let date = Index::WeekIndex.index_to_date(1).unwrap();
        assert_eq!(date, Date::from(WeekIndex::from(1_usize)));
    }

    #[test]
    fn test_index_to_date_monthindex() {
        let date = Index::MonthIndex.index_to_date(12).unwrap();
        assert_eq!(date, Date::from(MonthIndex::from(12_usize)));
    }

    #[test]
    fn test_index_to_date_yearindex() {
        let date = Index::YearIndex.index_to_date(5).unwrap();
        assert_eq!(date, Date::from(YearIndex::from(5_usize)));
    }

    #[test]
    fn test_index_to_date_quarterindex() {
        let date = Index::QuarterIndex.index_to_date(4).unwrap();
        assert_eq!(date, Date::from(QuarterIndex::from(4_usize)));
    }

    #[test]
    fn test_index_to_date_semesterindex() {
        let date = Index::SemesterIndex.index_to_date(2).unwrap();
        assert_eq!(date, Date::from(SemesterIndex::from(2_usize)));
    }

    #[test]
    fn test_index_to_date_decadeindex() {
        let date = Index::DecadeIndex.index_to_date(1).unwrap();
        assert_eq!(date, Date::from(DecadeIndex::from(1_usize)));
    }

    #[test]
    fn test_index_to_date_height_returns_none() {
        assert!(Index::Height.index_to_date(100).is_none());
    }

    #[test]
    fn test_index_to_date_txindex_returns_none() {
        assert!(Index::TxIndex.index_to_date(100).is_none());
    }
}
