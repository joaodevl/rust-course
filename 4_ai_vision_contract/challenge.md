The port AI vision system is online. It identifies different vessel types, and your backend needs to process their docking fees uniformly.

To remind you of the strict requirements:

Define a trait named Vessel with a single required method: calculate_fee(&self) -> f64.

Define a struct ContainerShip (with a teu_capacity: u32 field) and implement the Vessel trait for it. (The fee is 5.00 per TEU).

Define a struct OilTanker (with a barrels: u32 field) and implement the Vessel trait for it. (The fee is 0.10 per barrel).

Write a function called process_vessel that accepts any struct that implements the Vessel trait, calls its calculate_fee method, and prints the result.

Write the code. How are you going to construct the signature for process_vessel to ensure the compiler enforces this contract?
