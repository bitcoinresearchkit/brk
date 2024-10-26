use super::{Height, MapKey};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Epoch(pub u16);

impl Epoch {
    pub const BLOCKS_PER_EPOCH: usize = 210_000;
}

impl From<Height> for Epoch {
    fn from(height: Height) -> Self {
        Self(((height.to_usize() / Self::BLOCKS_PER_EPOCH) + 1) as u16)
    }
}

impl From<&Height> for Epoch {
    fn from(height: &Height) -> Self {
        Self(((height.to_usize() / Self::BLOCKS_PER_EPOCH) + 1) as u16)
    }
}
