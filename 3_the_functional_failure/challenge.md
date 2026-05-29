In C#, if an operation fails—for example, if a container account doesn't have enough balance to pay the docking fee—you might throw an InvalidOperationException.

Rust does not have exceptions.

Given your current study into functional programming paradigms, you should already be familiar with the idea that failures should be treated as standard return values, not hidden control-flow jumps. Rust enforces this strictly.

Your new requirement: apply_docking_fee must check if tx.amount is at least 1.50. If it is, deduct it. If it is not, the function must fail gracefully without panicking and without mutating the data.

Per your strict instructions, I will not give you the answer, nor will I give you a hint that leads directly to the resolution.


code reference:

``` rust
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
			amount: 250f64
			sender: String::from("MAEU114442"),
		};
		
		apply_docking_fee(&mut current_tx);
		
		println!("Final amount for container tx {}: {}", current_tx.id, current_tx.amount);
	}

```
