#![allow(unused)]

use std::{
    iter::Sum,
    ops::{Add, AddAssign, SubAssign},
};

use brk_core::{CheckedSub, Sats, StoredU32};
use serde::Serialize;
use zerocopy_derive::{FromBytes, Immutable, IntoBytes, KnownLayout};

#[derive(Debug, Default, Clone, FromBytes, Immutable, IntoBytes, KnownLayout, Serialize)]
pub struct BlockState {
    pub utxos: usize,
    pub value: Sats,
}

impl Add<BlockState> for BlockState {
    type Output = Self;
    fn add(self, rhs: BlockState) -> Self::Output {
        Self {
            utxos: self.utxos + rhs.utxos,
            value: self.value + rhs.value,
        }
    }
}

impl AddAssign<&BlockState> for BlockState {
    fn add_assign(&mut self, rhs: &BlockState) {
        self.utxos += rhs.utxos;
        self.value += rhs.value;
    }
}

impl SubAssign for BlockState {
    fn sub_assign(&mut self, rhs: Self) {
        self.utxos = self.utxos.checked_sub(rhs.utxos).unwrap();
        self.value = self.value.checked_sub(rhs.value).unwrap();
    }
}
