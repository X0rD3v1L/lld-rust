use super::allocation_strategy::AllocationStrategy;
use crate::parking::slot::Slot;
use crate::vehicle::vehicle::VehicleType;

pub struct NearestSlotStrategy;

impl AllocationStrategy for NearestSlotStrategy {
    fn find_slot(&self, slots: &[Slot], vtype: VehicleType) -> Option<u32> {
        slots
            .iter()
            .find(|s| s.occupied_by.is_none() && s.slot_type == vtype)
            .map(|s| s.id)
    }
}