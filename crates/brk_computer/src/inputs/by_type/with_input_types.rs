//! Generic `all` + per-input-type container (11 spendable types — no
//! op_return since op_return outputs are non-spendable). Used by
//! `inputs/by_type/`. Mirrors `WithAddrTypes` and `WithOutputTypes`.

use brk_cohort::SpendableType;
use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Height, Version};
use schemars::JsonSchema;
use vecdb::{AnyStoredVec, AnyVec, Database, Exit, WritableVec};

use crate::{
    indexes,
    internal::{NumericValue, PerBlockCumulativeRolling, WindowStartVec, Windows},
};

/// `all` aggregate plus per-input-type breakdown across the 11 spendable
/// output types (everything except op_return). The "type" of an input is
/// the type of the previous output it spends.
#[derive(Clone, Traversable)]
pub struct WithInputTypes<T> {
    pub all: T,
    #[traversable(flatten)]
    pub by_type: SpendableType<T>,
}

impl<T, C> WithInputTypes<PerBlockCumulativeRolling<T, C>>
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
            by_type: SpendableType::try_new(|_, name| make(&per_type_name(name)))?,
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
        self.all
            .block
            .validate_and_truncate(dep_version, at_height)?;
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
