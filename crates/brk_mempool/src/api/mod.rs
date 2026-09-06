//! Read-side accessors on [`crate::Mempool`]. Each submodule groups a
//! cohesive method set. Types flow back through `pub use`.

pub mod addr;
pub mod block_template;
pub mod block_template_diff;
pub mod fees;
pub mod histogram;
pub mod rbf;
pub mod tx;

pub use block_template::BlockTemplateSource;
pub use block_template_diff::ResolvedBlockTemplateDiff;
pub use rbf::{RbfForTx, RbfNode};
