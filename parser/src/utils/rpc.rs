use biter::bitcoincore_rpc::{Auth, Client};

use crate::Config;

pub fn create_rpc(config: &Config) -> color_eyre::Result<Client> {
    Ok(Client::new(
        &format!(
            "http://{}:{}",
            config.rpcconnect.as_ref().unwrap(),
            config.rpcport.unwrap()
        ),
        Auth::UserPass(
            config.rpcuser.clone().unwrap(),
            config.rpcpassword.clone().unwrap(),
        ),
    )?)
}
