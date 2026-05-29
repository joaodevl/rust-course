We are migrating the Harbor Ledger to an asynchronous web backend.

Your setup requires the tokio crate. You no longer use std::thread::spawn; you use tokio::spawn. You no longer use std::sync::Mutex; you use tokio::sync::Mutex.

Read the skeleton below.


``` rust
	use std::sync::Arc;
	use tokio::sync::Mutex;
	// Note: We use the macro to start the Tokio runtime
	#[tokio::main] 
	async fn main() {
	    let ledger = Arc::new(Mutex::new(0.0_f64)); // A simple shared float
	    let mut handles = vec![];

	    for i in 0..5 {
	        let ledger_clone = Arc::clone(&ledger);
	        
	        // tokio::spawn requires an async block
	        let handle = tokio::spawn(async move {
	            println!("Processing async payment {}...", i);
	            
	            // TODO: Acquire the lock and add 150.0 to the ledger
	        });
	        handles.push(handle);
	    }

	    // TODO: Await all the handles to ensure the tasks finish before main exits

	    // TODO: Print the final total
	}

```

Your Directive:
Complete the three TODO sections in the asynchronous code above. Show me exactly how you acquire the async lock, how you wait for the Tokio tasks to finish, and how you read the final total.
