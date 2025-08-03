use serde_derive::{Deserialize, Serialize};

#[derive(Default, Debug, PartialEq, PartialOrd, Ord, Eq, Clone, Copy, Serialize, Deserialize)]
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
