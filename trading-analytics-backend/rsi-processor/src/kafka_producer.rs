use rdkafka::config::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};
use std::time::Duration;

pub fn create_producer(brokers: &str) -> FutureProducer {
    ClientConfig::new()
        .set("bootstrap.servers", brokers)
        .create()
        .expect("Failed to create Kafka producer")
}

pub async fn produce_message(
    producer: &FutureProducer,
    topic: &str,
    key: &str,
    payload: &str,
) {
    let record = FutureRecord::to(topic).key(key).payload(payload);

    if let Err((e, _)) = producer.send(record, Duration::from_secs(0)).await {
        eprintln!("❌ Failed to send message: {:?}", e);
    }
}
