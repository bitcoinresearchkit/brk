use std::{hint::black_box, time::Instant};

use super::*;

#[test]
fn attachment_names_cannot_change_header_syntax() {
    for (name, expected) in [
        (
            "price_close-Day1.csv",
            "attachment; filename=\"price_close-Day1.csv\"",
        ),
        ("a b.csv", "attachment; filename=\"a b.csv\""),
        ("a\"b", "attachment; filename*=UTF-8''%61%22%62"),
        ("a\\b", "attachment; filename*=UTF-8''%61%5C%62"),
        ("a\r\n\0", "attachment; filename*=UTF-8''%61%0D%0A%00"),
        ("é", "attachment; filename*=UTF-8''%C3%A9"),
    ] {
        let mut headers = HeaderMap::new();
        headers.insert_content_disposition_attachment(name);
        assert_eq!(headers[header::CONTENT_DISPOSITION], expected);
        assert_eq!(headers.len(), 1);
    }
}

#[test]
#[ignore = "attachment header construction comparison; no HTTP or query work"]
fn benchmark_attachment_header() {
    for name in [
        "price_close-Day1.csv".to_owned(),
        "price_close_".repeat(32) + "Day1.csv",
    ] {
        let mut times = [Vec::new(), Vec::new()];
        for round in 0..24 {
            for variant in [round % 2, 1 - round % 2] {
                let started = Instant::now();
                for _ in 0..10_000 {
                    let mut headers = HeaderMap::new();
                    if variant == 0 {
                        headers.insert(
                            header::CONTENT_DISPOSITION,
                            format!("attachment; filename=\"{}\"", black_box(&name))
                                .parse()
                                .unwrap(),
                        );
                    } else {
                        headers.insert_content_disposition_attachment(black_box(&name));
                    }
                    black_box(headers);
                }
                if round >= 4 {
                    times[variant].push(started.elapsed() / 10_000);
                }
            }
        }
        for samples in &mut times {
            samples.sort_unstable();
        }
        eprintln!(
            "attachment bytes={}: previous {:?}, encoded {:?}",
            name.len(),
            times[0][10],
            times[1][10]
        );
    }
}
