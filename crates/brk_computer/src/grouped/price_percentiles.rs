use brk_error::Result;
use brk_traversable::{Traversable, TreeNode};
use brk_types::{Dollars, Height, Version};
use vecdb::{AnyExportableVec, AnyStoredVec, Database, EagerVec, Exit, GenericStoredVec, PcoVec};

use crate::{Indexes, indexes, stateful::Flushable};

use super::{ComputedVecsFromHeight, Source, VecBuilderOptions};

pub const PERCENTILES: [u8; 19] = [
    5, 10, 15, 20, 25, 30, 35, 40, 45, 50, 55, 60, 65, 70, 75, 80, 85, 90, 95,
];
pub const PERCENTILES_LEN: usize = PERCENTILES.len();

#[derive(Clone)]
pub struct PricePercentiles {
    pub vecs: [Option<ComputedVecsFromHeight<Dollars>>; PERCENTILES_LEN],
}

const VERSION: Version = Version::ZERO;

impl PricePercentiles {
    pub fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
        compute: bool,
    ) -> Result<Self> {
        let vecs = PERCENTILES.map(|p| {
            compute.then(|| {
                ComputedVecsFromHeight::forced_import(
                    db,
                    &format!("{name}_price_pct{p:02}"),
                    Source::Compute,
                    version + VERSION,
                    indexes,
                    VecBuilderOptions::default().add_last(),
                )
                .unwrap()
            })
        });

        Ok(Self { vecs })
    }

    pub fn truncate_push(
        &mut self,
        height: Height,
        percentile_prices: &[Dollars; PERCENTILES_LEN],
    ) -> Result<()> {
        for (i, vec) in self.vecs.iter_mut().enumerate() {
            if let Some(v) = vec {
                v.height
                    .as_mut()
                    .unwrap()
                    .truncate_push(height, percentile_prices[i])?;
            }
        }
        Ok(())
    }

    pub fn compute_rest(
        &mut self,
        indexes: &indexes::Vecs,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        for vec in self.vecs.iter_mut().flatten() {
            vec.compute_rest(
                indexes,
                starting_indexes,
                exit,
                None::<&EagerVec<PcoVec<Height, Dollars>>>,
            )?;
        }
        Ok(())
    }

    pub fn get(&self, percentile: u8) -> Option<&ComputedVecsFromHeight<Dollars>> {
        PERCENTILES
            .iter()
            .position(|&p| p == percentile)
            .and_then(|i| self.vecs[i].as_ref())
    }
}

impl Flushable for PricePercentiles {
    fn safe_flush(&mut self, exit: &Exit) -> Result<()> {
        for vec in self.vecs.iter_mut().flatten() {
            if let Some(height_vec) = vec.height.as_mut() {
                height_vec.safe_flush(exit)?;
            }
        }
        Ok(())
    }
}

impl PricePercentiles {
    pub fn safe_write(&mut self, exit: &Exit) -> Result<()> {
        for vec in self.vecs.iter_mut().flatten() {
            if let Some(height_vec) = vec.height.as_mut() {
                height_vec.safe_write(exit)?;
            }
        }
        Ok(())
    }
}

impl Traversable for PricePercentiles {
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
