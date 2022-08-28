use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread,
    time::Duration,
};

use components::{clock::Clock, cpu::Cpu, mem::Mem};
use status_bar::StatusBar;

mod components;
mod status_bar;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new status bar.
    let mut status_bar = StatusBar::new()?;

    // Handle running state.
    let running = Arc::new(AtomicBool::new(true));
    let running_sigint = running.clone();
    ctrlc::set_handler(move || {
        running_sigint.store(false, Ordering::SeqCst);
    })?;

    // Show the memory usage.
    status_bar.add_component(Mem::new());

    // Show the CPU usage.
    status_bar.add_component(Cpu::new());

    // Clock is a nice thing to peek at.
    status_bar.add_component(Clock);

    // Loop update each second.
    loop {
        if running.load(Ordering::SeqCst) == false {
            break;
        }

        // Update each status bar component.
        status_bar.tick()?;

        // Get the current status text from the bar.
        let status = status_bar.get_status()?;
        status_bar.set_status(status)?;

        // Sleep for 0.5 seconds.
        thread::sleep(Duration::from_millis(500));
    }

    // Clear the status on exit.
    status_bar.clear_status()?;

    Ok(())
}
