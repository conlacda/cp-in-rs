use std::{process, thread, time::Duration};

#[cfg(feature = "local")]
pub fn timeout_secs(timeout_secs: u64) {
    thread::spawn(move || {
        thread::sleep(Duration::from_secs(timeout_secs));
        eprintln!("⏰ Timeout reached ({timeout_secs}s), exiting.");
        process::exit(1);
    });
}
