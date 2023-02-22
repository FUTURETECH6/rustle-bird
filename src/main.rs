use bracket_lib::terminal::{main_loop, BError, BTermBuilder};
use rustle_bird::{config::CONFIG, state::State};

fn main() -> BError {
    let context = BTermBuilder::vga(CONFIG.width, CONFIG.height)
        .with_title("Rustle Bird")
        .with_vsync(true)
        .with_automatic_console_resize(false)
        .with_fps_cap(60.)
        .build()?;

    main_loop(context, State::new())
}
