use tokio::sync::watch;

use crate::state::State;

pub async fn pwm_loop(reciever: watch::Receiver<State>) {
    let mut rec = reciever.clone();

    loop {
        rec.changed().await.unwrap();
        let state = *rec.borrow();
    }
}

pub struct Pwm {
}

impl Pwm {
    pub async fn new() -> Result<Self, apigpio::Error> {
        // --------- \\

        // TODO: Replace unwrap with error handling `here

        Ok(Self {  })
    }

    pub fn setup(&self) {}
}
