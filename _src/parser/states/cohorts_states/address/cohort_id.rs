use crate::structs::{AddressLiquidity, AddressSize, AddressSplit, AddressType};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum AddressCohortId {
    All,

    Illiquid,
    Liquid,
    HighlyLiquid,

    Plankton,
    Shrimp,
    Crab,
    Fish,
    Shark,
    Whale,
    Humpback,
    Megalodon,

    P2PK,
    P2PKH,
    P2SH,
    P2WPKH,
    P2WSH,
    P2TR,
}

impl AddressCohortId {
    pub fn as_name(&self) -> Option<&str> {
        match self {
            Self::All => None,

            Self::Illiquid => Some("illiquid"),
            Self::Liquid => Some("liquid"),
            Self::HighlyLiquid => Some("highly_liquid"),

            Self::Plankton => Some("plankton"),
            Self::Shrimp => Some("shrimp"),
            Self::Crab => Some("crab"),
            Self::Fish => Some("fish"),
            Self::Shark => Some("shark"),
            Self::Whale => Some("whale"),
            Self::Humpback => Some("humpback"),
            Self::Megalodon => Some("megalodon"),

            Self::P2PK => Some("p2pk"),
            Self::P2PKH => Some("p2pkh"),
            Self::P2SH => Some("p2sh"),
            Self::P2WPKH => Some("p2wpkh"),
            Self::P2WSH => Some("p2wsh"),
            Self::P2TR => Some("p2tr"),
        }
    }

    pub fn as_split(&self) -> AddressSplit {
        match self {
            Self::All => AddressSplit::All,

            Self::Illiquid => AddressSplit::Liquidity(AddressLiquidity::Illiquid),
            Self::Liquid => AddressSplit::Liquidity(AddressLiquidity::Liquid),
            Self::HighlyLiquid => AddressSplit::Liquidity(AddressLiquidity::HighlyLiquid),

            Self::Plankton => AddressSplit::Size(AddressSize::Plankton),
            Self::Shrimp => AddressSplit::Size(AddressSize::Shrimp),
            Self::Crab => AddressSplit::Size(AddressSize::Crab),
            Self::Fish => AddressSplit::Size(AddressSize::Fish),
            Self::Shark => AddressSplit::Size(AddressSize::Shark),
            Self::Whale => AddressSplit::Size(AddressSize::Whale),
            Self::Humpback => AddressSplit::Size(AddressSize::Humpback),
            Self::Megalodon => AddressSplit::Size(AddressSize::Megalodon),

            Self::P2PK => AddressSplit::Type(AddressType::P2PK),
            Self::P2PKH => AddressSplit::Type(AddressType::P2PKH),
            Self::P2SH => AddressSplit::Type(AddressType::P2SH),
            Self::P2WPKH => AddressSplit::Type(AddressType::P2WPKH),
            Self::P2WSH => AddressSplit::Type(AddressType::P2WSH),
            Self::P2TR => AddressSplit::Type(AddressType::P2TR),
        }
    }
}
