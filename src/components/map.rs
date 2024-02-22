use crate::{components::pipe::Pipe, config::CONFIG};
use bracket_lib::terminal::BTerm;
use std::collections::LinkedList;

pub(crate) struct Map {
    pipes: LinkedList<Pipe>,
}

impl Map {
    pub fn new() -> Map {
        let mut map = Map {
            pipes: LinkedList::new(),
        };
        map.pipes.push_back(Pipe::new());
        map
    }

    /// remove all pipes
    pub fn init(&mut self) {
        self.pipes.clear();
    }

    /// update all pipes on tick
    ///
    /// return true if bird has passed a pipe during this update
    pub fn update(&mut self) -> bool {
        let mut has_passed = false;

        // add new
        if self.pipes.is_empty() || self.pipes.back().unwrap().x < (CONFIG.width - CONFIG.pipe_interval) as f64 {
            self.pipes.push_back(Pipe::new());
        }

        let old_passed_count = self.pipes.iter().filter(|&p| p.passed).count();
        for i in self.pipes.iter_mut() {
            i.update();
        }
        let new_passed_count = self.pipes.iter().filter(|&p| p.passed).count();

        if new_passed_count != old_passed_count {
            has_passed = true;
        }

        // remove old
        while !self.pipes.is_empty() && (self.pipes.front().unwrap().x + CONFIG.pipe_width as f64) < 0. {
            self.pipes.pop_front();
        }

        has_passed
    }

    /// check if object at (x, y) is collided with all pipe
    pub fn collide(&self, x: f64, y: f64) -> bool {
        for i in self.pipes.iter() {
            if i.collide(x, y) {
                return true;
            }
        }
        false
    }

    /// print all pipes
    pub fn print(&self, ctx: &mut BTerm) {
        for i in self.pipes.iter() {
            i.print(ctx);
        }
    }
}
