use std::ops::Add;

use derive_more::{Deref, DerefMut};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use crate::CheckedSub;
# [cfg (feature = "storage")] use vecdb :: { Formattable , Pco , PrintableIndex } ;

/// Index for 2-output transactions (oracle pair candidates)
///
/// This indexes all transactions with exactly 2 outputs, which are
/// candidates for the UTXOracle algorithm (payment + change pattern).
# [derive (Debug , PartialEq , Eq , PartialOrd , Ord , Clone , Copy , Deref , DerefMut , Default , Serialize , Deserialize , JsonSchema , Hash)] # [cfg_attr (feature = "storage" , derive (Pco))]
pub struct PairOutputIndex(u32);

impl PairOutputIndex {
    pub const ZERO: Self = Self(0);

    pub fn new(index: u32) -> Self {
        Self(index)
    }

    pub fn incremented(self) -> Self {
        Self(*self + 1)
    }
}

impl Add<usize> for PairOutputIndex {
    type Output = Self;
    fn add(self, rhs: usize) -> Self::Output {
        Self(self.0 + rhs as u32)
    }
}

impl CheckedSub < PairOutputIndex > for PairOutputIndex { fn checked_sub (self , rhs : PairOutputIndex) -> Option < Self > { self . 0 . checked_sub (rhs . 0) . map (PairOutputIndex :: from) } }
# [cfg (feature = "storage")] impl vecdb :: CheckedSub < PairOutputIndex > for PairOutputIndex { fn checked_sub (self , rhs : PairOutputIndex) -> Option < Self > { crate :: CheckedSub :: checked_sub (self , rhs) } }

impl From<u32> for PairOutputIndex {
    #[inline]
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl From<PairOutputIndex> for u32 {
    #[inline]
    fn from(value: PairOutputIndex) -> Self {
        value.0
    }
}

impl From<u64> for PairOutputIndex {
    #[inline]
    fn from(value: u64) -> Self {
        Self(value as u32)
    }
}

impl From<PairOutputIndex> for u64 {
    #[inline]
    fn from(value: PairOutputIndex) -> Self {
        value.0 as u64
    }
}

impl From<usize> for PairOutputIndex {
    #[inline]
    fn from(value: usize) -> Self {
        Self(value as u32)
    }
}

impl From<PairOutputIndex> for usize {
    #[inline]
    fn from(value: PairOutputIndex) -> Self {
        value.0 as usize
    }
}

impl PairOutputIndex { pub fn index_name () -> & 'static str { "pair_output_index" } pub fn index_aliases () -> & 'static [& 'static str] { & ["pairoutput" , "pair_output_index"] } }
#[cfg(feature = "storage")]
impl PrintableIndex for PairOutputIndex { fn to_string () -> & 'static str { Self :: index_name () } fn to_possible_strings () -> & 'static [& 'static str] { Self :: index_aliases () } }

impl std::fmt::Display for PairOutputIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buf = itoa::Buffer::new();
        let str = buf.format(self.0);
        f.write_str(str)
    }
}

#[cfg(feature = "storage")]
impl Formattable for PairOutputIndex {
    #[inline(always)]
    fn write_to(&self, buf: &mut Vec<u8>) {
        let mut b = itoa::Buffer::new();
        buf.extend_from_slice(b.format(self.0).as_bytes());
    }
}
