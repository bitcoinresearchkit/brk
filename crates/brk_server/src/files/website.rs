use clap_derive::ValueEnum;
use serde::{Deserialize, Serialize};

#[derive(
    Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize, ValueEnum,
)]
pub enum Website {
    #[default]
    None,
    #[value(name = "kibo.money")]
    KiboMoney,
    Custom,
}

impl Website {
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
