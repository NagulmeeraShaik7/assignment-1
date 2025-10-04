use tokio::sync::broadcast;

#[derive(Clone)]
pub struct WsManager {
    pub sender: broadcast::Sender<String>,
}

impl WsManager {
    pub fn new() -> Self {
        let (sender, _) = broadcast::channel(100);
        WsManager { sender }
    }

    pub async fn broadcast(&self, msg: &str) {
        let _ = self.sender.send(msg.to_string());
    }
}
