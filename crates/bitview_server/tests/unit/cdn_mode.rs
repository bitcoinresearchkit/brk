use std::net::Ipv4Addr;

use axum::http::HeaderMap;
use brk_types::Version;

use super::chain_fixture::run;
use crate::{CacheStrategy, CdnCacheMode, Server, ServerConfig};

#[test]
fn server_instances_keep_independent_cdn_modes() {
    run(|state, _| async move {
        let mut servers = Vec::new();
        for mode in [CdnCacheMode::Aggressive, CdnCacheMode::Live] {
            servers.push(
                Server::bind(
                    &state.query,
                    ServerConfig {
                        bind: Ipv4Addr::LOCALHOST.into(),
                        port: 0.into(),
                        cdn_cache_mode: mode,
                        ..ServerConfig::default()
                    },
                )
                .await
                .unwrap(),
            );
        }
        for _ in 0..2 {
            for (server, expected) in servers.iter().zip([
                "public, max-age=31536000, immutable",
                "public, max-age=1, must-revalidate",
            ]) {
                let response = server.state.respond_json_value(
                    &HeaderMap::new(),
                    CacheStrategy::Immutable(Version::ONE),
                    [0],
                );
                assert_eq!(response.headers()["cdn-cache-control"], expected);
                let live = server.state.respond_json_value(
                    &HeaderMap::new(),
                    CacheStrategy::LiveHash(1),
                    [1],
                );
                assert_eq!(
                    live.headers()["cdn-cache-control"],
                    "public, max-age=1, must-revalidate"
                );
            }
        }
    });
}
