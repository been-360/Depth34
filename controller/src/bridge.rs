use rppal::pwm::{Pwm, Channel, Polarity};

pub struct Bridge {
    pub pwm: Pwm,
}

impl Bridge {
    pub fn new() -> Self {
        let pwm = Pwm::new(Channel::Pwm0).unwrap();
        

        // --------- \\
        
        // TODO: Replace unwrap with error handling `here

        Self { pwm }
    }

    pub fn setup(
        &self
    ) {
        
    }

}