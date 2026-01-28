use std::fs;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub title: String,
    pub pins: PinConfig,
    pub pwm: PWMConfig,
    pub gamepad: GamepadConfig,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GamepadConfig {
    pub joystick_deadzone: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PinConfig {
    pub vertical_1: u32,
    pub vertical_2: u32,
    pub motor_1: u32,
    pub motor_2: u32,
    pub motor_3: u32,
    pub motor_4: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PWMConfig {
    pub neutral: i32,
    pub normal: i32,
    pub mini: i32,
}

pub async fn default_conf() -> Config {
    let title = "ROV Config".to_string();
    let default_pins = PinConfig {
        vertical_1: 13,
        vertical_2: 27,
        motor_1: 5,
        motor_2: 6,
        motor_3: 19,
        motor_4: 26,
    };
    let default_pwm = PWMConfig {
        neutral: 1500,
        normal: 200,
        mini: 50,
    };
    let default_gamepad = GamepadConfig {
        joystick_deadzone: 0.05,
    };

    Config {
        title: title,
        pwm: default_pwm,
        pins: default_pins,
        gamepad: default_gamepad,
    }
}

pub async fn load_config() -> Config {
    let config = match fs::read_to_string("config.toml") {
        Ok(toml_str) => {
            let conf: Config = toml::from_str(&toml_str).unwrap();
            conf
        }
        Err(e) => {
            eprintln!("{e}");
            default_conf().await
        }
    };

    config
}

pub async fn save_config(config: Config) {
    let toml_conf = toml::to_string_pretty(&config).unwrap();
    fs::write("config.toml", toml_conf).unwrap();
}

pub async fn ensure_config() {
    if fs::exists("config.toml").unwrap() {
        return;
    } else {
        let _ = save_config(default_conf().await).await;
    }
}
