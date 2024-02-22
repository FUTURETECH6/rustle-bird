pub struct Config {
    /// game window width
    pub width: u32,

    /// game window height
    pub height: u32,

    /// acceleration of gravity that pulls bird down
    pub gravity: f64,

    /// init speed after flap
    pub flap_speed: f64,

    /// horizontal length of pipe
    pub pipe_width: u32,

    /// horizontal space between two pipes
    pub pipe_interval: u32,

    /// vertical space between pipes
    pub pipe_mouth_height: u32,

    /// speed of pipe
    pub pipe_speed: f64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            width: 80,
            height: 24,
            gravity: 0.005,
            #[allow(deprecated)]
            flap_speed: 0.2,
            pipe_width: 5,
            pipe_interval: 40,
            pipe_mouth_height: 10,
            pipe_speed: 0.3,
        }
    }
}

use lazy_static::lazy_static;
lazy_static! {
    pub static ref CONFIG: Config = Config::default();
}
