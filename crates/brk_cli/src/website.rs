use std::{path::PathBuf, str::FromStr};

use serde::{Deserialize, Serialize};

use crate::paths::fix_user_path;

/// Website configuration:
/// - `true` or omitted: serve embedded website
/// - `false`: disable website serving
/// - `"/path/to/website"`: serve custom website from path
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
#[serde(untagged)]
pub enum WebsiteArg {
    Enabled(bool),
    Path(PathBuf),
}

impl Default for WebsiteArg {
    fn default() -> Self {
        Self::Enabled(true)
    }
}

impl FromStr for WebsiteArg {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.to_lowercase().as_str() {
            "true" | "1" | "yes" | "on" => Self::Enabled(true),
            "false" | "0" | "no" | "off" => Self::Enabled(false),
            _ => Self::Path(fix_user_path(s)),
        })
    }
}
