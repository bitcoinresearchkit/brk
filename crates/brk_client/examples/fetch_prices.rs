use std::fs::File;
use std::io::{BufWriter, Write};

use brk_client::{BrkClient, BrkClientOptions, Result};
use brk_types::Dollars;

const CHUNK_SIZE: usize = 10_000;
const END_HEIGHT: usize = 630_000;
const OUTPUT_FILE: &str = "prices_avg.txt";

fn main() -> Result<()> {
    let client = BrkClient::with_options(BrkClientOptions {
        base_url: "https://next.bitview.space".to_string(),
        timeout_secs: 60,
    });

    let file = File::create(OUTPUT_FILE).map_err(|e| brk_client::BrkError {
        message: e.to_string(),
    })?;
    let mut writer = BufWriter::new(file);

    for start in (0..END_HEIGHT).step_by(CHUNK_SIZE) {
        let end = (start + CHUNK_SIZE).min(END_HEIGHT);
        eprintln!("Fetching {start} to {end}...");

        let ohlcs = client
            .metrics()
            .price
            .cents
            .ohlc
            .by
            .height()
            .range(start..end)
            .fetch()?;

        for ohlc in ohlcs.data {
            let avg = (u64::from(*ohlc.open) + u64::from(*ohlc.close)) / 2;
            let avg = Dollars::from(avg);
            writeln!(writer, "{avg}").map_err(|e| brk_client::BrkError {
                message: e.to_string(),
            })?;
        }
    }

    writer.flush().map_err(|e| brk_client::BrkError {
        message: e.to_string(),
    })?;
    eprintln!("Done. Output in {OUTPUT_FILE}");

    Ok(())
}
