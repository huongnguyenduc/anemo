use crate::{
    chat::chat_client::ChatClient,
    config::KnownPeerConfig,
    discovery::{connect_to_known_peer, State},
    types::BroadcastMessage,
};
use anemo::Network;
use futures::future::join_all;
use std::{
    sync::{Arc, RwLock},
    vec,
};
use tokio::sync::mpsc;
use tracing::info;

pub struct Broadcaster {
    state: Arc<RwLock<State>>,
    network: Network,
    config: Option<KnownPeerConfig>,
    command_rx: mpsc::Receiver<BroadcastMessage>,
}

impl Broadcaster {
    pub fn new(
        state: Arc<RwLock<State>>,
        network: Network,
        config: Option<KnownPeerConfig>,
        command_rx: mpsc::Receiver<BroadcastMessage>,
    ) -> Self {
        Self {
            state,
            network,
            config,
            command_rx,
        }
    }

    pub async fn start(&mut self) {
        loop {
            let command = self.command_rx.recv().await;
            match command {
                Some(command) => match command {
                    BroadcastMessage::ChatMessage { content } => {
                        self.broadcast_message(content).await;
                    }

                    BroadcastMessage::Reconnect => {
                        if let Some(config) = self.config {
                            connect_to_known_peer(&self.network, config.addr, self.state.clone())
                                .await;
                        }
                    }
                },
                None => {
                    info!("Command channel closed");
                }
            }
        }
    }

    async fn broadcast_message(&self, content: String) {
        info!("Broadcasting message to connected peers: {}", content);
        let mut handles = vec![];
        if let Ok(state) = self.state.read() {
            state.connected_peers.keys().for_each(|peer_id| {
                let chat_client = ChatClient::new(self.network.clone());
                let content = content.clone();
                let peer_id = peer_id.clone();
                let handle = async move { chat_client.send_message(peer_id, content).await };
                handles.push(handle);
            });
        }

        join_all(handles).await;
    }
}
