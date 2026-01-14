//! Shared constant generation for static client data.
//!
//! Extracts common logic for generating INDEXES, POOL_ID_TO_POOL_NAME,
//! and cohort name constants across JavaScript and Python clients.

use std::collections::BTreeMap;

use brk_cohort::{
    AGE_RANGE_NAMES, AMOUNT_RANGE_NAMES, EPOCH_NAMES, GE_AMOUNT_NAMES, LT_AMOUNT_NAMES,
    MAX_AGE_NAMES, MIN_AGE_NAMES, SPENDABLE_TYPE_NAMES, TERM_NAMES, YEAR_NAMES,
};
use brk_types::{pools, Index, PoolSlug};
use serde::Serialize;
use serde_json::Value;

use crate::{to_camel_case, VERSION};

/// Collected constant data for client generation.
pub struct ClientConstants {
    pub version: String,
    pub indexes: Vec<&'static str>,
    pub pool_map: BTreeMap<PoolSlug, &'static str>,
}

impl ClientConstants {
    /// Collect all constant data.
    pub fn collect() -> Self {
        let indexes = Index::all();
        let indexes: Vec<&'static str> = indexes.iter().map(|i| i.serialize_long()).collect();

        let pools = pools();
        let mut sorted_pools: Vec<_> = pools.iter().collect();
        sorted_pools.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
        let pool_map: BTreeMap<PoolSlug, &'static str> =
            sorted_pools.iter().map(|p| (p.slug(), p.name)).collect();

        Self {
            version: format!("v{}", VERSION),
            indexes,
            pool_map,
        }
    }
}

/// Cohort name constants - shared data definitions.
pub struct CohortConstants;

impl CohortConstants {
    /// Get all cohort constants as name-value pairs for iteration.
    pub fn all() -> Vec<(&'static str, Value)> {
        fn to_value<T: Serialize>(v: &T) -> Value {
            serde_json::to_value(v).unwrap()
        }

        vec![
            ("TERM_NAMES", to_value(&TERM_NAMES)),
            ("EPOCH_NAMES", to_value(&EPOCH_NAMES)),
            ("YEAR_NAMES", to_value(&YEAR_NAMES)),
            ("SPENDABLE_TYPE_NAMES", to_value(&SPENDABLE_TYPE_NAMES)),
            ("AGE_RANGE_NAMES", to_value(&AGE_RANGE_NAMES)),
            ("MAX_AGE_NAMES", to_value(&MAX_AGE_NAMES)),
            ("MIN_AGE_NAMES", to_value(&MIN_AGE_NAMES)),
            ("AMOUNT_RANGE_NAMES", to_value(&AMOUNT_RANGE_NAMES)),
            ("GE_AMOUNT_NAMES", to_value(&GE_AMOUNT_NAMES)),
            ("LT_AMOUNT_NAMES", to_value(&LT_AMOUNT_NAMES)),
        ]
    }
}

/// Convert top-level keys of a JSON object to camelCase.
pub fn camel_case_keys(value: Value) -> Value {
    match value {
        Value::Object(map) => {
            let new_map: serde_json::Map<String, Value> = map
                .into_iter()
                .map(|(k, v)| (to_camel_case(&k), v))
                .collect();
            Value::Object(new_map)
        }
        other => other,
    }
}

/// Format a JSON value as a pretty-printed string.
pub fn format_json<T: Serialize>(value: &T) -> String {
    serde_json::to_string_pretty(value).unwrap()
}
