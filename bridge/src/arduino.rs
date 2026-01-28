use serialport::SerialPort;
use std::{io, time::Duration};

pub struct ArduinoBridge {
    port: Box<dyn SerialPort>,
}


impl ArduinoBridge {
    pub fn new(path: &str, baud_rate: u32) -> Self {
        let port = serialport::new(path, baud_rate)
            .timeout(Duration::from_millis(1000))
            .open()
            .expect("Connecting not work");

        ArduinoBridge { port }
    }

    pub fn send_button(&mut self, button: u8) -> io::Result<()> {
        self.port.write_all(&button.to_be_bytes())?;
        self.port.flush()?;
        Ok(())
    }

    pub fn _send_joystick(&mut self, _axis: char, ) {

    }
}

/*

use std::time::Duration;

use serialport::SerialPort;
use tokio::sync::mpsc::Receiver;

use crate::state::State;

pub struct Bridge {
    pub port: Box<dyn SerialPort>,
}

impl Bridge {
    pub fn new(path: &str, baud_rate: u32) -> Self {
        let port = serialport::new(path, baud_rate)
            .timeout(Duration::from_millis(1000))
            .open()
            .expect("Failed to connect to Arduino");
        Self { port }
    }

    pub fn run(&self, mut rx: Receiver<State>) {
        while let Some(state) = rx.blocking_recv() {
            
        }
    }

    pub async fn sync_state(&self, state: &State) {}
}
*/
