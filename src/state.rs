use crate::{bird::Bird, config::CONFIG};
use bracket_lib::terminal::{BTerm, GameState, Rect, VirtualKeyCode, CORNFLOWER_BLUE, WHITE};

enum ExecState {
    Ready,
    Playing,
    Pause,
    Dead,
}

pub struct State {
    mode: ExecState,
    bird: Bird,
}

impl State {
    pub fn new() -> State {
        State {
            mode: ExecState::Ready,
            bird: Bird::new(),
        }
    }

    fn restart(&mut self) {
        self.bird.init();
        self.mode = ExecState::Playing;
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

    fn play(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        self.bird.print(ctx);
        ctx.fill_region(
            Rect::with_size(0, CONFIG.get_height() as i32, CONFIG.get_width() as i32, 1),
            0,
            WHITE,
            CORNFLOWER_BLUE,
        );

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.mode = ExecState::Pause,
                _ => self.bird.flap(),
            }
        }

        if self.bird.get_height() < 0. {
            self.mode = ExecState::Dead;
        }

        self.bird.update();
    }

    fn pause(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(7, "Pause");
        self.bird.print(ctx);

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
            ExecState::Ready => self.menu(ctx),
            ExecState::Playing => self.play(ctx),
            ExecState::Pause => self.pause(ctx),
            ExecState::Dead => self.end(ctx),
        }
    }
}
