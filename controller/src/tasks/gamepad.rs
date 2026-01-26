use std::time::Duration;

use gilrs::{Axis, Button, EventType, Gilrs};
use tokio::sync::watch;

use crate::{
    logs::red,
    state::{L, Quad, R, Special, State},
};

pub async fn gamepad_loop(sender: watch::Sender<State>) {
    let mut axes = std::collections::HashMap::new();
    let mut gil = Gilrs::new().unwrap();
    const DEADZONE: f32 = 0.05;

    let mut state = State {
        l: L { lx: 0.0, ly: 0.0 },
        r: R { rx: 0.0, ry: 0.0 },
        dpad: Quad {
            up: false,
            down: false,
            left: false,
            right: false,
        },
        face: Quad {
            up: false,
            down: false,
            left: false,
            right: false,
        },
        special: Special {
            mode: false,
        }
    };


    for (_id, gamepad) in gil.gamepads() {
        println!("\x1b[32m{} is {:?}\x1b[0m", gamepad.name(), gamepad.power_info());
    }

    loop {
        while let Some(e) = gil.next_event() {
            match e.event {
                EventType::ButtonPressed(button, _) => {
                    match button {
                        Button::North => state.face.up = true,
                        Button::East => state.face.right = true,
                        Button::South => state.face.down = true,
                        Button::West => state.face.left = true,
                        Button::DPadUp => state.dpad.up = true,
                        Button::DPadDown => state.dpad.down = true,
                        Button::DPadLeft => state.dpad.left = true,
                        Button::DPadRight => state.dpad.right = true,
                        Button::Mode => state.special.mode = !state.special.mode,
                        _ => println!("Pressed unknown button: {:?}", button),
                    }
                }

                EventType::ButtonReleased(button, _) => {
                    match button {
                        Button::North => state.face.up = false,
                        Button::East => state.face.right = false,
                        Button::South => state.face.down = false,
                        Button::West => state.face.left = false,
                        Button::DPadUp => state.dpad.up = false,
                        Button::DPadDown => state.dpad.down = false,
                        Button::DPadLeft => state.dpad.left = false,
                        Button::DPadRight => state.dpad.right = false,
                        Button::Mode => {},
                        _ => println!("Released unknown button: {:?}", button),
                    }
                }
                EventType::AxisChanged(axis, val, _) => {
                    axes.insert(axis, val);

                    state.l.lx = *axes.get(&Axis::LeftStickX).unwrap_or(&0.0);
                    state.l.ly = *axes.get(&Axis::LeftStickY).unwrap_or(&0.0);
                    state.r.rx = *axes.get(&Axis::RightStickX).unwrap_or(&0.0);
                    state.r.ry = *axes.get(&Axis::RightStickY).unwrap_or(&0.0);

                    state.l.lx = if state.l.lx.abs() < DEADZONE { 0.0 } else { state.l.lx };
                    state.l.ly = if state.l.ly.abs() < DEADZONE { 0.0 } else { state.l.ly };
                    state.r.rx = if state.r.rx.abs() < DEADZONE { 0.0 } else { state.r.rx };
                    state.r.ry = if state.r.ry.abs() < DEADZONE { 0.0 } else { state.r.ry };
                }

                _ => {}
            }

            if sender.send(state).is_err() {
                eprintln!("{}", red("An error occured while updating the state"));
            }

        }

        tokio::time::sleep(Duration::from_millis(10)).await
    }
}
