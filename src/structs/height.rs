use std::{
    fmt,
    ops::{Add, AddAssign, Sub},
};

use allocative::Allocative;
use bincode::{Decode, Encode};
use biter::NUMBER_OF_UNSAFE_BLOCKS;
use derive_deref::{Deref, DerefMut};
use serde::{Deserialize, Serialize};

use super::{HeightMapChunkId, MapKey, HEIGHT_MAP_CHUNK_SIZE};

#[derive(
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Clone,
    Copy,
    Deref,
    DerefMut,
    Default,
    Serialize,
    Deserialize,
    Encode,
    Decode,
    Allocative,
)]
pub struct Height(u32);

impl Height {
    pub const ZERO: Height = Height(0);

    pub fn new(height: u32) -> Self {
        Self(height)
    }

    pub fn is_close_to_end(&self, block_count: usize) -> bool {
        **self > (block_count - (NUMBER_OF_UNSAFE_BLOCKS * 3)) as u32
    }

    pub fn is_safe(&self, block_count: usize) -> bool {
        **self < (block_count - NUMBER_OF_UNSAFE_BLOCKS) as u32
    }

    // pub fn iter_range_inclusive(first: Height, last: Height) -> impl Iterator<Item = Height> {
    //     let range = (*first)..=(*last);

    //     range.into_iter().map(Height::new)
    // }
}

impl PartialEq<u64> for Height {
    fn eq(&self, other: &u64) -> bool {
        **self == *other as u32
    }
}

impl Add<u32> for Height {
    type Output = Height;

    fn add(self, rhs: u32) -> Self::Output {
        Self::new(*self + rhs)
    }
}

impl Add<usize> for Height {
    type Output = Height;

    fn add(self, rhs: usize) -> Self::Output {
        Self::new(*self + rhs as u32)
    }
}

impl Sub<Height> for Height {
    type Output = Height;

    fn sub(self, rhs: Height) -> Self::Output {
        Self::new(*self - *rhs)
    }
}

impl Sub<u32> for Height {
    type Output = Height;

    fn sub(self, rhs: u32) -> Self::Output {
        Self::new(*self - rhs)
    }
}

impl Sub<usize> for Height {
    type Output = Height;

    fn sub(self, rhs: usize) -> Self::Output {
        Self::new(*self - rhs as u32)
    }
}

impl AddAssign<usize> for Height {
    fn add_assign(&mut self, rhs: usize) {
        *self = self.add(rhs);
    }
}

impl fmt::Display for Height {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", **self)
    }
}

impl MapKey<HeightMapChunkId> for Height {
    fn to_chunk_id(&self) -> HeightMapChunkId {
        HeightMapChunkId::new(self)
    }

    fn to_first_unsafe(&self) -> Option<Self> {
        let offset = NUMBER_OF_UNSAFE_BLOCKS - 1;

        self.checked_sub(offset)
    }

    fn to_serialized_key(&self) -> Self {
        Height::new(**self % HEIGHT_MAP_CHUNK_SIZE)
    }

    fn is_out_of_bounds(&self) -> bool {
        !(0..=2_100_000).contains(&**self)
    }

    fn is_first(&self) -> bool {
        **self == 0
    }

    fn checked_sub(&self, x: usize) -> Option<Self> {
        (**self).checked_sub(x as u32).map(Height::new)
    }

    fn min_percentile_key() -> Self {
        Self(160_000)
    }

    fn iter_up_to(&self, other: &Self) -> impl Iterator<Item = Self> {
        (**self..=**other).map(Height::new)
    }

    fn map_name<'a>() -> &'a str {
        "height"
    }

    fn to_usize(&self) -> usize {
        (**self) as usize
    }

    fn from_usize(h: usize) -> Self {
        Self(h as u32)
    }
}
