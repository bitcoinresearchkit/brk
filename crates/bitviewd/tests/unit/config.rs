use std::net::Ipv4Addr;

use super::*;

#[test]
fn server_listener_is_typed_and_defaults_are_centralized() {
    let defaults = Config::default();
    assert_eq!(defaults.serverbind(), DEFAULT_BIND);
    assert_eq!(defaults.serverport(), Port::DEFAULT);

    let config: Config = toml::from_str(
        r#"
            serverbind = "127.0.0.1"
            serverport = 3111
        "#,
    )
    .unwrap();
    assert_eq!(config.serverbind(), IpAddr::V4(Ipv4Addr::LOCALHOST));
    assert_eq!(config.serverport(), Port::new(3111));
}
