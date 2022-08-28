use sysinfo::{CpuExt, CpuRefreshKind, RefreshKind, System, SystemExt};

use crate::status_bar::Component;

pub struct Cpu {
    system: System,
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            system: System::new_with_specifics(
                RefreshKind::new().with_cpu(CpuRefreshKind::everything()),
            ),
        }
    }
}

impl Component for Cpu {
    fn tick(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.system.refresh_cpu();

        Ok(())
    }

    fn output(&self) -> Result<String, Box<dyn std::error::Error>> {
        let usage = self.system.global_cpu_info().cpu_usage();

        Ok(format!("CPU: {:.2}%", usage).to_string())
    }
}
