use super::vehicle::{Vehicle, VehicleType};

pub struct Car;

impl Vehicle for Car {
    fn vehicle_type(&self) -> VehicleType {
        VehicleType::Car
    }
}