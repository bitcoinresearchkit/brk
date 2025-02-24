#[cfg(feature = "core")]
pub mod core {
    #[doc(inline)]
    pub use brk_core::*;
}

#[cfg(feature = "computer")]
pub mod computer {
    #[doc(inline)]
    pub use brk_computer::*;
}

#[cfg(feature = "fetcher")]
pub mod fetcher {
    #[doc(inline)]
    pub use brk_fetcher::*;
}

#[cfg(feature = "indexer")]
pub mod indexer {
    #[doc(inline)]
    pub use brk_indexer::*;
}

#[cfg(feature = "logger")]
pub mod logger {
    #[doc(inline)]
    pub use brk_logger::*;
}

#[cfg(feature = "parser")]
pub mod parser {
    #[doc(inline)]
    pub use brk_parser::*;
}

#[cfg(feature = "server")]
pub mod server {
    #[doc(inline)]
    pub use brk_server::*;
}
