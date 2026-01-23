mod pigpio;
mod tasks;

use std::time::Duration;

use tokio::task::spawn_blocking;
use tasks::gamepad::gamepad_loop;

#[tokio::main]
async fn main() {
    pigpio::initialize();
        

    let gamepad_task = spawn_blocking(|| gamepad_loop());

    tokio::time::sleep(Duration::from_millis(5000)).await;

    gamepad_task.abort();
}
