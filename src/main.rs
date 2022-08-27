use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread,
    time::Duration,
};

use components::clock::Clock;
use status_bar::{Component, StatusBar};

mod components;
mod status_bar;

struct Dummy;

impl Component for Dummy {
    fn output(&self) -> Result<String, Box<dyn std::error::Error>> {
        Ok("My ass burns".into())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new status bar.
    let mut status_bar = StatusBar::new()?;

    // Handle running state.
    let running = Arc::new(AtomicBool::new(true));
    let running_sigint = running.clone();
    ctrlc::set_handler(move || {
        running_sigint.store(false, Ordering::SeqCst);
    })?;

    // Add a test components.

    // Clock is a nice thing to peek at.
    status_bar.add_component(Clock);

    // Prove that our system works.
    status_bar.add_component(Dummy);

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

        // Sleep for 1 second.
        // TODO: (SeedyROM) Is 1 second the right way to handle this?
        thread::sleep(Duration::from_millis(1000));
    }

    // Clear the status on exit.
    status_bar.clear_status()?;

    Ok(())
}
