unsafe extern "C" {
    fn gpioInitialise() -> i32;
    fn gpioTerminate() -> i32;
    fn gpioServo(gpio: u32, pulsewidth: u32) -> i32;
    fn _gpioPWM(gpio: u32, dutycycle: u32) -> i32;
    fn gpioGetServoPulsewidth(gpio: u32) -> u32;
}

pub fn initialize() -> Result<(), i32> {
    let result = unsafe { gpioInitialise() };
    if result < 0 { Err(result) } else { Ok(()) }
}

pub fn terminate() {
    unsafe {
        gpioTerminate();
    }
}

pub fn servo(pin: u32, pulse: u32) {
    unsafe { gpioServo(pin, pulse) };
}

pub fn _pwm(pin: u32, duty: u32) {
    unsafe { _gpioPWM(pin, duty) };
}

pub fn get_servo_pulsewidth(pin: u32) -> u32 {
    unsafe { gpioGetServoPulsewidth(pin) }
}
