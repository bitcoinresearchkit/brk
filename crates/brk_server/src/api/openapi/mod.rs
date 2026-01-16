//
// https://docs.rs/schemars/latest/schemars/derive.JsonSchema.html
//
// Scalar:
// - Documentation: https://guides.scalar.com/scalar/scalar-api-references
// - Configuration: https://guides.scalar.com/scalar/scalar-api-references/configuration
// - Examples:
//   - https://docs.machines.dev/
//   - https://tailscale.com/api
//   - https://api.supabase.com/api/v1
//

mod trim;

pub use trim::trim_openapi_json;

use aide::openapi::{Contact, Info, License, OpenApi, Tag};

use crate::VERSION;

pub fn create_openapi() -> OpenApi {
    let info = Info {
        title: "Bitcoin Research Kit".to_string(),
        description: Some(
            r#"API for querying Bitcoin blockchain data and on-chain metrics.

### Features

- **Metrics**: Thousands of time-series metrics across multiple indexes (date, block height, etc.)
- **[Mempool.space](https://mempool.space/docs/api/rest) compatible** (WIP): Most non-metrics endpoints follow the mempool.space API format
- **Multiple formats**: JSON and CSV output
- **LLM-optimized**: Compact OpenAPI spec at [`/api.json`](/api.json) for AI tools (full spec at [`/openapi.json`](/openapi.json))

### Client Libraries

- [JavaScript](https://www.npmjs.com/package/brk-client)
- [Python](https://pypi.org/project/brk-client/)
- [Rust](https://crates.io/crates/brk_client)

### Links

- [GitHub](https://github.com/bitcoinresearchkit/brk)
- [Bitview](https://bitview.space) - Web app built on this API"#
                .to_string(),
        ),
        version: format!("v{VERSION}"),
        contact: Some(Contact {
            name: Some("Bitcoin Research Kit".to_string()),
            url: Some("https://github.com/bitcoinresearchkit/brk".to_string()),
            email: Some("hello@bitcoinresearchkit.org".to_string()),
            ..Contact::default()
        }),
        license: Some(License {
            name: "MIT".to_string(),
            url: Some(
                "https://github.com/bitcoinresearchkit/brk/blob/main/docs/LICENSE.md".to_string(),
            ),
            ..License::default()
        }),
        ..Info::default()
    };

    let tags = vec![
        Tag {
            name: "Server".to_string(),
            description: Some(
                "API metadata, health monitoring, and OpenAPI specifications.".to_string(),
            ),
            ..Default::default()
        },
        Tag {
            name: "Metrics".to_string(),
            description: Some(
                "Access thousands of Bitcoin network metrics and time-series data. Query historical statistics \
                across various indexes (date, week, month, block height) with JSON or CSV output."
                    .to_string(),
            ),
            ..Default::default()
        },
        Tag {
            name: "Blocks".to_string(),
            description: Some(
                "Retrieve block data by hash or height. Access block headers, transaction lists, \
                and raw block bytes.\n\n\
                *[Mempool.space](https://mempool.space/docs/api/rest) compatible (WIP).*"
                    .to_string(),
            ),
            ..Default::default()
        },
        Tag {
            name: "Transactions".to_string(),
            description: Some(
                "Retrieve transaction data by txid. Access full transaction details, confirmation \
                status, raw hex, and output spend information.\n\n\
                *[Mempool.space](https://mempool.space/docs/api/rest) compatible (WIP).*"
                    .to_string(),
            ),
            ..Default::default()
        },
        Tag {
            name: "Addresses".to_string(),
            description: Some(
                "Query Bitcoin address data including balances, transaction history, and UTXOs. \
                Supports all address types: P2PKH, P2SH, P2WPKH, P2WSH, and P2TR.\n\n\
                *[Mempool.space](https://mempool.space/docs/api/rest) compatible (WIP).*"
                    .to_string(),
            ),
            ..Default::default()
        },
        Tag {
            name: "Mempool".to_string(),
            description: Some(
                "Monitor unconfirmed transactions and fee estimates. Get mempool statistics, \
                transaction IDs, and recommended fee rates for different confirmation targets.\n\n\
                *[Mempool.space](https://mempool.space/docs/api/rest) compatible (WIP).*"
                    .to_string(),
            ),
            ..Default::default()
        },
        Tag {
            name: "Mining".to_string(),
            description: Some(
                "Mining statistics including pool distribution, hashrate, difficulty adjustments, \
                block rewards, and fee rates across configurable time periods.\n\n\
                *[Mempool.space](https://mempool.space/docs/api/rest) compatible (WIP).*"
                    .to_string(),
            ),
            ..Default::default()
        },
    ];

    OpenApi {
        info,
        tags,
        ..OpenApi::default()
    }
}
