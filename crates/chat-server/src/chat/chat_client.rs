use super::{ChatPeerClient, ChatRequest, ChatResponse};
use anemo::{PeerId, Response};

#[derive(Clone)]
pub struct ChatClient {
    network: anemo::Network,
}

impl ChatClient {
    pub fn new(network: anemo::Network) -> Self {
        Self { network }
    }

    pub async fn send_message(&self, peer: PeerId, content: String) -> Response<ChatResponse> {
        let peer = self.network.peer(peer).expect("Connected peer not found");
        let mut client = ChatPeerClient::new(peer);
        client
            .send_message(ChatRequest { message: content })
            .await
            .expect("Send failed")
    }
}
