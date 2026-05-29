struct Transaction {
    id: u32,
    amount: f64,
    sender: String,
}

// change the method to expected a explicit shared reference of Transaction
fn verify_transaction(tx: &Transaction) {
    println!("Verifying transaction id {} from {}", tx.id, tx.sender);
}

fn main() {
    let curr_tx = Transaction {
        id: 1,
        amount: 150.0,
        sender: String::from("node_a")
    };

    // passing the value as & to allow read the value
    verify_transaction(&curr_tx);

    println!("Ready to commit transaction id {}", curr_tx.id);
}
