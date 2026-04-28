use std::time::Duration;

pub trait PricingStrategy {
    fn calculate_fee(&self, duration: Duration) -> f64;
}