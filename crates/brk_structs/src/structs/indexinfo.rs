use schemars::JsonSchema;
use serde::Serialize;

use super::Index;

#[derive(Serialize, JsonSchema)]
/// Information about an available index and its query aliases
pub struct IndexInfo {
    /// The canonical index name
    pub index: Index,

    /// All Accepted query aliases
    #[schemars(example = vec!["d", "date", "dateindex"])]
    pub aliases: &'static [&'static str],
}
