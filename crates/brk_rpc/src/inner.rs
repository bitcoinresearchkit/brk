use std::{thread::sleep, time::Duration};

use bitcoincore_rpc::{Client as CoreClient, Error as RpcError, jsonrpc};
use brk_error::Result;
use log::info;
use parking_lot::RwLock;

pub use bitcoincore_rpc::Auth;

#[derive(Debug)]
pub struct ClientInner {
    url: String,
    auth: Auth,
    client: RwLock<CoreClient>,
    max_retries: usize,
    retry_delay: Duration,
}

impl ClientInner {
    pub fn new(url: &str, auth: Auth, max_retries: usize, retry_delay: Duration) -> Result<Self> {
        let client = Self::retry(max_retries, retry_delay, || {
            CoreClient::new(url, auth.clone()).map_err(Into::into)
        })?;

        Ok(Self {
            url: url.to_string(),
            auth,
            client: RwLock::new(client),
            max_retries,
            retry_delay,
        })
    }

    fn recreate(&self) -> Result<()> {
        *self.client.write() = CoreClient::new(&self.url, self.auth.clone())?;
        Ok(())
    }

    fn is_retriable(error: &RpcError) -> bool {
        matches!(
            error,
            RpcError::JsonRpc(jsonrpc::Error::Rpc(e))
                if e.code == -32600 || e.code == 401 || e.code == -28
        ) || matches!(error, RpcError::JsonRpc(jsonrpc::Error::Transport(_)))
    }

    fn retry<F, T>(max_retries: usize, delay: Duration, mut f: F) -> Result<T>
    where
        F: FnMut() -> Result<T>,
    {
        let mut last_error = None;

        for attempt in 0..=max_retries {
            if attempt > 0 {
                info!(
                    "Retrying to connect to Bitcoin Core (attempt {}/{})",
                    attempt, max_retries
                );
                sleep(delay);
            }

            match f() {
                Ok(value) => {
                    if attempt > 0 {
                        info!(
                            "Successfully connected to Bitcoin Core after {} retries",
                            attempt
                        );
                    }
                    return Ok(value);
                }
                Err(e) => {
                    if attempt == 0 {
                        info!("Could not connect to Bitcoin Core, retrying: {}", e);
                    }
                    last_error = Some(e);
                }
            }
        }

        let err = last_error.unwrap();
        info!(
            "Failed to connect to Bitcoin Core after {} attempts",
            max_retries + 1
        );
        Err(err)
    }

    pub fn call_with_retry<F, T>(&self, f: F) -> Result<T, RpcError>
    where
        F: Fn(&CoreClient) -> Result<T, RpcError>,
    {
        for attempt in 0..=self.max_retries {
            if attempt > 0 {
                info!(
                    "Trying to reconnect to Bitcoin Core (attempt {}/{})",
                    attempt, self.max_retries
                );
                self.recreate().ok();
                sleep(self.retry_delay);
            }

            match f(&self.client.read()) {
                Ok(value) => {
                    if attempt > 0 {
                        info!(
                            "Successfully reconnected to Bitcoin Core after {} attempts",
                            attempt
                        );
                    }
                    return Ok(value);
                }
                Err(e) if Self::is_retriable(&e) => {
                    if attempt == 0 {
                        info!("Lost connection to Bitcoin Core, reconnecting...");
                    }
                }
                Err(e) => return Err(e),
            }
        }

        info!(
            "Could not reconnect to Bitcoin Core after {} attempts",
            self.max_retries + 1
        );
        Err(RpcError::JsonRpc(jsonrpc::Error::Rpc(
            jsonrpc::error::RpcError {
                code: -1,
                message: "Max retries exceeded".to_string(),
                data: None,
            },
        )))
    }

    pub fn call_once<F, T>(&self, f: F) -> Result<T, RpcError>
    where
        F: Fn(&CoreClient) -> Result<T, RpcError>,
    {
        f(&self.client.read())
    }
}
