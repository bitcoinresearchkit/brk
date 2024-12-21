use std::collections::BTreeSet;

use color_eyre::eyre::{eyre, ContextCompat};
use serde::Deserialize;

use crate::structs::{AnyMap, Date, Height, MapKey};

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum Kind {
    Date,
    Height,
    Last,
}

impl TryFrom<&String> for Kind {
    type Error = color_eyre::Report;

    fn try_from(str: &String) -> Result<Self, Self::Error> {
        Ok(
            match str
                .to_lowercase()
                .chars()
                .next()
                .context("Expect kind to have first letter")?
            {
                'd' => Self::Date,
                'h' => Self::Height,
                'l' => Self::Last,
                _ => return Err(eyre!("Bad kind")),
            },
        )
    }
}

impl From<&(dyn AnyMap + Send + Sync)> for BTreeSet<Kind> {
    fn from(map: &(dyn AnyMap + Send + Sync)) -> Self {
        let mut s = Self::new();
        if map.key_name() == Date::map_name() {
            s.insert(Kind::Date);
        }
        if map.key_name() == Height::map_name() {
            s.insert(Kind::Height);
        }
        if map.last_value().is_some() {
            s.insert(Kind::Last);
        }
        s
    }
}

// impl std::fmt::Display for Kind {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         match *self {
//             Self::Date => write!(f, "date"),
//             Self::Height => write!(f, "height"),
//             Self::Last => write!(f, "last"),
//         }
//     }
// }
