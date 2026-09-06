//! Projected next block: full template and incremental diff.

use crate::internals::*;

use std::sync::Arc;

use brk_error::Result;

use brk_types::{
    BlockTemplate, BlockTemplateDiff, BlockTemplateDiffEntry, MempoolBlock, NextBlockHash, Txid,
};
use rustc_hash::{FxHashMap, FxHashSet};

use crate::{Mempool, ResolvedBlockTemplateDiff, Snapshot};

/// One immutable published template, retained for validation and body construction.
#[derive(Clone)]
pub struct BlockTemplateSource {
    snapshot: Arc<Snapshot>,
}

impl PartialEq for BlockTemplateSource {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.snapshot, &other.snapshot)
    }
}

impl Eq for BlockTemplateSource {}

impl BlockTemplateSource {
    fn new(snapshot: Arc<Snapshot>) -> Self {
        Self { snapshot }
    }

    /// Identity of the captured body; unavailable selections must not revalidate.
    pub fn hash(&self) -> Result<NextBlockHash> {
        self.snapshot.ensure_projection()?;
        Ok(self.snapshot.next_block_hash)
    }

    /// Build only after validation, without observing a newer publication.
    pub fn build(self) -> Result<BlockTemplate> {
        let hash = self.hash()?;
        Ok(BlockTemplate {
            hash,
            stats: self
                .snapshot
                .block_stats
                .first()
                .map(MempoolBlock::from)
                .unwrap_or_default(),
            transactions: self
                .snapshot
                .template_transactions()
                .iter()
                .map(|tx| tx.as_ref().clone())
                .collect(),
        })
    }
}

impl Mempool {
    pub fn next_block_hash(&self) -> Result<NextBlockHash> {
        self.block_template_source().hash()
    }

    /// Capture the currently published template without cloning transaction bodies.
    #[must_use]
    pub fn block_template_source(&self) -> BlockTemplateSource {
        let snapshot = self.snapshot();
        BlockTemplateSource::new(snapshot)
    }

    /// Full projected next block: Core's `getblocktemplate` selection
    /// (block 0) with aggregate stats and full tx bodies in GBT order.
    #[must_use]
    pub fn block_template(&self) -> Result<BlockTemplate> {
        self.block_template_source().build()
    }

    /// Full template and the exact source identity used to assemble it.
    #[must_use]
    pub fn block_template_with_source(&self) -> Result<(BlockTemplate, BlockTemplateSource)> {
        let source = self.block_template_source();
        let template = source.clone().build()?;
        Ok((template, source))
    }

    /// Delta of the projected next block since `since`. `None` when
    /// `since` has aged out of the rebuilder's history (server should
    /// 404 -> client falls back to `block_template`).
    ///
    /// `order` walks the new template in template order. Each entry is
    /// either a `Retained` index into the prior template (which the
    /// client cached when it obtained `since`) or a `New` inline body.
    /// `removed` is the convenience list of txids that left.
    #[must_use]
    pub fn block_template_diff(&self, since: NextBlockHash) -> Result<Option<BlockTemplateDiff>> {
        self.resolve_block_template_diff(since)
            .map(|resolved| {
                self.block_template_diff_resolved(resolved)
                    .map(|(diff, _)| diff)
            })
            .transpose()
    }

    /// Validate and capture the historical side of one diff request.
    #[must_use]
    pub fn resolve_block_template_diff(
        &self,
        since: NextBlockHash,
    ) -> Option<ResolvedBlockTemplateDiff> {
        let past = self.rebuilder().historical_block0(since)?;
        let source = self.block_template_source();
        Some(ResolvedBlockTemplateDiff::new(since, past, source))
    }

    /// Build a diff from already-resolved history and return its actual source.
    #[must_use]
    pub fn block_template_diff_resolved(
        &self,
        resolved: ResolvedBlockTemplateDiff,
    ) -> Result<(BlockTemplateDiff, BlockTemplateSource)> {
        let source = resolved.source().clone();
        let diff = resolved.build()?;
        Ok((diff, source))
    }
}

impl ResolvedBlockTemplateDiff {
    /// Build against the publication captured when history was resolved.
    pub fn build(self) -> Result<BlockTemplateDiff> {
        let (since, past, source) = self.into_parts();
        let hash = source.hash()?;
        let prior_index: FxHashMap<Txid, u32> = past
            .iter()
            .enumerate()
            .map(|(idx, tx)| (tx.txid, idx as u32))
            .collect();
        let snap = &source.snapshot;
        let mut order = Vec::with_capacity(snap.blocks.first().map_or(0, Vec::len));
        let mut current: FxHashSet<Txid> = FxHashSet::default();
        for tx in snap.template_transactions().iter() {
            let txid = tx.txid;
            current.insert(txid);
            match prior_index.get(&txid) {
                Some(&idx) if Arc::ptr_eq(tx, &past[idx as usize]) => {
                    order.push(BlockTemplateDiffEntry::Retained(idx))
                }
                _ => order.push(BlockTemplateDiffEntry::New(tx.as_ref().clone())),
            }
        }
        let removed = past
            .iter()
            .filter(|tx| !current.contains(&tx.txid))
            .map(|tx| tx.txid)
            .collect();
        Ok(BlockTemplateDiff {
            hash,
            since,
            order,
            removed,
        })
    }
}

#[cfg(test)]
#[path = "../../tests/unit/api/block_template.rs"]
mod tests;
