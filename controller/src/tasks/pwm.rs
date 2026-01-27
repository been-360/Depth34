use tokio::sync::watch;

use crate::config::{load_config};
use crate::logs::{cyan, green, red, yellow};
use crate::pigpio;
use crate::pigpio::{servo, get_servo_pulsewidth};
use crate::state::State;
use std::env;
use std::time::Duration;


pub struct Pwm {
    rov_up1: u32,
    rov_up2: u32,
    rov_m1: u32,
    rov_m2: u32,
    rov_m3: u32,
    rov_m4: u32,
}

struct HorizontalPWM {
    m1: u32,
    m2: u32,
    m3: u32,
    m4: u32,
}

struct ROVConstant {
    neutral: i32,
    normal: i32,
    mini: i32,
}

pub async fn pwm_loop(pwm: Pwm, reciever: watch::Receiver<State>) {
    let mut rec = reciever.clone();

    let config = load_config().await;

    let consts = ROVConstant {
        neutral: config.pwm.neutral,
        normal: config.pwm.normal,
        mini: config.pwm.mini,
    };

    let is_production = env::var("PRODUCTION").map(|v| v == "true").unwrap_or(false);

    loop {
        rec.changed().await.unwrap();
        let state = *rec.borrow();
        pwm.esc(&state, &consts).await;

        if !is_production {
            print_state(&state);
        }
    }
}

fn print_state(state: &State) {
    let left_j = format!("({:.2}, {:.2})", state.l.lx, state.l.ly);
    let right_j = format!("({:.2}, {:.2})", state.r.rx, state.r.ry);

    let dpad = format!(
        "Up={} Down={} Left={} Right={}",
        state.dpad.up, state.dpad.down, state.dpad.left, state.dpad.right
    );

    let face = format!(
        "Up={} Down={} Left={} Right={}",
        state.face.up, state.face.down, state.face.left, state.face.right
    );

    let mode = if state.special.mode { "ON" } else { "OFF" };

    let pwm = get_servo_pulsewidth(13).to_string();

    println!("{}", yellow("|-------------------------------|"));

    println!("Left Joystick:  {}", cyan(&left_j));
    println!("Right Joystick: {}", cyan(&right_j));
    println!("DPad:           {}", cyan(&dpad));
    println!("Face Buttons:   {}", cyan(&face));
    println!("Special Mode:   {}", cyan(&mode));
    println!("PWM:            {}", cyan(&pwm));

    println!("{}", yellow("|-------------------------------|"));
}

impl Pwm {
    pub async fn new() -> Self {
        let config = load_config().await;

        Self {
            rov_up1: config.pins.vertical_1,
            rov_up2: config.pins.vertical_2,
            rov_m1: config.pins.motor_1,
            rov_m2: config.pins.motor_2,
            rov_m3: config.pins.motor_3,
            rov_m4: config.pins.motor_4,
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

        tokio::time::sleep(Duration::from_millis(10000)).await
    }

    async fn esc(&self, state: &State, consts: &ROVConstant) {
        let vertical = self.vertical_math(state, consts).await;
        let horizontal = self.horizontal_math(state, consts).await;

        servo(self.rov_up1, vertical);
        servo(self.rov_up2, vertical);

        servo(self.rov_m1, horizontal.m1);
        servo(self.rov_m2, horizontal.m2);
        servo(self.rov_m3, horizontal.m3);
        servo(self.rov_m4, horizontal.m4);

        let up1 = get_servo_pulsewidth(self.rov_up1);

        println!("{up1}");
    }

    async fn horizontal_math(&self, state: &State, consts: &ROVConstant) -> HorizontalPWM {
        let ms = consts.neutral as f32;
        let apply_mini: bool = state.special.mode;
        let adj = if apply_mini {
            consts.mini as f32
        } else {
            consts.normal as f32
        };

        let ad1 = adj * state.l.lx + adj * state.l.ly;
        let ad2 = adj * state.l.lx - adj * state.l.ly;
        let ad3 = adj * state.l.lx + adj * state.l.ly;
        let ad4 = adj * state.l.lx - adj * state.l.ly;

        let m1 = self.clamp((ms + ad1) as i32).await as u32;
        let m2 = self.clamp((ms + ad2) as i32).await as u32;
        let m3 = self.clamp((ms + ad3) as i32).await as u32;
        let m4 = self.clamp((ms + ad4) as i32).await as u32;

        HorizontalPWM { m1, m2, m3, m4 }
    }

    async fn vertical_math(&self, state: &State, consts: &ROVConstant) -> u32 {
        let mut ms = consts.neutral;
        let apply_mini: bool = state.special.mode;
        let adj = if apply_mini {
            consts.mini
        } else {
            consts.normal
        };

        match (state.dpad.down, state.dpad.up) {
            (true, false) => ms -= adj,
            (false, true) => ms += adj,
            _ => {}
        }

        let pwm = self.clamp(ms).await;

        pwm as u32
    }

    async fn clamp(&self, mut pwm: i32) -> i32 {
        if pwm < 500 {
            pwm = 500;
        }
        if pwm > 2500 {
            pwm = 2500;
        }

        pwm
    }
}
