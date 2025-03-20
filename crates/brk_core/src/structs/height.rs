use std::{
    fmt::{self, Debug},
    ops::{Add, AddAssign, Rem},
};

use bitcoincore_rpc::{Client, RpcApi};
use serde::{Deserialize, Serialize};
use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout};

use crate::CheckedSub;

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Default,
    Serialize,
    Deserialize,
    FromBytes,
    Immutable,
    IntoBytes,
    KnownLayout,
)]
pub struct Height(u32);

impl Height {
    pub const ZERO: Self = Height(0);
    pub const MAX: Self = Height(u32::MAX);

    pub fn new(height: u32) -> Self {
        Self(height)
    }

    pub fn write(&self, path: &std::path::Path) -> Result<(), std::io::Error> {
        std::fs::write(path, self.as_bytes())
    }

    pub fn increment(&mut self) {
        self.0 += 1;
    }

    pub fn incremented(self) -> Self {
        Self(self.0 + 1)
    }

    pub fn decrement(&mut self) {
        *self = self.decremented().unwrap();
    }

    pub fn decremented(self) -> Option<Self> {
        self.checked_sub(1_u32)
    }

    pub fn is_zero(self) -> bool {
        self == Self::ZERO
    }
}

impl PartialEq<u64> for Height {
    fn eq(&self, other: &u64) -> bool {
        self.0 == *other as u32
    }
}

impl Add<Height> for Height {
    type Output = Self;

    fn add(self, rhs: Height) -> Self::Output {
        Self::from(self.0 + rhs.0)
    }
}

impl Add<u32> for Height {
    type Output = Self;

    fn add(self, rhs: u32) -> Self::Output {
        Self::from(self.0 + rhs)
    }
}

impl Add<usize> for Height {
    type Output = Self;

    fn add(self, rhs: usize) -> Self::Output {
        Self::from(self.0 + rhs as u32)
    }
}

impl CheckedSub<Height> for Height {
    fn checked_sub(self, rhs: Self) -> Option<Self> {
        self.0.checked_sub(rhs.0).map(Self)
    }
}

impl CheckedSub<u32> for Height {
    fn checked_sub(self, rhs: u32) -> Option<Self> {
        self.0.checked_sub(rhs).map(Height::from)
    }
}

impl CheckedSub<usize> for Height {
    fn checked_sub(self, rhs: usize) -> Option<Self> {
        self.0.checked_sub(rhs as u32).map(Height::from)
    }
}

impl AddAssign<usize> for Height {
    fn add_assign(&mut self, rhs: usize) {
        *self = self.add(rhs);
    }
}

impl Rem<Height> for Height {
    type Output = Self;
    fn rem(self, rhs: Height) -> Self::Output {
        Self(self.0.rem(rhs.0))
    }
}

impl Rem<usize> for Height {
    type Output = Self;
    fn rem(self, rhs: usize) -> Self::Output {
        Self(self.0.rem(Height::from(rhs).0))
    }
}

impl fmt::Display for Height {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<u32> for Height {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl From<u64> for Height {
    fn from(value: u64) -> Self {
        Self(value as u32)
    }
}

impl From<usize> for Height {
    fn from(value: usize) -> Self {
        Self(value as u32)
    }
}
impl From<Height> for usize {
    fn from(value: Height) -> Self {
        value.0 as usize
    }
}

impl From<Height> for u32 {
    fn from(value: Height) -> Self {
        value.0
    }
}
impl From<Height> for u64 {
    fn from(value: Height) -> Self {
        value.0 as u64
    }
}

impl TryFrom<&Client> for Height {
    type Error = bitcoincore_rpc::Error;
    fn try_from(value: &Client) -> Result<Self, Self::Error> {
        Ok((value.get_block_count()? as usize).into())
    }
}

impl From<bitcoin::locktime::absolute::Height> for Height {
    fn from(value: bitcoin::locktime::absolute::Height) -> Self {
        Self(value.to_consensus_u32())
    }
}

impl From<Height> for bitcoin::locktime::absolute::Height {
    fn from(value: Height) -> Self {
        bitcoin::locktime::absolute::Height::from_consensus(value.0).unwrap()
    }
}

impl TryFrom<&std::path::Path> for Height {
    type Error = crate::Error;
    fn try_from(value: &std::path::Path) -> Result<Self, Self::Error> {
        Ok(Self::read_from_bytes(std::fs::read(value)?.as_slice())?.to_owned())
    }
}

impl TryFrom<byteview::ByteView> for Height {
    type Error = crate::Error;
    fn try_from(value: byteview::ByteView) -> Result<Self, Self::Error> {
        Ok(Self::read_from_bytes(&value)?)
    }
}

impl From<Height> for byteview::ByteView {
    fn from(value: Height) -> Self {
        Self::new(value.as_bytes())
    }
}
