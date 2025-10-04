use crate::ws_manager::WsManager;
use futures_util::{StreamExt, SinkExt};
use warp::Filter;

pub fn ws_routes(
    ws_manager: WsManager,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("ws")
        .and(warp::ws())
        .map(move |ws: warp::ws::Ws| {
            let ws_manager = ws_manager.clone();
            ws.on_upgrade(move |socket| handle_connection(socket, ws_manager))
        })
}

async fn handle_connection(ws: warp::ws::WebSocket, ws_manager: WsManager) {
    let (mut tx, mut rx) = ws.split();
    let mut receiver = ws_manager.sender.subscribe();

    println!("New WebSocket client connected");

    // Task to send RSI updates to client
    tokio::spawn(async move {
        while let Ok(msg) = receiver.recv().await {
            println!("Broadcasting to WS client: {}", msg);
            if tx.send(warp::ws::Message::text(msg)).await.is_err() {
                println!("WebSocket client disconnected");
                break;
            }
        }
    });

    // Read incoming messages from client (optional)
    while let Some(Ok(msg)) = rx.next().await {
        if msg.is_text() {
            println!("Received from WS client: {}", msg.to_str().unwrap());
        }
    }

    println!("WebSocket connection closed");
}
