struct Transaction {
    id: u32,
    amount: f64,
    sender: String,
}

fn apply_docking_fee(tx: &mut Transaction) {
    tx.amount -= 1.50;
}

fn main() {
    let mut current_tx = Transaction {
        id: 1042,
        amount: 250.50,
        sender: String::from("MAEU8114442"),
    };

    apply_docking_fee(&mut current_tx);
    
    println!("Final amount for container tx {}: {}", current_tx.id, current_tx.amount);
}