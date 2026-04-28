use super::ticket::Ticket;

pub struct TicketFactory;

impl TicketFactory {
    pub fn create(id: u32, slot_id: u32) -> Ticket {
        Ticket { id, slot_id }
    }
}