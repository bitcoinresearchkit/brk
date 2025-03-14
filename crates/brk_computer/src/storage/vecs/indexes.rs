use std::{fs, ops::Deref, path::Path};

use brk_core::{Date, Dateindex, Height, Txindex, Txinindex, Txoutindex};
use brk_exit::Exit;
use brk_indexer::Indexer;
use brk_vec::{AnyStorableVec, Compressed, Value, Version};

use super::StorableVec;

#[derive(Clone)]
pub struct Vecs {
    // pub height_to_last_addressindex: StorableVec<Height, Addressindex>,
    // pub height_to_last_txoutindex: StorableVec<Height, Txoutindex>,
    pub dateindex_to_date: StorableVec<Dateindex, Date>,
    pub dateindex_to_dateindex: StorableVec<Dateindex, Dateindex>,
    pub dateindex_to_first_height: StorableVec<Dateindex, Height>,
    pub dateindex_to_last_height: StorableVec<Dateindex, Height>,
    pub height_to_dateindex: StorableVec<Height, Dateindex>,
    pub height_to_fixed_date: StorableVec<Height, Date>,
    pub height_to_height: StorableVec<Height, Height>,
    pub height_to_last_txindex: StorableVec<Height, Txindex>,
    pub height_to_real_date: StorableVec<Height, Date>,
    pub txindex_to_last_txinindex: StorableVec<Txindex, Txinindex>,
    pub txindex_to_last_txoutindex: StorableVec<Txindex, Txoutindex>,
}

impl Vecs {
    pub fn import(path: &Path, compressed: Compressed) -> color_eyre::Result<Self> {
        fs::create_dir_all(path)?;

        Ok(Self {
            dateindex_to_date: StorableVec::import(
                &path.join("dateindex_to_date"),
                Version::from(1),
                compressed,
            )?,
            dateindex_to_dateindex: StorableVec::import(
                &path.join("dateindex_to_dateindex"),
                Version::from(1),
                compressed,
            )?,
            dateindex_to_first_height: StorableVec::import(
                &path.join("dateindex_to_first_height"),
                Version::from(1),
                compressed,
            )?,
            dateindex_to_last_height: StorableVec::import(
                &path.join("dateindex_to_last_height"),
                Version::from(1),
                compressed,
            )?,
            height_to_real_date: StorableVec::import(
                &path.join("height_to_real_date"),
                Version::from(1),
                compressed,
            )?,
            height_to_fixed_date: StorableVec::import(
                &path.join("height_to_fixed_date"),
                Version::from(1),
                compressed,
            )?,
            height_to_dateindex: StorableVec::import(
                &path.join("height_to_dateindex"),
                Version::from(1),
                compressed,
            )?,
            height_to_height: StorableVec::import(
                &path.join("height_to_height"),
                Version::from(1),
                compressed,
            )?,
            height_to_last_txindex: StorableVec::import(
                &path.join("height_to_last_txindex"),
                Version::from(1),
                compressed,
            )?,

            txindex_to_last_txinindex: StorableVec::import(
                &path.join("txindex_to_last_txinindex"),
                Version::from(1),
                compressed,
            )?,
            txindex_to_last_txoutindex: StorableVec::import(
                &path.join("txindex_to_last_txoutindex"),
                Version::from(1),
                compressed,
            )?,
        })
    }

    pub fn compute(
        &mut self,
        indexer: &mut Indexer,
        starting_indexes: brk_indexer::Indexes,
        exit: &Exit,
    ) -> color_eyre::Result<Indexes> {
        let indexer_vecs = indexer.mut_vecs();

        let height_count = indexer_vecs.height_to_size.len();
        let txindexes_count = indexer_vecs.txindex_to_txid.len();
        let txinindexes_count = indexer_vecs.txinindex_to_txoutindex.len();
        let txoutindexes_count = indexer_vecs.txoutindex_to_addressindex.len();

        self.height_to_height.compute_transform(
            starting_indexes.height,
            &mut indexer_vecs.height_to_timestamp,
            |(h, ..)| (h, h),
            exit,
        )?;

        self.height_to_real_date.compute_transform(
            starting_indexes.height,
            &mut indexer_vecs.height_to_timestamp,
            |(h, t, ..)| (h, Date::from(t)),
            exit,
        )?;

        self.height_to_fixed_date.compute_transform(
            starting_indexes.height,
            &mut self.height_to_real_date,
            |(h, d, s, ..)| {
                let d = h
                    .decremented()
                    .and_then(|h| s.read(h).ok())
                    .flatten()
                    .map_or(d, |prev_d| {
                        let prev_d = *prev_d;
                        if prev_d > d { prev_d } else { d }
                    });
                (h, d)
            },
            exit,
        )?;

        self.height_to_dateindex.compute_transform(
            starting_indexes.height,
            &mut self.height_to_fixed_date,
            |(h, d, ..)| (h, Dateindex::try_from(d).unwrap()),
            exit,
        )?;

        let starting_dateindex = self
            .height_to_dateindex
            .get(starting_indexes.height.decremented().unwrap_or_default())?
            .map(Value::into_inner)
            .unwrap_or_default();

        self.dateindex_to_first_height
            .compute_inverse_more_to_less(
                starting_indexes.height,
                &mut self.height_to_dateindex,
                exit,
            )?;

        self.dateindex_to_last_height
            .compute_last_index_from_first(
                starting_dateindex,
                &mut self.dateindex_to_first_height,
                height_count,
                exit,
            )?;

        self.dateindex_to_dateindex.compute_transform(
            starting_dateindex,
            &mut self.dateindex_to_first_height,
            |(di, ..)| (di, di),
            exit,
        )?;

        self.dateindex_to_date.compute_transform(
            starting_dateindex,
            &mut self.dateindex_to_dateindex,
            |(di, ..)| (di, Date::from(di)),
            exit,
        )?;

        self.txindex_to_last_txinindex
            .compute_last_index_from_first(
                starting_indexes.txindex,
                &mut indexer_vecs.txindex_to_first_txinindex,
                txinindexes_count,
                exit,
            )?;

        self.txindex_to_last_txoutindex
            .compute_last_index_from_first(
                starting_indexes.txindex,
                &mut indexer_vecs.txindex_to_first_txoutindex,
                txoutindexes_count,
                exit,
            )?;

        self.height_to_last_txindex.compute_last_index_from_first(
            starting_indexes.height,
            &mut indexer_vecs.height_to_first_txindex,
            txindexes_count,
            exit,
        )?;

        Ok(Indexes::from((starting_indexes, starting_dateindex)))
    }

    pub fn as_any_vecs(&self) -> Vec<&dyn AnyStorableVec> {
        vec![
            &*self.dateindex_to_date,
            &*self.dateindex_to_dateindex,
            &*self.dateindex_to_first_height,
            &*self.dateindex_to_last_height,
            &*self.height_to_dateindex,
            &*self.height_to_fixed_date,
            &*self.height_to_height,
            &*self.height_to_last_txindex,
            &*self.height_to_real_date,
            &*self.txindex_to_last_txinindex,
            &*self.txindex_to_last_txoutindex,
        ]
    }
}

pub struct Indexes {
    indexes: brk_indexer::Indexes,
    pub dateindex: Dateindex,
}

impl Deref for Indexes {
    type Target = brk_indexer::Indexes;
    fn deref(&self) -> &Self::Target {
        &self.indexes
    }
}
impl From<(brk_indexer::Indexes, Dateindex)> for Indexes {
    fn from((indexes, dateindex): (brk_indexer::Indexes, Dateindex)) -> Self {
        Self { indexes, dateindex }
    }
}
