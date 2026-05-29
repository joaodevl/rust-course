use std::{sync::{Arc, Mutex}, thread::{self, JoinHandle}};


struct HarborLedger {
    total_collected: f64,
}

impl HarborLedger {
    fn add_funds(&mut self, amount: f64) {
        self.total_collected += amount;
    }
}

fn main() {
    let ledger = Arc::new(Mutex::new(
        HarborLedger { total_collected: 0.0 }
    ));
    
    let mut handles: Vec<JoinHandle<()>> = vec![];

    for i in 0..5 {
        let ledger = Arc::clone(&ledger);
        let handle = thread::spawn(move || {
            println!("Processing payment from ship {} ...", i);
            ledger.lock()
                .unwrap()
                .add_funds(150.0);
        });
        handles.push(handle);
    }

    for h in handles {
        h.join().unwrap();
    }

    println!("Total Harbor Funds: ${}", ledger.lock().unwrap().total_collected);
}
