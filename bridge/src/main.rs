mod arduino;

use gilrs::{Button, Event, Gilrs};

fn main() {
    let mut gil = Gilrs::new().unwrap();
    let mut active = None;

    for (_id, gamepad) in gil.gamepads() {
        println!("{} is {:?}", gamepad.name(), gamepad.power_info());
    }

    loop {
        while let Some(Event {
            id, event, time, ..
        }) = gil.next_event()
        {
            println!("{:?} New event from {}: {:?}", time, id, event);
            active = Some(id);
        }

        if let Some(gamepad) = active.map(|id| gil.gamepad(id)) {
            if gamepad.is_pressed(Button::South) {
                println!("Button South is pressed");
            }
        if gamepad.is_pressed(Button::North) {
                println!("Button North is pressed");
            }
        }
    }
}
