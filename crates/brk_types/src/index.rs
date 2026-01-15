use std::fmt::{self, Debug};

use brk_error::Error;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use vecdb::PrintableIndex;

use crate::PairOutputIndex;

use super::{
    DateIndex, DecadeIndex, DifficultyEpoch, EmptyAddressIndex, EmptyOutputIndex, HalvingEpoch,
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
