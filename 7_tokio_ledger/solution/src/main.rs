use std::{ops::{AddAssign}, sync::Arc};
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    let ledger = Arc::new(Mutex::new(0.0_f64));
    let mut handles = vec![];

    for i in 0..5 {
        let ledger = Arc::clone(&ledger);

        let handle = tokio::spawn(async move {
            println!("Processing async payment {}...", i);
            
            ledger.lock().await.add_assign(150.0)
        });

        handles.push(handle);
    }

    for h in handles { h.await.unwrap() }

    println!("Final total: ${}", ledger.lock().await)
}
