use std::ops::AddAssign;

use allocative::Allocative;

use crate::structs::{AddressData, AddressLiquidity, AddressSize, AddressSplit, AddressType};

use super::AddressCohortId;

#[derive(Default, Allocative)]
pub struct SplitByAddressCohort<T> {
    pub all: T,

    pub illiquid: T,
    pub liquid: T,
    pub highly_liquid: T,

    pub plankton: T,
    pub shrimp: T,
    pub crab: T,
    pub fish: T,
    pub shark: T,
    pub whale: T,
    pub humpback: T,
    pub megalodon: T,

    pub p2pk: T,
    pub p2pkh: T,
    pub p2sh: T,
    pub p2wpkh: T,
    pub p2wsh: T,
    pub p2tr: T,
}

impl<T> SplitByAddressCohort<T> {
    pub fn get(&self, split: &AddressSplit) -> Option<&T> {
        match &split {
            AddressSplit::All => Some(&self.all),

            AddressSplit::Liquidity(address_liquidity) => match address_liquidity {
                AddressLiquidity::Illiquid => Some(&self.illiquid),
                AddressLiquidity::Liquid => Some(&self.liquid),
                AddressLiquidity::HighlyLiquid => Some(&self.highly_liquid),
            },

            AddressSplit::Type(address_type) => match address_type {
                AddressType::P2PK => Some(&self.p2pk),
                AddressType::P2PKH => Some(&self.p2pkh),
                AddressType::P2SH => Some(&self.p2sh),
                AddressType::P2WPKH => Some(&self.p2wpkh),
                AddressType::P2WSH => Some(&self.p2wsh),
                AddressType::P2TR => Some(&self.p2tr),
                AddressType::MultiSig => None,
                AddressType::Unknown => None,
                AddressType::OpReturn => None,
                AddressType::PushOnly => None,
                AddressType::Empty => None,
            },

            AddressSplit::Size(address_size) => match address_size {
                AddressSize::Plankton => Some(&self.plankton),
                AddressSize::Shrimp => Some(&self.shrimp),
                AddressSize::Crab => Some(&self.crab),
                AddressSize::Fish => Some(&self.fish),
                AddressSize::Shark => Some(&self.shark),
                AddressSize::Whale => Some(&self.whale),
                AddressSize::Humpback => Some(&self.humpback),
                AddressSize::Megalodon => Some(&self.megalodon),
                AddressSize::Empty => None,
            },
        }
    }

    pub fn iterate(
        &mut self,
        address_data: &AddressData,
        normal_iteration: impl Fn(&mut T) -> color_eyre::Result<()>,
        liquified_iteration: impl Fn(AddressLiquidity, &mut T) -> color_eyre::Result<()>,
    ) -> color_eyre::Result<()> {
        normal_iteration(self.get_mut_from_split(&AddressSplit::All).unwrap())?;

        let mut _liquified_iteration = |address_liquidity| {
            liquified_iteration(
                address_liquidity,
                self.get_mut_from_split(&AddressSplit::Liquidity(address_liquidity))
                    .unwrap(),
            )
        };

        _liquified_iteration(AddressLiquidity::Illiquid)?;
        _liquified_iteration(AddressLiquidity::Liquid)?;
        _liquified_iteration(AddressLiquidity::HighlyLiquid)?;

        if let Some(state) = self.get_mut_from_split(&AddressSplit::Type(address_data.address_type))
        {
            normal_iteration(state)?;
        }

        if let Some(state) = self.get_mut_from_split(&AddressSplit::Size(AddressSize::from_amount(
            address_data.amount,
        ))) {
            normal_iteration(state)?;
        }

        Ok(())
    }

    fn get_mut_from_split(&mut self, split: &AddressSplit) -> Option<&mut T> {
        match &split {
            AddressSplit::All => Some(&mut self.all),

            AddressSplit::Liquidity(address_liquidity) => match address_liquidity {
                AddressLiquidity::Illiquid => Some(&mut self.illiquid),
                AddressLiquidity::Liquid => Some(&mut self.liquid),
                AddressLiquidity::HighlyLiquid => Some(&mut self.highly_liquid),
            },

            AddressSplit::Type(address_type) => match address_type {
                AddressType::P2PK => Some(&mut self.p2pk),
                AddressType::P2PKH => Some(&mut self.p2pkh),
                AddressType::P2SH => Some(&mut self.p2sh),
                AddressType::P2WPKH => Some(&mut self.p2wpkh),
                AddressType::P2WSH => Some(&mut self.p2wsh),
                AddressType::P2TR => Some(&mut self.p2tr),
                AddressType::MultiSig => None,
                AddressType::Unknown => None,
                AddressType::OpReturn => None,
                AddressType::PushOnly => None,
                AddressType::Empty => None,
            },

            AddressSplit::Size(address_size) => match address_size {
                AddressSize::Plankton => Some(&mut self.plankton),
                AddressSize::Shrimp => Some(&mut self.shrimp),
                AddressSize::Crab => Some(&mut self.crab),
                AddressSize::Fish => Some(&mut self.fish),
                AddressSize::Shark => Some(&mut self.shark),
                AddressSize::Whale => Some(&mut self.whale),
                AddressSize::Humpback => Some(&mut self.humpback),
                AddressSize::Megalodon => Some(&mut self.megalodon),
                AddressSize::Empty => None,
            },
        }
    }

