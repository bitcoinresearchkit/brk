use flume::Sender;
use lsm_tree::Tree;

use crate::{locked_file::LockedFileGuard, worker_pool::WorkerMessage};

/// Shared keyspace state.
pub struct Inner {
    /// Stable keyspace name.
    pub name: String,
    /// Immutable-table LSM tree.
    pub tree: Tree,
    /// Background worker sender.
    pub worker: Sender<WorkerMessage>,
    /// Keeps the database lock alive while handles exist.
    pub _lock: LockedFileGuard,
}
