use std::path::Path;

use brk_core::{Dateindex, Decadeindex, Monthindex, Quarterindex, Weekindex, Yearindex};
use brk_exit::Exit;
use brk_indexer::Indexer;
use brk_vec::{AnyStoredVec, Compressed, Result, Version};

use crate::storage::vecs::{Indexes, base::ComputedVec, indexes};

use super::{ComputedType, ComputedVecBuilder, StorableVecGeneatorOptions};

#[derive(Clone)]
pub struct ComputedVecsFromDateindex<T>
where
    T: ComputedType + PartialOrd,
{
    pub dateindex: ComputedVec<Dateindex, T>,
    pub dateindex_extra: ComputedVecBuilder<Dateindex, T>,
    pub weekindex: ComputedVecBuilder<Weekindex, T>,
    pub monthindex: ComputedVecBuilder<Monthindex, T>,
    pub quarterindex: ComputedVecBuilder<Quarterindex, T>,
    pub yearindex: ComputedVecBuilder<Yearindex, T>,
    pub decadeindex: ComputedVecBuilder<Decadeindex, T>,
}

const VERSION: Version = Version::ZERO;

impl<T> ComputedVecsFromDateindex<T>
where
    T: ComputedType,
{
    pub fn forced_import(
        path: &Path,
        name: &str,
        version: Version,
        compressed: Compressed,
        options: StorableVecGeneatorOptions,
    ) -> color_eyre::Result<Self> {
        let version = VERSION + version;

        let dateindex_extra = ComputedVecBuilder::forced_import(
            path,
            name,
            version,
            compressed,
            options.copy_self_extra(),
        )?;

        let options = options.remove_percentiles();

        Ok(Self {
            dateindex: ComputedVec::forced_import(
                &path.join(format!("dateindex_to_{name}")),
                version,
                compressed,
            )?,
            dateindex_extra,
            weekindex: ComputedVecBuilder::forced_import(path, name, version, compressed, options)?,
            monthindex: ComputedVecBuilder::forced_import(
                path, name, version, compressed, options,
            )?,
            quarterindex: ComputedVecBuilder::forced_import(
                path, name, version, compressed, options,
            )?,
            yearindex: ComputedVecBuilder::forced_import(path, name, version, compressed, options)?,
            decadeindex: ComputedVecBuilder::forced_import(
                path, name, version, compressed, options,
            )?,
        })
    }

    pub fn compute<F>(
        &mut self,
        indexer: &mut Indexer,
        indexes: &mut indexes::Vecs,
        starting_indexes: &Indexes,
        exit: &Exit,
        mut compute: F,
    ) -> color_eyre::Result<()>
    where
        F: FnMut(
            &mut ComputedVec<Dateindex, T>,
            &mut Indexer,
            &mut indexes::Vecs,
            &Indexes,
            &Exit,
        ) -> Result<()>,
    {
        compute(
            &mut self.dateindex,
            indexer,
            indexes,
            starting_indexes,
            exit,
        )?;

        self.dateindex_extra
            .extend(starting_indexes.dateindex, self.dateindex.mut_vec(), exit)?;

        self.weekindex.compute(
            starting_indexes.weekindex,
            self.dateindex.mut_vec(),
            indexes.weekindex_to_first_dateindex.mut_vec(),
            indexes.weekindex_to_last_dateindex.mut_vec(),
            exit,
        )?;

        self.monthindex.compute(
            starting_indexes.monthindex,
            self.dateindex.mut_vec(),
            indexes.monthindex_to_first_dateindex.mut_vec(),
            indexes.monthindex_to_last_dateindex.mut_vec(),
            exit,
        )?;

        self.quarterindex.from_aligned(
            starting_indexes.quarterindex,
            &mut self.monthindex,
            indexes.quarterindex_to_first_monthindex.mut_vec(),
            indexes.quarterindex_to_last_monthindex.mut_vec(),
            exit,
        )?;

        self.yearindex.from_aligned(
            starting_indexes.yearindex,
            &mut self.monthindex,
            indexes.yearindex_to_first_monthindex.mut_vec(),
            indexes.yearindex_to_last_monthindex.mut_vec(),
            exit,
        )?;

        self.decadeindex.from_aligned(
            starting_indexes.decadeindex,
            &mut self.yearindex,
            indexes.decadeindex_to_first_yearindex.mut_vec(),
            indexes.decadeindex_to_last_yearindex.mut_vec(),
            exit,
        )?;

        Ok(())
    }

    pub fn any_vecs(&self) -> Vec<&dyn AnyStoredVec> {
        [
            vec![self.dateindex.any_vec()],
            self.dateindex_extra.any_vecs(),
            self.weekindex.any_vecs(),
            self.monthindex.any_vecs(),
            self.quarterindex.any_vecs(),
            self.yearindex.any_vecs(),
            self.decadeindex.any_vecs(),
        ]
        .concat()
    }
}
