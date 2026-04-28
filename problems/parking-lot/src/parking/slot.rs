use crate::vehicle::vehicle::VehicleType;

pub struct Slot {
    pub id: u32,
    pub slot_type: VehicleType,
    pub occupied_by: Option<u32>, // stores ticket id, None when free
}