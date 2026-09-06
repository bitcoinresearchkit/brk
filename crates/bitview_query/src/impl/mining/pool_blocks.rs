use crate::internals::*;

use brk_error::{Error, OptionData, Result};
use brk_types::{BlockHash, BlockInfoV1, Dollars, Height, PoolSlug};
use vecdb::ReadableVec;

use crate::{Query, ResolvedBlocks};

/// A pool-block page resolved against one exact published chain view.
pub struct ResolvedPoolBlocks {
    _publication: PluginReadGuard,
    chain: ResolvedBlocks,
    heights: Vec<Height>,
    prices: Vec<Dollars>,
    activity_anchor: Option<BlockHash>,
}

impl ResolvedPoolBlocks {
    #[inline]
    pub const fn activity_anchor(&self) -> Option<BlockHash> {
        self.activity_anchor
    }

    pub fn heights(&self) -> &[Height] {
        &self.heights
    }

    /// Captured prices in the same descending order as the selected heights.
    pub fn prices(&self) -> &[Dollars] {
        &self.prices
    }
}

impl Query {
    /// Resolve the page's exact block heights and activity anchor once.
    pub fn resolve_pool_blocks(
        &self,
        slug: PoolSlug,
        before_height: Option<Height>,
        limit: usize,
    ) -> Result<ResolvedPoolBlocks> {
        let publication = self.read_plugin(self.indexer())?;
        let chain = self.resolve_blocks(None, 0)?;
        let tip = chain.last_height().ok_or(Error::StateUpdating)?;
        let through_height = before_height.unwrap_or(tip).min(tip);
        let heights = self
            .plugins()
            .pools
            .heights
            .latest_heights(slug, through_height, limit);
        let activity_anchor = heights
            .first()
            .map(|height| {
                self.indexer()
                    .vecs()
                    .blocks
                    .blockhash
                    .inner
                    .collect_one(*height)
                    .data()
            })
            .transpose()?;
        let prices = heights
            .iter()
            .map(|height| {
                self.price()
                    .spot
                    .cents
                    .height
                    .inner
                    .collect_one(*height)
                    .data()
                    .map(Dollars::from)
            })
            .collect::<Result<Vec<_>>>()?;

        Ok(ResolvedPoolBlocks {
            _publication: publication,
            chain,
            heights,
            prices,
            activity_anchor,
        })
    }

    /// Load a resolved page without repeating its pool-height lookup.
    pub fn pool_blocks_resolved(&self, resolved: ResolvedPoolBlocks) -> Result<Vec<BlockInfoV1>> {
        let ResolvedPoolBlocks {
            _publication,
            chain,
            heights,
            prices,
            ..
        } = resolved;
        chain.build_v1_heights(self, &heights, &prices)
    }

    /// Page of blocks mined by `slug`, in descending height order, capped at
    /// `limit`. `before_height` is the inclusive upper bound to paginate from.
    pub fn pool_blocks(
        &self,
        slug: PoolSlug,
        before_height: Option<Height>,
        limit: usize,
    ) -> Result<Vec<BlockInfoV1>> {
        let resolved = self.resolve_pool_blocks(slug, before_height, limit)?;
        self.pool_blocks_resolved(resolved)
    }
}
use bitview_plugin::PluginReadGuard;
