mod base;
mod core;
mod full;

pub use base::ActivityBase;
pub use self::core::ActivityCore;
pub use full::ActivityFull;

use brk_error::Result;
use brk_types::{Height, Indexes, Sats, Version};
use vecdb::Exit;

use crate::blocks;

pub trait ActivityLike: Send + Sync {
    fn as_base(&self) -> &ActivityBase;
    fn as_base_mut(&mut self) -> &mut ActivityBase;
    fn min_len(&self) -> usize;
    fn truncate_push(
        &mut self,
        height: Height,
        sent: Sats,
        satblocks_destroyed: Sats,
        satdays_destroyed: Sats,
    ) -> Result<()>;
    fn validate_computed_versions(&mut self, base_version: Version) -> Result<()>;
    fn compute_from_stateful(
        &mut self,
        starting_indexes: &Indexes,
        others: &[&ActivityBase],
        exit: &Exit,
    ) -> Result<()>;
    fn compute_rest_part1(
        &mut self,
        blocks: &blocks::Vecs,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()>;
}

impl ActivityLike for ActivityBase {
    fn as_base(&self) -> &ActivityBase { self }
    fn as_base_mut(&mut self) -> &mut ActivityBase { self }
    fn min_len(&self) -> usize { self.min_len() }
    fn truncate_push(&mut self, height: Height, sent: Sats, satblocks_destroyed: Sats, satdays_destroyed: Sats) -> Result<()> {
        self.truncate_push(height, sent, satblocks_destroyed, satdays_destroyed)
    }
    fn validate_computed_versions(&mut self, base_version: Version) -> Result<()> {
        self.validate_computed_versions(base_version)
    }
    fn compute_from_stateful(&mut self, starting_indexes: &Indexes, others: &[&ActivityBase], exit: &Exit) -> Result<()> {
        self.compute_from_stateful(starting_indexes, others, exit)
    }
    fn compute_rest_part1(&mut self, blocks: &blocks::Vecs, starting_indexes: &Indexes, exit: &Exit) -> Result<()> {
        self.compute_rest_part1(blocks, starting_indexes, exit)
    }
}

impl ActivityLike for ActivityFull {
    fn as_base(&self) -> &ActivityBase { &self.inner }
    fn as_base_mut(&mut self) -> &mut ActivityBase { &mut self.inner }
    fn min_len(&self) -> usize { self.inner.min_len() }
    fn truncate_push(&mut self, height: Height, sent: Sats, satblocks_destroyed: Sats, satdays_destroyed: Sats) -> Result<()> {
        self.inner.truncate_push(height, sent, satblocks_destroyed, satdays_destroyed)
    }
    fn validate_computed_versions(&mut self, base_version: Version) -> Result<()> {
        self.inner.validate_computed_versions(base_version)
    }
    fn compute_from_stateful(&mut self, starting_indexes: &Indexes, others: &[&ActivityBase], exit: &Exit) -> Result<()> {
        self.compute_from_stateful(starting_indexes, others, exit)
    }
    fn compute_rest_part1(&mut self, blocks: &blocks::Vecs, starting_indexes: &Indexes, exit: &Exit) -> Result<()> {
        self.compute_rest_part1(blocks, starting_indexes, exit)
    }
}
