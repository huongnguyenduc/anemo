use crate::types::BroadcastMessage;
use tokio::{sync::mpsc, task::JoinHandle};

pub struct ChatReconnect {
    reconnect_tx: mpsc::Sender<BroadcastMessage>,
    interval: u64,
}

impl ChatReconnect {
    pub fn new(reconnect_tx: mpsc::Sender<BroadcastMessage>, interval: u64) -> Self {
        ChatReconnect {
            reconnect_tx,
            interval,
        }
    }

    pub fn start(self) -> JoinHandle<()> {
        tokio::spawn(async move {
            loop {
                tokio::time::sleep(std::time::Duration::from_millis(self.interval)).await;
                self.reconnect_tx
                    .send(BroadcastMessage::Reconnect)
                    .await
                    .expect("Reconnect failed");
            }
        })
    }
}
