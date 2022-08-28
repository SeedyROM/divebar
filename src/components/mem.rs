use sysinfo::{RefreshKind, System, SystemExt};

use crate::status_bar::Component;

pub struct Mem {
    system: System,
}

impl Mem {
    pub fn new() -> Self {
        Self {
            system: System::new_with_specifics(RefreshKind::new().with_memory()),
        }
    }
}

impl Component for Mem {
    fn tick(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.system.refresh_memory();

        Ok(())
    }

    fn output(&self) -> Result<String, Box<dyn std::error::Error>> {
        let used_memory = self.system.used_memory() as f64 / 1000.0 / 1000.0;
        let total_memory = self.system.total_memory() as f64 / 1000.0 / 1000.0;

        Ok(format!("MEM: {:.1}/{:.1}Gb", used_memory, total_memory).to_string())
    }
}
