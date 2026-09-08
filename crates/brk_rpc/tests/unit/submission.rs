use std::{fs, sync::Arc, time::Duration};

use brk_error::Error;
use serde_json::{Value, json};
use tokio::{
    io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader},
    net::{TcpListener, TcpStream},
    task::spawn_blocking,
    time::timeout,
};

use crate::{Auth, Client};

use super::Submission;

async fn request(socket: &mut BufReader<TcpStream>) -> (Value, String) {
    let mut line = String::new();
    let mut length = 0;
    let mut auth = String::new();
    loop {
        line.clear();
        assert_ne!(socket.read_line(&mut line).await.unwrap(), 0);
        if line == "\r\n" {
            break;
        }
        let lower = line.to_ascii_lowercase();
        if let Some(value) = lower.strip_prefix("content-length: ") {
            length = value.trim().parse::<usize>().unwrap();
        }
        if lower.starts_with("authorization: ") {
            auth = line[15..].trim().to_owned();
        }
    }
    assert!(length < 1024);
    let mut bytes = vec![0; length];
    socket.read_exact(&mut bytes).await.unwrap();
    let body: Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(body["method"], "sendrawtransaction");
    (body, auth)
}

async fn reply(socket: &mut BufReader<TcpStream>, status: u16, body: &str) {
    socket
        .get_mut()
        .write_all(
            format!(
                "HTTP/1.1 {status} Response\r\nContent-Length: {}\r\n\r\n{body}",
                body.len()
            )
            .as_bytes(),
        )
        .await
        .unwrap();
}

#[tokio::test]
async fn public_submissions_reuse_connection_and_preserve_core_rejections() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let client = Client::new(
        &format!("http://{}", listener.local_addr().unwrap()),
        Auth::UserPass("u".into(), "p".into()),
    )
    .unwrap();
    let worker = spawn_blocking(move || {
        assert_eq!(
            client.send_raw_transaction("aa").unwrap().to_string(),
            "a".repeat(64)
        );
        for _ in 0..4 {
            assert!(matches!(
                client.send_raw_transaction("aa"),
                Err(Error::Parse(_))
            ));
        }
        assert!(matches!(
            client.send_raw_transaction("aa"),
            Err(Error::CorepcRPC(_))
        ));
    });
    timeout(Duration::from_secs(5), async {
        let mut socket = BufReader::new(listener.accept().await.unwrap().0);
        for code in [0, -22, -25, -26, -27, -28] {
            let (body, auth) = request(&mut socket).await;
            assert_eq!(auth, "Basic dTpw");
            assert_eq!(body["params"], json!(["aa"]));
            let body = if code == 0 {
                json!({"id":1,"result":"a".repeat(64)})
            } else {
                json!({"id":1,"error":{"code":code,"message":"rejected"}})
            };
            reply(
                &mut socket,
                if code == 0 { 200 } else { 500 },
                &body.to_string(),
            )
            .await;
        }
        worker.await.unwrap();
    })
    .await
    .unwrap();
}

#[tokio::test]
async fn fresh_and_reused_submissions_never_replay_response_loss() {
    for reused in [false, true] {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let client = Client::new(
            &format!("http://{}", listener.local_addr().unwrap()),
            Auth::None,
        )
        .unwrap();
        let worker = spawn_blocking(move || {
            if reused {
                client.send_raw_transaction("aa").unwrap();
            }
            assert!(client.send_raw_transaction("bb").is_err());
        });
        timeout(Duration::from_secs(5), async {
            let mut socket = BufReader::new(listener.accept().await.unwrap().0);
            if reused {
                assert_eq!(request(&mut socket).await.0["params"], json!(["aa"]));
                reply(
                    &mut socket,
                    200,
                    &json!({"id":1,"result":"a".repeat(64)}).to_string(),
                )
                .await;
            }
            assert_eq!(request(&mut socket).await.0["params"], json!(["bb"]));
            drop(socket);
            worker.await.unwrap();
            assert!(
                timeout(Duration::from_millis(50), listener.accept())
                    .await
                    .is_err()
            );
        })
        .await
        .unwrap();
    }
}

