use aide::openapi::{Info, OpenApi, Tag};

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

use crate::VERSION;

pub fn create_openapi() -> OpenApi {
    let info = Info {
        title: "Bitcoin Research Kit".to_string(),
        description: Some(
            "API for querying Bitcoin blockchain data including addresses, transactions, and chain statistics. This API provides low-level access to indexed blockchain data with advanced analytics capabilities."
                .to_string(),
        ),
        version: format!("v{VERSION}"),
        ..Info::default()
    };

    let tags = vec![
        Tag {
            name: "Addresses".to_string(),
            description: Some(
                "Query Bitcoin address data including balances, transaction history, and UTXOs. \
                Supports all address types: P2PKH, P2SH, P2WPKH, P2WSH, and P2TR."
                    .to_string()
            ),
            ..Default::default()
        },
        Tag {
            name: "Blocks".to_string(),
            description: Some(
                "Retrieve block data by hash or height. Access block headers, transaction lists, \
                and raw block bytes."
                    .to_string()
            ),
            ..Default::default()
        },
        Tag {
            name: "Mempool".to_string(),
            description: Some(
                "Monitor unconfirmed transactions and fee estimates. Get mempool statistics, \
                transaction IDs, and recommended fee rates for different confirmation targets."
                    .to_string()
            ),
            ..Default::default()
        },
        Tag {
            name: "Metrics".to_string(),
            description: Some(
                "Access Bitcoin network metrics and time-series data. Query historical statistics \
                across various indexes (date, week, month, year, halving epoch) with JSON or CSV output."
                    .to_string()
            ),
            ..Default::default()
        },
        Tag {
            name: "Mining".to_string(),
            description: Some(
                "Mining statistics including pool distribution, hashrate, difficulty adjustments, \
                block rewards, and fee rates across configurable time periods."
                    .to_string()
            ),
            ..Default::default()
        },
        Tag {
            name: "Server".to_string(),
            description: Some(
                "API metadata and health monitoring. Version information and service status."
                    .to_string()
            ),
            ..Default::default()
        },
        Tag {
            name: "Transactions".to_string(),
            description: Some(
                "Retrieve transaction data by txid. Access full transaction details, confirmation \
                status, raw hex, and output spend information."
                    .to_string()
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