    pub fn get_mut_from_id(&mut self, id: &AddressCohortId) -> &mut T {
        match id {
            AddressCohortId::All => &mut self.all,

            AddressCohortId::Illiquid => &mut self.illiquid,
            AddressCohortId::Liquid => &mut self.liquid,
            AddressCohortId::HighlyLiquid => &mut self.highly_liquid,

            AddressCohortId::Plankton => &mut self.plankton,
            AddressCohortId::Shrimp => &mut self.shrimp,
            AddressCohortId::Crab => &mut self.crab,
            AddressCohortId::Fish => &mut self.fish,
            AddressCohortId::Shark => &mut self.shark,
            AddressCohortId::Whale => &mut self.whale,
            AddressCohortId::Humpback => &mut self.humpback,
            AddressCohortId::Megalodon => &mut self.megalodon,

            AddressCohortId::P2PK => &mut self.p2pk,
            AddressCohortId::P2PKH => &mut self.p2pkh,
            AddressCohortId::P2SH => &mut self.p2sh,
            AddressCohortId::P2WPKH => &mut self.p2wpkh,
            AddressCohortId::P2WSH => &mut self.p2wsh,
            AddressCohortId::P2TR => &mut self.p2tr,
        }
    }

    pub fn as_vec(&self) -> Vec<(&T, AddressCohortId)> {
        vec![
            (&self.all, AddressCohortId::All),
            (&self.illiquid, AddressCohortId::Illiquid),
            (&self.liquid, AddressCohortId::Liquid),
            (&self.highly_liquid, AddressCohortId::HighlyLiquid),
            (&self.plankton, AddressCohortId::Plankton),
            (&self.shrimp, AddressCohortId::Shrimp),
            (&self.crab, AddressCohortId::Crab),
            (&self.fish, AddressCohortId::Fish),
            (&self.shark, AddressCohortId::Shark),
            (&self.whale, AddressCohortId::Whale),
            (&self.humpback, AddressCohortId::Humpback),
            (&self.megalodon, AddressCohortId::Megalodon),
            (&self.p2pk, AddressCohortId::P2PK),
            (&self.p2pkh, AddressCohortId::P2PKH),
            (&self.p2sh, AddressCohortId::P2SH),
            (&self.p2wpkh, AddressCohortId::P2WPKH),
            (&self.p2wsh, AddressCohortId::P2WSH),
            (&self.p2tr, AddressCohortId::P2TR),
        ]
    }

    pub fn as_mut_vec(&mut self) -> Vec<(&mut T, AddressCohortId)> {
        vec![
            (&mut self.all, AddressCohortId::All),
            (&mut self.illiquid, AddressCohortId::Illiquid),
            (&mut self.liquid, AddressCohortId::Liquid),
            (&mut self.highly_liquid, AddressCohortId::HighlyLiquid),
            (&mut self.plankton, AddressCohortId::Plankton),
            (&mut self.shrimp, AddressCohortId::Shrimp),
            (&mut self.crab, AddressCohortId::Crab),
            (&mut self.fish, AddressCohortId::Fish),
            (&mut self.shark, AddressCohortId::Shark),
            (&mut self.whale, AddressCohortId::Whale),
            (&mut self.humpback, AddressCohortId::Humpback),
            (&mut self.megalodon, AddressCohortId::Megalodon),
            (&mut self.p2pk, AddressCohortId::P2PK),
            (&mut self.p2pkh, AddressCohortId::P2PKH),
            (&mut self.p2sh, AddressCohortId::P2SH),
            (&mut self.p2wpkh, AddressCohortId::P2WPKH),
            (&mut self.p2wsh, AddressCohortId::P2WSH),
            (&mut self.p2tr, AddressCohortId::P2TR),
        ]
    }
}

impl<T> AddAssign for SplitByAddressCohort<T>
where
    T: AddAssign,
{
    fn add_assign(&mut self, rhs: Self) {
        self.all += rhs.all;

        self.illiquid += rhs.illiquid;
        self.liquid += rhs.liquid;
        self.highly_liquid += rhs.highly_liquid;

        self.plankton += rhs.plankton;
        self.shrimp += rhs.shrimp;
        self.crab += rhs.crab;
        self.fish += rhs.fish;
        self.shark += rhs.shark;
        self.whale += rhs.whale;
        self.humpback += rhs.humpback;
        self.megalodon += rhs.megalodon;

        self.p2pk += rhs.p2pk;
        self.p2pkh += rhs.p2pkh;
        self.p2sh += rhs.p2sh;
        self.p2wpkh += rhs.p2wpkh;
        self.p2wsh += rhs.p2wsh;
        self.p2tr += rhs.p2tr;
    }
}
