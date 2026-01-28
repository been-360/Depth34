mod config;
mod logs;
mod state;
mod tasks;


use crate::{config::ensure_config, logs::green, state::get_default_state, tasks::{bridge::Bridge, gamepad::gamepad_loop}};
use tokio::{sync::watch, task::spawn};

#[tokio::main]
async fn main() {
    let banner = include_str!("./assets/banner.txt");
    println!("{banner}");

    ensure_config().await;

    println!("{}", green("Config loaded"));

    let state = get_default_state().await;
    let (sender, reciever) = watch::channel(state);

    let gamepad_task = spawn(gamepad_loop(sender));
    println!("{}", green("Gamepad Loop started"));

    let bridge = Bridge::new("/dev/ttyACM0", 9600).await;
    println!("{}", green("Arduino connected"));


    loop {}
}
