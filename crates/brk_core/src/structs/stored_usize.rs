use std::ops::{Add, Div};

use derive_deref::Deref;
use serde::Serialize;
use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout};

use crate::CheckedSub;

use super::{
    DateIndex, EmptyOutputIndex, Height, InputIndex, MonthIndex, OpReturnIndex, OutputIndex,
    P2AIndex, P2MSIndex, P2PK33Index, P2PK65Index, P2PKHIndex, P2SHIndex, P2TRIndex, P2WPKHIndex,
    P2WSHIndex, TxIndex, UnknownOutputIndex, YearIndex,
};

#[derive(
    Debug,
    Deref,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    FromBytes,
    Immutable,
    IntoBytes,
    KnownLayout,
    Serialize,
)]
pub struct StoredUsize(usize);

impl StoredUsize {
    pub const ZERO: Self = Self(0);

    pub fn new(counter: usize) -> Self {
        Self(counter)
    }
}

impl From<usize> for StoredUsize {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

impl CheckedSub<StoredUsize> for StoredUsize {
    fn checked_sub(self, rhs: Self) -> Option<Self> {
        self.0.checked_sub(rhs.0).map(Self)
    }
}

impl Div<usize> for StoredUsize {
    type Output = Self;
    fn div(self, rhs: usize) -> Self::Output {
        Self(self.0 / rhs)
    }
}

impl Add for StoredUsize {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl From<f64> for StoredUsize {
    fn from(value: f64) -> Self {
        if value < 0.0 || value > u32::MAX as f64 {
            panic!()
        }
        Self(value as usize)
    }
}

impl From<StoredUsize> for f64 {
    fn from(value: StoredUsize) -> Self {
        value.0 as f64
    }
}

impl From<Height> for StoredUsize {
    fn from(value: Height) -> Self {
        Self::from(usize::from(value))
    }
}

impl From<DateIndex> for StoredUsize {
    fn from(value: DateIndex) -> Self {
        Self::from(usize::from(value))
    }
}

impl From<MonthIndex> for StoredUsize {
    fn from(value: MonthIndex) -> Self {
        Self::from(usize::from(value))
    }
}

impl From<YearIndex> for StoredUsize {
    fn from(value: YearIndex) -> Self {
        Self::from(usize::from(value))
    }
}

impl From<OutputIndex> for StoredUsize {
    fn from(value: OutputIndex) -> Self {
        Self::from(usize::from(value))
    }
}

impl From<InputIndex> for StoredUsize {
    fn from(value: InputIndex) -> Self {
        Self::from(usize::from(value))
    }
}

impl From<TxIndex> for StoredUsize {
    fn from(value: TxIndex) -> Self {
        Self::from(usize::from(value))
    }
}

impl From<P2PK65Index> for StoredUsize {
    fn from(value: P2PK65Index) -> Self {
        Self::from(usize::from(value))
    }
}

impl From<P2PK33Index> for StoredUsize {
    fn from(value: P2PK33Index) -> Self {
        Self::from(usize::from(value))
    }
}

impl From<P2PKHIndex> for StoredUsize {
    fn from(value: P2PKHIndex) -> Self {
        Self::from(usize::from(value))
    }
}

impl From<OpReturnIndex> for StoredUsize {
    fn from(value: OpReturnIndex) -> Self {
        Self::from(usize::from(value))
    }
}

impl From<P2MSIndex> for StoredUsize {
    fn from(value: P2MSIndex) -> Self {
        Self::from(usize::from(value))
    }
}

impl From<P2SHIndex> for StoredUsize {
    fn from(value: P2SHIndex) -> Self {
        Self::from(usize::from(value))
    }
}

impl From<P2WSHIndex> for StoredUsize {
    fn from(value: P2WSHIndex) -> Self {
        Self::from(usize::from(value))
    }
}

impl From<P2WPKHIndex> for StoredUsize {
    fn from(value: P2WPKHIndex) -> Self {
        Self::from(usize::from(value))
    }
}

impl From<P2TRIndex> for StoredUsize {
    fn from(value: P2TRIndex) -> Self {
        Self::from(usize::from(value))
    }
}

impl From<P2AIndex> for StoredUsize {
    fn from(value: P2AIndex) -> Self {
        Self::from(usize::from(value))
    }
}

impl From<UnknownOutputIndex> for StoredUsize {
    fn from(value: UnknownOutputIndex) -> Self {
        Self::from(usize::from(value))
    }
}

impl From<EmptyOutputIndex> for StoredUsize {
    fn from(value: EmptyOutputIndex) -> Self {
        Self::from(usize::from(value))
    }
}
