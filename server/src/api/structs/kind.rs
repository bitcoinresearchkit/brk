use std::collections::BTreeSet;

use color_eyre::eyre::{eyre, ContextCompat};
use serde::Deserialize;

use crate::structs::{AnyMap, Date, Height, MapKey};

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum Kind {
    Date,
    Height,
    // Timestamp,
    // Last,
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
                // 't' => Self::Timestamp,
                // 'l' => Self::Last,
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
            s.insert(Kind::Timestamp);
        }
        if map.last_value().is_some() {
            s.insert(Kind::Last);
        }
        s
    }
}
