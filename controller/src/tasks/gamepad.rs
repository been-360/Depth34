use std::time::Duration;

use tokio::sync::watch;
use gilrs::{Axis, Button, EventType, Gilrs};

use crate::{logs::red, state::{L, R, State}};

pub fn gamepad_loop(sender: watch::Sender<State>) {
    let mut axes = std::collections::HashMap::new();
    let mut gil = Gilrs::new().unwrap();

    for (_id, gamepad) in gil.gamepads() {
        println!("{} is {:?}", gamepad.name(), gamepad.power_info());
    }

    loop {
        while let Some(e) = gil.next_event() {
            match e.event {
                EventType::ButtonPressed(button, _) => match button {
                    Button::North => println!("Press N"),
                    Button::East => println!("Press E"),
                    Button::South => println!("Press S"),
                    Button::West => println!("Press W"),
                    Button::DPadDown => println!("Press DPad Down"),
                    Button::DPadLeft => println!("Press DPad Left"),
                    Button::DPadUp => println!("Press DPad Up"),
                    Button::DPadRight => println!("Press DPad Right"),
                    _ => {}
                },
                EventType::ButtonReleased(button, _) => match button {
                    Button::North => println!("Release N"),
                    Button::East => println!("Release E"),
                    Button::South => println!("Release S"),
                    Button::West => println!("Release W"),
                    Button::DPadDown => println!("Release DPad Down"),
                    Button::DPadLeft => println!("Release DPad Left"),
                    Button::DPadUp => println!("Release DPad Up"),
                    Button::DPadRight => println!("Release DPad Right"),
                    _ => {}
                },
                EventType::AxisChanged(axis, val, _) => {
                    axes.insert(axis, val);

                    let lx = *axes.get(&Axis::LeftStickX).unwrap_or(&0.0);
                    let ly = *axes.get(&Axis::LeftStickY).unwrap_or(&0.0);
                    let rx = *axes.get(&Axis::RightStickX).unwrap_or(&0.0);
                    let ry = *axes.get(&Axis::RightStickY).unwrap_or(&0.0);

                    let state = State {
                        l: L { lx, ly },
                        r: R { rx, ry },
                    };

                    if sender.send(state).is_err() {
                        eprintln!("{}", red("An error occured while updating the state"));
                    }
                }

                _ => {}
            }
        }
        
        std::thread::sleep(Duration::from_millis(10));
    }
}
