use allocative::Allocative;
use bincode::{Decode, Encode};
use derive_deref::{Deref, DerefMut};
use serde::{Deserialize, Serialize};

#[derive(
    Debug, Deref, DerefMut, Default, Clone, Copy, Encode, Decode, Serialize, Deserialize, Allocative,
)]
pub struct Counter(u32);

impl Counter {
    #[inline(always)]
    pub fn increment(&mut self) {
        self.0 += 1;
    }

    #[inline(always)]
    pub fn decrement(&mut self) {
        self.0 -= 1;
    }

    #[inline(always)]
    pub fn reset(&mut self) {
        self.0 = 0;
    }

    #[inline(always)]
    pub fn inner(&self) -> u32 {
        self.0
    }
}
