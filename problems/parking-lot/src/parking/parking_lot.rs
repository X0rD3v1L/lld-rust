use crate::parking::slot::Slot;
use crate::pricing::pricing_strategy::PricingStrategy;
use crate::payment::payment::PaymentStrategy;
use crate::strategy::allocation_strategy::AllocationStrategy;
use crate::ticket::ticket::Ticket;
use crate::ticket::ticket_factory::TicketFactory;
use crate::vehicle::vehicle::Vehicle;

pub struct ParkingLot {
    pub slots: Vec<Slot>,
    pub strategy: Box<dyn AllocationStrategy>,
    pub ticket_counter: u32,
}

impl ParkingLot {
    pub fn park(&mut self, vehicle: &dyn Vehicle) -> Option<Ticket> {
        // Step 1: find a free slot using the configured strategy
        let slot_id = self.strategy.find_slot(&self.slots, vehicle.vehicle_type())?;

        // Step 2: create ticket
        self.ticket_counter += 1;
        let ticket = TicketFactory::create(self.ticket_counter, slot_id);

        // Step 3: mark slot as occupied
        if let Some(slot) = self.slots.iter_mut().find(|s| s.id == slot_id) {
            slot.occupied_by = Some(ticket.id);
        }

        Some(ticket)
    }
    pub fn unpark(
        &mut self,
        ticket: crate::ticket::ticket::Ticket,
        duration: std::time::Duration,
        pricing: &dyn PricingStrategy,
        payment: &dyn PaymentStrategy,
    ) {
        // 1. Calculate fee from provided duration
        let fee = pricing.calculate_fee(duration);

        // 3. Process payment
        payment.pay(fee);

        // 4. Free slot
        if let Some(slot) = self.slots.iter_mut().find(|s| s.id == ticket.slot_id) {
            slot.occupied_by = None;
        }

        println!("Unparked. Fee: {}", fee);
    }
}