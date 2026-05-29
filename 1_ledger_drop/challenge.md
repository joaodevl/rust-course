Imagine you are developing the core logic for an open-ledger payment system.
You need to verify a transaction before committing it. 
Read the following Rust snippet.


``` rust
struct Transaction {
	id: u32,
	amount: f64,
	sender: String,
}

fn verify_transaction(tx: Transaction) {
	println!("verifying transaction id {} from {}", tx.id, tx.sender)
}


fn main() {
	let current_tx = Transaction {
		id: 1,
		amount: 250.0
		sender: String::from("node_a")
	};

	verify_transaction(current_tx);

	println!("Ready to commit transaction id {}", current_tx.id);
}
```

If you try to compile this code, the Rust compiler (rustc) will throw a fatal error on the very last `println!` inside main.

I am not giving you the exact code. Knowing that the `&` operator creates an immutable reference, 
rewrite only the function signature for `fn verify_transaction(...)` and the function call inside `main()` to properly borrow the transaction.
