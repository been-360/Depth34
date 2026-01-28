#![no_std]
#![no_main]

use arduino_hal::simple_pwm::*;
use panic_halt as _;

fn init_timer() {
    
}

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let timer0 = Timer0Pwm::new(dp.TC0, Prescaler::Prescale64);

    loop {
        arduino_hal::delay_ms(1000);
    }
}
