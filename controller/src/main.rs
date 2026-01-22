mod bridge;

use gilrs::{Button, EventType, Gilrs, Axis};
use bridge::Bridge;

fn main() {
    let mut gil = Gilrs::new().unwrap();
    let bridge = Bridge::new();

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
                    _ => {}
                }

                EventType::AxisChanged(axis, _, _) => match axis {
                    Axis::LeftStickX => {}
                    Axis::LeftStickY => {}
                    Axis::RightStickX => {}
                    Axis::RightStickY => {}
                    _ => {}
                }

                EventType::ButtonReleased(button, _) => match button {
                    Button::North => println!("Release N"),
                    Button::East => println!("Release E"),
                    Button::South => println!("Release S"),
                    Button::West => println!("Release W"),
                    _ => {}
                }
                _ => {}
            }
        }
    }
}
