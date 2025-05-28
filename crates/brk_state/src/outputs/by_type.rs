use std::ops::{Add, AddAssign};

use brk_core::OutputType;

use super::{OutputsBySpendableType, OutputsByUnspendableType};

#[derive(Default, Clone)]
pub struct OutputsByType<T> {
    pub spendable: OutputsBySpendableType<T>,
    pub unspendable: OutputsByUnspendableType<T>,
}

impl<T> OutputsByType<T> {
    pub fn get(&self, output_type: OutputType) -> &T {
        match output_type {
            OutputType::P2PK65 => &self.spendable.p2pk65,
            OutputType::P2PK33 => &self.spendable.p2pk33,
            OutputType::P2PKH => &self.spendable.p2pkh,
            OutputType::P2MS => &self.spendable.p2ms,
            OutputType::P2SH => &self.spendable.p2sh,
            OutputType::P2WPKH => &self.spendable.p2wpkh,
            OutputType::P2WSH => &self.spendable.p2wsh,
            OutputType::P2TR => &self.spendable.p2tr,
            OutputType::P2A => &self.spendable.p2a,
            OutputType::Empty => &self.spendable.empty,
            OutputType::Unknown => &self.spendable.unknown,
            OutputType::OpReturn => &self.unspendable.opreturn,
        }
    }

    pub fn get_mut(&mut self, output_type: OutputType) -> &mut T {
        match output_type {
            OutputType::P2PK65 => &mut self.spendable.p2pk65,
            OutputType::P2PK33 => &mut self.spendable.p2pk33,
            OutputType::P2PKH => &mut self.spendable.p2pkh,
            OutputType::P2MS => &mut self.spendable.p2ms,
            OutputType::P2SH => &mut self.spendable.p2sh,
            OutputType::P2WPKH => &mut self.spendable.p2wpkh,
            OutputType::P2WSH => &mut self.spendable.p2wsh,
            OutputType::P2TR => &mut self.spendable.p2tr,
            OutputType::P2A => &mut self.spendable.p2a,
            OutputType::Unknown => &mut self.spendable.unknown,
            OutputType::Empty => &mut self.spendable.empty,
            OutputType::OpReturn => &mut self.unspendable.opreturn,
        }
    }

    // pub fn as_vec(&self) -> Vec<&T> {
    // self.spendable
    //     .as_vec()
    //     .into_iter()
    //     .chain(self.unspendable.as_vec())
    //     .collect::<Vec<_>>()
    // }

    // pub fn as_mut_vec(&mut self) -> Vec<&mut T> {
    //     self.spendable
    //         .as_mut_vec()
    //         .into_iter()
    //         .chain(self.unspendable.as_mut_vec())
    //         .collect::<Vec<_>>()
    // }
}

impl<T> Add for OutputsByType<T>
where
    T: Add<Output = T>,
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            spendable: self.spendable + rhs.spendable,
            unspendable: self.unspendable + rhs.unspendable,
        }
    }
}

impl<T> AddAssign for OutputsByType<T>
where
    T: AddAssign,
{
    fn add_assign(&mut self, rhs: Self) {
        self.spendable += rhs.spendable;
        self.unspendable += rhs.unspendable;
    }
}
