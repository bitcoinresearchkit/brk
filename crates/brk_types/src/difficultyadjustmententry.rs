use schemars::JsonSchema;
use serde::ser::SerializeTuple;
use serde::{Serialize, Serializer};

use crate::{Height, Timestamp};

/// A single difficulty adjustment entry.
/// Serializes as array: [timestamp, height, difficulty, change_percent]
#[derive(Debug, JsonSchema)]
pub struct DifficultyAdjustmentEntry {
    pub timestamp: Timestamp,
    pub height: Height,
    pub difficulty: f64,
    pub change_percent: f64,
}

impl Serialize for DifficultyAdjustmentEntry {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut tup = serializer.serialize_tuple(4)?;
        tup.serialize_element(&self.timestamp)?;
        tup.serialize_element(&self.height)?;
        tup.serialize_element(&self.difficulty)?;
        tup.serialize_element(&self.change_percent)?;
        tup.end()
    }
}
