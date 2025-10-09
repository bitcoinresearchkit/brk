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
            "API for querying Bitcoin blockchain data including addresses, transactions, and chain statistics. This API provides low-level access to indexed blockchain data with advanced analytics capabilities.\n\n\
            ⚠️ **Early Development**: This API is in very early stages of development. Breaking changes may occur without notice. For a more stable experience, [self-host](/install) or use the [hosting service](/service)."
                .to_string(),
        ),
        version: format!("v{VERSION}"),
        ..Info::default()
    };

    let tags = vec![
        Tag {
            name: "Chain".to_string(),
            description: Some(
                "Explore Bitcoin blockchain data: addresses, transactions, blocks, balances, and UTXOs."
                    .to_string()
            ),
            ..Default::default()
        },
        Tag {
            name: "Metrics".to_string(),
            description: Some(
                "Access Bitcoin network metrics and time-series data. Query historical and real-time \
                statistics across various blockchain dimensions and aggregation levels."
                    .to_string()
            ),
            ..Default::default()
        },
        Tag {
            name: "Server".to_string(),
            description: Some(
                "Metadata and utility endpoints for API status, health checks, and system information."
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
