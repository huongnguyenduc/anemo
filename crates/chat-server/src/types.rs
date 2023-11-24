use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum ChatCommand {
    Reconnect,
    BroadcastMessage { content: String },
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum BroadcastMessage {
    Reconnect,
    ChatMessage { content: String },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum BroadcastMessageV2 {
    Reconnect,
    ChatMessage { content: String },
}
