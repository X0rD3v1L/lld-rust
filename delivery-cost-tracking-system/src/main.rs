use std::collections::HashMap;

type DriverId = String;

#[derive(Debug)]
struct Delivery {
    start_time: i64,
    end_time: i64,
    cost: i64
}

impl Delivery {
    fn new(start_time: i64, end_time: i64) -> Result<Self, String> {
        if end_time <= start_time {
            return Err("endTime must be greater than startTime".into());
        }

        Ok(
            Self { start_time, end_time, cost: end_time - start_time }
        )
    }
}

#[derive(Debug)]
struct Driver {
    id: DriverId,
    deliveries: Vec<Delivery>
}

impl Driver {
    fn new(id: DriverId) -> Self {
        Self { id, deliveries: Vec::new() }
    }

    fn add_delivery(&mut self, delivery: Delivery) {
        self.deliveries.push(delivery);
    }
}

#[derive(Debug)]
struct DeliveryCostSystem {
    drivers: HashMap<DriverId, Driver>,
    total_cost: i64,
}

impl DeliveryCostSystem {
    fn new() -> Self {
        Self {
            drivers: HashMap::new(),
            total_cost: 0,
        }
    }

    fn add_driver(&mut self, driver_id: DriverId) {
        self.drivers
            .entry(driver_id.clone())
            .or_insert_with(|| Driver::new(driver_id));
    }

    fn add_delivery(
        &mut self,
        driver_id: &str,
        start_time: i64,
        end_time: i64,
    ) -> Result<(), String> {
        let delivery = Delivery::new(start_time, end_time)?;

        let driver = self
            .drivers
            .get_mut(driver_id)
            .ok_or("Driver not found")?;

        self.total_cost += delivery.cost;
        driver.add_delivery(delivery);

        Ok(())
    }

    fn get_total_cost(&self) -> i64 {
        self.total_cost
    }
}


fn main() {
    let mut system = DeliveryCostSystem::new();

    system.add_driver("driver1".into());
    system.add_driver("driver2".into());

    system.add_delivery("driver1", 10, 40).unwrap(); // cost = 30
    system.add_delivery("driver2", 50, 80).unwrap(); // cost = 30

    println!("Total Cost: {}", system.get_total_cost()); // 60
}   

