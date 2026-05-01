#![doc = include_str!("../README.md")]
#![allow(clippy::module_inception)]

use std::{path::Path, sync::Arc};

use brk_computer::Computer;
use brk_error::{OptionData, Result};
use brk_indexer::Indexer;
use brk_mempool::Mempool;
use brk_reader::Reader;
use brk_rpc::Client;
use brk_types::{BlockHash, BlockHashPrefix, Height, SyncStatus};
use vecdb::{ReadOnlyClone, ReadableVec, Ro};

#[cfg(feature = "tokio")]
mod r#async;
mod vecs;

mod r#impl;

#[cfg(feature = "tokio")]
pub use r#async::*;
pub use r#impl::{BLOCK_TXS_PAGE_SIZE, ResolvedQuery};
pub use vecs::Vecs;

#[derive(Clone)]
pub struct Query(Arc<QueryInner<'static>>);
struct QueryInner<'a> {
    vecs: &'a Vecs<'a>,
    client: Client,
    reader: Reader,
    indexer: &'a Indexer<Ro>,
    computer: &'a Computer<Ro>,
    mempool: Option<Mempool>,
}

impl Query {
    pub fn build(
        reader: &Reader,
        indexer: &Indexer,
        computer: &Computer,
        mempool: Option<Mempool>,
    ) -> Self {
        let client = reader.client().clone();
        let reader = reader.clone();
        let indexer = Box::leak(Box::new(indexer.read_only_clone()));
        let computer = Box::leak(Box::new(computer.read_only_clone()));
        let vecs = Box::leak(Box::new(Vecs::build(indexer, computer)));

        Self(Arc::new(QueryInner {
            vecs,
            client,
            reader,
            indexer,
            computer,
            mempool,
        }))
    }

    /// Current indexed height
    pub fn indexed_height(&self) -> Height {
        Height::from(self.indexer().vecs.blocks.blockhash.inner.stamp())
    }

    /// Current computed height (series)
    pub fn computed_height(&self) -> Height {
        Height::from(self.computer().distribution.supply_state.stamp())
    }

    /// Minimum of indexed and computed heights
    pub fn height(&self) -> Height {
        self.indexed_height().min(self.computed_height())
    }

    /// Tip block hash, cached in the indexer.
    #[inline]
    pub fn tip_blockhash(&self) -> BlockHash {
        self.indexer().tip_blockhash()
    }

    /// Tip block hash prefix for cache etags.
    #[inline]
    pub fn tip_hash_prefix(&self) -> BlockHashPrefix {
        BlockHashPrefix::from(&self.tip_blockhash())
    }

    /// Build sync status with the given tip height
    pub fn sync_status(&self, tip_height: Height) -> Result<SyncStatus> {
        let indexed_height = self.indexed_height();
        let computed_height = self.computed_height();
        let blocks_behind = Height::from(tip_height.saturating_sub(*indexed_height));
        let last_indexed_at_unix = self
            .indexer()
            .vecs
            .blocks
            .timestamp
            .collect_one(indexed_height)
            .data()?;

        Ok(SyncStatus {
            indexed_height,
            computed_height,
            tip_height,
            blocks_behind,
            last_indexed_at: last_indexed_at_unix.to_iso8601(),
            last_indexed_at_unix,
        })
    }

    #[inline]
    pub fn reader(&self) -> &Reader {
        &self.0.reader
    }

    #[inline]
    pub fn client(&self) -> &Client {
        &self.0.client
    }

    #[inline]
    pub fn blocks_dir(&self) -> &Path {
        self.0.reader.blocks_dir()
    }

    #[inline]
    pub fn indexer(&self) -> &Indexer<Ro> {
        self.0.indexer
    }

    #[inline]
    pub fn computer(&self) -> &Computer<Ro> {
        self.0.computer
    }

    #[inline]
    pub fn mempool(&self) -> Option<&Mempool> {
        self.0.mempool.as_ref()
    }

    #[inline]
    pub fn vecs(&self) -> &'static Vecs<'static> {
        self.0.vecs
    }
}
