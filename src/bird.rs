use crate::config::Config;

pub struct Bird {
    height: i32,
    /// vertical speed
    speed: f64,
}

impl Bird {
    pub fn new() -> Bird {
        Bird {
            height: Config::new().get_height() as i32 / 2,
            speed: 0.,
        }
    }
}
