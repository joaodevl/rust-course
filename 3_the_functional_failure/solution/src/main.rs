struct Transaction {
    id: u32,
    amount: f64,
    sender: String,
}


const CHAR_INDEX_MAP: &'static str = "0123456789A?BCDEFGHIJK?LMNOPQRSTU?VWXYZ";

fn get_container_index(chr: char) -> u32 {
    CHAR_INDEX_MAP.find(chr)
    .map(|v| { v as u32 })
    .unwrap_or_else(|| { 1 } )
}


fn is_valid_container_sender(sender: &String) -> Result<bool, &'static str> {
    let mut f_d = '\0';
    let mut d: u8 = (sender.chars()
        .enumerate()
        .map(|(i, c)| {
            if i == sender.len() -1 { f_d = c; }

            get_container_index(c) * (2_u32.pow(i as u32))
        })
        .sum::<u32>() % 11) as u8;

    d = if d == 10 { 0 } else { d };
    
    f_d.to_digit(10)
        .map_or_else(
            || { Err("Fail to convert final char in a number") },
            |n| { Ok(n == d as u32) }
        )
}

fn apply_docking_fee(tx: &mut Transaction) -> Result<(), &'static str> {
    if tx.amount < 1.5 { return Err("Fail to apply fee: Insufficient amount"); }

    tx.amount -= 1.5;
    Ok(())
}

fn main() {
    let mut current_tx = Transaction {
        id: 1,
        amount: 250.0,
        sender: String::from("MAEU1114442")
    };

    is_valid_container_sender(&current_tx.sender)
        .map_or_else(
            |e| { println!("Fail to validate container: {e}"); },
            |r| {
                if !r { println!("Invalid container.") }
                else {    
                    println!("{}", apply_docking_fee(&mut current_tx)
                        .map(|_| { format!("Final amount for container tx {}: {}", current_tx.id, current_tx.amount) })
                        .unwrap_or_else(|err| { String::from(err) } )
                    );
                }
            }
        );
}
