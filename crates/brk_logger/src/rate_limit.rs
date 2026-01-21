use std::{
    fs::OpenOptions,
    io::{self, Write},
    path::PathBuf,
    sync::{
        Arc,
        atomic::{AtomicU64, Ordering},
    },
    time::{SystemTime, UNIX_EPOCH},
};

use jiff::{Timestamp, tz};
use tracing_subscriber::fmt::MakeWriter;

const MAX_WRITES_PER_SEC: u64 = 100;

struct Inner {
    dir: PathBuf,
    prefix: String,
    count: AtomicU64,
    last_second: AtomicU64,
}

impl Inner {
    fn can_write(&self) -> bool {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let last = self.last_second.load(Ordering::Relaxed);
        if now != last {
            self.last_second.store(now, Ordering::Relaxed);
            self.count.store(1, Ordering::Relaxed);
            true
        } else {
            self.count.fetch_add(1, Ordering::Relaxed) < MAX_WRITES_PER_SEC
        }
    }

    fn path(&self) -> PathBuf {
        let date = Timestamp::now()
            .to_zoned(tz::TimeZone::system())
            .strftime("%Y-%m-%d")
            .to_string();
        self.dir.join(format!("{}.{}", self.prefix, date))
    }
}

#[derive(Clone)]
pub struct RateLimitedFile(Arc<Inner>);

impl RateLimitedFile {
    pub fn new(dir: &std::path::Path, prefix: &str) -> Self {
        Self(Arc::new(Inner {
            dir: dir.to_path_buf(),
            prefix: prefix.to_string(),
            count: AtomicU64::new(0),
            last_second: AtomicU64::new(0),
        }))
    }
}

pub struct FileWriter(Arc<Inner>);

impl Write for FileWriter {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        if !self.0.can_write() {
            return Ok(buf.len());
        }

        OpenOptions::new()
            .create(true)
            .append(true)
            .open(self.0.path())?
            .write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl<'a> MakeWriter<'a> for RateLimitedFile {
    type Writer = FileWriter;

    fn make_writer(&'a self) -> Self::Writer {
        FileWriter(Arc::clone(&self.0))
    }
}
