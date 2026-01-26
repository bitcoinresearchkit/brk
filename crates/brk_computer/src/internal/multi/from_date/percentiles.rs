use brk_error::Result;
use brk_traversable::{Traversable, TreeNode};
use brk_types::{DateIndex, Dollars, Version};
use rayon::prelude::*;
use vecdb::{
    AnyExportableVec, AnyStoredVec, AnyVec, Database, EagerVec, Exit, GenericStoredVec, PcoVec,
};

use crate::{ComputeIndexes, indexes};

use super::Price;

pub const PERCENTILES: [u8; 19] = [
    5, 10, 15, 20, 25, 30, 35, 40, 45, 50, 55, 60, 65, 70, 75, 80, 85, 90, 95,
];
pub const PERCENTILES_LEN: usize = PERCENTILES.len();

#[derive(Clone)]
pub struct CostBasisPercentiles {
    pub vecs: [Option<Price>; PERCENTILES_LEN],
}

const VERSION: Version = Version::ZERO;

impl CostBasisPercentiles {
    pub fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
        compute: bool,
    ) -> Result<Self> {
        let vecs = PERCENTILES.map(|p| {
            compute.then(|| {
                let metric_name = if name.is_empty() {
                    format!("cost_basis_pct{p:02}")
                } else {
                    format!("{name}_cost_basis_pct{p:02}")
                };
                Price::forced_import(db, &metric_name, version + VERSION, indexes).unwrap()
            })
        });

        Ok(Self { vecs })
    }

    /// Get minimum length across dateindex-indexed vectors written in block loop.
    pub fn min_stateful_dateindex_len(&self) -> usize {
        self.vecs
            .iter()
            .filter_map(|v| v.as_ref())
            .map(|v| v.dateindex.len())
            .min()
            .unwrap_or(usize::MAX)
    }

    /// Push percentile prices at date boundary.
    /// Only called when dateindex is Some (last height of the day).
    pub fn truncate_push(
        &mut self,
        dateindex: DateIndex,
        percentile_prices: &[Dollars; PERCENTILES_LEN],
    ) -> Result<()> {
        for (i, vec) in self.vecs.iter_mut().enumerate() {
            if let Some(v) = vec {
                v.dateindex.truncate_push(dateindex, percentile_prices[i])?;
            }
        }
        Ok(())
    }

    pub fn compute_rest(&mut self, starting_indexes: &ComputeIndexes, exit: &Exit) -> Result<()> {
        for vec in self.vecs.iter_mut().flatten() {
            vec.compute_rest(
                starting_indexes,
                exit,
                None::<&EagerVec<PcoVec<DateIndex, Dollars>>>,
            )?;
        }
        Ok(())
    }

    pub fn get(&self, percentile: u8) -> Option<&Price> {
        PERCENTILES
            .iter()
            .position(|&p| p == percentile)
            .and_then(|i| self.vecs[i].as_ref())
    }
}

impl CostBasisPercentiles {
    pub fn write(&mut self) -> Result<()> {
        for vec in self.vecs.iter_mut().flatten() {
            vec.dateindex.write()?;
        }
        Ok(())
    }

    /// Returns a parallel iterator over all vecs for parallel writing.
    pub fn par_iter_mut(&mut self) -> impl ParallelIterator<Item = &mut dyn AnyStoredVec> {
        self.vecs
            .iter_mut()
            .flatten()
            .map(|v| &mut v.dateindex as &mut dyn AnyStoredVec)
            .collect::<Vec<_>>()
            .into_par_iter()
    }

    /// Validate computed versions or reset if mismatched.
    pub fn validate_computed_version_or_reset(&mut self, version: Version) -> Result<()> {
        for vec in self.vecs.iter_mut().flatten() {
            vec.dateindex.validate_computed_version_or_reset(version)?;
        }
        Ok(())
    }
}

impl Traversable for CostBasisPercentiles {
    fn to_tree_node(&self) -> TreeNode {
        TreeNode::Branch(
            PERCENTILES
                .iter()
                .zip(self.vecs.iter())
                .filter_map(|(p, v)| v.as_ref().map(|v| (format!("pct{p:02}"), v.to_tree_node())))
                .collect(),
        )
    }

    fn iter_any_exportable(&self) -> impl Iterator<Item = &dyn AnyExportableVec> {
        self.vecs
            .iter()
            .flatten()
            .flat_map(|p| p.iter_any_exportable())
    }
}
