use std::cmp::Ordering;

use brk_error::{Error, Result};
use brk_types::ReadBlock;
use crossbeam::channel::Sender;
use rustc_hash::FxHashMap;

pub(super) struct ReorderState {
    pub(super) next_offset: u32,
    pending: FxHashMap<u32, ReadBlock>,
    send_to_consumer: Sender<Result<ReadBlock>>,
    consumer_dropped: bool,
}

impl ReorderState {
    pub(super) fn new(send_to_consumer: Sender<Result<ReadBlock>>) -> Self {
        Self {
            next_offset: 0,
            pending: FxHashMap::default(),
            send_to_consumer,
            consumer_dropped: false,
        }
    }

    pub(super) fn finalize(self, expected_count: usize) -> Result<()> {
        if !self.consumer_dropped && (self.next_offset as usize) < expected_count {
            return Err(Error::Internal(
                "forward pipeline: blk files missing canonical blocks",
            ));
        }
        Ok(())
    }

    pub(super) fn try_emit(&mut self, offset: u32, block: ReadBlock) -> bool {
        match offset.cmp(&self.next_offset) {
            Ordering::Equal => {
                if !self.send(block) {
                    return false;
                }
                while let Some(next) = self.pending.remove(&self.next_offset) {
                    if !self.send(next) {
                        return false;
                    }
                }
                true
            }
            Ordering::Greater => {
                self.pending.insert(offset, block);
                true
            }
            Ordering::Less => true,
        }
    }

    fn send(&mut self, block: ReadBlock) -> bool {
        if self.send_to_consumer.send(Ok(block)).is_err() {
            self.consumer_dropped = true;
            return false;
        }
        self.next_offset += 1;
        true
    }
}
