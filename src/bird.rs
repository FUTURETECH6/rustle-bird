use crate::config::CONFIG;
use bracket_lib::terminal::BTerm;

pub(crate) struct Bird {
    /// positive is up side
    height: f64,

    /// vertical speed, positive is up side
    speed: f64,
}

impl Bird {
    pub fn new() -> Bird {
        Bird {
            height: CONFIG.height as f64 / 2.,
            speed: 0.,
        }
    }

    pub fn init(&mut self) {
        self.height = CONFIG.height as f64 / 2.;
        self.speed = 0.;
    }

    pub fn get_height(&self) -> f64 {
        self.height
    }

    /// update height and speed on every time tick,
    /// called periodically only by [State](crate::state::State)
    /// when the game is being playing
    pub fn update(&mut self) {
        self.height = self.height as f64 + self.speed;
        self.speed -= CONFIG.gravity;
    }

    /// update speed of bird, only called when player trigger
    pub fn flap(&mut self) {
        if self.speed < CONFIG.flap_speed {
            self.speed = CONFIG.flap_speed;
        }
    }

    pub fn print(&self, ctx: &mut BTerm) {
        ctx.print_centered((CONFIG.height as f64 - self.get_height()) as i32, "bird");
    }
}
