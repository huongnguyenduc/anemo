use super::{server::Server, Discovery, DiscoveryEventLoop, DiscoveryServer, State};
use crate::config::KnownPeerConfig;
use anemo::Network;
use std::sync::{Arc, RwLock};
use tokio::task::{JoinHandle, JoinSet};

#[derive(Default)]
pub struct DiscoveryConfig {
    pub known_peer: Option<KnownPeerConfig>,
}

#[derive(Default)]
pub struct DiscoveryBuilder {
    config: DiscoveryConfig,
}

impl DiscoveryBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn config(mut self, known_peer_config: Option<KnownPeerConfig>) -> Self {
        self.config = DiscoveryConfig {
            known_peer: known_peer_config,
        };
        self
    }

    pub fn build(
        self,
    ) -> (
        UnstartedDiscovery,
        DiscoveryServer<impl Discovery>,
        Arc<RwLock<State>>,
    ) {
        let state = Arc::new(RwLock::new(State::default()));
        let server = DiscoveryServer::new(Server {
            state: state.clone(),
        });

        (
            UnstartedDiscovery::new(state.clone(), self.config),
            server,
            state,
        )
    }
}

pub struct UnstartedDiscovery {
    config: DiscoveryConfig,
    state: Arc<RwLock<State>>,
}

impl UnstartedDiscovery {
    pub fn new(state: Arc<RwLock<State>>, config: DiscoveryConfig) -> Self {
        Self { config, state }
    }

    pub fn build(self, network: Network) -> DiscoveryEventLoop {
        DiscoveryEventLoop {
            network,
            tasks: JoinSet::new(),
            config: self.config,
            state: self.state,
        }
    }

    pub fn start(self, network: anemo::Network) -> JoinHandle<()> {
        let mut event_loop = self.build(network);
        tokio::spawn(async move { event_loop.start().await })
    }
}
