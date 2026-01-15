use serialport::SerialPort;
use std::time::Duration;

struct ArduinoBridge {
    
}

impl ArduinoBridge {
    fn connect(&mut self, path: &str, baud_rate: u32) {
        let port = serialport::new(path, baud_rate)
        .timeout(Duration::from_millis(1000))
        .open()
        .expect("Connect not work");
                
    }

    
}
