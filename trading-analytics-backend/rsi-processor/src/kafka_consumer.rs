use crate::models::{Trade, RsiMessage};
use crate::rsi::RsiCalculator;
use crate::kafka_producer::{produce_message, create_producer};
use crate::{Config, ws_manager::WsManager};
use rdkafka::config::ClientConfig;
use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::message::Message;
use serde_json;
use std::collections::HashMap;
use tokio_stream::StreamExt;

pub async fn start_consumer(config: Config, ws_manager: WsManager) {
    let consumer: StreamConsumer = ClientConfig::new()
        .set("group.id", &config.group_id)
        .set("bootstrap.servers", &config.brokers)
        .set("auto.offset.reset", "earliest")
        .create()
        .expect("Failed to create consumer");

    consumer
        .subscribe(&[&config.input_topic])
        .expect("Failed to subscribe to topic");

    let producer = create_producer(&config.brokers);
    let mut rsi_map: HashMap<String, RsiCalculator> = HashMap::new();
    let mut stream = consumer.stream();

    println!("🚀 RSI Processor started. Waiting for messages...");

    while let Some(result) = stream.next().await {
        match result {
            Ok(m) => {
                if let Some(Ok(payload)) = m.payload_view::<str>() {
                    if let Ok(trade) = serde_json::from_str::<Trade>(payload) {
                        let token = trade.token_address.clone();
                        let calculator = rsi_map
                            .entry(token.clone())
                            .or_insert_with(|| RsiCalculator::new(14));

                        if let Some(rsi) = calculator.add_price(trade.price_in_sol) {
                            let rsi_msg = RsiMessage {
                                token_address: token.clone(),
                                rsi,
                                timestamp: trade.block_time.clone(),
                            };

                            if let Ok(json) = serde_json::to_string(&rsi_msg) {
                                // ✅ 1. Publish to Kafka output topic
                                produce_message(&producer, &config.output_topic, &token, &json).await;
                                
                                // ✅ 2. Broadcast to WebSocket clients
                                ws_manager.broadcast(&json).await;

                                println!("📊 Published + Broadcast RSI for {}: {:.2}", token, rsi);
                            }
                        }
                    } else {
                        eprintln!("⚠️ Failed to parse trade JSON: {}", payload);
                    }
                }
            }
            Err(e) => eprintln!("❌ Kafka error: {:?}", e),
        }
    }
}
