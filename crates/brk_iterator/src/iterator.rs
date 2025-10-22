use brk_structs::Block;

use crate::State;

pub struct BlockIterator(State);

impl BlockIterator {
    pub fn new(state: State) -> Self {
        Self(state)
    }
}

impl Iterator for BlockIterator {
    type Item = Block;

    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.0 {
            State::Rpc {
                client,
                heights,
                prev_hash,
            } => {
                let height = heights.next()?;
                let hash = client.get_block_hash(height).ok()?;
                let block = client.get_block(&hash).ok()?;

                if prev_hash
                    .as_ref()
                    .is_some_and(|prev_hash| block.header.prev_blockhash != prev_hash.into())
                {
                    return None;
                }

                prev_hash.replace(hash.clone());

                Some(Block::from((height, hash, block)))
            }
            State::Reader {
                receiver,
                after_hash,
            } => {
                let block = Block::from(receiver.recv().ok()?);

                // Only validate the first block (Reader validates the rest)
                if let Some(expected_prev) = after_hash.take()
                    && block.header.prev_blockhash != expected_prev.into()
                {
                    return None;
                }

                Some(block)
            }
        }
    }
}
