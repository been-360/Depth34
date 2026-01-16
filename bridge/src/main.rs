mod arduino;

use gilrs::{Button, EventType, Gilrs};
use arduino::ArduinoBridge;

fn main() {
    // let path = "/dev/ttyACM0";
    let path = "/dev/pts/6";
    let baud = 115200;

    let mut gil = Gilrs::new().unwrap();
    let mut bridge = ArduinoBridge::new(path, baud);
    
    bridge.send_button(1).unwrap();

    for (_id, gamepad) in gil.gamepads() {
        println!("{} is {:?}", gamepad.name(), gamepad.power_info());
    }


    loop {
        while let Some(e) = gil.next_event() {            
            match e.event {    
                EventType::ButtonPressed(button, _) => match button {
                    Button::North => bridge.send_button(1).unwrap(),
                    Button::East => bridge.send_button(2).unwrap(),
                    Button::South => bridge.send_button(3).unwrap(),
                    Button::West => bridge.send_button(4).unwrap(),
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
