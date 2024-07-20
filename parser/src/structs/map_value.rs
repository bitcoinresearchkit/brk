use std::fmt::Debug;

use allocative::Allocative;
use bincode::{Decode, Encode};
use serde::{de::DeserializeOwned, Serialize};

use crate::datasets::OHLC;

use super::{Date, Height};

pub trait MapValue:
    Clone
    + Copy
    + Default
    + Debug
    + Serialize
    + DeserializeOwned
    + Encode
    + Decode
    + Sync
    + Send
    + Allocative
{
}

impl MapValue for u16 {}
impl MapValue for u32 {}
impl MapValue for u64 {}
impl MapValue for usize {}
impl MapValue for f32 {}
impl MapValue for f64 {}
impl MapValue for Date {}
impl MapValue for OHLC {}
impl MapValue for Height {}
