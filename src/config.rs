pub struct Config {
    width: u32,
    height: u32,
    gravity: f64,
    accelerate: f64,
}

impl Config {
    pub fn new() -> Config {
        Config {
            width: 80,
            height: 24,
            gravity: 0.005,
            accelerate: 0.2,
        }
    }

    pub fn get_width(&self) -> u32 {
        self.width
    }

    pub fn get_height(&self) -> u32 {
        self.height
    }

    pub fn get_gravity(&self) -> f64 {
        self.gravity
    }

    pub fn get_acc(&self) -> f64 {
        self.accelerate
    }
}

use lazy_static::lazy_static;
lazy_static! {
    pub static ref CONFIG: Config = Config::new();
}
