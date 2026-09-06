//! Synchronous action transport. Read RPC recovery remains independent.

use std::{
    fs,
    time::{Duration, Instant},
};

use base64::{Engine, engine::general_purpose::STANDARD};
use brk_error::{Error, Result};
use brk_types::Txid;
use parking_lot::Mutex;

use crate::{Auth, rpc_response};

#[cfg(test)]
#[path = "../../tests/unit/submission.rs"]
mod tests;

#[derive(Debug)]
pub struct Submission {
    // Serialize actions and reuse a connection without the read transport's
    // implicit resend. The mutex wait is included in the operation deadline.
    agent: Mutex<ureq::Agent>,
}

impl Submission {
    pub fn new() -> Self {
        let agent = ureq::Agent::config_builder()
            .http_status_as_error(false)
            .max_redirects(0)
            .proxy(None)
            .max_idle_connections(1)
            .max_idle_connections_per_host(1)
            .build()
            .into();
        Self {
            agent: Mutex::new(agent),
        }
    }

    pub fn send(&self, url: &str, auth: &Auth, hex: &str) -> Result<Txid> {
        self.send_with_timeout(url, auth, hex, Duration::from_secs(60))
    }

    fn send_with_timeout(
        &self,
        url: &str,
        auth: &Auth,
        hex: &str,
        timeout: Duration,
    ) -> Result<Txid> {
        let deadline = Instant::now() + timeout;
        let agent = self
            .agent
            .try_lock_until(deadline)
            .ok_or(Error::Internal("node submission queue timed out"))?;
        let body = serde_json::to_vec(&serde_json::json!({
            "jsonrpc": "2.0", "id": 1, "method": "sendrawtransaction", "params": [hex],
        }))?;
        let mut authorization = authorization(auth)?;
        let mut refresh = true;
        loop {
            let remaining = deadline.saturating_duration_since(Instant::now());
            if remaining.is_zero() {
                return Err(Error::Internal("node submission timed out"));
            }
            let mut request = agent
                .post(url)
                .config()
                .timeout_global(Some(remaining))
                .build()
                .header("content-type", "application/json");
            if let Some(header) = &authorization {
                request = request.header("authorization", header);
            }
            let mut response = request.send(body.as_slice())?;
            let status = response.status();
            let bytes = response
                .body_mut()
                .with_config()
                .limit(rpc_response::MAX_RESPONSE_BYTES as u64)
                .read_to_vec()?;
            if status == ureq::http::StatusCode::UNAUTHORIZED {
                if refresh && matches!(auth, Auth::CookieFile(_)) {
                    let updated = self::authorization(auth)?;
                    if updated != authorization {
                        authorization = updated;
                        refresh = false;
                        continue;
                    }
                }
                return Err(Error::Internal("node RPC authentication failed"));
            }
            return rpc_response::decode::<bitcoin::Txid>(status.is_success(), &bytes)
                .map(Txid::from)
                .map_err(super::transaction_error);
        }
    }
}

fn authorization(auth: &Auth) -> Result<Option<String>> {
    let credentials = match auth {
        Auth::None => return Ok(None),
        Auth::UserPass(user, password) => format!("{user}:{password}"),
        Auth::CookieFile(path) => fs::read_to_string(path)?.trim().to_owned(),
    };
    Ok(Some(format!("Basic {}", STANDARD.encode(credentials))))
}
