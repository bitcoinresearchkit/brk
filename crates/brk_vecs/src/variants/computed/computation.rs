use clap_derive::ValueEnum;
use serde::{Deserialize, Serialize};

#[derive(
    Default, Debug, PartialEq, PartialOrd, Ord, Eq, Clone, Copy, Serialize, Deserialize, ValueEnum,
)]
pub enum Computation {
    Eager,
    #[default]
    Lazy,
}

impl Computation {
    pub fn eager(&self) -> bool {
        *self == Self::Eager
    }

    pub fn lazy(&self) -> bool {
        *self == Self::Lazy
    }
}
