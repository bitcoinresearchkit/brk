use std::path::Path;

use brk_core::{
    Dateindex, Decadeindex, Difficultyepoch, Height, Monthindex, Quarterindex, Txindex, Weekindex,
    Yearindex,
};
use brk_exit::Exit;
use brk_indexer::Indexer;
use brk_vec::{AnyStorableVec, Compressed, Result, Version};

use crate::storage::vecs::{Indexes, base::ComputedVec, indexes};

use super::{ComputedType, ComputedVecBuilder, StorableVecGeneatorOptions};

#[derive(Clone)]
pub struct ComputedVecsFromTxindex<T>
where
    T: ComputedType + PartialOrd,
{
    pub txindex: ComputedVec<Txindex, T>,
    pub height: ComputedVecBuilder<Height, T>,
    pub dateindex: ComputedVecBuilder<Dateindex, T>,
    pub weekindex: ComputedVecBuilder<Weekindex, T>,
    pub difficultyepoch: ComputedVecBuilder<Difficultyepoch, T>,
    pub monthindex: ComputedVecBuilder<Monthindex, T>,
    pub quarterindex: ComputedVecBuilder<Quarterindex, T>,
    pub yearindex: ComputedVecBuilder<Yearindex, T>,
    // TODO: pub halvingepoch: StorableVecGeneator<Halvingepoch, T>,
    pub decadeindex: ComputedVecBuilder<Decadeindex, T>,
}

impl<T> ComputedVecsFromTxindex<T>
where
    T: ComputedType + Ord + From<f64>,
    f64: From<T>,
{
    pub fn forced_import(
        path: &Path,
        name: &str,
        version: Version,
        compressed: Compressed,
        options: StorableVecGeneatorOptions,
    ) -> color_eyre::Result<Self> {
        let txindex = ComputedVec::forced_import(
            &path.join(format!("txindex_to_{name}")),
            version,
            compressed,
        )?;

        let height = ComputedVecBuilder::forced_import(path, name, compressed, options)?;
        let dateindex = ComputedVecBuilder::forced_import(path, name, compressed, options)?;

        let options = options.remove_percentiles();

        Ok(Self {
            txindex,
            height,
            dateindex,
            weekindex: ComputedVecBuilder::forced_import(path, name, compressed, options)?,
            difficultyepoch: ComputedVecBuilder::forced_import(path, name, compressed, options)?,
            monthindex: ComputedVecBuilder::forced_import(path, name, compressed, options)?,
            quarterindex: ComputedVecBuilder::forced_import(path, name, compressed, options)?,
            yearindex: ComputedVecBuilder::forced_import(path, name, compressed, options)?,
            // halvingepoch: StorableVecGeneator::forced_import(path, name, compressed, options)?,
            decadeindex: ComputedVecBuilder::forced_import(path, name, compressed, options)?,
        })
    }

    pub fn compute<F>(
        &mut self,
        mut compute: F,
        indexer: &mut Indexer,
        indexes: &mut indexes::Vecs,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> color_eyre::Result<()>
    where
        F: FnMut(&mut ComputedVec<Txindex, T>) -> Result<()>,
    {
        compute(&mut self.txindex)?;

        self.height.compute(
            starting_indexes.height,
            &mut self.txindex,
            indexer.mut_vecs().height_to_first_txindex.mut_vec(),
            indexes.height_to_last_txindex.mut_vec(),
            exit,
        )?;

        self.dateindex.from_aligned(
            starting_indexes.dateindex,
            &mut self.height,
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

        self.difficultyepoch.from_aligned(
            starting_indexes.difficultyepoch,
            &mut self.height,
            indexes.difficultyepoch_to_first_height.mut_vec(),
            indexes.difficultyepoch_to_last_height.mut_vec(),
            exit,
        )?;

        Ok(())
    }

    pub fn any_vecs(&self) -> Vec<&dyn AnyStorableVec> {
        [
            vec![self.txindex.any_vec()],
            self.height.any_vecs(),
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
