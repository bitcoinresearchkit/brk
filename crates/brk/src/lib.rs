#![doc = include_str!("../README.md")]

#[cfg(feature = "bencher")]
#[doc(inline)]
pub use brk_bencher as bencher;

#[cfg(feature = "binder")]
#[doc(inline)]
pub use brk_binder as binder;

#[cfg(feature = "bundler")]
#[doc(inline)]
pub use brk_bundler as bundler;

#[cfg(feature = "computer")]
#[doc(inline)]
pub use brk_computer as computer;

#[cfg(feature = "error")]
#[doc(inline)]
pub use brk_error as error;

#[cfg(feature = "fetcher")]
#[doc(inline)]
pub use brk_fetcher as fetcher;

#[cfg(feature = "grouper")]
#[doc(inline)]
pub use brk_grouper as grouper;

#[cfg(feature = "indexer")]
#[doc(inline)]
pub use brk_indexer as indexer;

#[cfg(feature = "iterator")]
#[doc(inline)]
pub use brk_iterator as iterator;

#[cfg(feature = "logger")]
#[doc(inline)]
pub use brk_logger as logger;

#[cfg(feature = "mcp")]
#[doc(inline)]
pub use brk_mcp as mcp;

#[cfg(feature = "mempool")]
#[doc(inline)]
pub use brk_mempool as mempool;

#[cfg(feature = "query")]
#[doc(inline)]
pub use brk_query as query;

#[cfg(feature = "reader")]
#[doc(inline)]
pub use brk_reader as reader;

#[cfg(feature = "rpc")]
#[doc(inline)]
pub use brk_rpc as rpc;

#[cfg(feature = "server")]
#[doc(inline)]
pub use brk_server as server;

#[cfg(feature = "store")]
#[doc(inline)]
pub use brk_store as store;

#[cfg(feature = "traversable")]
#[doc(inline)]
pub use brk_traversable as traversable;

#[cfg(feature = "types")]
#[doc(inline)]
pub use brk_types as types;
