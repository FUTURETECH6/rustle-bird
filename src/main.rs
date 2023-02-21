use bracket_lib::terminal::{main_loop, BError, BTermBuilder};
use rustle_bird::state::State;

fn main() -> BError {
    let context = BTermBuilder::vga(80, 24)
        .with_title("Rustle Bird")
        .with_vsync(true)
        .with_automatic_console_resize(true)
        .build()?;

    main_loop(context, State::new())
}
