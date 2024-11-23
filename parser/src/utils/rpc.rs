use biter::bitcoincore_rpc::Client;

use crate::Config;

pub fn create_rpc(config: &Config) -> color_eyre::Result<Client> {
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
