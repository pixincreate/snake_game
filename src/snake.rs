use std::collections::LinkedList;
use opengl_graphics;
use piston::input;

#[derive(Clone, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Right,
    Left,
}

#[derive(Clone)]
pub struct SnakeBody(pub u32, pub u32);

pub struct Snake {
    pub gl_graphics: opengl_graphics::GlGraphics,
    pub snake_body: LinkedList<SnakeBody>,
    pub width: u32,
    pub dir: Direction,
}

impl Snake {
    pub fn render(&mut self, args: &input::RenderArgs) {
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let squares: Vec<graphics::types::Rectangle> = self
            .snake_body
            .iter()
            .map(|body| SnakeBody(body.0 * self.width, body.1 * self.width))
            .map(|body| {
                graphics::rectangle::square(body.0 as f64, body.1 as f64, self.width as f64)
            })
            .collect();

        self.gl_graphics
            .draw(args.viewport(), |context, gl_graphics| {
                let transform = context.transform;
                squares
                    .into_iter()
                    .for_each(|square| graphics::rectangle(RED, square, transform, gl_graphics));
            })
    }

    pub fn update(&mut self, ate_food: bool, columns: u32, rows: u32) -> bool {
        let mut next_step: SnakeBody = (*self.snake_body.front().expect("No face!")).clone();

        if (self.dir == Direction::Up && next_step.1 == 0)
            || (self.dir == Direction::Down && next_step.1 == rows - 1)
            || (self.dir == Direction::Left && next_step.0 == 0)
            || (self.dir == Direction::Right && next_step.0 == columns - 1)
        {
            return false;
        }

        match self.dir {
            Direction::Up => next_step.1 -= 1,
            Direction::Down => next_step.1 += 1,
            Direction::Right => next_step.0 += 1,
            Direction::Left => next_step.0 -= 1,
        }

        if !ate_food {
            self.snake_body.pop_back();
        }

        if self.is_collide(next_step.0, next_step.1) {
            return false;
        }
        self.snake_body.push_front(next_step);
        true
    }

    pub fn is_collide(&self, x: u32, y: u32) -> bool {
        self.snake_body
            .iter()
            .any(|body| x == body.0 && y == body.1)
    }
}
