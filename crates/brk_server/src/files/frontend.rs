use clap::ValueEnum;
use serde::{Deserialize, Serialize};

#[derive(
    Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize, ValueEnum,
)]
pub enum Frontend {
    None,
    #[default]
    KiboMoney,
    Custom,
}

impl Frontend {
    pub fn is_none(&self) -> bool {
        self == &Self::None
    }

    pub fn is_some(&self) -> bool {
        !self.is_none()
    }

    pub fn to_folder_name(&self) -> &str {
        match self {
            Self::Custom => "custom",
            Self::KiboMoney => "kibo.money",
            Self::None => unreachable!(),
        }
    }
}
