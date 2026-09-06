use std::{
    io::{BufRead, BufReader, Read, Write},
    net::TcpListener,
    thread,
    time::Duration,
};

use bitview_client::BitviewClient;

#[test]
fn broadcast_decodes_plain_text_and_does_not_replay_lost_outcomes() {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let client = BitviewClient::new(format!("http://{}", listener.local_addr().unwrap()));
    let peer = thread::spawn(move || {
        for (expected, response) in [
            ("aa", Some("a".repeat(64))),
            ("bb", Some("invalid".into())),
            ("cc", None),
        ] {
            let socket = listener.accept().unwrap().0;
            socket
                .set_read_timeout(Some(Duration::from_secs(5)))
                .unwrap();
            let mut socket = BufReader::new(socket);
            let mut line = String::new();
            socket.read_line(&mut line).unwrap();
            assert_eq!(line, "POST /api/tx HTTP/1.1\r\n");
            let mut length = 0;
            loop {
                line.clear();
                assert_ne!(socket.read_line(&mut line).unwrap(), 0);
                if line == "\r\n" {
                    break;
                }
                if let Some(value) = line.to_ascii_lowercase().strip_prefix("content-length: ") {
                    length = value.trim().parse::<usize>().unwrap();
                }
            }
            assert_eq!(length, 2);
            let mut body = [0; 2];
            socket.read_exact(&mut body).unwrap();
            assert_eq!(&body, expected.as_bytes());
            if let Some(body) = response {
                socket.get_mut().write_all(format!("HTTP/1.1 200 OK\r\nConnection: close\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{body}", body.len()).as_bytes()).unwrap();
            }
        }
        listener.set_nonblocking(true).unwrap();
        // Retain the listener until the client has returned: an extra connection
        // is visible even when no server accepts it.
        listener
    });
    assert_eq!(client.post_tx("aa").unwrap().to_string(), "a".repeat(64));
    assert!(
        client
            .post_tx("bb")
            .unwrap_err()
            .to_string()
            .contains("outcome may be unknown")
    );
    assert!(client.post_tx("cc").is_err());
    assert_eq!(
        peer.join().unwrap().accept().unwrap_err().kind(),
        std::io::ErrorKind::WouldBlock
    );
}
