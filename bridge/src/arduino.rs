use serialport::SerialPort;
use std::time::Duration;

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
}
