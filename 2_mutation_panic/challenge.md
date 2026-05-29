Your payment system now needs to process a docking fee for a port operation. This requires modifying the transaction data in memory before it gets committed to the ledger.

Read the following updated implementation:

``` rust
    struct Transaction {
        id: u32,
        amount: f64,
        sender: String,
    }

    fn apply_docking_fee(tx: &Transaction) {
        // Deduct a standard 1.50 network fee
        tx.amount -= 1.50;
    }

    fn main() {
        let current_tx = Transaction {
            id: 1042,
            amount: 250.50,
            sender: String::from("MAEU8114442"),
        };

        apply_docking_fee(&current_tx);
        
        println!("Final amount for container tx {}: {}", current_tx.id, current_tx.amount);
    }
```

If you compile this, the Rust compiler will violently reject it. You are attempting to mutate data through a reference that is strictly read-only, and your base variable current_tx is locked down as immutable by default.

Your next task is to rewrite three specific things to implement a valid mutable borrow:

The variable declaration in main.

The function signature for apply_docking_fee.

The function call inside main.

Provide the corrected lines of code.