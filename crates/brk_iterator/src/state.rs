use std::vec;

use brk_reader::{Reader, Receiver};
use brk_rpc::Client;
use brk_structs::{BlockHash, Height, ReadBlock};

pub enum State {
    Rpc {
        client: Client,
        heights: vec::IntoIter<Height>,
        prev_hash: Option<BlockHash>,
    },
    Reader {
        receiver: Receiver<ReadBlock>,
        after_hash: Option<BlockHash>,
    },
}

impl State {
    pub fn new_rpc(
        client: Client,
        start: Height,
        end: Height,
        prev_hash: Option<BlockHash>,
    ) -> Self {
        let heights = (*start..=*end)
            .map(Height::new)
            .collect::<Vec<_>>()
            .into_iter();

        Self::Rpc {
            client,
            heights,
            prev_hash,
        }
    }

    pub fn new_reader(
        reader: Reader,
        start: Height,
        end: Height,
        after_hash: Option<BlockHash>,
    ) -> Self {
        State::Reader {
            receiver: reader.read(Some(start), Some(end)),
            after_hash,
        }
    }
}
