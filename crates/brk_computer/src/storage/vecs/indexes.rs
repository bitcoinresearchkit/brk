use std::{fs, ops::Deref, path::Path};

use brk_core::{Date, Dateindex, Height, Txindex, Txinindex, Txoutindex};
use brk_exit::Exit;
use brk_indexer::Indexer;
use brk_vec::{AnyStorableVec, Compressed, Version};

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
    pub fn forced_import(path: &Path, compressed: Compressed) -> color_eyre::Result<Self> {
        fs::create_dir_all(path)?;

        Ok(Self {
            dateindex_to_date: StorableVec::forced_import(
                &path.join("dateindex_to_date"),
                Version::from(1),
                compressed,
            )?,
            dateindex_to_dateindex: StorableVec::forced_import(
                &path.join("dateindex_to_dateindex"),
                Version::from(1),
                compressed,
            )?,
            dateindex_to_first_height: StorableVec::forced_import(
                &path.join("dateindex_to_first_height"),
                Version::from(1),
                compressed,
            )?,
            dateindex_to_last_height: StorableVec::forced_import(
                &path.join("dateindex_to_last_height"),
                Version::from(1),
                compressed,
            )?,
            height_to_real_date: StorableVec::forced_import(
                &path.join("height_to_real_date"),
                Version::from(1),
                compressed,
            )?,
            height_to_fixed_date: StorableVec::forced_import(
                &path.join("height_to_fixed_date"),
                Version::from(1),
                compressed,
            )?,
            height_to_dateindex: StorableVec::forced_import(
                &path.join("height_to_dateindex"),
                Version::from(1),
                compressed,
            )?,
            height_to_height: StorableVec::forced_import(
                &path.join("height_to_height"),
                Version::from(1),
                compressed,
            )?,
            height_to_last_txindex: StorableVec::forced_import(
                &path.join("height_to_last_txindex"),
                Version::from(1),
                compressed,
            )?,

            txindex_to_last_txinindex: StorableVec::forced_import(
                &path.join("txindex_to_last_txinindex"),
                Version::from(1),
                compressed,
            )?,
            txindex_to_last_txoutindex: StorableVec::forced_import(
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
            indexer_vecs.height_to_timestamp.mut_vec(),
            |(h, ..)| (h, h),
            exit,
        )?;

        self.height_to_real_date.compute_transform(
            starting_indexes.height,
            indexer_vecs.height_to_timestamp.mut_vec(),
            |(h, t, ..)| (h, Date::from(t)),
            exit,
        )?;

        self.height_to_fixed_date.compute_transform(
            starting_indexes.height,
            self.height_to_real_date.mut_vec(),
            |(h, d, s, ..)| {
                let d = h
                    .decremented()
                    .and_then(|h| s.get(h).ok())
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
            self.height_to_fixed_date.mut_vec(),
            |(h, d, ..)| (h, Dateindex::try_from(d).unwrap()),
            exit,
        )?;

        let starting_dateindex = self
            .height_to_dateindex
            .get(starting_indexes.height.decremented().unwrap_or_default())?
            .copied()
            .unwrap_or_default();

        self.dateindex_to_first_height
            .compute_inverse_more_to_less(
                starting_indexes.height,
                self.height_to_dateindex.mut_vec(),
                exit,
            )?;

        self.dateindex_to_last_height
            .compute_last_index_from_first(
                starting_dateindex,
                self.dateindex_to_first_height.mut_vec(),
                height_count,
                exit,
            )?;

        self.dateindex_to_dateindex.compute_transform(
            starting_dateindex,
            self.dateindex_to_first_height.mut_vec(),
            |(di, ..)| (di, di),
            exit,
        )?;

        self.dateindex_to_date.compute_transform(
            starting_dateindex,
            self.dateindex_to_dateindex.mut_vec(),
            |(di, ..)| (di, Date::from(di)),
            exit,
        )?;

        self.txindex_to_last_txinindex
            .compute_last_index_from_first(
                starting_indexes.txindex,
                indexer_vecs.txindex_to_first_txinindex.mut_vec(),
                txinindexes_count,
                exit,
            )?;

        self.txindex_to_last_txoutindex
            .compute_last_index_from_first(
                starting_indexes.txindex,
                indexer_vecs.txindex_to_first_txoutindex.mut_vec(),
                txoutindexes_count,
                exit,
            )?;

        self.height_to_last_txindex.compute_last_index_from_first(
            starting_indexes.height,
            indexer_vecs.height_to_first_txindex.mut_vec(),
            txindexes_count,
            exit,
        )?;

        Ok(Indexes::from((starting_indexes, starting_dateindex)))
    }

    pub fn as_any_vecs(&self) -> Vec<&dyn AnyStorableVec> {
        vec![
            self.dateindex_to_date.any_vec(),
            self.dateindex_to_dateindex.any_vec(),
            self.dateindex_to_first_height.any_vec(),
            self.dateindex_to_last_height.any_vec(),
            self.height_to_dateindex.any_vec(),
            self.height_to_fixed_date.any_vec(),
            self.height_to_height.any_vec(),
            self.height_to_last_txindex.any_vec(),
            self.height_to_real_date.any_vec(),
            self.txindex_to_last_txinindex.any_vec(),
            self.txindex_to_last_txoutindex.any_vec(),
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
