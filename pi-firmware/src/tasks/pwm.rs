use tokio::sync::watch;

use crate::bridge::Bridge;
use crate::config::load_config;
use crate::logs::{cyan, yellow};
use crate::state::State;
use std::env;
use std::time::Duration;

pub struct Pwm {
    bridge: Bridge,
}

struct HorizontalPWM {
    m1: u16,
    m2: u16,
    m3: u16,
    m4: u16,
}

struct ROVConstant {
    neutral: i32,
    normal: i32,
    mini: i32,
}

pub async fn pwm_loop(mut pwm: Pwm, reciever: watch::Receiver<State>) {
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

    println!("{}", yellow("|-------------------------------|"));

    println!("Left Joystick:  {}", cyan(&left_j));
    println!("Right Joystick: {}", cyan(&right_j));
    println!("DPad:           {}", cyan(&dpad));
    println!("Face Buttons:   {}", cyan(&face));
    println!("Special Mode:   {}", cyan(&mode));

    println!("{}", yellow("|-------------------------------|"));
}

impl Pwm {
    pub async fn new() -> Self {
        let config = load_config().await;
        let bridge = Bridge::new(&config.bridge.path, 115200);

        Self {
            bridge,
        }
    }

    pub async fn init(&mut self) {
        let config = load_config().await;
        let neutral = config.pwm.neutral as u16;

        let pwm_vals: [u16; 6] = [neutral, neutral, neutral, neutral, neutral, neutral];

        self.bridge.servo(&pwm_vals);

        tokio::time::sleep(Duration::from_millis(5000)).await
    }

    async fn esc(&mut self, state: &State, consts: &ROVConstant) {
        let vertical = self.vertical_math(state, consts).await;
        let horizontal = self.horizontal_math(state, consts).await;

        let pwm_vals: [u16; 6] = [
            horizontal.m1,
            horizontal.m2,
            horizontal.m3,
            horizontal.m4,
            vertical,
            vertical,
        ];

        self.bridge.servo(&pwm_vals);
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

        let m1 = self.clamp((ms + ad1) as i32).await as u16;
        let m2 = self.clamp((ms + ad2) as i32).await as u16;
        let m3 = self.clamp((ms + ad3) as i32).await as u16;
        let m4 = self.clamp((ms + ad4) as i32).await as u16;

        HorizontalPWM { m1, m2, m3, m4 }
    }

    async fn vertical_math(&self, state: &State, consts: &ROVConstant) -> u16 {
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

        pwm as u16
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
