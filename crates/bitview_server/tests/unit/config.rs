use super::*;

#[test]
fn defaults_to_all_interfaces_on_the_default_port() {
    let config = ServerConfig::default();

    assert_eq!(config.bind, DEFAULT_BIND);
    assert_eq!(config.port, Port::DEFAULT);
}
