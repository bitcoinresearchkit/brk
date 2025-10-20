use brk_rpc::Client;

pub struct BlockIterator {
    client: Client,
}

impl BlockIterator {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    pub fn iter() {}
}
