mod config;
mod kafka_consumer;
mod kafka_producer;
mod models;
mod rsi;
mod metrics;
mod ws_manager;
mod ws_routes;

use config::Config;
use ws_manager::WsManager;
use warp::Filter;

#[tokio::main]
async fn main() {
    env_logger::init();
    let config = Config::from_env();

    // Create a shared WebSocket manager
    let ws_manager = WsManager::new();

    // Spawn WebSocket server
    let ws_filter = ws_routes::ws_routes(ws_manager.clone());
    tokio::spawn(async move {
        println!("🌐 WebSocket server running on 0.0.0.0:8081/ws");
        warp::serve(ws_filter).run(([0, 0, 0, 0], 8081)).await;
    });

    // Start Kafka consumer
    kafka_consumer::start_consumer(config, ws_manager).await;
}
