use std::path::Path;

use brk_core::{
    Dateindex, Decadeindex, Difficultyepoch, Height, Monthindex, Quarterindex, Weekindex, Yearindex,
};
use brk_exit::Exit;
use brk_indexer::Indexer;
use brk_vec::{AnyStoredVec, Compressed, Result, StoredVec, Version};

use crate::storage::vecs::{Indexes, base::ComputedVec, indexes};

use super::{ComputedType, ComputedVecBuilder, StorableVecGeneatorOptions};

#[derive(Clone)]
pub struct ComputedVecsFromHeight<T>
where
    T: ComputedType + PartialOrd,
{
    pub height: Option<ComputedVec<Height, T>>,
    pub height_extra: ComputedVecBuilder<Height, T>,
    pub dateindex: ComputedVecBuilder<Dateindex, T>,
    pub weekindex: ComputedVecBuilder<Weekindex, T>,
    pub difficultyepoch: ComputedVecBuilder<Difficultyepoch, T>,
    pub monthindex: ComputedVecBuilder<Monthindex, T>,
    pub quarterindex: ComputedVecBuilder<Quarterindex, T>,
    pub yearindex: ComputedVecBuilder<Yearindex, T>,
    // TODO: pub halvingepoch: StorableVecGeneator<Halvingepoch, T>,
    pub decadeindex: ComputedVecBuilder<Decadeindex, T>,
}

const VERSION: Version = Version::ZERO;

impl<T> ComputedVecsFromHeight<T>
where
    T: ComputedType + Ord + From<f64>,
    f64: From<T>,
{
    pub fn forced_import(
        path: &Path,
        name: &str,
        compute_source: bool,
        version: Version,
        compressed: Compressed,
        options: StorableVecGeneatorOptions,
    ) -> color_eyre::Result<Self> {
        let version = VERSION + version;

        let height = compute_source.then(|| {
            ComputedVec::forced_import(&path.join(format!("height_to_{name}")), version, compressed)
                .unwrap()
        });

        let height_extra = ComputedVecBuilder::forced_import(
            path,
            name,
            version,
            compressed,
            options.copy_self_extra(),
        )?;

        let dateindex =
            ComputedVecBuilder::forced_import(path, name, version, compressed, options)?;

        let options = options.remove_percentiles();

        Ok(Self {
            height,
            height_extra,
            dateindex,
            weekindex: ComputedVecBuilder::forced_import(path, name, version, compressed, options)?,
            difficultyepoch: ComputedVecBuilder::forced_import(
                path, name, version, compressed, options,
            )?,
            monthindex: ComputedVecBuilder::forced_import(
                path, name, version, compressed, options,
            )?,
            quarterindex: ComputedVecBuilder::forced_import(
                path, name, version, compressed, options,
            )?,
            yearindex: ComputedVecBuilder::forced_import(path, name, version, compressed, options)?,
            // halvingepoch: StorableVecGeneator::forced_import(path, name, version, compressed, options)?,
            decadeindex: ComputedVecBuilder::forced_import(
                path, name, version, compressed, options,
            )?,
        })
    }

    pub fn compute_all<F>(
        &mut self,
        indexer: &mut Indexer,
        indexes: &mut indexes::Vecs,
        starting_indexes: &Indexes,
        exit: &Exit,
        mut compute: F,
    ) -> color_eyre::Result<()>
    where
        F: FnMut(
            &mut ComputedVec<Height, T>,
            &mut Indexer,
            &mut indexes::Vecs,
            &Indexes,
            &Exit,
        ) -> Result<()>,
    {
        compute(
            self.height.as_mut().unwrap(),
            indexer,
            indexes,
            starting_indexes,
            exit,
        )?;

        self.compute_rest(indexes, starting_indexes, exit, None)?;

        Ok(())
    }

    pub fn compute_rest(
        &mut self,
        indexes: &mut indexes::Vecs,
        starting_indexes: &Indexes,
        exit: &Exit,
        height: Option<&mut StoredVec<Height, T>>,
    ) -> color_eyre::Result<()> {
        let height = height.unwrap_or_else(|| self.height.as_mut().unwrap().mut_vec());

        self.height_extra
            .extend(starting_indexes.height, height, exit)?;

        self.dateindex.compute(
            starting_indexes.dateindex,
            height,
            indexes.dateindex_to_first_height.mut_vec(),
            indexes.dateindex_to_last_height.mut_vec(),
            exit,
        )?;

        self.weekindex.from_aligned(
            starting_indexes.weekindex,
            &mut self.dateindex,
            indexes.weekindex_to_first_dateindex.mut_vec(),
            indexes.weekindex_to_last_dateindex.mut_vec(),
            exit,
        )?;

        self.monthindex.from_aligned(
            starting_indexes.monthindex,
            &mut self.dateindex,
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

        self.difficultyepoch.compute(
            starting_indexes.difficultyepoch,
            height,
            indexes.difficultyepoch_to_first_height.mut_vec(),
            indexes.difficultyepoch_to_last_height.mut_vec(),
            exit,
        )?;

        Ok(())
    }

    pub fn any_vecs(&self) -> Vec<&dyn AnyStoredVec> {
        [
            self.height.as_ref().map_or(vec![], |v| vec![v.any_vec()]),
            self.height_extra.any_vecs(),
            self.dateindex.any_vecs(),
            self.weekindex.any_vecs(),
            self.difficultyepoch.any_vecs(),
            self.monthindex.any_vecs(),
            self.quarterindex.any_vecs(),
            self.yearindex.any_vecs(),
            // self.halvingepoch.as_any_vecs(),
            self.decadeindex.any_vecs(),
        ]
        .concat()
    }
}
