use crate::{bird::Bird, config::CONFIG, map::Map};
use bracket_lib::terminal::{BTerm, GameState, VirtualKeyCode};

enum ExecState {
    Ready,
    Playing,
    Pause,
    Dead,
}

pub struct State {
    mode: ExecState,
    bird: Bird,
    map: Map,
    score: u32,
}

impl State {
    pub fn new() -> State {
        State {
            mode: ExecState::Ready,
            bird: Bird::new(),
            map: Map::new(),
            score: 0,
        }
    }

    fn restart(&mut self) {
        self.mode = ExecState::Playing;
        self.bird.init();
        self.map.init();
        self.score = 0;
    }

    fn menu(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(7, "Rustle Bird");
        ctx.print_centered(15, "Press any button to start");
        ctx.print_centered(16, "Press Esc or Q to quit");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::Escape | VirtualKeyCode::Q => ctx.quitting = true,
                _ => self.restart(),
            }
        }
    }

    fn play(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        self.print_scene(ctx);

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.mode = ExecState::Pause,
                _ => self.bird.flap(),
            }
        }

        if self.bird.get_height() < 0.
            || self
                .map
                .collide(CONFIG.width as f64 / 2., self.bird.get_height() as f64)
        {
            self.mode = ExecState::Dead;
        }

        self.bird.update();
        if self.map.update() {
            self.score += 1;
        }
    }

    fn pause(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(7, "Pause");
        self.print_scene(ctx);

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.mode = ExecState::Playing,
                _ => {}
            }
        }
    }

    fn end(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(7, "You are dead");
        ctx.print_centered(8, format!("Score: {:?}", self.score));
        ctx.print_centered(15, "Press any button to restart");
        ctx.print_centered(16, "Press Esc or Q to quit");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::Escape | VirtualKeyCode::Q => ctx.quitting = true,
                _ => self.restart(),
            }
        }
    }

    fn print_scene(&self, ctx: &mut BTerm) {
        self.map.print(ctx);
        self.bird.print(ctx);
        ctx.print(CONFIG.width - 5, 1, self.score);
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        match self.mode {
            ExecState::Ready => self.menu(ctx),
            ExecState::Playing => self.play(ctx),
            ExecState::Pause => self.pause(ctx),
            ExecState::Dead => self.end(ctx),
        }
    }
}
