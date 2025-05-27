#![doc = include_str!(concat!("../", env!("CARGO_PKG_README")))]

#[cfg(feature = "core")]
#[doc(inline)]
pub use brk_core as core;

#[cfg(feature = "computer")]
#[doc(inline)]
pub use brk_computer as computer;

#[cfg(feature = "exit")]
#[doc(inline)]
pub use brk_exit as exit;

#[cfg(feature = "fetcher")]
#[doc(inline)]
pub use brk_fetcher as fetcher;

#[cfg(feature = "indexer")]
#[doc(inline)]
pub use brk_indexer as indexer;

#[cfg(feature = "logger")]
#[doc(inline)]
pub use brk_logger as logger;

#[cfg(feature = "parser")]
#[doc(inline)]
pub use brk_parser as parser;

#[cfg(feature = "query")]
#[doc(inline)]
pub use brk_query as query;

#[cfg(feature = "server")]
#[doc(inline)]
pub use brk_server as server;

#[cfg(feature = "store")]
#[doc(inline)]
pub use brk_store as store;

#[cfg(feature = "vec")]
#[doc(inline)]
pub use brk_vec as vec;
