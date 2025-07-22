use std::io::{self, Write};

use crossterm::{
    ExecutableCommand,
    cursor::{self},
    queue,
    style::{self, StyledContent},
    terminal::{
        self, EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
    },
};

use crate::canvas::Canvas;

pub struct Paintress {
    out: io::Stdout,
}

impl Paintress {
    pub fn create() -> io::Result<Paintress> {
        let stdout = io::stdout();

        Ok(Paintress { out: stdout })
    }

    pub fn setup(&mut self) -> io::Result<()> {
        self.out.execute(EnterAlternateScreen)?;
        self.out.execute(cursor::Hide)?;
        enable_raw_mode()
    }

    pub fn clear(&mut self) -> io::Result<()> {
        disable_raw_mode()?;
        self.out.execute(cursor::Show)?;
        self.out.execute(LeaveAlternateScreen)?;
        Ok(())
    }

    pub fn paint(&mut self, canvas: &Canvas<StyledContent<char>>) -> io::Result<()> {
        let stdout = &mut self.out;

        let buffer = canvas.get_buffer();
        let (size_x, size_y) = buffer.get_dimensions();
        let (term_x, term_y) = terminal::size()?;
        let padding_x = term_x / 2 - size_x / 2;
        let padding_y = if size_y > term_y {
            0
        } else {
            term_y / 2 - size_y / 2
        };
        // eprintln!("Size: {}, Term: {}, Padding: {}", size_x, term_x, padding);

        for y in 0..size_y {
            for x in 0..size_x {
                queue!(stdout, cursor::MoveTo(x + padding_x, y + padding_y))?;
                queue!(stdout, style::PrintStyledContent(buffer[(x, y)]))?;
            }
        }

        // for y in 0..40 {
        //     for x in 0..150 {
        //         if (y == 0 || y == 40 - 1) || (x == 0 || x == 150 - 1) {
        //             stdout
        //                 .queue(cursor::MoveTo(x, y))?
        //                 .queue(style::PrintStyledContent('█'.green()))?;
        //         }
        //     }
        // }

        stdout.flush()?;
        Ok(())
    }
}

impl Drop for Paintress {
    fn drop(&mut self) {
        let _ = self.clear();
    }
}
