use bracket_lib::terminal::{main_loop, BError, BTermBuilder};
use rustle_bird::{config::Config, state::State};

fn main() -> BError {
    let context = BTermBuilder::vga(Config::new().get_width(), Config::new().get_height())
        .with_title("Rustle Bird")
        .with_vsync(true)
        .with_automatic_console_resize(true)
        .build()?;

    main_loop(context, State::new())
}
