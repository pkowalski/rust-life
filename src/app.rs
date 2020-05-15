use ggez;
use ggez::event;
use ggez::graphics;
use ggez::graphics::{Rect};
use ggez::nalgebra as na;
use crate::grid::*;

struct MainState {
    grid: Grid
}

impl MainState {
    fn new(grid: Grid) -> ggez::GameResult<MainState> {
        let s = MainState { grid };
        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut ggez::Context) -> ggez::GameResult {
        self.grid.run_rules();
        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());
        let width = 25.0;
        let length = 25.0;

        for (i, vec) in self.grid.cells.iter().enumerate() {
            for (j, val) in vec.iter().enumerate() {
                let mut color = graphics::WHITE;
                if *val == 1 {
                    color = graphics::BLACK;
                }
                let square = graphics::Mesh::new_rectangle(
                    ctx, graphics::DrawMode::fill(), Rect::new(width, length, width, length), color)?;
        
                graphics::draw(ctx, &square, (na::Point2::new((j as f32 * width) - width, (i as f32 * length) - length),))?;
            }
        }

        graphics::present(ctx)?;
        Ok(())
    }
}

pub fn main(grid: Grid) -> ggez::GameResult { 
    let cb = ggez::ContextBuilder::new("super_simple", "ggez");
    let (ctx, event_loop) = &mut cb.build()?;
    let state = &mut MainState::new(grid)?;
    event::run(ctx, event_loop, state)
}