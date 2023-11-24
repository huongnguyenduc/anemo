use crate::config::NetworkConfig;
use tracing::info;

pub fn random_key() -> [u8; 32] {
    let mut rng = rand::thread_rng();
    let mut bytes = [0u8; 32];
    rand::RngCore::fill_bytes(&mut rng, &mut bytes[..]);
    bytes
}

pub fn build_network(
    f: impl FnOnce(anemo::Router) -> anemo::Router,
    config: NetworkConfig,
    anemo_config: anemo::Config,
) -> anemo::Network {
    let NetworkConfig {
        server_addr,
        server_name,
        ..
    } = config;

    let router = f(anemo::Router::new());
    let network = anemo::Network::bind(server_addr.clone().as_str())
        .config(anemo_config)
        .private_key(random_key())
        .server_name(server_name)
        .start(router)
        .unwrap();

    info!("My local ip: {}", network.local_addr().ip());
    info!("My local port: {}", network.local_addr().port());
    network
}
