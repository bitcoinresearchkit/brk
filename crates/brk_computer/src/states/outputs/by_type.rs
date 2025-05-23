use brk_core::OutputType;

use super::{OutputsBySpendableType, OutputsByUnspendableType};

#[derive(Default, Clone)]
pub struct OutputsByType<T> {
    pub spendable: OutputsBySpendableType<T>,
    pub unspendable: OutputsByUnspendableType<T>,
}

impl<T> OutputsByType<T> {
    // pub fn get(&self, output_type: OutputType) -> &T {
    //     match output_type {
    //         OutputType::P2PK65 => &self.spendable.p2pk65,
    //         OutputType::P2PK33 => &self.spendable.p2pk33,
    //         OutputType::P2PKH => &self.spendable.p2pkh,
    //         OutputType::P2MS => &self.spendable.p2ms,
    //         OutputType::P2SH => &self.spendable.p2sh,
    //         OutputType::P2WPKH => &self.spendable.p2wpkh,
    //         OutputType::P2WSH => &self.spendable.p2wsh,
    //         OutputType::P2TR => &self.spendable.p2tr,
    //         OutputType::P2A => &self.spendable.p2a,
    //         OutputType::OpReturn => &self.unspendable.op_return,
    //         OutputType::Empty => &self.unspendable.empty,
    //         OutputType::Unknown => &self.unspendable.unknown,
    //     }
    // }

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
            OutputType::OpReturn => &mut self.unspendable.op_return,
            OutputType::Empty => &mut self.unspendable.empty,
        }
    }

    pub fn as_vec(&self) -> Vec<&T> {
        self.spendable
            .as_vec()
            .into_iter()
            .chain(self.unspendable.as_vec())
            .collect::<Vec<_>>()
    }

    // pub fn as_mut_vec(&mut self) -> Vec<&mut T> {
    //     self.spendable
    //         .as_mut_vec()
    //         .into_iter()
    //         .chain(self.unspendable.as_mut_vec())
    //         .collect::<Vec<_>>()
    // }
}
