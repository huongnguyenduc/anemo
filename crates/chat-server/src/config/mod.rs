use std::net::SocketAddr;

use anemo::QuicConfig;
use clap::*;

#[derive(Parser, Clone, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short, long, default_value_t = String::from("127.0.0.1"))]
    pub ip: String,
    #[arg(short, long)]
    pub port: Option<u16>,
    #[arg(short = 'n', long, default_value_t = String::from("chat"))]
    pub server_name: String,
    #[arg(short = 'a', long, default_value_t = String::from("0.0.0.0:0"))]
    pub server_addr: String,
    #[arg(short, long, default_value_t = 100)]
    pub max_chat_channels: usize,
    #[arg(short = 'r', long, default_value_t = 30000)]
    pub network_reconnect_interval: u64,
    #[arg(short = 't', long, default_value_t = 30000)]
    pub max_idle_timeout_ms: u64,
}

impl Args {
    pub fn get_network_config(&self) -> NetworkConfig {
        NetworkConfig {
            server_name: self.server_name.clone(),
            server_addr: self.server_addr.clone(),
        }
    }

    pub fn get_known_peer_config(&self) -> Option<KnownPeerConfig> {
        if let Some(port) = &self.port {
            return Some(KnownPeerConfig {
                addr: SocketAddr::new(self.ip.parse().unwrap(), *port),
            });
        }
        None
    }

    pub fn get_anemo_config(&self) -> anemo::Config {
        let mut config = anemo::Config::default();
        let mut quic_config = QuicConfig::default();
        quic_config.max_idle_timeout_ms = Some(self.max_idle_timeout_ms);
        config.quic = Some(quic_config);

        config
    }
}

pub struct NetworkConfig {
    pub server_name: String,
    pub server_addr: String,
}

#[derive(Copy, Clone)]
pub struct KnownPeerConfig {
    pub addr: SocketAddr,
}
