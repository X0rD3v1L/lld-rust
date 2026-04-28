use super::payment::PaymentStrategy;

pub struct CardPayment;
pub struct UPIPayment;

impl PaymentStrategy for CardPayment {
    fn pay(&self, amount: f64) {
        println!("Paid {} via Card", amount);
    }
}

impl PaymentStrategy for UPIPayment {
    fn pay(&self, amount: f64) {
        println!("Paid {} via UPI", amount);
    }
}