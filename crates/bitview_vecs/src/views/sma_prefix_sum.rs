use std::{convert::Infallible, sync::Arc};

use brk_types::{Cents, Height, StoredU64, Version};
use vecdb::{
    AnyVec, PrintableIndex, ReadableBoxedVec, ReadableCloneableVec, ReadableVec, TypedVec,
    short_type_name,
};

use super::prefix_sum_checkpoints::PrefixSumCheckpoints;

#[derive(Clone)]
pub struct SmaPrefixSumVec {
    name: Arc<str>,
    version: Version,
    spot_price: ReadableBoxedVec<Height, Cents>,
    checkpoints: PrefixSumCheckpoints,
}

impl SmaPrefixSumVec {
    /// One compact prefix checkpoint set is shared by every SMA window.
    pub fn new(
        name: &str,
        version: Version,
        spot_price: &(impl ReadableCloneableVec<Height, Cents> + ?Sized),
    ) -> Self {
        Self {
            name: Arc::from(name),
            version,
            spot_price: spot_price.read_only_boxed_clone(),
            checkpoints: PrefixSumCheckpoints::default(),
        }
    }

    fn try_for_each_value<E>(
        &self,
        from: usize,
        to: usize,
        mut each: impl FnMut(StoredU64) -> Result<(), E>,
    ) -> Result<(), E> {
        let to = to.min(self.spot_price.len());
        if from >= to {
            return Ok(());
        }

        let mut sum = self
            .checkpoints
            .sum_before(&self.spot_price, from, |price| price.inner());
        self.spot_price
            .try_fold_range_at(from, to, (), |(), price| {
                sum = sum
                    .checked_add(price.inner())
                    .expect("price SMA prefix sum overflow");
                each(StoredU64::from(sum))
            })
    }

    fn for_each_value(&self, from: usize, to: usize, mut each: impl FnMut(StoredU64)) {
        let result = self.try_for_each_value(from, to, |value| {
            each(value);
            Ok::<_, Infallible>(())
        });
        match result {
            Ok(()) => {}
            Err(error) => match error {},
        }
    }
}

impl AnyVec for SmaPrefixSumVec {
    fn version(&self) -> Version {
        self.version + self.spot_price.version()
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn len(&self) -> usize {
        self.spot_price.len()
    }

    fn index_type_to_string(&self) -> &'static str {
        <Height as PrintableIndex>::to_string()
    }

    fn region_names(&self) -> Vec<String> {
        Vec::new()
    }

    fn value_type_to_size_of(&self) -> usize {
        size_of::<StoredU64>()
    }

    fn value_type_to_string(&self) -> &'static str {
        short_type_name::<StoredU64>()
    }
}

impl TypedVec for SmaPrefixSumVec {
    type I = Height;
    type T = StoredU64;
}

impl ReadableVec<Height, StoredU64> for SmaPrefixSumVec {
    fn read_into_at(&self, from: usize, to: usize, buf: &mut Vec<StoredU64>) {
        buf.reserve(to.min(self.len()).saturating_sub(from));
        self.for_each_value(from, to, |value| buf.push(value));
    }

    fn for_each_range_dyn_at(&self, from: usize, to: usize, each: &mut dyn FnMut(StoredU64)) {
        self.for_each_value(from, to, each);
    }

    fn fold_range_at<B, F: FnMut(B, StoredU64) -> B>(
        &self,
        from: usize,
        to: usize,
        init: B,
        mut fold: F,
    ) -> B {
        let mut acc = Some(init);
        self.for_each_value(from, to, |value| {
            acc = Some(fold(acc.take().unwrap(), value));
        });
        acc.unwrap()
    }

    fn try_fold_range_at<B, E, F: FnMut(B, StoredU64) -> Result<B, E>>(
        &self,
        from: usize,
        to: usize,
        init: B,
        mut fold: F,
    ) -> Result<B, E> {
        let mut acc = Some(init);
        self.try_for_each_value(from, to, |value| {
            acc = Some(fold(acc.take().unwrap(), value)?);
            Ok(())
        })?;
        Ok(acc.unwrap())
    }
}
