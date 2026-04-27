use std::collections::HashMap;

type DriverId = String;

#[derive(Debug)]
struct Delivery {
    start_time: i64,
    end_time: i64,
    paid: bool,
}

impl Delivery {
    fn new(start_time: i64, end_time: i64) -> Result<Self, String> {
        if end_time <= start_time {
            return Err("endTime must be greater than startTime".into());
        }
        Ok(Self { start_time, end_time, paid: false })
    }

    fn cost(&self) -> i64 {
        self.end_time - self.start_time
    }
}

#[derive(Debug)]
struct Driver {
    id: DriverId,
    deliveries: Vec<Delivery>,
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
    unpaid_cost: i64,
}

impl DeliveryCostSystem {
    fn new() -> Self {
        Self {
            drivers: HashMap::new(),
            total_cost: 0,
            unpaid_cost: 0,
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

        self.total_cost += delivery.cost();
        self.unpaid_cost += delivery.cost();

        driver.add_delivery(delivery);

        Ok(())
    }

    fn pay_up_to_time(&mut self, up_to_time: i64) {
        for driver in self.drivers.values_mut() {
            for delivery in &mut driver.deliveries {
                if !delivery.paid && delivery.end_time <= up_to_time {
                    delivery.paid = true;
                    self.unpaid_cost -= delivery.cost();
                }
            }
        }
    }

    fn get_total_cost(&self) -> i64 {
        self.total_cost
    }

    fn get_cost_to_be_paid(&self) -> i64 {
        self.unpaid_cost
    }

    fn driver_costs(&self) -> Vec<(&str, i64)> {
        self.drivers
            .values()
            .map(|d| (d.id.as_str(), d.deliveries.iter().map(|del| del.cost()).sum()))
            .collect()
    }
}


fn main() {
    let mut system = DeliveryCostSystem::new();

    system.add_driver("driver1".into());
    system.add_driver("driver2".into());

    system.add_delivery("driver1", 10, 40).unwrap(); // cost = 30
    system.add_delivery("driver2", 50, 80).unwrap(); // cost = 30

    println!("Total Cost: {}", system.get_total_cost()); // 60
    println!("Unpaid Cost: {}", system.get_cost_to_be_paid()); // 60

    for (id, cost) in system.driver_costs() {
        println!("Driver {id}: {cost}");
    }

    system.pay_up_to_time(45);

    println!("Unpaid Cost after settlement: {}", system.get_cost_to_be_paid()); // 30
}
