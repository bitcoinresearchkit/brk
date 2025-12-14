use schemars::JsonSchema;
use serde::Serialize;

use super::{Height, Sats};

#[derive(Debug, Clone, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct RewardStats {
    pub start_block: Height,
    pub end_block: Height,
    #[serde(serialize_with = "sats_as_string")]
    pub total_reward: Sats,
    #[serde(serialize_with = "sats_as_string")]
    pub total_fee: Sats,
    #[serde(serialize_with = "u64_as_string")]
    pub total_tx: u64,
}

fn sats_as_string<S>(value: &Sats, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(&value.to_string())
}

fn u64_as_string<S>(value: &u64, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(&value.to_string())
}
