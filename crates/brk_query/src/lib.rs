#![doc = include_str!("../README.md")]
#![allow(clippy::module_inception)]

use std::sync::Arc;

use brk_computer::Computer;
use brk_indexer::Indexer;
use brk_mempool::Mempool;
use brk_reader::Reader;
use brk_types::Height;
use vecdb::AnyStoredVec;

// Infrastructure modules
#[cfg(feature = "tokio")]
mod r#async;
mod output;
mod vecs;

// Query impl blocks (extend Query with domain methods)
mod r#impl;

// Re-exports
#[cfg(feature = "tokio")]
pub use r#async::*;
pub use brk_types::{
    DataRange, DataRangeFormat, MetricSelection, MetricSelectionLegacy, PaginatedMetrics,
    Pagination, PaginationIndex,
};
pub use r#impl::BLOCK_TXS_PAGE_SIZE;
pub use output::{LegacyValue, Output, OutputLegacy};

pub use vecs::Vecs;

#[derive(Clone)]
pub struct Query(Arc<QueryInner<'static>>);
struct QueryInner<'a> {
    vecs: &'a Vecs<'a>,
    reader: Reader,
    indexer: &'a Indexer,
    computer: &'a Computer,
    mempool: Option<Mempool>,
}

impl Query {
    pub fn build(
        reader: &Reader,
        indexer: &Indexer,
        computer: &Computer,
        mempool: Option<Mempool>,
    ) -> Self {
        let reader = reader.clone();
        let indexer = Box::leak(Box::new(indexer.clone()));
        let computer = Box::leak(Box::new(computer.clone()));
        let vecs = Box::leak(Box::new(Vecs::build(indexer, computer)));

        Self(Arc::new(QueryInner {
            vecs,
            reader,
            indexer,
            computer,
            mempool,
        }))
    }

    /// Current indexed height
    pub fn height(&self) -> Height {
        Height::from(self.indexer().vecs.block.height_to_blockhash.stamp())
    }

    #[inline]
    pub fn reader(&self) -> &Reader {
        &self.0.reader
    }

    #[inline]
    pub fn indexer(&self) -> &Indexer {
        self.0.indexer
    }

    #[inline]
    pub fn computer(&self) -> &Computer {
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
