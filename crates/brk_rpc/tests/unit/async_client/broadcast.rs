use brk_error::Error;
use serde_json::json;
use tokio::spawn;

use super::*;

async fn reply(socket: &mut BufReader<TcpStream>, body: Value) {
    let body = body.to_string();
    socket
        .get_mut()
        .write_all(
            format!(
                "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{body}",
                body.len()
            )
            .as_bytes(),
        )
        .await
        .unwrap();
}

#[tokio::test]
async fn submissions_reuse_connections_and_preserve_rejections() {
    let (client, listener) = client().await;
    let server = spawn(async move {
        let mut socket = BufReader::new(listener.accept().await.unwrap().0);
        for code in [0, -22, -25, -26, -27, -28] {
            let (body, _) = message(&mut socket).await;
            assert_eq!(body["method"], "sendrawtransaction");
            assert_eq!(body["params"], json!(["AA00"]));
            let response = if code == 0 {
                json!({"id": 1, "result": "a".repeat(64)})
            } else {
                json!({"id": 1, "error": {"code": code, "message": "rejected"}})
            };
            reply(&mut socket, response).await;
        }
    });
    timeout(Duration::from_secs(5), async {
        assert_eq!(
            client
                .send_raw_transaction("AA00")
                .await
                .unwrap()
                .to_string(),
            "a".repeat(64)
        );
        for _ in 0..4 {
            assert!(matches!(
                client.send_raw_transaction("AA00").await,
                Err(Error::Parse(_))
            ));
        }
        assert!(matches!(
            client.send_raw_transaction("AA00").await,
            Err(Error::CorepcRPC(_))
        ));
        server.await.unwrap();
    })
    .await
    .unwrap();
}

#[tokio::test]
async fn submissions_never_replay_after_response_loss() {
    for reused in [false, true] {
        let (client, listener) = client().await;
        let server = spawn(async move {
            let mut socket = BufReader::new(listener.accept().await.unwrap().0);
            if reused {
                request(&mut socket).await;
                reply(&mut socket, json!({"id": 1, "result": 0})).await;
            }
            let (body, _) = message(&mut socket).await;
            assert_eq!(body["method"], "sendrawtransaction");
            drop(socket); // The node received the action, but its outcome is lost.
            assert!(
                timeout(Duration::from_millis(100), listener.accept())
                    .await
                    .is_err()
            );
        });
        timeout(Duration::from_secs(5), async {
            if reused {
                client.get_last_height().await.unwrap();
            }
            assert!(client.send_raw_transaction("aa").await.is_err());
            server.await.unwrap();
        })
        .await
        .unwrap();
    }
}

#[tokio::test]
async fn cancelled_waiter_cannot_submit_after_the_connection_is_released() {
    let (client, listener) = client().await;
    let (entered, observed) = oneshot::channel();
    let (release, released) = oneshot::channel();
    let server = spawn(async move {
        let mut socket = BufReader::new(listener.accept().await.unwrap().0);
        request(&mut socket).await;
        entered.send(()).unwrap();
        released.await.unwrap();
        reply(&mut socket, json!({"id": 1, "result": 0})).await;
        let (body, _) = message(&mut socket).await;
        assert_eq!(
            body["params"],
            json!(["bb"]),
            "cancelled aa must never dispatch"
        );
        reply(&mut socket, json!({"id": 1, "result": "b".repeat(64)})).await;
    });
    let active = spawn({
        let client = client.clone();
        async move { client.get_last_height().await }
    });
    timeout(Duration::from_secs(5), async {
        observed.await.unwrap();
        assert!(
            timeout(Duration::from_millis(30), client.send_raw_transaction("aa"))
                .await
                .is_err()
        );
        release.send(()).unwrap();
        active.await.unwrap().unwrap();
        assert_eq!(
            client.send_raw_transaction("bb").await.unwrap().to_string(),
            "b".repeat(64)
        );
        server.await.unwrap();
    })
    .await
    .unwrap();
}

#[tokio::test]
async fn cancellation_after_dispatch_closes_the_connection() {
    let (client, listener) = client().await;
    let server = spawn(async move {
        let mut socket = BufReader::new(listener.accept().await.unwrap().0);
        let (body, _) = message(&mut socket).await;
        assert_eq!(body["method"], "sendrawtransaction");
        assert_eq!(socket.read(&mut [0]).await.unwrap(), 0);
    });
    assert!(
        timeout(
            Duration::from_millis(100),
            client.send_raw_transaction("aa")
        )
        .await
        .is_err()
    );
    timeout(Duration::from_secs(2), server)
        .await
        .unwrap()
        .unwrap();
}
