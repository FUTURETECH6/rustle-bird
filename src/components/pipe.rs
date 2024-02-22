use crate::config::CONFIG;
use bracket_lib::{
    random::RandomNumberGenerator,
    terminal::{BTerm, Rect, WHITE},
};

pub(crate) struct Pipe {
    /// x of left-down side of the mouth
    pub x: f64,

    /// y of left-down side of the mouth
    pub y: i32,

    /// whether the bird has passed this pip
    pub passed: bool,
}

impl Pipe {
    pub fn new() -> Pipe {
        Pipe {
            x: CONFIG.width as f64,
            y: (RandomNumberGenerator::new().rand::<u32>() % (CONFIG.height - CONFIG.pipe_mouth_height)) as i32,
            passed: false,
        }
    }

    /// update x on tick
    pub fn update(&mut self) {
        self.x -= CONFIG.pipe_speed;
        if (self.x + CONFIG.pipe_width as f64) < CONFIG.width as f64 / 2. {
            self.passed = true;
        }
    }

    /// check if object at (x, y) is collided with the pipe
    pub fn collide(&self, x: f64, y: f64) -> bool {
        self.x <= x
            && x <= self.x + CONFIG.pipe_width as f64
            && (y < self.y as f64 || y > (self.y + CONFIG.pipe_mouth_height as i32) as f64)
    }

    /// print a pipe
    pub fn print(&self, ctx: &mut BTerm) {
        // up
        ctx.fill_region(
            Rect::with_size(
                self.x as i32,
                0,
                CONFIG.pipe_width as i32,
                CONFIG.height as i32 - (self.y + CONFIG.pipe_mouth_height as i32),
            ),
            0,
            WHITE,
            WHITE,
        );

        // down
        ctx.fill_region(
            Rect::with_size(
                self.x as i32,
                CONFIG.height as i32 - self.y,
                CONFIG.pipe_width as i32,
                self.y,
            ),
            0,
            WHITE,
            WHITE,
        );
    }
}
