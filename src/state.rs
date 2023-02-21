use bracket_lib::terminal::{BTerm, GameState, VirtualKeyCode};

enum GameMode {
    Ready,
    Playing,
    Dead,
}

pub struct State {
    mode: GameMode,
}

impl State {
    pub fn new() -> State {
        State { mode: GameMode::Ready }
    }

    fn restart(&mut self) {
        self.mode = GameMode::Playing;
    }

    fn menu(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(7, "Rustle Bird");
        ctx.print_centered(15, "Press any buttons to start");
        ctx.print_centered(16, "Press Esc or Q to quit");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::Escape | VirtualKeyCode::Q => ctx.quitting = true,
                _ => self.restart(),
            }
        }
    }

    fn play(&mut self, _ctx: &mut BTerm) {
        self.mode = GameMode::Dead;
    }

    fn end(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(7, "You are dead");
        ctx.print_centered(15, "Press any buttons to restart");
        ctx.print_centered(16, "Press Esc or Q to quit");
        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::Escape | VirtualKeyCode::Q => ctx.quitting = true,
                _ => self.restart(),
            }
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        match self.mode {
            GameMode::Ready => self.menu(ctx),
            GameMode::Playing => self.play(ctx),
            GameMode::Dead => self.end(ctx),
        }
    }
}
