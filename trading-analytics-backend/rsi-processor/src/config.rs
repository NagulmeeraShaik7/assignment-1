pub struct Config {
    pub brokers: String,
    pub group_id: String,
    pub input_topic: String,
    pub output_topic: String,
    pub ws_port: u16,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            brokers: std::env::var("BROKERS").unwrap_or_else(|_| "localhost:19092".to_string()),
            group_id: std::env::var("GROUP_ID").unwrap_or_else(|_| "rsi-processor-group".to_string()),
            input_topic: std::env::var("INPUT_TOPIC").unwrap_or_else(|_| "trade-data".to_string()),
            output_topic: std::env::var("OUTPUT_TOPIC").unwrap_or_else(|_| "rsi-data".to_string()),
            ws_port: std::env::var("WS_PORT")
                .unwrap_or_else(|_| "8081".to_string())
                .parse()
                .unwrap(),
        }
    }
}
