mod pigpio;
mod state;
mod tasks;
mod logs;

use crate::{state::{L, R, State}, tasks::pwm::pwm_loop};

use tokio::{spawn, sync::watch, task::spawn_blocking};
use tasks::gamepad::gamepad_loop;
use std::time::Duration;
use logs::{green, red};

#[tokio::main]
async fn main() {
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

    let l = L { lx: 0.0, ly: 0.0 };
    let r = R { rx: 0.0, ry: 0.0 };
    let state = State { l, r };
    let (sender, reciever) = watch::channel(state);

    let _gamepad_task = spawn_blocking(move || gamepad_loop(sender));
    let _pwm_task = spawn(pwm_loop(reciever));

    tokio::time::sleep(Duration::from_millis(5000)).await;


}