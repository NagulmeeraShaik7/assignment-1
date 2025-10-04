use std::collections::VecDeque;

pub struct RsiCalculator {
    period: usize,
    prices: VecDeque<f64>,
}

impl RsiCalculator {
    pub fn new(period: usize) -> Self {
        Self {
            period,
            prices: VecDeque::with_capacity(period + 1),
        }
    }

    pub fn add_price(&mut self, price: f64) -> Option<f64> {
        self.prices.push_back(price);
        if self.prices.len() > self.period + 1 {
            self.prices.pop_front();
        }

        if self.prices.len() < self.period + 1 {
            return None;
        }

        let mut gains = 0.0;
        let mut losses = 0.0;

        for i in 1..self.prices.len() {
            let diff = self.prices[i] - self.prices[i - 1];
            if diff >= 0.0 {
                gains += diff;
            } else {
                losses += -diff;
            }
        }

        let avg_gain = gains / self.period as f64;
        let avg_loss = losses / self.period as f64;

        if avg_loss == 0.0 {
            return Some(100.0);
        }

        let rs = avg_gain / avg_loss;
        Some(100.0 - (100.0 / (1.0 + rs)))
    }
}
