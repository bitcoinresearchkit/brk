use brk_reader::Reader;
use brk_rpc::Client;

/// Source configuration for block iteration
pub enum Source {
    /// Automatic selection based on range
    Smart { client: Client, reader: Reader },
    /// Always use RPC
    Rpc { client: Client },
    /// Always use Reader
    Reader { reader: Reader },
}

impl Source {
    pub fn client(&self) -> &Client {
        match self {
            Source::Smart { client, .. } => client,
            Source::Rpc { client } => client,
            Source::Reader { reader } => reader.client(),
        }
    }
}
