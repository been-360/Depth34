mod logs;
mod pigpio;
mod state;
mod tasks;

use crate::{
    state::{L, Quad, R, Special, State},
    tasks::pwm::{Pwm, pwm_loop},
};

use logs::{green, red};
use std::time::Duration;
use tasks::gamepad::gamepad_loop;
use tokio::{spawn, sync::watch, task::spawn_blocking};

#[tokio::main]
async fn main() {
    let pwm = Pwm::new().await;
    pwm.init().await;

    let l = L { lx: 0.0, ly: 0.0 };
    let r = R { rx: 0.0, ry: 0.0 };
    let spec_default = Special {
        mode: false,
    };
    let quad_default = Quad {
        up: false,
        down: false,
        left: false,
        right: false,
    };

    let state = State {
        l,
        r,
        dpad: quad_default,
        face: quad_default,
        special: spec_default,
    };
    let (sender, reciever) = watch::channel(state);

    let _gamepad_task = spawn(gamepad_loop(sender));
    let _pwm_task = spawn(pwm_loop(pwm, reciever));

    loop {
        tokio::time::sleep(Duration::from_secs(60)).await;
    }
}
