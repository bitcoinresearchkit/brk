use schemars::JsonSchema;
use serde::Deserialize;

use brk_types::{Cohort, Date};

/// Path parameters for `/api/urpd/{cohort}/{date}`.
#[derive(Deserialize, JsonSchema)]
pub struct UrpdParams {
    pub cohort: Cohort,
    /// Calendar date of the URPD snapshot in `YYYY-MM-DD` format.
    #[schemars(with = "String", example = &"2024-01-01")]
    pub date: Date,
}
