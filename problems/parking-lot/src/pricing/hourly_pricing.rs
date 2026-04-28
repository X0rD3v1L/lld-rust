use super::pricing_strategy::PricingStrategy;
use std::time::Duration;

pub struct HourlyPricing {
    pub rate_per_hour: f64,
}

impl PricingStrategy for HourlyPricing {
    fn calculate_fee(&self, duration: Duration) -> f64 {
        let hours = duration.as_secs_f64() / 3600.0;
        hours.ceil() * self.rate_per_hour
    }
}