use std::vec;

use brk_reader::Receiver;
use brk_rpc::Client;
use brk_structs::{BlockHash, Height, ReadBlock};

pub enum Source {
    Rpc {
        client: Client,
        heights: vec::IntoIter<Height>,
        prev_hash: Option<BlockHash>,
    },
    Reader {
        receiver: Receiver<ReadBlock>,
    },
}

impl Source {
    pub fn new_rpc(client: Client, start: Height, end: Height) -> Self {
        let heights = (*start..=*end)
            .map(Height::new)
            .collect::<Vec<_>>()
            .into_iter();

        Self::Rpc {
            client,
            heights,
            prev_hash: None,
        }
    }

    pub fn new_reader(client: Client, start: Height, end: Height) -> Self {
        let heights = (*start..=*end)
            .map(Height::new)
            .collect::<Vec<_>>()
            .into_iter();

        Self::Rpc {
            client,
            heights,
            prev_hash: None,
        }
    }
}
