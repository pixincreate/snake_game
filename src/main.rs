// Snake Xenzia
//
// Author: Pa1NarK
use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::{input, window, ButtonEvent, RenderEvent, UpdateEvent};
use snake_game::{food, game, snake};
use std::{collections::LinkedList, iter::FromIterator};

fn main() {
    let opengl = OpenGL::V4_5;

    const COLUMNS: u32 = 50;
    const ROWS: u32 = 50;
    const SQUARE_WIDTH: u32 = 15;

    const WIDTH: u32 = COLUMNS * SQUARE_WIDTH;
    const HEIGHT: u32 = ROWS * SQUARE_WIDTH;
    const TITLE: &str = "SNAKE XENZIA";

    let mut window: GlutinWindow = window::WindowSettings::new(TITLE, [WIDTH, HEIGHT])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap_or_else(|err| {
            eprintln!("Error while creating the window: {}", err);
            std::process::exit(1);
        });

    let mut game = game::Game {
        gl_graphics: GlGraphics::new(opengl),
        rows: ROWS,
        columns: COLUMNS,
        square_width: SQUARE_WIDTH,
        ate_food: false,
        food: food::Food { x: 1, y: 1 },
        score: 0,
        snake: snake::Snake {
            gl_graphics: GlGraphics::new(opengl),
            snake_body: LinkedList::from_iter(
                (vec![snake::SnakeBody(COLUMNS / 2, ROWS / 2)]).into_iter(),
            ),
            width: SQUARE_WIDTH,
            dir: snake::Direction::Down,
        },
    };

    let mut events = piston::EventLoop::ups(piston::Events::new(piston::EventSettings::new()), 10);
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            game.render(&r);
        }
        if let Some(u) = e.update_args() {
            if !game.update(&u) {
                break;
            }
        }
        if let Some(button_arg) = e.button_args() {
            if button_arg.state == input::ButtonState::Press {
                game.button_press(&button_arg.button);
            }
        }
    }
    println!("Score: {}", game.score);
}
