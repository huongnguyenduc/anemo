use anemo::{rpc::Status, Request, Response};
use tracing::info;

use super::{ChatPeer, ChatRequest, ChatResponse};

#[derive(Default)]
pub struct ChatServer {}

#[anemo::async_trait]
impl ChatPeer for ChatServer {
    async fn send_message(
        &self,
        request: Request<ChatRequest>,
    ) -> Result<Response<ChatResponse>, Status> {
        info!(
            "Got a message from: {}",
            request.peer_id().unwrap().short_display(4),
        );

        info!("Message Content: {}", request.into_body().message);

        let reply = ChatResponse { accepted: true };

        Ok(Response::new(reply))
    }
}
