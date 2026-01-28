use std::time::Duration;
use serialport::SerialPort;

pub struct Bridge {
    pub port: Box<dyn SerialPort>,
}

impl Bridge {
    pub fn new(path: &str, baud_rate: u32) -> Self {
        let port = serialport::new(path, baud_rate)
            .timeout(Duration::from_millis(100))
            .open()
            .expect("Failed to connect to Arduino");
        Self { port }
    }

    pub fn servo(&mut self, pwm_val: &[u16]) {
        let mut packet = vec![0xAA];

        for &value in pwm_val {
            packet.extend(&value.to_le_bytes());
        }

        self.port.write_all(&packet);
        self.port.flush();
    }

    pub fn initialize(&mut self) {
        self.port.write_all(b"INIT\n").unwrap();
        self.port.flush();
    }
}

