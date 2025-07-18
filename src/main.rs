use std::{
    io, thread,
    time::{Duration, Instant},
};

mod canvas;
mod paintress;

use crossterm::{
    event::{Event, KeyCode, KeyModifiers, poll, read},
    style::Stylize,
};

use crate::{canvas::Canvas, paintress::Paintress};

#[derive(PartialEq)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

fn main() -> io::Result<()> {
    let mut paintress = Paintress::create()?;

    paintress.setup()?;
    let mut canvas = Canvas::new(50, 50, ' '.stylize());
    let mut snake_moved = Instant::now();
    let mut snake_x = 1;
    let mut snake_y = 25;
    let mut snake_dx = 1;
    let mut snake_dy = 1;
    let mut snake_dir = Direction::RIGHT;
    loop {
        let paint_start = Instant::now();

        canvas.clear();
        canvas.frame('█'.green());

        if poll(Duration::from_millis(1))? {
            let ev = read()?;
            if let Event::Key(ev) = ev {
                if ev.code == KeyCode::Char('c')
                    && ev.is_press()
                    && ev.modifiers == KeyModifiers::CONTROL
                {
                    break;
                }

                if ev.code == KeyCode::Char('q') && ev.is_press() {
                    break;
                }

                if ev.code == KeyCode::Up && ev.is_press() && snake_dir != Direction::DOWN {
                    snake_dir = Direction::UP;
                }

                if ev.code == KeyCode::Down && ev.is_press() && snake_dir != Direction::UP {
                    snake_dir = Direction::DOWN;
                }

                if ev.code == KeyCode::Left && ev.is_press() && snake_dir != Direction::RIGHT {
                    snake_dir = Direction::LEFT;
                }

                if ev.code == KeyCode::Right && ev.is_press() && snake_dir != Direction::LEFT {
                    snake_dir = Direction::RIGHT;
                }
            }
        }

        match snake_dir {
            Direction::UP => {
                snake_dx = 0;
                snake_dy = -1;
            }, 
            Direction::DOWN => {
                snake_dx = 0;
                snake_dy = 1;
            }, 
            Direction::LEFT => {
                snake_dx = -1;
                snake_dy = 0;
            }, 
            Direction::RIGHT => {
                snake_dx = 1;
                snake_dy = 0;
            }, 
        }


        if snake_moved.elapsed() > Duration::from_millis(30) {
            snake_x += snake_dx;
            snake_y += snake_dy;

            snake_x = warp(snake_x , 50, 1);
            snake_y = warp(snake_y, 50, 1);
            snake_moved = Instant::now();
        }

        canvas.draw_snake(snake_x as u16, snake_y as u16, 's'.grey());

        paintress.paint(&canvas)?;

        let paint_duration = paint_start.elapsed();
        thread::sleep(Duration::from_millis(32) - paint_duration);
    }
    paintress.clear()?;

    Ok(())
}

fn warp(c: i16, lim: i16, pad: i16) -> i16 {
    if c < pad {
        lim - pad - 1
    } else if c >= lim - pad {
        pad
    } else {
        c
    }
}
