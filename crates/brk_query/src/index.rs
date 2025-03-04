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
    Txinindex,
    Txoutindex,
}

impl Index {
    pub fn all() -> [Self; 13] {
        [
            Self::Addressindex,
            Self::Dateindex,
            Self::Height,
            Self::P2PK33index,
            Self::P2PK65index,
            Self::P2PKHindex,
            Self::P2SHindex,
            Self::P2TRindex,
            Self::P2WPKHindex,
            Self::P2WSHindex,
            Self::Txindex,
            Self::Txinindex,
            Self::Txoutindex,
        ]
    }

    pub fn possible_values(&self) -> &[&str] {
        // Always have the "correct" id at the end
        match self {
            Self::Dateindex => &["d", "date", "dateindex"],
            Self::Height => &["h", "height"],
            Self::Txindex => &["txi", "txindex"],
            Self::Txinindex => &["txini", "txinindex"],
            Self::Txoutindex => &["txouti", "txoutindex"],
            Self::Addressindex => &["addri", "addressindex"],
            Self::P2PK33index => &["p2pk33i", "p2pk33index"],
            Self::P2PK65index => &["p2pk65i", "p2pk65index"],
            Self::P2PKHindex => &["p2pkhi", "p2pkhindex"],
            Self::P2SHindex => &["p2shi", "p2shindex"],
            Self::P2TRindex => &["p2tri", "p2trindex"],
            Self::P2WPKHindex => &["p2wpkhi", "p2wpkhindex"],
            Self::P2WSHindex => &["p2wshi", "p2wshindex"],
        }
    }

    pub fn all_possible_values() -> Vec<String> {
        Self::all()
            .iter()
            .flat_map(|i| i.possible_values().iter().map(|s| s.to_string()))
            .collect::<Vec<_>>()
    }
}

impl TryFrom<&str> for Index {
    type Error = color_eyre::Report;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(match value {
            v if (Self::Dateindex).possible_values().contains(&v) => Self::Dateindex,
            v if (Self::Height).possible_values().contains(&v) => Self::Height,
            v if (Self::Txindex).possible_values().contains(&v) => Self::Txindex,
            v if (Self::Txinindex).possible_values().contains(&v) => Self::Txinindex,
            v if (Self::Txoutindex).possible_values().contains(&v) => Self::Txoutindex,
            v if (Self::Addressindex).possible_values().contains(&v) => Self::Addressindex,
            v if (Self::P2PK33index).possible_values().contains(&v) => Self::P2PK33index,
            v if (Self::P2PK65index).possible_values().contains(&v) => Self::P2PK65index,
            v if (Self::P2PKHindex).possible_values().contains(&v) => Self::P2PKHindex,
            v if (Self::P2SHindex).possible_values().contains(&v) => Self::P2SHindex,
            v if (Self::P2TRindex).possible_values().contains(&v) => Self::P2TRindex,
            v if (Self::P2WPKHindex).possible_values().contains(&v) => Self::P2WPKHindex,
            v if (Self::P2WSHindex).possible_values().contains(&v) => Self::P2WSHindex,
            _ => return Err(eyre!("Bad index")),
        })
    }
}

impl fmt::Display for Index {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Debug::fmt(self, f)
    }
}
