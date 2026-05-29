Read the following code. We are attempting to spawn 5 threads. Each thread represents a ship concurrently paying a $150.00 fee into a central, shared HarborLedger.

``` rust
	use std::thread;

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
	    let mut handles = vec![];

	    // Attempt to spawn 5 concurrent transactions
	    for i in 0..5 {
	        let handle = thread::spawn(|| {
	            println!("Processing payment from ship {}...", i);
	            ledger.add_funds(150.0);
	        });
	        handles.push(handle);
	    }

	    // Wait for all threads to finish
	    for h in handles {
	        h.join().unwrap();
	    }

	    println!("Total Harbor Funds: ${}", ledger.total_collected);
	}	
```

If you try to compile this, the Rust compiler will hit you with a massive wall of errors. The borrow checker is going to scream that the closure may outlive the current function, and that you cannot borrow ledger as mutable more than once at a time.

You cannot fix this just by tweaking the borrows (&mut). The core architecture is fundamentally flawed for a multi-threaded environment.
To share and mutate state safely across multiple threads in Rust, you must wrap HarborLedger in two distinct standard library wrappers (smart pointers).
