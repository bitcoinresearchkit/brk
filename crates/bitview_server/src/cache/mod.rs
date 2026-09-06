//! HTTP cache layer. ETag-based revalidation with separate browser and CDN
//! directives (RFC 9213), plus serialized representations owned by the server:
//!
//! - [`CacheStrategy`] — *what kind of resource* the handler is returning
//!   (input enum picked by the route).
//! - [`CacheParams`]   — the *resolved* etag + Cache-Control + CDN-Cache-Control,
//!   derived from a strategy plus current chain tip.
//! - [`CdnCacheMode`]  — operator-level toggle for the CDN cached tier
//!   (owned by each server instance).

mod mode;
mod params;
mod strategy;
pub use mode::CdnCacheMode;
pub use params::CacheParams;
pub use params::ErrorCachePolicy;
pub use strategy::CacheStrategy;
