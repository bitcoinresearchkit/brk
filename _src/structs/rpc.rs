use brk_parser::bitcoincore_rpc::Client;

use crate::structs::Config;

impl From<&Config> for Client {
    fn from(config: &Config) -> Self {
        Client::new(
            &format!(
                "http://{}:{}",
                config.rpcconnect().unwrap_or(&"localhost".to_owned()),
                config.rpcport().unwrap_or(8332)
            ),
            config.to_rpc_auth().unwrap(),
        )
        .unwrap()
    }
}
