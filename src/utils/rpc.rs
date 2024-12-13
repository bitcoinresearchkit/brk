use biter::bitcoincore_rpc::Client;

use crate::structs::Config;

impl From<&Config> for Client {
    fn from(config: &Config) -> Self {
        create_rpc(config).unwrap()
    }
}

fn create_rpc(config: &Config) -> color_eyre::Result<Client> {
    Ok(Client::new(
        &format!(
            "http://{}:{}",
            config
                .rpcconnect
                .as_ref()
                .unwrap_or(&"localhost".to_owned()),
            config.rpcport.unwrap_or(8332)
        ),
        config.to_rpc_auth().unwrap(),
    )?)
}
