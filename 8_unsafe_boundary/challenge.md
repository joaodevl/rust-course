You are responsible for writing the Rust side of this contract.

In C#, IntPtr is just a raw memory address. In Rust, you cannot just accept an IntPtr into a safe, idiomatic function. You must accept a raw C-style pointer, and then you must explicitly tell the Rust compiler to trust you by using the unsafe keyword to read that memory.

Furthermore, Rust's compiler will normally "mangle" function names to include type information, meaning C# won't be able to find CalculateVesselRisk when it looks for it in the compiled binary. You have to disable this.

Your Directive:
Based on the C# signature above, write the exact Rust function signature and the first few lines of the function body.

You must:

Use the correct macro to prevent the Rust compiler from mangling the function name.

Define the function to use the C ABI.

Define the parameters using Rust's raw pointer types (*const f64) to match the C# IntPtr and int.

Open an unsafe block and use std::slice::from_raw_parts to convert that raw pointer and length into a safe Rust slice (&[f64]) so the rest of your Rust code can iterate over it safely.

Show me the Rust code.
