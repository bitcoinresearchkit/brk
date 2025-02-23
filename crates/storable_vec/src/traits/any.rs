use std::{io, mem};

use crate::{Result, StorableVec, STATELESS};

use super::{StoredIndex, StoredType};

pub trait AnyStorableVec: Send + Sync {
    fn file_name(&self) -> String;
    fn index_type_to_string(&self) -> &str;
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
    fn flush(&mut self) -> io::Result<()>;
}

impl<I, T, const MODE: u8> AnyStorableVec for StorableVec<I, T, MODE>
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
}

#[cfg(feature = "json")]
pub trait AnyJsonStorableVec: AnyStorableVec {
    fn collect_range_values(&self, from: Option<i64>, to: Option<i64>) -> Result<Vec<serde_json::Value>>;
}

#[cfg(feature = "json")]
impl<I, T, const MODE: u8> AnyJsonStorableVec for StorableVec<I, T, MODE>
where
    I: StoredIndex,
    T: StoredType + serde::Serialize,
{
    fn collect_range_values(&self, from: Option<i64>, to: Option<i64>) -> Result<Vec<serde_json::Value>> {
        if MODE == STATELESS {
            Ok(
                unsafe { mem::transmute::<&StorableVec<I, T, MODE>, &StorableVec<I, T, STATELESS>>(self) }
                    .collect_range(from, to)?
                    .into_iter()
                    .map(|v| serde_json::to_value(v).unwrap())
                    .collect::<Vec<_>>(),
            )
        } else {
            todo!("todo ?")
        }
    }
}
