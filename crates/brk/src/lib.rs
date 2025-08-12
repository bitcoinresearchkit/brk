#![doc = include_str!("../README.md")]

#[cfg(feature = "bundler")]
#[doc(inline)]
pub use brk_bundler as bundler;

#[doc(inline)]
pub use brk_cli as cli;

#[cfg(feature = "structs")]
#[doc(inline)]
pub use brk_structs as structs;

#[cfg(feature = "computer")]
#[doc(inline)]
pub use brk_computer as computer;

#[cfg(feature = "error")]
#[doc(inline)]
pub use brk_error as error;

#[cfg(feature = "fetcher")]
#[doc(inline)]
pub use brk_fetcher as fetcher;

#[cfg(feature = "indexer")]
#[doc(inline)]
pub use brk_indexer as indexer;

#[cfg(feature = "logger")]
#[doc(inline)]
pub use brk_logger as logger;

#[cfg(feature = "mcp")]
#[doc(inline)]
pub use brk_mcp as mcp;

#[cfg(feature = "parser")]
#[doc(inline)]
pub use brk_parser as parser;

#[cfg(feature = "interface")]
#[doc(inline)]
pub use brk_interface as interface;

#[cfg(feature = "server")]
#[doc(inline)]
pub use brk_server as server;

#[cfg(feature = "store")]
#[doc(inline)]
pub use brk_store as store;
