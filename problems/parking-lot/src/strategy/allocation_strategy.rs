use crate::parking::slot::Slot;
use crate::vehicle::vehicle::VehicleType;

pub trait AllocationStrategy {
    fn find_slot(&self, slots: &[Slot], vtype: VehicleType) -> Option<u32>;
}