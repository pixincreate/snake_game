use crate::snake::Snake;
use opengl_graphics::GlGraphics;
use piston::input;

pub struct Food {
    pub x: u32,
    pub y: u32,
}

impl Food {
    pub(crate) fn ate_the_food(&mut self, snake: &Snake) -> bool {
        let face = snake.snake_body.front().unwrap();
        face.0 == self.x && face.1 == self.y
    }

    pub(crate) fn render(
        &mut self,
        gl_graphics: &mut GlGraphics,
        args: &input::RenderArgs,
        width: u32,
    ) {
        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

        let x = self.x * width;
        let y = self.y * width;

        let square = graphics::rectangle::square(x as f64, y as f64, width as f64);

        gl_graphics.draw(args.viewport(), |context, gl_graphics| {
            let transform = context.transform;
            graphics::rectangle(WHITE, square, transform, gl_graphics)
        });
    }
}
