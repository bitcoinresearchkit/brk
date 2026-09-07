use brk_error::{Error, Result};
use brk_mempool::Mempool;
use std::{sync::Arc, time::Instant};
use vecdb::ReadOnlyClone;

use bitview_plugin::Plugin;
use tokio::sync::Semaphore;
use tokio::task::spawn_blocking;

use crate::{Query, QueryPluginSet};

#[derive(Clone)]
pub struct AsyncQuery(Query);

impl AsyncQuery {
    pub fn read_deadline(&self) -> Instant {
        self.0
            .1
            .unwrap_or_else(|| Instant::now() + Query::UPDATE_WAIT_TIMEOUT)
    }
    /// Retry only the read operation, with no guards or worker admission held
    /// while awaiting a resource notification. Actions must use `run` instead.
    pub async fn read<F, T>(&self, f: F) -> Result<T>
    where
        F: FnOnce(&Query) -> Result<T> + Clone + Send + 'static,
        T: Send + 'static,
    {
        self.read_with_admission(None, f).await
    }

    pub async fn read_with_admission<F, T>(
        &self,
        admission: Option<&Arc<Semaphore>>,
        f: F,
    ) -> Result<T>
    where
        F: FnOnce(&Query) -> Result<T> + Clone + Send + 'static,
        T: Send + 'static,
    {
        let deadline = self.read_deadline();
        let read = async {
            loop {
                let permit = match admission {
                    Some(admission) => Some(
                        admission
                            .clone()
                            .acquire_owned()
                            .await
                            .map_err(|_| Error::Internal("query admission closed"))?,
                    ),
                    None => None,
                };
                // Subscribe before the attempt, including for chain/mempool
                // joins whose individual resources can both be readable.
                let mut chain = self.0.indexer().gate().changes();
                let mempool = self.0.0.mempool.as_ref().map(Mempool::changes);
                let attempt = Arc::new(crate::read_attempt::ReadAttempt::default());
                let mut query = self.0.with_deadline(deadline);
                query.2 = Some(attempt.clone());
                let operation = f.clone();
                let result = spawn_blocking(move || {
                    let _permit = permit;
                    query.check_deadline()?;
                    operation(&query)
                })
                .await?;
                if !matches!(result, Err(Error::StateUpdating)) {
                    return result;
                }
                if let Some(mut changes) = attempt.take() {
                    changes
                        .changed()
                        .await
                        .map_err(|_| Error::Internal("publication source closed"))?;
                } else if let Some(mut mempool) = mempool {
                    tokio::select! {
                        result = chain.changed() => result.map_err(|_| Error::Internal("publication source closed"))?,
                        result = mempool.changed() => result.map_err(|_| Error::Internal("mempool source closed"))?,
                    }
                } else {
                    chain
                        .changed()
                        .await
                        .map_err(|_| Error::Internal("publication source closed"))?;
                }
            }
        };
        tokio::time::timeout_at(tokio::time::Instant::from_std(deadline), read)
            .await
            .unwrap_or(Err(Error::ReadTimeout))
    }

    pub fn with_deadline(&self, deadline: Instant) -> Self {
        Self(self.0.with_deadline(deadline))
    }
    pub fn build<P>(plugins: &P, mempool: Option<Mempool>) -> Self
    where
        P: ReadOnlyClone,
        P::ReadOnly: QueryPluginSet + 'static,
    {
        Self(Query::build(plugins, mempool))
    }

    /// Run one blocking operation without retrying it. Use this for actions
    /// and already-prepared immutable work; use `read` for publication-aware
    /// queries that can be retried after releasing guards and worker admission.
    pub async fn run<F, T>(&self, f: F) -> Result<T>
    where
        F: FnOnce(&Query) -> Result<T> + Send + 'static,
        T: Send + 'static,
    {
        let query = self.0.clone();
        spawn_blocking(move || {
            query.check_deadline()?;
            f(&query)
        })
        .await?
    }

    /// Run a cheap sync operation directly without spawn_blocking.
    /// Use this for simple accessors that don't do I/O.
    ///
    /// # Example
    /// ```ignore
    /// let height = query.sync(|q| q.height());
    /// ```
    pub fn sync<F, T>(&self, f: F) -> T
    where
        F: FnOnce(&Query) -> T,
    {
        f(&self.0)
    }

    #[inline]
    pub fn inner(&self) -> &Query {
        &self.0
    }
}
