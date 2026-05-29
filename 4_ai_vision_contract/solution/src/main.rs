trait Vessel {
    fn calculate_fee(&self) -> f64;
}

struct ContainerShip {
    teu_capacity: u32
}

impl Vessel for ContainerShip {
    fn calculate_fee(&self) -> f64 {
        self.teu_capacity as f64 * 5.0
    }
}


struct OilTanker {
    barrels: u32
}

impl Vessel for OilTanker {
    fn calculate_fee(&self) -> f64 {
        self.barrels as f64 * 0.1
    }
}

fn process_fee(v: &impl Vessel) {
    println!("fee: {}", v.calculate_fee())
}



fn main() {
    let ship = ContainerShip { teu_capacity: 15000 };
    let tanker = OilTanker { barrels: 500000 };

    process_fee(&ship);
    process_fee(&tanker);
}
