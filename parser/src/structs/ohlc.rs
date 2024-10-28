use allocative::Allocative;
use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Default, Deserialize, Serialize, Encode, Decode, Clone, Copy, Allocative)]
pub struct OHLC {
    pub open: f32,
    pub high: f32,
    pub low: f32,
    pub close: f32,
}
