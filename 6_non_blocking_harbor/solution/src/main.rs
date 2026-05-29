use std::{sync::mpsc::{self, TryRecvError}, thread, time};

struct HarborLedger {
    total_collected: f64,
}

impl HarborLedger {
    fn add_funds(&mut self, amount: f64) {
        self.total_collected += amount;
    }
}


fn main() {
    let mut ledger = HarborLedger { total_collected: 0.0 };
    let (sender, receiver) = mpsc::channel::<f64>();
    
    for _ in 0..5 { 
        let sender = sender.clone();
        thread::spawn(move || { sender.send(150.0).unwrap() });
    }

    drop(sender);

    loop {
        match receiver.try_recv() {
            Ok(amount) => ledger.add_funds(amount),
            Err(TryRecvError::Empty) => {
                println!("Scanning harbor for threats...");
                thread::sleep(time::Duration::from_millis(10));
            },
            Err(TryRecvError::Disconnected) => {
                println!("Receiver disconnected...");
                break;
            }
        }
    }

    println!("Total Harbor Funds: ${}", ledger.total_collected);
}
