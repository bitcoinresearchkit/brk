use std::{net::SocketAddr, str::from_utf8};

use bitview_query::AsyncQuery;
use brk_types::{Txid, Vout};
use serde_json::to_vec;
use tokio::task::JoinSet;

use super::server_routes::{exchange_bytes, exchange_with_etag};

const SUFFIXES: [&str; 10] = [
    "",
    "/status",
    "/hex",
    "/raw",
    "/outspend/0",
    "/outspends",
    "/cpfp",
    "/rbf",
    "/replacements",
    "/fullrbf/replacements",
];

pub struct TransactionPublication {
    txid: Txid,
    tags: Vec<String>,
}

impl TransactionPublication {
    pub fn new(txid: Txid) -> Self {
        Self {
            txid,
            tags: Vec::new(),
        }
    }

    fn path(&self, suffix: &str) -> String {
        if suffix == "/cpfp" {
            format!("/api/v1/cpfp/{}", self.txid)
        } else if suffix == "/rbf" {
            format!("/api/v1/tx/{}/rbf", self.txid)
        } else if suffix.ends_with("/replacements") {
            format!("/api/v1{suffix}")
        } else {
            format!("/api/tx/{}{suffix}", self.txid)
        }
    }

    pub async fn check_unavailable(&self, address: SocketAddr) {
        let mut requests = JoinSet::new();
        for (index, suffix) in SUFFIXES.iter().enumerate() {
            let path = self.path(suffix);
            for method in ["GET", "HEAD"] {
                for tag in [
                    "*",
                    self.tags
                        .get(index)
                        .map(String::as_str)
                        .unwrap_or("\"old\""),
                ] {
                    let path = path.clone();
                    let tag = tag.to_owned();
                    requests.spawn(async move {
                        let response = exchange_with_etag(address, method, &path, &tag).await;
                        assert!(response.starts_with("HTTP/1.1 503"), "{path}: {response}");
                        assert!(!response.contains("\r\netag:"));
                        assert!(response.contains("\r\ncache-control: no-store\r\n"));
                    });
                }
            }
        }
        while let Some(result) = requests.join_next().await {
            result.unwrap();
        }
    }

    pub async fn check_available(&mut self, query: &AsyncQuery, address: SocketAddr) {
        self.tags.clear();
        for suffix in SUFFIXES {
            let expected = query.sync(|q| match suffix {
                "" => to_vec(&q.transaction(&self.txid).unwrap()).unwrap(),
                "/status" => to_vec(&q.transaction_status(&self.txid).unwrap()).unwrap(),
                "/hex" => q.transaction_hex(&self.txid).unwrap().into_bytes(),
                "/raw" => q.transaction_raw(&self.txid).unwrap(),
                "/outspend/0" => to_vec(&q.outspend(&self.txid, Vout::ZERO).unwrap()).unwrap(),
                "/outspends" => to_vec(&q.outspends(&self.txid).unwrap()).unwrap(),
                "/cpfp" => to_vec(&q.cpfp(&self.txid).unwrap()).unwrap(),
                "/rbf" => to_vec(&q.tx_rbf(&self.txid).unwrap()).unwrap(),
                "/replacements" | "/fullrbf/replacements" => to_vec(
                    &q.recent_replacements(suffix == "/fullrbf/replacements")
                        .unwrap(),
                )
                .unwrap(),
                _ => unreachable!(),
            });
            let path = self.path(suffix);
            let response = exchange_bytes(address, "GET", &path, "\"old\"", 4_100_000).await;
            let header_end = response
                .windows(4)
                .position(|window| window == b"\r\n\r\n")
                .unwrap()
                + 4;
            let headers = from_utf8(&response[..header_end]).unwrap();
            assert!(headers.starts_with("HTTP/1.1 200"), "{path}: {headers}");
            assert_eq!(&response[header_end..], expected);
            let tag = headers
                .lines()
                .find_map(|line| line.strip_prefix("etag: "))
                .unwrap()
                .to_owned();
            for method in ["GET", "HEAD"] {
                for validator in [tag.as_str(), "*"] {
                    let response = exchange_with_etag(address, method, &path, validator).await;
                    assert!(response.starts_with("HTTP/1.1 304"), "{path}: {response}");
                    assert!(response.ends_with("\r\n\r\n"));
                }
            }
            self.tags.push(tag);
        }
    }
}
