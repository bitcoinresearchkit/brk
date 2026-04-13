//! Generic `all` + per-`OutputType` container (12 output types, including
//! op_return). Used by `outputs/by_type/`. Mirrors `WithAddrTypes` and
//! `WithInputTypes`.

use brk_cohort::ByType;
use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Height, Version};
use schemars::JsonSchema;
use vecdb::{AnyStoredVec, AnyVec, Database, Exit, WritableVec};

use crate::{
    indexes,
    internal::{NumericValue, PerBlockCumulativeRolling, WindowStartVec, Windows},
};

/// `all` aggregate plus per-`OutputType` breakdown across all 12 output
/// types (spendable + op_return).
#[derive(Clone, Traversable)]
pub struct WithOutputTypes<T> {
    pub all: T,
    #[traversable(flatten)]
    pub by_type: ByType<T>,
}

impl<T, C> WithOutputTypes<PerBlockCumulativeRolling<T, C>>
where
    T: NumericValue + JsonSchema + Into<C>,
    C: NumericValue + JsonSchema,
{
    pub(crate) fn forced_import_with(
        db: &Database,
        all_name: &str,
        per_type_name: impl Fn(&str) -> String,
        version: Version,
        indexes: &indexes::Vecs,
        cached_starts: &Windows<&WindowStartVec>,
    ) -> Result<Self> {
        let make = |name: &str| {
            PerBlockCumulativeRolling::forced_import(db, name, version, indexes, cached_starts)
        };
        Ok(Self {
            all: make(all_name)?,
            by_type: ByType::try_new(|_, name| make(&per_type_name(name)))?,
        })
    }

    pub(crate) fn min_stateful_len(&self) -> usize {
        self.by_type
            .iter()
            .map(|v| v.block.len())
            .min()
            .unwrap()
            .min(self.all.block.len())
    }

    pub(crate) fn write(&mut self) -> Result<()> {
        self.all.block.write()?;
        for v in self.by_type.iter_mut() {
            v.block.write()?;
        }
        Ok(())
    }

    pub(crate) fn validate_and_truncate(
        &mut self,
        dep_version: Version,
        at_height: Height,
    ) -> Result<()> {
        self.all.block.validate_and_truncate(dep_version, at_height)?;
        for v in self.by_type.iter_mut() {
            v.block.validate_and_truncate(dep_version, at_height)?;
        }
        Ok(())
    }

    pub(crate) fn truncate_if_needed_at(&mut self, len: usize) -> Result<()> {
        self.all.block.truncate_if_needed_at(len)?;
        for v in self.by_type.iter_mut() {
            v.block.truncate_if_needed_at(len)?;
        }
        Ok(())
    }

    pub(crate) fn compute_rest(&mut self, max_from: Height, exit: &Exit) -> Result<()> {
        self.all.compute_rest(max_from, exit)?;
        for v in self.by_type.iter_mut() {
            v.compute_rest(max_from, exit)?;
        }
        Ok(())
    }
}
