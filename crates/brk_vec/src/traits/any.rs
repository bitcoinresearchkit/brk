use std::time::Duration;

use crate::Result;

pub trait AnyVec: Send + Sync {
    fn name(&self) -> String;
    fn index_type_to_string(&self) -> &str;
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
    fn collect_range_serde_json(
        &self,
        from: Option<i64>,
        to: Option<i64>,
    ) -> Result<Vec<serde_json::Value>>;
    fn modified_time(&self) -> Result<Duration>;
    fn any_vec(&self) -> &dyn AnyVec
    where
        Self: Sized,
    {
        self
    }
}
