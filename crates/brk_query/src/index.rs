use std::fmt::{self, Debug};

use color_eyre::eyre::eyre;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
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

    pub fn possible_values(&self) -> &[&str] {
        // Always have the "correct" id at the end
        match self {
            Self::DateIndex => &["d", "date", "dateindex"],
            Self::DecadeIndex => &["decade", "decadeindex"],
            Self::DifficultyEpoch => &["difficulty", "difficultyepoch"],
            Self::EmptyOutputIndex => &["empty", "emptyoutputindex"],
            Self::HalvingEpoch => &["h", "halving", "halvingepoch"],
            Self::Height => &["h", "height"],
            Self::InputIndex => &["txin", "inputindex"],
            Self::MonthIndex => &["m", "month", "monthindex"],
            Self::OpReturnIndex => &["opreturn", "opreturnindex"],
            Self::OutputIndex => &["txout", "outputindex"],
            Self::P2AIndex => &["p2a", "p2aindex"],
            Self::P2MSIndex => &["p2ms", "p2msindex"],
            Self::P2PK33Index => &["p2pk33", "p2pk33index"],
            Self::P2PK65Index => &["p2pk65", "p2pk65index"],
            Self::P2PKHIndex => &["p2pkh", "p2pkhindex"],
            Self::P2SHIndex => &["p2sh", "p2shindex"],
            Self::P2TRIndex => &["p2tr", "p2trindex"],
            Self::P2WPKHIndex => &["p2wpkh", "p2wpkhindex"],
            Self::P2WSHIndex => &["p2wsh", "p2wshindex"],
            Self::QuarterIndex => &["q", "quarter", "quarterindex"],
            Self::TxIndex => &["tx", "txindex"],
            Self::UnknownOutputIndex => &["unknown", "unknownoutputindex"],
            Self::WeekIndex => &["w", "week", "weekindex"],
            Self::YearIndex => &["y", "year", "yearindex"],
        }
    }

    pub fn all_possible_values() -> Vec<String> {
        Self::all()
            .iter()
            .flat_map(|i| i.possible_values().iter().map(|s| s.to_string()))
            .collect::<Vec<_>>()
    }

    pub fn serialize_short(&self) -> String {
        self.possible_values()
            .iter()
            .find(|str| str.len() > 1)
            .unwrap()
            .to_string()
    }

    pub fn serialize_long(&self) -> String {
        self.possible_values().last().unwrap().to_string()
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
        Debug::fmt(self, f)
    }
}
