use crate::{Error, Result};

use super::{AnyIterableVec, AnyVec, StoredIndex, StoredType};

pub trait CollectableVec<I, T>: AnyVec + AnyIterableVec<I, T>
where
    Self: Clone,
    I: StoredIndex,
    T: StoredType,
{
    fn collect_range(&self, from: Option<usize>, to: Option<usize>) -> Result<Vec<T>> {
        let len = self.len();
        let from = from.unwrap_or_default();
        let to = to.map_or(len, |to| to.min(len));

        if from >= len || from >= to {
            return Ok(vec![]);
        }

        Ok(self
            .iter_at_(from)
            .take(to - from)
            .map(|(_, v)| v.into_inner())
            .collect::<Vec<_>>())
    }

    #[inline]
    fn i64_to_usize(i: i64, len: usize) -> usize {
        if i >= 0 {
            i as usize
        } else {
            let v = len as i64 + i;
            if v < 0 { 0 } else { v as usize }
        }
    }

    #[doc(hidden)]
    fn collect_signed_range(&self, from: Option<i64>, to: Option<i64>) -> Result<Vec<T>> {
        let len = self.len();
        let from = from.map(|i| Self::i64_to_usize(i, len));
        let to = to.map(|i| Self::i64_to_usize(i, len));
        self.collect_range(from, to)
    }

    #[inline]
    fn collect_range_serde_json(
        &self,
        from: Option<i64>,
        to: Option<i64>,
    ) -> Result<Vec<serde_json::Value>> {
        self.collect_signed_range(from, to)?
            .into_iter()
            .map(|v| serde_json::to_value(v).map_err(Error::from))
            .collect::<Result<Vec<_>>>()
    }
}

impl<I, T, V> CollectableVec<I, T> for V
where
    V: AnyVec + AnyIterableVec<I, T> + Clone,
    I: StoredIndex,
    T: StoredType,
{
}

pub trait AnyCollectableVec: AnyVec {
    fn collect_range_serde_json(
        &self,
        from: Option<i64>,
        to: Option<i64>,
    ) -> Result<Vec<serde_json::Value>>;
}
