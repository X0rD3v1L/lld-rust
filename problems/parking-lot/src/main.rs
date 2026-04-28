mod vehicle;
mod parking;
mod strategy;
mod payment;
mod ticket;
mod pricing;

use std::time::Duration;

use crate::vehicle::car::Car;
use crate::vehicle::vehicle::VehicleType;

use crate::parking::parking_lot::ParkingLot;
use crate::parking::slot::Slot;

use crate::strategy::nearest_slot::NearestSlotStrategy;

use crate::pricing::hourly_pricing::HourlyPricing;
use crate::payment::card::CardPayment;

fn main() {
    let mut lot = ParkingLot {
        slots: vec![
            Slot { id: 1, slot_type: VehicleType::Car, occupied_by: None },
            Slot { id: 2, slot_type: VehicleType::Car, occupied_by: None },
        ],
        strategy: Box::new(NearestSlotStrategy),
        ticket_counter: 0,
    };

    let pricing = HourlyPricing { rate_per_hour: 10.0 };
    let payment = CardPayment;

    // Car 1 — gets slot 1
    println!("--- Car 1 entering ---");
    let ticket1 = lot.park(&Car).expect("should get slot");
    println!("Ticket issued: {:?}", ticket1);

    // Car 2 — gets slot 2
    println!("\n--- Car 2 entering ---");
    let ticket2 = lot.park(&Car).expect("should get slot");
    println!("Ticket issued: {:?}", ticket2);

    // Car 3 — lot is full
    println!("\n--- Car 3 entering ---");
    match lot.park(&Car) {
        Some(ticket) => println!("Ticket issued: {:?}", ticket),
        None => println!("No slot available — lot is full!"),
    }

    // Car 1 exits
    println!("\n--- Car 1 exiting ---");
    let parked_hours: u64 = 3;
    println!("Simulated parking: {} hour(s)", parked_hours);
    lot.unpark(ticket1, Duration::from_secs(parked_hours * 3600), &pricing, &payment);

    // Car 2 exits
    println!("\n--- Car 2 exiting ---");
    let parked_hours: u64 = 1;
    println!("Simulated parking: {} hour(s)", parked_hours);
    lot.unpark(ticket2, Duration::from_secs(parked_hours * 3600), &pricing, &payment);
}
