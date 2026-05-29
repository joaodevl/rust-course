In the functional world, having 5 different threads fighting over a shared lock to mutate a central variable is an anti-pattern. It creates contention and bottlenecks. There is a famous concurrency proverb: "Do not communicate by sharing memory; instead, share memory by communicating."

Rust supports this out of the box using Channels (Message Passing).

Instead of wrapping the ledger in an Arc<Mutex> and letting threads fight to mutate it, we can spawn a transmitter for each thread. The threads simply send their $150.00 payment down the channel as a message. The main thread holds the receiver, collects all the messages, and mutates the ledger itself.

No locks. No shared state. No Arc. No Mutex.

Your Directive:
We are going to rewrite main using std::sync::mpsc::channel.

Look up how to create a channel: let (tx, rx) = mpsc::channel();

Inside the for loop, clone the transmitter (tx.clone()) and move it into the thread.

Inside the thread, use the transmitter to send the f64 value 150.0.

After the for loop, the main thread must iterate over the receiver (rx) and sum up the values into a standard, single-threaded HarborLedger. (Note: you must explicitly drop(tx) in the main thread before listening to rx, or the receiver will wait forever).

I am not giving you the syntax. Consult the mpsc documentation and build a lock-free, message-passing ledger. Show me the code.
