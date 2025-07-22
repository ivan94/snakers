use std::{
    io, thread,
    time::{Duration, Instant},
};

mod canvas;
mod game_state;
mod paintress;

use crossterm::{
    event::{Event, KeyCode, KeyModifiers, poll, read},
    style::Stylize,
};

use crate::{canvas::Canvas, game_state::GameState, paintress::Paintress};

const SIZE_X: i16 = 50;
const SIZE_Y: i16 = 50;
const TICKS_PER_SECOND: u64 = 15;

fn main() -> io::Result<()> {
    let mut paintress = Paintress::create()?;

    let mut tps = TICKS_PER_SECOND;

    paintress.setup()?;
    let mut canvas = Canvas::new(SIZE_X as u16, SIZE_Y as u16, ' '.stylize());
    let mut state = GameState::new(SIZE_X, SIZE_Y, tps);
    let mut pause = false;
    loop {
        let paint_start = Instant::now();

        canvas.clear();
        canvas.frame('█'.grey());

        if poll(Duration::from_millis(0))? {
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

                if ev.code == KeyCode::Char('p') && ev.is_press() {
                    pause = !pause;
                }

                if ev.code == KeyCode::Char('r') && ev.is_press() {
                    state = GameState::new(SIZE_X, SIZE_Y, TICKS_PER_SECOND);
                }

                if ev.code == KeyCode::Char('1') && ev.is_press() && tps > 5 {
                    tps -= 5;
                    state.set_tps(tps);
                }

                if ev.code == KeyCode::Char('2') && ev.is_press() && tps < 80 {
                    tps += 5;
                    state.set_tps(tps);
                }

                if !pause {
                    state.handle_input(ev);
                }

            }
        }

        if !pause {
            state.tick();
        }
        state.draw(&mut canvas);

        paintress.paint(&canvas)?;

        let paint_duration = paint_start.elapsed();
        thread::sleep(Duration::from_millis(8) - paint_duration);
    }
    paintress.clear()?;

    Ok(())
}
