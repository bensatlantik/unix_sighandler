use libc::{sigaction, sigemptyset, sighandler_t, SIGINT, SIGTERM, SIGHUP};
use std::sync::{Mutex, Once};
use std::collections::HashMap;

// Define the available signals as an enum
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Signal {
    SIGINT,
    SIGTERM,
    SIGHUP,
}

// Static map to store signal handlers
static mut HANDLERS: Option<Mutex<HashMap<Signal, fn()>>> = None;
static INIT: Once = Once::new();

// Function to convert Signal to libc's signal constants
fn signal_to_libc(signal: Signal) -> i32 {
    match signal {
        Signal::SIGINT => SIGINT,
        Signal::SIGTERM => SIGTERM,
        Signal::SIGHUP => SIGHUP,
    }
}

// Register a handler for a given signal
pub fn register_signal_handler(signal: Signal, handler: fn()) -> Result<(), String> {
    // Initialize the HANDLERS map once
    INIT.call_once(|| {
        unsafe {
            HANDLERS = Some(Mutex::new(HashMap::new()));
        }
    });

    // Store the handler in the HANDLERS map
    unsafe {
        if let Some(ref handlers) = HANDLERS {
            let mut handlers = handlers.lock().map_err(|_| "Failed to lock handlers map")?;
            handlers.insert(signal, handler);
        }
    }

    // Setup the signal handler
    unsafe {
        let mut action: sigaction = std::mem::zeroed();
        action.sa_sigaction = signal_handler as sighandler_t;
        sigemptyset(&mut action.sa_mask);
        action.sa_flags = 0;

        if sigaction(signal_to_libc(signal), &action, std::ptr::null_mut()) != 0 {
            return Err(format!("Failed to register signal handler for {:?}", signal));
        }
    }

    Ok(())
}

// Signal handler function
extern "C" fn signal_handler(signum: i32) {
    let signal = match signum {
        SIGINT => Signal::SIGINT,
        SIGTERM => Signal::SIGTERM,
        SIGHUP => Signal::SIGHUP,
        _ => return, // Unknown signal, do nothing
    };

    // Call the registered handler, if it exists
    unsafe {
        if let Some(ref handlers) = HANDLERS {
            if let Ok(handlers) = handlers.lock() {
                if let Some(handler) = handlers.get(&signal) {
                    handler();
                }
            }
        }
    }
}
