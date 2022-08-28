use snafu::Snafu;
use xcb::{x, ConnError};

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Failed to connect to X server"))]
    ConnectionError { source: ConnError },
    #[snafu(display("Screen {screen_num} not found"))]
    ScreenNotFound { screen_num: i32 },
}

pub struct StatusBar {
    conn: xcb::Connection,
    screen: xcb::x::ScreenBuf,
    components: Vec<Box<dyn Component>>,
}

impl StatusBar {
    pub fn new() -> Result<Self, Error> {
        let (conn, screen_num) =
            xcb::Connection::connect(None).map_err(|err| Error::ConnectionError { source: err })?;
        let setup = conn.get_setup().to_owned();
        let screen = setup
            .roots()
            .nth(screen_num as usize)
            .ok_or(Error::ScreenNotFound {
                screen_num: screen_num,
            })?
            .to_owned();

        Ok(Self {
            conn,
            screen,
            components: Vec::new(),
        })
    }

    pub fn set_status(&self, value: impl Into<String>) -> Result<(), Box<dyn std::error::Error>> {
        let string: String = value.into();

        self.conn.send_and_check_request(&x::ChangeProperty {
            mode: x::PropMode::Replace,
            window: self.screen.root(),
            property: x::ATOM_WM_NAME,
            r#type: x::ATOM_STRING,
            data: string.as_bytes(),
        })?;
        Ok(())
    }

    pub fn clear_status(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.conn.send_and_check_request(&x::ChangeProperty {
            mode: x::PropMode::Replace,
            window: self.screen.root(),
            property: x::ATOM_WM_NAME,
            r#type: x::ATOM_STRING,
            data: &[] as &[u8],
        })?;
        Ok(())
    }

    pub fn add_component<T: Component + 'static>(&mut self, component: T) {
        self.components.push(Box::new(component));
    }

    pub fn tick(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        for component in self.components.iter_mut() {
            component.tick()?;
        }

        Ok(())
    }

    pub fn get_status(&self) -> Result<String, Box<dyn std::error::Error>> {
        let mut output = "".to_string();
        let mut index = 0;

        for component in self.components.iter() {
            output.push('[');
            // TODO: (SeedyROM) I guess N/A default works for now, this should probably be a constant.
            output.push_str(component.output().unwrap_or("N/A".to_string()).as_str());
            output.push(']');
            if self.components.len() > 1 && index < self.components.len() - 1 {
                output.push(' ');
            }
            index += 1;
        }

        Ok(output)
    }
}

pub trait Component {
    fn tick(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    fn output(&self) -> Result<String, Box<dyn std::error::Error>>;
}
