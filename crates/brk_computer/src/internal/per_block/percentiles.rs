use brk_error::Result;
use brk_traversable::{Traversable, TreeNode};
use brk_types::{Cents, Height, Version};
use vecdb::{AnyExportableVec, Database, ReadOnlyClone, Ro, Rw, StorageMode, WritableVec};

use crate::indexes;
use crate::internal::{ComputedPerBlock, Price};

pub const PERCENTILES: [u8; 19] = [
    5, 10, 15, 20, 25, 30, 35, 40, 45, 50, 55, 60, 65, 70, 75, 80, 85, 90, 95,
];
pub const PERCENTILES_LEN: usize = PERCENTILES.len();

pub struct PercentilesVecs<M: StorageMode = Rw> {
    pub vecs: [Price<ComputedPerBlock<Cents, M>>; PERCENTILES_LEN],
}

const VERSION: Version = Version::ONE;

impl PercentilesVecs {
    pub(crate) fn forced_import(
        db: &Database,
        prefix: &str,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let vecs = PERCENTILES
            .into_iter()
            .map(|p| {
                let metric_name = format!("{prefix}_pct{p:02}");
                Price::forced_import(db, &metric_name, version + VERSION, indexes)
            })
            .collect::<Result<Vec<_>>>()?
            .try_into()
            .ok()
            .expect("PERCENTILES length mismatch");

        Ok(Self { vecs })
    }

    /// Push percentile prices at this height (in cents).
    pub(crate) fn truncate_push(
        &mut self,
        height: Height,
        percentile_prices: &[Cents; PERCENTILES_LEN],
    ) -> Result<()> {
        for (i, v) in self.vecs.iter_mut().enumerate() {
            v.cents.height.truncate_push(height, percentile_prices[i])?;
        }
        Ok(())
    }

    /// Validate computed versions or reset if mismatched.
    pub(crate) fn validate_computed_version_or_reset(&mut self, version: Version) -> Result<()> {
        for vec in self.vecs.iter_mut() {
            vec.cents
                .height
                .validate_computed_version_or_reset(version)?;
        }
        Ok(())
    }
}

impl ReadOnlyClone for PercentilesVecs {
    type ReadOnly = PercentilesVecs<Ro>;

    fn read_only_clone(&self) -> Self::ReadOnly {
        PercentilesVecs {
            vecs: self.vecs.each_ref().map(|v| v.read_only_clone()),
        }
    }
}

impl<M: StorageMode> Traversable for PercentilesVecs<M>
where
    Price<ComputedPerBlock<Cents, M>>: Traversable,
{
    fn to_tree_node(&self) -> TreeNode {
        TreeNode::Branch(
            PERCENTILES
                .iter()
                .zip(self.vecs.iter())
                .map(|(p, v)| (format!("pct{p:02}"), v.to_tree_node()))
                .collect(),
        )
    }

    fn iter_any_exportable(&self) -> impl Iterator<Item = &dyn AnyExportableVec> {
        self.vecs.iter().flat_map(|p| p.iter_any_exportable())
    }
}
