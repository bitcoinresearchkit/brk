use std::{
    fs,
    time::{Duration, Instant},
};

use tempfile::tempdir;
use tokio::{
    io::{AsyncWriteExt, BufReader},
    net::TcpListener,
    task::spawn_blocking,
};

use super::request;
use crate::{AsyncClient, Auth, Client};

/// Opt-in loopback comparison: no live node, connection setup excluded.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn warm_height_latency() {
    const WARMUP: usize = 100;
    const SAMPLES: usize = 2000;
    let directory = tempdir().unwrap();
    let cookie = directory.path().join(".cookie");
    fs::write(&cookie, "u:p").unwrap();
    for auth in [
        Auth::UserPass("u".into(), "p".into()),
        Auth::CookieFile(cookie),
    ] {
        for asynchronous in [false, true] {
            let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
            let url = format!("http://{}", listener.local_addr().unwrap());
            let server = tokio::spawn(async move {
                let mut socket = BufReader::new(listener.accept().await.unwrap().0);
                for _ in 0..WARMUP + SAMPLES {
                    let (id, auth) = request(&mut socket).await;
                    assert_eq!(auth, "Basic dTpw");
                    let body = format!("{{\"id\":{id},\"result\":900000}}");
                    let response = format!(
                        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{body}",
                        body.len()
                    );
                    socket
                        .get_mut()
                        .write_all(response.as_bytes())
                        .await
                        .unwrap();
                }
            });
            let sync = Client::new_with(&url, auth.clone(), 0, Duration::ZERO).unwrap();
            let asynchronous_client = AsyncClient::new(&url, auth.clone()).unwrap();
            let mut times = Vec::with_capacity(SAMPLES);
            for i in 0..WARMUP + SAMPLES {
                let start = Instant::now();
                let height = if asynchronous {
                    asynchronous_client.get_last_height().await.unwrap()
                } else {
                    let sync = sync.clone();
                    spawn_blocking(move || sync.get_last_height())
                        .await
                        .unwrap()
                        .unwrap()
                };
                assert_eq!(*height, 900000);
                if i >= WARMUP {
                    times.push(start.elapsed().as_nanos());
                }
            }
            server.await.unwrap();
            times.sort_unstable();
            eprintln!(
                "async={asynchronous} cookie={} n={SAMPLES} p50={}ns p95={}ns p99={}ns",
                matches!(auth, Auth::CookieFile(_)),
                times[SAMPLES / 2],
                times[SAMPLES * 95 / 100],
                times[SAMPLES * 99 / 100]
            );
        }
    }
}
