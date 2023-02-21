pub struct Config {
    width: u32,
    height: u32,
}

impl Config {
    pub fn new() -> Config {
        Config { width: 80, height: 24 }
    }

    pub fn get_width(self) -> u32 {
        self.width
    }

    pub fn get_height(self) -> u32 {
        self.height
    }
}
