use crate::status_bar::Component;

const DEFAULT_CLOCK_FORMAT: &str = "%b %d %I:%M:%S%p";

pub struct Clock;

impl Component for Clock {
    fn output(&self) -> Result<String, Box<dyn std::error::Error>> {
        let now = chrono::offset::Local::now();
        let clock_time = now.format(DEFAULT_CLOCK_FORMAT);
        Ok(format!("{}", clock_time.to_string()))
    }
}
