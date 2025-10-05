use std::ops::{Add, AddAssign};

use brk_vecs::{IVecs, TreeNode};
use vecdb::AnyCollectableVec;

use crate::{GroupFilter, OutputType};

use super::{BySpendableType, ByUnspendableType};

#[derive(Default, Clone, Debug)]
pub struct GroupedByType<T> {
    pub spendable: BySpendableType<T>,
    pub unspendable: ByUnspendableType<T>,
}

impl<T> GroupedByType<T> {
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
            _ => unreachable!(),
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
            _ => unreachable!(),
        }
    }
}

impl<T> Add for GroupedByType<T>
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

impl<T> AddAssign for GroupedByType<T>
where
    T: AddAssign,
{
    fn add_assign(&mut self, rhs: Self) {
        self.spendable += rhs.spendable;
        self.unspendable += rhs.unspendable;
    }
}

// impl<T: IVecs> IVecs for GroupedByType<(GroupFilter, T)> {
//     fn to_tree_node(&self) -> TreeNode {
//         TreeNode::Branch(
//             [
//                 ("spendable", self.spendable.to_tree_node()),
//                 ("unspendable", self.unspendable.to_tree_node()),
//             ]
//             .into_iter()
//             .map(|(name, node)| (name.to_string(), node))
//             .collect(),
//         )
//     }

//     fn iter(&self) -> impl Iterator<Item = &dyn AnyCollectableVec> {
//         let mut iter: Box<dyn Iterator<Item = &dyn AnyCollectableVec>> =
//             Box::new(self.spendable.iter());
//         iter = Box::new(iter.chain(self.unspendable.iter()));
//         iter
//     }
// }
