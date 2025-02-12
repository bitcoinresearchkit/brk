use std::fmt::{self, Debug};

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

impl TryFrom<&str> for Index {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(match value {
            "d" | "date" | "dateindex" => Self::Dateindex,
            "h" | "height" => Self::Height,
            "txi" | "txindex" => Self::Txindex,
            "txini" | "txinindex" => Self::Txinindex,
            "txouti" | "txoutindex" => Self::Txoutindex,
            "addri" | "addressindex" => Self::Addressindex,
            "p2pk33i" | "p2pk33index" => Self::P2PK33index,
            "p2pk65i" | "p2pk65index" => Self::P2PK65index,
            "p2pkhi" | "p2pkhindex" => Self::P2PKHindex,
            "p2shi" | "p2shindex" => Self::P2SHindex,
            "p2tri" | "p2trindex" => Self::P2TRindex,
            "p2wpkhi" | "p2wpkhindex" => Self::P2WPKHindex,
            "p2wshi" | "p2wshindex" => Self::P2WSHindex,
            _ => return Err(()),
        })
    }
}

impl fmt::Display for Index {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Debug::fmt(self, f)
    }
}
