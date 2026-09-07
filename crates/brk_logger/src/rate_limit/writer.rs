use std::{
    io::{self, Write},
    sync::Arc,
};
use tracing::Level;

use super::{Inner, LEVEL_SUFFIX, level_index, today};

pub struct FileWriter {
    pub(super) inner: Arc<Inner>,
    pub(super) level: Option<Level>,
}

impl Write for FileWriter {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let level_idx = self.level.map(level_index);

        if let Some(i) = level_idx
            && !self.inner.level_limits[i].can_write()
        {
            return Ok(buf.len());
        }

        let date = today();
        let dir = &self.inner.dir;

        self.inner
            .combined_slot
            .write(dir, &date, buf, || dir.join(format!("{date}.txt")))?;

        if let Some(i) = level_idx {
            self.inner.level_slots[i].write(dir, &date, buf, || {
                dir.join(format!("{date}_{}.txt", LEVEL_SUFFIX[i]))
            })?;
        }

        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
