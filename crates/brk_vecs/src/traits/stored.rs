use crate::{AnyVec, Exit, File, Result, Stamp, variants::Header};

pub trait AnyStoredVec: AnyVec {
    fn file(&self) -> &File;

    fn region_index(&self) -> usize;

    fn header(&self) -> &Header;

    fn mut_header(&mut self) -> &mut Header;

    fn flush(&mut self) -> Result<()>;

    #[inline]
    fn safe_flush(&mut self, exit: &Exit) -> Result<()> {
        let _lock = exit.lock();
        self.flush()
    }

    fn stored_len(&self) -> usize;

    fn update_stamp(&mut self, stamp: Stamp) {
        self.mut_header().update_stamp(stamp);
    }

    fn stamp(&self) -> Stamp {
        self.header().stamp()
    }

    #[inline]
    fn stamped_flush(&mut self, stamp: Stamp) -> Result<()> {
        self.update_stamp(stamp);
        self.flush()
    }
}
