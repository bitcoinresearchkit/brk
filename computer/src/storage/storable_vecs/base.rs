use std::{
    fmt::Debug,
    io,
    ops::{Deref, DerefMut},
    path::Path,
};

use bindex::{Indexer, Version};

use crate::Computer;

pub struct StorableVec<I, T> {
    vec: bindex::StorableVec<I, T>,
    f: Box<dyn Fn(&Indexer, &Computer) -> storable_vec::Result<Vec<(I, T)>>>,
}

impl<I, T> StorableVec<I, T>
where
    I: TryInto<usize>,
    T: Sized + Debug + Clone,
{
    pub fn import<F>(path: &Path, version: Version, f: F) -> io::Result<Self>
    where
        F: Fn(&Indexer, &Computer) -> storable_vec::Result<Vec<(I, T)>> + 'static,
    {
        let vec = bindex::StorableVec::import(path, version)?;

        Ok(Self { vec, f: Box::new(f) })
    }

    pub fn compute(&mut self, indexer: &Indexer, computer: &Computer) -> storable_vec::Result<()> {
        (self.f)(indexer, computer)?
            .into_iter()
            .try_for_each(|(i, v)| self.push_if_needed(i, v))
    }

    // pub fn fill(&mut self) {
    //     self
    //         .vecs()
    //         .height_to_timestamp
    //         .read_iter(move |(_height, timestamp)| {
    //             let height = Height::from(_height);
    //             let date = Date::from(timestamp);
    //             self.vecs.date_to_first_height.push_if_needed(date, height)?;
    //             Ok(())
    //         })?;
    // }
}

impl<I, T> Deref for StorableVec<I, T> {
    type Target = bindex::StorableVec<I, T>;
    fn deref(&self) -> &Self::Target {
        &self.vec
    }
}
impl<I, T> DerefMut for StorableVec<I, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.vec
    }
}

pub trait AnyComputedStorableVec {
    fn compute(&mut self, indexer: &Indexer, computer: &Computer) -> storable_vec::Result<()>;
}

impl<I, T> AnyComputedStorableVec for StorableVec<I, T>
where
    I: TryInto<usize>,
    T: Sized + Debug + Clone,
{
    fn compute(&mut self, indexer: &Indexer, computer: &Computer) -> storable_vec::Result<()> {
        self.compute(indexer, computer)
    }
}
