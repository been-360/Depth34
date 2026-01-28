mod config;
mod pigpio;
mod state;
mod tasks;
mod logs;

use crate::{
    config::{ensure_config}, logs::{green, yellow}, state::get_default_state, tasks::pwm::{Pwm, pwm_loop}
};

use std::io::{self, Read};
use tasks::gamepad::gamepad_loop;
use tokio::{spawn, sync::watch, task};

#[tokio::main]
async fn main() {
    let banner = include_str!("assets/banner.txt");
    println!("{banner}");

    ensure_config().await;

    let pwm = Pwm::new().await;
    pwm.init().await;

    let state = get_default_state().await;
    let (sender, reciever) = watch::channel(state);

    let gamepad_task = spawn(gamepad_loop(sender));
    let pwm_task = spawn(pwm_loop(pwm, reciever));

    let quit_task = task::spawn_blocking(|| {
        let stdin = io::stdin();
        for byte in stdin.bytes() {
            if let Ok(b'q') = byte {
                break;
            }
        }
    });

    quit_task.await.unwrap();

    handle_shutdown(gamepad_task, pwm_task);
}

fn handle_shutdown(
    gamepad_task: tokio::task::JoinHandle<()>,
    pwm_task: tokio::task::JoinHandle<()>,
) {
    println!("\n {}", yellow("Quitting ROV controller..."));

    pigpio::terminate();
    println!("\n {}", green("PiGPIO terminated"));

    gamepad_task.abort();
    println!("\n {}", green("Gamepad loop terminated"));

    pwm_task.abort();
    println!("\n {}", green("PWM bridge terminated"));

    println!("\n {}", yellow("ROV controller exit complete."));
}
