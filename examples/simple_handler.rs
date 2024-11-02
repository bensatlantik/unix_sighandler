use unix_sighandler::{register_signal_handler, Signal};

fn main() {
    // Register a handler for SIGINT (Ctrl+C)
    register_signal_handler(Signal::SIGINT, || {
        println!("SIGINT received! Cleaning up...");
        // Perform cleanup actions here
        std::process::exit(0);
    }).expect("Failed to register SIGINT handler");

    println!("Running... Press Ctrl+C to exit.");

    // Simulate a long-running process
    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
