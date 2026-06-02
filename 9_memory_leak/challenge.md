You need to send an array of processed data back to C#.

If you create a Vec<f64> inside Rust, get its raw pointer using .as_mut_ptr(), and return that pointer to C#, C# can read it. But the C# Garbage Collector does not have jurisdiction over the Rust memory allocator. C# cannot free() memory it didn't create. When the C# function ends, that array remains abandoned on the heap. Your server's RAM will explode in minutes.

Using your engineering intuition about memory ownership across language boundaries: If Rust allocates memory on the heap and hands a raw pointer to C#, what specific architectural mechanism must you build to allow C# to cleanly free that memory when it is done with it?
