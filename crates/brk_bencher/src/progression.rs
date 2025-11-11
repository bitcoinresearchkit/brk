use parking_lot::Mutex;
use std::{
    fs,
    io::{self, BufWriter, Write},
    path::Path,
    time::Instant,
};

pub struct ProgressionMonitor {
    csv_file: Mutex<BufWriter<fs::File>>,
    start_time: Instant,
}

impl ProgressionMonitor {
    pub fn new(csv_path: &Path) -> io::Result<Self> {
        let mut csv_file = BufWriter::new(fs::File::create(csv_path)?);
        writeln!(csv_file, "timestamp_ms,block_number")?;

        Ok(Self {
            csv_file: Mutex::new(csv_file),
            start_time: Instant::now(),
        })
    }

    /// Fast inline check and record
    #[inline]
    pub fn check_and_record(&self, message: &str) {
        if !message.contains("block ") {
            return;
        }

        if let Some(block_num) = parse_block_number(message)
            && block_num % 10 == 0
        {
            let elapsed_ms = self.start_time.elapsed().as_millis();
            let mut writer = self.csv_file.lock();
            let _ = writeln!(writer, "{},{}", elapsed_ms, block_num);
        }
    }

    pub fn flush(&self) -> io::Result<()> {
        self.csv_file.lock().flush()
    }
}

#[inline]
fn parse_block_number(message: &str) -> Option<u64> {
    let start = message.find("block ")?;
    let after_block = &message[start + 6..];

    let end = after_block
        .find(|c: char| !c.is_ascii_digit())
        .unwrap_or(after_block.len());

    after_block[..end].parse::<u64>().ok()
}
