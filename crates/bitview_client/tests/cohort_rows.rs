use bitview_cohort::*;
use brk_types::Sats;
use serde::{Serialize, de::DeserializeOwned};
use std::{
    io::{BufRead, BufReader, Write},
    net::TcpListener,
    thread,
    time::Duration,
};

fn round_trip<T: Default + Serialize + DeserializeOwned>() {
    let encoded = serde_json::to_value(T::default()).unwrap();
    let decoded: T = serde_json::from_value(encoded.clone()).unwrap();
    assert_eq!(serde_json::to_value(decoded).unwrap(), encoded);
}

#[test]
fn structured_series_rows_deserialize_without_erasing_their_wrapper() {
    round_trip::<ByTerm<Sats>>();
    round_trip::<Class<Sats>>();
    round_trip::<SpendableType<Sats>>();
    round_trip::<ByAge<Sats>>();
    round_trip::<ByEpoch<Sats>>();
    round_trip::<ByEntry<Sats>>();
    round_trip::<AgeRange<Sats>>();
    round_trip::<AmountRange<Sats>>();
    round_trip::<ProfitabilityRange<Sats>>();
    round_trip::<OverAge<Sats>>();
    round_trip::<UnderAge<Sats>>();
    round_trip::<OverAmount<Sats>>();
    round_trip::<UnderAmount<Sats>>();
    round_trip::<UTXOAggregate<Sats>>();
    round_trip::<UTXOAllAndSth<Sats>>();

    let row: ByTerm<Sats> = serde_json::from_str(r#"{"short":42,"long":7}"#).unwrap();
    assert_eq!(row.short, Sats::from(42_u64));
    assert_eq!(row.long, Sats::from(7_u64));

    let row = ProfitabilityRow {
        range: ProfitabilityRange::<Sats>::default(),
        profit: Profit::default(),
        loss: Loss::default(),
    };
    let encoded = serde_json::to_value(row).unwrap();
    let decoded: ProfitabilityRow<Sats> = serde_json::from_value(encoded.clone()).unwrap();
    assert_eq!(serde_json::to_value(decoded).unwrap(), encoded);
}

#[test]
fn generated_matrix_path_fetches_a_structured_row() {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let client =
        bitview_client::BitviewClient::new(format!("http://{}", listener.local_addr().unwrap()));
    let row = serde_json::to_value(ByEpoch::<Sats>::default()).unwrap();
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
        let body = serde_json::json!({
            "version": 1, "index": brk_types::Index::Height, "type": "ByEpoch<Sats>",
            "start": 0, "end": 1, "stamp": "2026-09-07T00:00:00Z", "data": [row]
        })
        .to_string();
        socket.get_mut().write_all(format!("HTTP/1.1 200 OK\r\nConnection: close\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{body}", body.len()).as_bytes()).unwrap();
    });
    let endpoint = &client.series().cohorts.supply.total.epoch_matrix;
    assert_eq!(endpoint.name(), "supply_sats_by_epoch");
    let response: bitview_client::SeriesData<ByEpoch<Sats>> = endpoint.by.height().fetch().unwrap();
    assert_eq!(response.data.len(), 1);
    assert_eq!(response.version, 1_u32);
    assert_eq!(serde_json::to_value(&response.data[0]).unwrap(), expected);
    peer.join().unwrap();
}
