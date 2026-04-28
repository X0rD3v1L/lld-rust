#[derive(Clone, Copy, PartialEq)]
pub enum VehicleType {
    Bike,
    Car,
    Truck,
}

pub trait Vehicle {
    fn vehicle_type(&self) -> VehicleType;
}