use std::{
    io::{BufRead, BufReader, Write},
    net::TcpListener,
    thread,
    time::Duration,
};

use bitview_cohort::ByEpoch;
use brk_types::Sats;
use serde_json::{json, to_value};

#[test]
fn generated_storage_path_fetches_a_structured_row() {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let client =
        bitview_client::BitviewClient::new(format!("http://{}", listener.local_addr().unwrap()));
    let row = to_value(ByEpoch::<Sats>::default()).unwrap();
    let expected = row.clone();
    let peer = thread::spawn(move || {
        let socket = listener.accept().unwrap().0;
        socket
            .set_read_timeout(Some(Duration::from_secs(5)))
            .unwrap();
        let mut socket = BufReader::new(socket);
        let mut line = String::new();
        socket.read_line(&mut line).unwrap();
        assert_eq!(
            line,
            "GET /api/series/supply_sats_by_epoch/height HTTP/1.1\r\n"
        );
        loop {
            line.clear();
            assert_ne!(socket.read_line(&mut line).unwrap(), 0);
            if line == "\r\n" {
                break;
            }
        }
        let body = json!({
            "version": 1, "index": brk_types::Index::Height, "type": "ByEpoch<Sats>",
            "start": 0, "end": 1, "stamp": "2026-09-07T00:00:00Z", "data": [row]
        })
        .to_string();
        socket.get_mut().write_all(format!("HTTP/1.1 200 OK\r\nConnection: close\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{body}", body.len()).as_bytes()).unwrap();
    });
    let endpoint = &client.series().cohorts.supply.total.stored.epoch.height;
    assert_eq!(endpoint.name(), "supply_sats_by_epoch");
    let response: bitview_client::SeriesData<ByEpoch<Sats>> = endpoint.by.height().fetch().unwrap();
    assert_eq!(response.data.len(), 1);
    assert_eq!(response.version, 1_u32);
    assert_eq!(to_value(&response.data[0]).unwrap(), expected);
    peer.join().unwrap();
}