#[tokio::test]
async fn redirects_and_invalid_or_oversized_results_do_not_replay() {
    for case in 0..4 {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let url = format!("http://{}", listener.local_addr().unwrap());
        let client = Client::new(&url, Auth::None).unwrap();
        let worker = spawn_blocking(move || assert!(client.send_raw_transaction("aa").is_err()));
        timeout(Duration::from_secs(5), async {
            let mut socket = BufReader::new(listener.accept().await.unwrap().0);
            request(&mut socket).await;
            if case < 2 {
                socket.get_mut().write_all(format!("HTTP/1.1 {} Redirect\r\nLocation: {url}/elsewhere\r\nContent-Length: 0\r\n\r\n", if case == 0 { 307 } else { 308 }).as_bytes()).await.unwrap();
            } else {
                let body = if case == 2 { json!({"id":2,"result":"a".repeat(64)}).to_string() }
                    else { " ".repeat(crate::rpc_response::MAX_RESPONSE_BYTES + 1) };
                reply(&mut socket, 200, &body).await;
            }
            worker.await.unwrap();
            assert!(timeout(Duration::from_millis(50), listener.accept()).await.is_err());
            // A redirect replay on the same connection would also be wrong.
            match timeout(Duration::from_millis(30), socket.read(&mut [0])).await {
                Err(_) | Ok(Ok(0)) => {}
                other => panic!("unexpected bytes after the completed submission: {other:?}"),
            }
        }).await.unwrap();
    }
}

#[tokio::test]
async fn queued_timeout_does_not_dispatch_later() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let url = format!("http://{}", listener.local_addr().unwrap());
    let submission = Arc::new(Submission::new());
    let queued = spawn_blocking({
        let submission = submission.clone();
        let url = url.clone();
        move || {
            let _held = submission.agent.lock();
            submission.send_with_timeout(&url, &Auth::None, "aa", Duration::from_millis(30))
        }
    });
    assert!(matches!(
        queued.await.unwrap(),
        Err(Error::Internal("node submission queue timed out"))
    ));
    let next = spawn_blocking(move || {
        submission.send_with_timeout(&url, &Auth::None, "bb", Duration::from_secs(2))
    });
    timeout(Duration::from_secs(5), async {
        let mut socket = BufReader::new(listener.accept().await.unwrap().0);
        assert_eq!(request(&mut socket).await.0["params"], json!(["bb"]));
        reply(
            &mut socket,
            200,
            &json!({"id":1,"result":"b".repeat(64)}).to_string(),
        )
        .await;
        assert!(next.await.unwrap().is_ok());
    })
    .await
    .unwrap();
}

#[tokio::test]
async fn in_flight_timeout_closes_socket_and_releases_admission() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let url = format!("http://{}", listener.local_addr().unwrap());
    let submission = Arc::new(Submission::new());
    let stalled = spawn_blocking({
        let submission = submission.clone();
        let url = url.clone();
        move || submission.send_with_timeout(&url, &Auth::None, "aa", Duration::from_millis(100))
    });
    timeout(Duration::from_secs(5), async {
        let mut socket = BufReader::new(listener.accept().await.unwrap().0);
        request(&mut socket).await;
        assert_eq!(socket.read(&mut [0]).await.unwrap(), 0);
        assert!(stalled.await.unwrap().is_err());
        let next = spawn_blocking(move || {
            submission.send_with_timeout(&url, &Auth::None, "bb", Duration::from_secs(2))
        });
        let mut socket = BufReader::new(listener.accept().await.unwrap().0);
        assert_eq!(request(&mut socket).await.0["params"], json!(["bb"]));
        reply(
            &mut socket,
            200,
            &json!({"id":1,"result":"b".repeat(64)}).to_string(),
        )
        .await;
        assert!(next.await.unwrap().is_ok());
    })
    .await
    .unwrap();
}

#[tokio::test]
async fn cookie_refresh_requires_explicit_rejection_and_stops_after_one_refresh() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join(".cookie");
    fs::write(&path, "u:p\n").unwrap();
    let client = Client::new(
        &format!("http://{}", listener.local_addr().unwrap()),
        Auth::CookieFile(path.clone()),
    )
    .unwrap();
    let worker = spawn_blocking(move || {
        assert!(client.send_raw_transaction("aa").is_ok());
        assert!(client.send_raw_transaction("bb").is_err());
    });
    timeout(Duration::from_secs(5), async {
        let mut socket = BufReader::new(listener.accept().await.unwrap().0);
        for (expected, updated, accepted) in [
            ("Basic dTpw", "u:q\n", false),
            ("Basic dTpx", "u:q\n", true),
            ("Basic dTpx", "u:r\n", false),
            ("Basic dTpy", "u:s\n", false),
        ] {
            assert_eq!(request(&mut socket).await.1, expected);
            fs::write(&path, updated).unwrap();
            if accepted {
                reply(
                    &mut socket,
                    200,
                    &json!({"id":1,"result":"a".repeat(64)}).to_string(),
                )
                .await;
            } else {
                reply(&mut socket, 401, "").await;
            }
        }
        worker.await.unwrap();
    })
    .await
    .unwrap();
}
