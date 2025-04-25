use std::fmt::{self, Debug};

use color_eyre::eyre::eyre;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Index {
    Addressindex,
    Dateindex,
    Height,
    P2PK33index,
    P2PK65index,
    P2PKHindex,
    P2SHindex,
    P2TRindex,
    P2WPKHindex,
    P2WSHindex,
    Txindex,
    Inputindex,
    Outputindex,
    Weekindex,
    Monthindex,
    Quarterindex,
    Yearindex,
    Decadeindex,
    Difficultyepoch,
    Halvingepoch,
    Emptyindex,
    P2MSindex,
    Opreturnindex,
    Pushonlyindex,
    Unknownindex,
}

impl Index {
    pub fn all() -> [Self; 25] {
        [
            Self::Height,
            Self::Dateindex,
            Self::Weekindex,
            Self::Difficultyepoch,
            Self::Monthindex,
            Self::Quarterindex,
            Self::Yearindex,
            Self::Decadeindex,
            Self::Halvingepoch,
            Self::Addressindex,
            Self::P2PK33index,
            Self::P2PK65index,
            Self::P2PKHindex,
            Self::P2SHindex,
            Self::P2TRindex,
            Self::P2WPKHindex,
            Self::P2WSHindex,
            Self::Txindex,
            Self::Inputindex,
            Self::Outputindex,
            Self::Emptyindex,
            Self::P2MSindex,
            Self::Opreturnindex,
            Self::Pushonlyindex,
            Self::Unknownindex,
        ]
    }

    pub fn possible_values(&self) -> &[&str] {
        // Always have the "correct" id at the end
        match self {
            Self::Height => &["h", "height"],
            Self::Dateindex => &["d", "date", "dateindex"],
            Self::Weekindex => &["w", "week", "weekindex"],
            Self::Difficultyepoch => &["difficulty", "difficultyepoch"],
            Self::Monthindex => &["m", "month", "monthindex"],
            Self::Quarterindex => &["q", "quarter", "quarterindex"],
            Self::Yearindex => &["y", "year", "yearindex"],
            Self::Decadeindex => &["decade", "decadeindex"],
            Self::Halvingepoch => &["h", "halving", "halvingepoch"],
            Self::Txindex => &["tx", "txindex"],
            Self::Inputindex => &["txin", "inputindex"],
            Self::Outputindex => &["txout", "outputindex"],
            Self::Addressindex => &["a", "address", "addressindex"],
            Self::P2PK33index => &["p2pk33", "p2pk33index"],
            Self::P2PK65index => &["p2pk65", "p2pk65index"],
            Self::P2PKHindex => &["p2pkh", "p2pkhindex"],
            Self::P2SHindex => &["p2sh", "p2shindex"],
            Self::P2TRindex => &["p2tr", "p2trindex"],
            Self::P2WPKHindex => &["p2wpkh", "p2wpkhindex"],
            Self::P2WSHindex => &["p2wsh", "p2wshindex"],
            Self::Emptyindex => &["empty", "emptyoutputindex"],
            Self::P2MSindex => &["multisig", "p2msindex"],
            Self::Opreturnindex => &["opreturn", "opreturnindex"],
            Self::Pushonlyindex => &["pushonly", "pushonlyindex"],
            Self::Unknownindex => &["unknown", "unknownoutputindex"],
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
            v if (Self::Dateindex).possible_values().contains(&v) => Self::Dateindex,
            v if (Self::Height).possible_values().contains(&v) => Self::Height,
            v if (Self::Txindex).possible_values().contains(&v) => Self::Txindex,
            v if (Self::Inputindex).possible_values().contains(&v) => Self::Inputindex,
            v if (Self::Outputindex).possible_values().contains(&v) => Self::Outputindex,
            v if (Self::Addressindex).possible_values().contains(&v) => Self::Addressindex,
            v if (Self::P2PK33index).possible_values().contains(&v) => Self::P2PK33index,
            v if (Self::P2PK65index).possible_values().contains(&v) => Self::P2PK65index,
            v if (Self::P2PKHindex).possible_values().contains(&v) => Self::P2PKHindex,
            v if (Self::P2SHindex).possible_values().contains(&v) => Self::P2SHindex,
            v if (Self::P2TRindex).possible_values().contains(&v) => Self::P2TRindex,
            v if (Self::P2WPKHindex).possible_values().contains(&v) => Self::P2WPKHindex,
            v if (Self::P2WSHindex).possible_values().contains(&v) => Self::P2WSHindex,
            v if (Self::Weekindex).possible_values().contains(&v) => Self::Weekindex,
            v if (Self::Monthindex).possible_values().contains(&v) => Self::Monthindex,
            v if (Self::Yearindex).possible_values().contains(&v) => Self::Yearindex,
            v if (Self::Decadeindex).possible_values().contains(&v) => Self::Decadeindex,
            v if (Self::Difficultyepoch).possible_values().contains(&v) => Self::Difficultyepoch,
            v if (Self::Halvingepoch).possible_values().contains(&v) => Self::Halvingepoch,
            v if (Self::Quarterindex).possible_values().contains(&v) => Self::Quarterindex,
            v if (Self::Quarterindex).possible_values().contains(&v) => Self::Quarterindex,
            v if (Self::Emptyindex).possible_values().contains(&v) => Self::Emptyindex,
            v if (Self::P2MSindex).possible_values().contains(&v) => Self::P2MSindex,
            v if (Self::Opreturnindex).possible_values().contains(&v) => Self::Opreturnindex,
            v if (Self::Pushonlyindex).possible_values().contains(&v) => Self::Pushonlyindex,
            v if (Self::Unknownindex).possible_values().contains(&v) => Self::Unknownindex,
            _ => return Err(eyre!("Bad index")),
        })
    }
}

impl fmt::Display for Index {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Debug::fmt(self, f)
    }
}
