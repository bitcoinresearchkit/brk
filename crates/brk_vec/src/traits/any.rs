use std::io;

use crate::{Result, StorableVec};

use super::{StoredIndex, StoredType};

pub trait AnyStorableVec: Send + Sync {
    fn file_name(&self) -> String;
    fn index_type_to_string(&self) -> &str;
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
    fn collect_range_values(
        &self,
        from: Option<i64>,
        to: Option<i64>,
    ) -> Result<Vec<serde_json::Value>>;
    fn flush(&mut self) -> io::Result<()>;
}

impl<I, T> AnyStorableVec for StorableVec<I, T>
where
    I: StoredIndex,
    T: StoredType,
{
    fn file_name(&self) -> String {
        self.file_name()
    }

    fn index_type_to_string(&self) -> &str {
        self.index_type_to_string()
    }

    fn len(&self) -> usize {
        self.len()
    }

    fn is_empty(&self) -> bool {
        self.is_empty()
    }

    fn flush(&mut self) -> io::Result<()> {
        self.flush()
    }

    fn collect_range_values(
        &self,
        from: Option<i64>,
        to: Option<i64>,
    ) -> Result<Vec<serde_json::Value>> {
        Ok(self
            .collect_range(from, to)?
            .into_iter()
            .map(|v| serde_json::to_value(v).unwrap())
            .collect::<Vec<_>>())
    }
}
