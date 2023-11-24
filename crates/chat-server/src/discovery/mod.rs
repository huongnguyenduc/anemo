pub mod builder;
pub mod server;

use anemo::{types::PeerEvent, Network, Peer, PeerId, Request, Response};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::{Arc, RwLock},
    time::Duration,
};
use tokio::task::JoinSet;
use tracing::info;

mod generated {
    include!(concat!(env!("OUT_DIR"), "/chat.Discovery.rs"));
}
pub use generated::{
    discovery_client::DiscoveryClient,
    discovery_server::{Discovery, DiscoveryServer},
};
pub use server::GetKnownPeersResponse;

use self::builder::DiscoveryConfig;

const TIMEOUT: Duration = Duration::from_secs(1);

#[derive(Default)]
pub struct State {
    pub connected_peers: HashMap<PeerId, ()>,
    our_info: Option<NodeInfo>,
    pub known_peers: HashMap<PeerId, NodeInfo>,
}

/// The information necessary to dial another peer.
///
/// `NodeInfo` contains all the information that is shared with other nodes via the discovery
/// service to advertise how a node can be reached.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct NodeInfo {
    pub peer_id: PeerId,
    pub addresses: Vec<SocketAddr>,
}

pub struct DiscoveryEventLoop {
    pub network: Network,
    pub state: Arc<RwLock<State>>,
    pub config: DiscoveryConfig,
    tasks: JoinSet<()>,
}

impl DiscoveryEventLoop {
    pub async fn start(&mut self) {
        info!("Discovery started");

        self.construct_our_info();

        if let Some(known_peer) = self.config.known_peer {
            if let Some(peer) =
                connect_to_known_peer(&self.network, known_peer.addr, self.state.clone()).await
            {
                info!("Continue querying peers...");
                self.tasks.spawn(query_peer_for_their_known_peers(
                    peer,
                    self.state.clone(),
                    self.network.clone(),
                ));
            }
        }

        let mut peer_events = {
            let (subscriber, _peers) = self.network.subscribe().unwrap();
            subscriber
        };

        loop {
            let peer_event = peer_events.recv().await;
            match peer_event {
                Ok(event) => match event {
                    PeerEvent::NewPeer(peer_id) => {
                        if let Some(peer) = self.network.peer(peer_id) {
                            info!("Peer connected: {}", peer_id.short_display(4));
                            self.state
                                .write()
                                .unwrap()
                                .connected_peers
                                .insert(peer_id, ());

                            self.tasks.spawn(query_peer_for_their_known_peers(
                                peer,
                                self.state.clone(),
                                self.network.clone(),
                            ));
                        }
                    }
                    PeerEvent::LostPeer(peer_id, _) => {
                        info!("Peer disconnected: {}", peer_id.short_display(4));
                        if let Ok(mut state) = self.state.write() {
                            state.connected_peers.remove(&peer_id);
                        }
                    }
                },
                Err(e) => {
                    info!("Error receiving peer event: {}", e);
                }
            }
        }
    }

    fn construct_our_info(&mut self) {
        if self.state.read().unwrap().our_info.is_some() {
            return;
        }

        let address: SocketAddr = self.network.local_addr();
        let our_info = NodeInfo {
            peer_id: self.network.peer_id(),
            addresses: vec![address],
        };

        self.state.write().unwrap().our_info = Some(our_info);
    }
}

async fn query_peer_for_their_known_peers(peer: Peer, state: Arc<RwLock<State>>, network: Network) {
    info!("Querying peers...");
    let mut client = DiscoveryClient::new(peer);

    let request = Request::new(()).with_timeout(TIMEOUT);
    if let Some(found_peers) = client
        .get_known_peers(request)
        .await
        .ok()
        .map(Response::into_inner)
        .map(
            |GetKnownPeersResponse {
                 own_info,
                 mut known_peers,
             }| {
                if !own_info.addresses.is_empty() {
                    known_peers.push(own_info)
                }
                known_peers
            },
        )
    {
        for peer in found_peers {
            if peer.peer_id == network.peer_id() || peer.addresses.is_empty() {
                continue;
            }

            if let Ok(state) = state.read() {
                if
                // state.connected_peers.contains_key(&peer.peer_id)
                // ||
                state.known_peers.contains_key(&peer.peer_id) {
                    continue;
                }
            }

            connect_to_known_peer(&network, peer.addresses[0], state.clone()).await;
        }
    }
}

pub async fn connect_to_known_peer(
    network: &Network,
    address: SocketAddr,
    state: Arc<RwLock<State>>,
) -> Option<Peer> {
    if let Ok(peer_id) = network.connect(address).await {
        let node_info = NodeInfo {
            peer_id,
            addresses: vec![address],
        };

        state
            .write()
            .unwrap()
            .known_peers
            .insert(peer_id, node_info);

        info!("Connected to peer: {}", peer_id);

        if let Some(peer) = network.peer(peer_id) {
            state.write().unwrap().connected_peers.insert(peer_id, ());
            return Some(peer);
        }
    }

    None
}
