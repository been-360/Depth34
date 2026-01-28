use std::time::Duration;

use serialport::SerialPort;

use crate::state::State;

pub struct Bridge {
    pub port: Box<dyn SerialPort>,
}

impl Bridge {
    pub async fn new(path: &str, baud_rate: u32) -> Self {
        let port = serialport::new(path, baud_rate)
            .timeout(Duration::from_millis(1000))
            .open()
            .expect("Connecting not work");
        Self {
            port
        }
    }

    pub async fn sync_state(&self, state: &State) {
        
    }
}
