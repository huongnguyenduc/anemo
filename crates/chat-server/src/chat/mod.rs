pub mod chat_client;
pub mod chat_server;
use serde::{Deserialize, Serialize};

pub use generated::{
    chat_peer_client::ChatPeerClient,
    chat_peer_server::{ChatPeer, ChatPeerServer},
};

pub mod generated {
    include!(concat!(env!("OUT_DIR"), "/chat.ChatPeer.rs"));
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ChatRequest {
    pub message: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ChatResponse {
    pub accepted: bool,
}
