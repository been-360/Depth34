use tokio::sync::watch;

use crate::logs::{green, red};
use crate::pigpio;
use crate::pigpio::servo;
use crate::state::State;

pub async fn pwm_loop(pwm: Pwm,reciever: watch::Receiver<State>) {
    let mut rec = reciever.clone();
    

    loop {
        rec.changed().await.unwrap();
        let state = *rec.borrow();
        pwm.esc(&state).await;

        println!("{state:?}")
    }
}

pub struct Pwm {
    rov_up1: u32,
    rov_up2: u32,
    rov_m1: u32,
    rov_m2: u32,
    rov_m3: u32,
    rov_m4: u32,
    stop: u32,
}

impl Pwm {
    pub async fn new() -> Self {
        Self {
            rov_up1: 17,
            rov_up2: 27,
            rov_m1: 5,
            rov_m2: 6,
            rov_m3: 19,
            rov_m4: 26,
            stop: 1500,
        }
    }

    pub async fn init(&self) {
        match pigpio::initialize() {
            Ok(_) => {
                println!("{}", green("PiGPIO has been initialized"))
            }
            Err(e) => {
                eprintln!(
                    "{}: {e}",
                    red("PiGPIO has failed to initialize with error code")
                )
            }
        }

        let pins = [
            self.rov_m1,
            self.rov_m2,
            self.rov_m3,
            self.rov_m4,
            self.rov_up1,
            self.rov_up2,
        ];
        for pin in pins {
            servo(pin, 1500);
        }
    }

    pub async fn esc(&self, state: &State) {
        servo(self.rov_up1, self.vertical_math(1).await);
    }

    async fn vertical_math(&self, motor: u8) -> u32 {
        let mut ms = self.stop;
        
        let pwm = self.clamp(ms).await;

        pwm
    }

    async fn clamp(&self, mut pwm: u32) -> u32 {
        if pwm < 500 {
            pwm = 500;
        }
        if pwm > 2500 {
            pwm = 2500;
        }

        pwm
    }
}
