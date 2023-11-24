use crate::types::BroadcastMessage;
use tokio::{io, io::AsyncBufReadExt, sync::mpsc};

pub struct ChatCli {
    chat_tx: mpsc::Sender<BroadcastMessage>,
}

impl ChatCli {
    pub fn new(chat_tx: mpsc::Sender<BroadcastMessage>) -> Self {
        ChatCli { chat_tx }
    }

    pub async fn start(&self) {
        // Broadcast message from stdin
        let mut stdin = io::BufReader::new(io::stdin()).lines();
        loop {
            let line = stdin.next_line().await;
            if let Ok(Some(line)) = line {
                self.chat_tx
                    .send(BroadcastMessage::ChatMessage {
                        content: line.clone(),
                    })
                    .await
                    .expect("Send failed");
            }
        }
    }
}
