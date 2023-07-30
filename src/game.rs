use opengl_graphics::GlGraphics;
use piston::input;
use rand::Rng;

use crate::{
    food::Food,
    snake::{Direction, Snake},
};

pub struct Game {
    pub gl_graphics: GlGraphics,
    pub rows: u32,
    pub columns: u32,
    pub snake: Snake,
    pub ate_food: bool,
    pub square_width: u32,
    pub food: Food,
    pub score: u32,
}

impl Game {
    pub fn render(&mut self, args: &input::RenderArgs) {
        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];

        self.gl_graphics
            .draw(args.viewport(), |_context, gl_graphics| {
                graphics::clear(GREEN, gl_graphics);
            });

        self.snake.render(args);
        self.food
            .render(&mut self.gl_graphics, args, self.square_width);
    }

    pub fn update(&mut self, _args: &input::UpdateArgs) -> bool {
        if !self.snake.update(self.ate_food, self.columns, self.rows) {
            return false;
        }
        if self.ate_food {
            self.score += 1;
            self.ate_food = false;
        }
        self.ate_food = self.food.ate_the_food(&self.snake);

        if self.ate_food {
            let mut random = rand::thread_rng();
            loop {
                let new_coord_x = random.gen_range(0..self.columns);
                let new_coord_y = random.gen_range(0..self.rows);

                if !self.snake.is_collide(new_coord_x, new_coord_y) {
                    self.food = Food {
                        x: new_coord_x,
                        y: new_coord_y,
                    };
                    break;
                }
            }
        }
        true
    }

    pub fn button_press(&mut self, button: &input::Button) {
        let last_direction = self.snake.dir.clone();
        self.snake.dir = match *button {
            input::Button::Keyboard(piston::Key::Up) if last_direction != Direction::Down => {
                Direction::Up
            }
            input::Button::Keyboard(piston::Key::Down) if last_direction != Direction::Up => {
                Direction::Down
            }
            input::Button::Keyboard(piston::Key::Right) if last_direction != Direction::Left => {
                Direction::Right
            }
            input::Button::Keyboard(piston::Key::Left) if last_direction != Direction::Right => {
                Direction::Left
            }
            _ => last_direction,
        }
    }
}
