use std::{
    io::{Read, Write},
    net::TcpListener,
    process::Command,
    thread,
};

#[test]
fn pretty_only_changes_valid_json_and_preserves_other_bodies() {
    for (pretty, body, expected) in [
        (false, r#"{"value":1}"#, r#"{"value":1}"#),
        (true, r#"{"value":1}"#, "{\n  \"value\": 1\n}\n"),
        (true, "value\n1\n", "value\n1\n"),
        (false, "value\n1\n", "value\n1\n"),
    ] {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap();
        let server = thread::spawn(move || {
            let (mut stream, _) = listener.accept().unwrap();
            let mut request = Vec::new();
            while !request.ends_with(b"\r\n\r\n") {
                let mut byte = [0];
                stream.read_exact(&mut byte).unwrap();
                request.push(byte[0]);
            }
            write!(
                stream,
                "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
                body.len()
            )
            .unwrap();
        });
        let mut command = Command::new(env!("CARGO_BIN_EXE_bitview-cli"));
        command.args([
            "--url",
            &format!("http://{address}"),
            "get-series",
            "example",
            "height",
        ]);
        if pretty {
            command.arg("--pretty");
        }
        let output = command.output().unwrap();
        server.join().unwrap();
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        assert_eq!(output.stdout, expected.as_bytes());
    }
}
