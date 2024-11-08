## Archived Repository

**This repository was a gift from me to the Rust and open-source community. It is no longer actively maintained and has been archived. Feel free to fork and continue development on your own.**

## unix_sighandler
unix_sighandler is a lightweight and straightforward Rust library for handling Unix signals with minimal overhead. It provides a simple API to register and manage signal handlers, making it ideal for system programming and applications that require clean and efficient signal handling.

## Features
Register signal handlers for common Unix signals: SIGINT, SIGTERM, and SIGHUP.
Execute custom cleanup or shutdown logic when signals are received.
Designed for simplicity and performance, with minimal dependencies.

##Installation
Add unix_sighandler to your Cargo.toml:
```[dependencies]
unix_sighandler = "0.1.0"
```
## Usage
Here's a basic example demonstrating how to use unix_sighandler to handle SIGINT (triggered by Ctrl+C):

```rust
use unix_sighandler::{register_signal_handler, Signal};

fn main() {
    // Register a handler for SIGINT (Ctrl+C)
    register_signal_handler(Signal::SIGINT, || {
        println!("SIGINT received! Cleaning up...");
        std::process::exit(0); // Ensure the program exits
    }).expect("Failed to register SIGINT handler");

    println!("Running... Press Ctrl+C to exit.");

    // Simulate a long-running process
    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
```
## Examples
For more usage examples, check out the examples directory in the project repository.

Running the Example: You can run the simple_handler.rs example using the following command:

```cargo run --example simple_handler
```

## Signals Supported
SIGINT: Typically sent when you press Ctrl+C in the terminal.
SIGTERM: A termination signal that can be sent programmatically or by the system.
SIGHUP: Often used to indicate that a terminal has been closed or to signal configuration reload.

## How It Works
Registering Handlers: Use register_signal_handler to specify what actions to take when a signal is received.
Synchronous Execution: Handlers are executed immediately, allowing you to perform cleanup or shutdown tasks before exiting.

## License
This project is licensed under either of:

MIT License
Apache License, Version 2.0
You may choose which license to use.

## About
unix_sighandler was created to provide a simple, efficient, and developer-friendly solution for signal handling in Unix-like environments. It’s designed to be lightweight and easy to integrate into your Rust projects.

## Author
Ben Santora 
