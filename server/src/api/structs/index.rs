use std::fmt::{self, Debug};

use computer::Date;
use indexer::{
    Addressindex, Height, P2PK33index, P2PK65index, P2PKHindex, P2SHindex, P2TRindex, P2WPKHindex, P2WSHindex, Txindex,
    Txinindex, Txoutindex,
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Index {
    Addressindex,
    Date,
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
            "addri" | "addressindex" => Self::Addressindex,
            "d" | "date" => Self::Date,
            "h" | "height" => Self::Height,
            "p2pk33index" => Self::P2PK33index,
            "p2pk65index" => Self::P2PK65index,
            "p2pkhindex" => Self::P2PKHindex,
            "p2shindex" => Self::P2SHindex,
            "p2trindex" => Self::P2TRindex,
            "p2wpkhindex" => Self::P2WPKHindex,
            "p2wshindex" => Self::P2WSHindex,
            "txi" | "txindex" => Self::Txindex,
            "txini" | "txinindex" => Self::Txinindex,
            "txouti" | "txoutindex" => Self::Txoutindex,
            _ => return Err(()),
        })
    }
}

impl fmt::Display for Index {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Debug::fmt(self, f)
    }
}

pub trait IndexTypeToIndexEnum {
    fn to_enum() -> Index;
}

impl IndexTypeToIndexEnum for Addressindex {
    fn to_enum() -> Index {
        Index::Addressindex
    }
}
impl IndexTypeToIndexEnum for Date {
    fn to_enum() -> Index {
        Index::Date
    }
}
impl IndexTypeToIndexEnum for Height {
    fn to_enum() -> Index {
        Index::Height
    }
}
impl IndexTypeToIndexEnum for Txindex {
    fn to_enum() -> Index {
        Index::Txindex
    }
}
impl IndexTypeToIndexEnum for Txinindex {
    fn to_enum() -> Index {
        Index::Txinindex
    }
}
impl IndexTypeToIndexEnum for Txoutindex {
    fn to_enum() -> Index {
        Index::Txoutindex
    }
}
impl IndexTypeToIndexEnum for P2PK33index {
    fn to_enum() -> Index {
        Index::P2PK33index
    }
}
impl IndexTypeToIndexEnum for P2PK65index {
    fn to_enum() -> Index {
        Index::P2PK65index
    }
}
impl IndexTypeToIndexEnum for P2PKHindex {
    fn to_enum() -> Index {
        Index::P2PKHindex
    }
}
impl IndexTypeToIndexEnum for P2SHindex {
    fn to_enum() -> Index {
        Index::P2SHindex
    }
}
impl IndexTypeToIndexEnum for P2TRindex {
    fn to_enum() -> Index {
        Index::P2TRindex
    }
}
impl IndexTypeToIndexEnum for P2WPKHindex {
    fn to_enum() -> Index {
        Index::P2WPKHindex
    }
}
impl IndexTypeToIndexEnum for P2WSHindex {
    fn to_enum() -> Index {
        Index::P2WSHindex
    }
}
