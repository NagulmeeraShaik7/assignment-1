use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct Trade {
    pub block_time: String,
    pub token_address: String,
    pub price_in_sol: f64,
    pub transaction_signature: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
pub struct RsiMessage {
    pub token_address: String,
    pub rsi: f64,
    pub timestamp: String,
}
