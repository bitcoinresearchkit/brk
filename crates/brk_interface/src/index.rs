use std::fmt::{self, Debug};

use brk_core::{
    DateIndex, DecadeIndex, DifficultyEpoch, EmptyOutputIndex, HalvingEpoch, Height, InputIndex,
    MonthIndex, OpReturnIndex, OutputIndex, P2AIndex, P2MSIndex, P2PK33Index, P2PK65Index,
    P2PKHIndex, P2SHIndex, P2TRIndex, P2WPKHIndex, P2WSHIndex, Printable, QuarterIndex, TxIndex,
    UnknownOutputIndex, WeekIndex, YearIndex,
};
use color_eyre::eyre::eyre;
use schemars::JsonSchema;
use serde::{Deserialize, de::Error};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, JsonSchema)]
pub enum Index {
    DateIndex,
    DecadeIndex,
    DifficultyEpoch,
    EmptyOutputIndex,
    HalvingEpoch,
    Height,
    InputIndex,
    MonthIndex,
    OpReturnIndex,
    OutputIndex,
    P2AIndex,
    P2MSIndex,
    P2PK33Index,
    P2PK65Index,
    P2PKHIndex,
    P2SHIndex,
    P2TRIndex,
    P2WPKHIndex,
    P2WSHIndex,
    QuarterIndex,
    TxIndex,
    UnknownOutputIndex,
    WeekIndex,
    YearIndex,
}

impl Index {
    pub fn all() -> [Self; 24] {
        [
            Self::DateIndex,
            Self::DecadeIndex,
            Self::DifficultyEpoch,
            Self::EmptyOutputIndex,
            Self::HalvingEpoch,
            Self::Height,
            Self::InputIndex,
            Self::MonthIndex,
            Self::OpReturnIndex,
            Self::OutputIndex,
            Self::P2AIndex,
            Self::P2MSIndex,
            Self::P2PK33Index,
            Self::P2PK65Index,
            Self::P2PKHIndex,
            Self::P2SHIndex,
            Self::P2TRIndex,
            Self::P2WPKHIndex,
            Self::P2WSHIndex,
            Self::QuarterIndex,
            Self::TxIndex,
            Self::UnknownOutputIndex,
            Self::WeekIndex,
            Self::YearIndex,
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
            Self::InputIndex => InputIndex::to_possible_strings(),
            Self::MonthIndex => MonthIndex::to_possible_strings(),
            Self::OpReturnIndex => OpReturnIndex::to_possible_strings(),
            Self::OutputIndex => OutputIndex::to_possible_strings(),
            Self::P2AIndex => P2AIndex::to_possible_strings(),
            Self::P2MSIndex => P2MSIndex::to_possible_strings(),
            Self::P2PK33Index => P2PK33Index::to_possible_strings(),
            Self::P2PK65Index => P2PK65Index::to_possible_strings(),
            Self::P2PKHIndex => P2PKHIndex::to_possible_strings(),
            Self::P2SHIndex => P2SHIndex::to_possible_strings(),
            Self::P2TRIndex => P2TRIndex::to_possible_strings(),
            Self::P2WPKHIndex => P2WPKHIndex::to_possible_strings(),
            Self::P2WSHIndex => P2WSHIndex::to_possible_strings(),
            Self::QuarterIndex => QuarterIndex::to_possible_strings(),
            Self::TxIndex => TxIndex::to_possible_strings(),
            Self::UnknownOutputIndex => UnknownOutputIndex::to_possible_strings(),
            Self::WeekIndex => WeekIndex::to_possible_strings(),
            Self::YearIndex => YearIndex::to_possible_strings(),
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
}

impl TryFrom<&str> for Index {
    type Error = color_eyre::Report;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(match value.to_lowercase().as_str() {
            v if (Self::DateIndex).possible_values().contains(&v) => Self::DateIndex,
            v if (Self::DecadeIndex).possible_values().contains(&v) => Self::DecadeIndex,
            v if (Self::DifficultyEpoch).possible_values().contains(&v) => Self::DifficultyEpoch,
            v if (Self::EmptyOutputIndex).possible_values().contains(&v) => Self::EmptyOutputIndex,
            v if (Self::HalvingEpoch).possible_values().contains(&v) => Self::HalvingEpoch,
            v if (Self::Height).possible_values().contains(&v) => Self::Height,
            v if (Self::InputIndex).possible_values().contains(&v) => Self::InputIndex,
            v if (Self::MonthIndex).possible_values().contains(&v) => Self::MonthIndex,
            v if (Self::OpReturnIndex).possible_values().contains(&v) => Self::OpReturnIndex,
            v if (Self::OutputIndex).possible_values().contains(&v) => Self::OutputIndex,
            v if (Self::P2AIndex).possible_values().contains(&v) => Self::P2AIndex,
            v if (Self::P2MSIndex).possible_values().contains(&v) => Self::P2MSIndex,
            v if (Self::P2PK33Index).possible_values().contains(&v) => Self::P2PK33Index,
            v if (Self::P2PK65Index).possible_values().contains(&v) => Self::P2PK65Index,
            v if (Self::P2PKHIndex).possible_values().contains(&v) => Self::P2PKHIndex,
            v if (Self::P2SHIndex).possible_values().contains(&v) => Self::P2SHIndex,
            v if (Self::P2TRIndex).possible_values().contains(&v) => Self::P2TRIndex,
            v if (Self::P2WPKHIndex).possible_values().contains(&v) => Self::P2WPKHIndex,
            v if (Self::P2WSHIndex).possible_values().contains(&v) => Self::P2WSHIndex,
            v if (Self::QuarterIndex).possible_values().contains(&v) => Self::QuarterIndex,
            v if (Self::QuarterIndex).possible_values().contains(&v) => Self::QuarterIndex,
            v if (Self::TxIndex).possible_values().contains(&v) => Self::TxIndex,
            v if (Self::WeekIndex).possible_values().contains(&v) => Self::WeekIndex,
            v if (Self::YearIndex).possible_values().contains(&v) => Self::YearIndex,
            v if (Self::UnknownOutputIndex).possible_values().contains(&v) => {
                Self::UnknownOutputIndex
            }
            _ => return Err(eyre!("Bad index")),
        })
    }
}

impl fmt::Display for Index {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
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
            Err(Error::custom("Bad index"))
        }
    }
}
