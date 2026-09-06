// CDN-facing (RFC 9213). Two tiers: live (chain-state, changes per block /
// mempool event) and cached (stable, ETag-invalidated).
//
// `Live` permits one second of edge reuse, then requires origin validation.
// Dependency failures do not extend that freshness window.
// `Aggressive` caches stable responses for up to a year and treats them as
// `immutable` (RFC 8246) — the operator must purge the CDN on every deploy.
/// CDN caching strategy for stable responses (immutable / block-bound /
/// historical series). Live-tier and deployment-bound responses are unaffected.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum CdnCacheMode {
    /// One second of CDN freshness, then revalidation. No stale-on-error reuse.
    #[default]
    Live,
    /// CDN holds stable responses for up to a year and treats them as immutable.
    /// Operator must purge on every deploy.
    Aggressive,
}
