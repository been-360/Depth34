mod arduino;

use gilrs::{Button, EventType, Gilrs};
use arduino::ArduinoBridge;

fn main() {
    let path = "/dev/ttyACM0";
    let baud = 115200;

    let mut gil = Gilrs::new().unwrap();
    let mut bridge = ArduinoBridge::new(path, baud);
    
    bridge.send_data("Hello Arduino!\n").unwrap();

    for (_id, gamepad) in gil.gamepads() {
        println!("{} is {:?}", gamepad.name(), gamepad.power_info());
    }


    loop {
        while let Some(e) = gil.next_event() {
            match e.event {
                EventType::ButtonPressed(button, _) => match button {
                    Button::South => println!("South"),
                    Button::North => println!("North"),
                    Button::East => println!("East"),
                    Button::West => println!("West"),
                    _ => {}
                },

                EventType::ButtonReleased(button, _) => match button {
                    Button::South => println!("South Release"),
                    Button::North => println!("North Release"),
                    Button::East => println!("East Release"),
                    Button::West => println!("West Release"),
                    _ => {}
                }

                _ => {}
            }
        }
    }
}

/*
loop {
    while let Some(Event { id, event, time, .. }) = gil.next_event() {
        active = Some(id);
    }

    if let Some(gamepad) = active.map(|id| gil.gamepad(id)) {

    }

}
*/
