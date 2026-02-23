#![doc = include_str!("../README.md")]
#![allow(clippy::module_inception)]

use std::sync::Arc;

use brk_computer::Computer;
use brk_indexer::Indexer;
use brk_mempool::Mempool;
use brk_reader::Reader;
use brk_rpc::Client;
use brk_types::Height;
use vecdb::{ReadOnlyClone, Ro};

#[cfg(feature = "tokio")]
mod r#async;
mod vecs;

mod r#impl;

#[cfg(feature = "tokio")]
pub use r#async::*;
pub use r#impl::BLOCK_TXS_PAGE_SIZE;
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
    pub fn height(&self) -> Height {
        Height::from(self.indexer().vecs.blocks.blockhash.stamp())
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
    pub fn blocks_dir(&self) -> &std::path::Path {
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
