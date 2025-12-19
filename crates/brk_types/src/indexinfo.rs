use schemars::JsonSchema;
use serde::Serialize;

use super::Index;

/// Information about an available index and its query aliases
#[derive(Clone, Copy, Serialize, JsonSchema)]
pub struct IndexInfo {
    /// The canonical index name
    pub index: Index,

    /// All Accepted query aliases
    #[schemars(example = vec!["d", "date", "dateindex"])]
    pub aliases: &'static [&'static str],
}
