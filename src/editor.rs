use crate::Terminal;
use termion::event::Key;

pub struct Editor {
    should_quit: bool,
    terminal: Terminal,
}

impl Editor {
    pub fn default() -> Self {
        Self {
            should_quit: false,
            terminal: Terminal::default().expect("Failed to initialize terminal"),
        }
    }
    pub fn run(&mut self) {
        loop {
            if let Err(ref err) = self.refresh_screen() {
                die(err);
            }
            if self.should_quit {
                break;
            }
            if let Err(ref err) = self.process_keypress() {
                die(err);
            }
        }
    }

    fn process_keypress(&mut self) -> Result<(), std::io::Error> {
        let pressed_key = Terminal::read_key()?;
        match pressed_key {
            Key::Char(c) => {
                if c.is_control() {
                    println!("{}\r", c as u8);
                } else {
                    println!("{} ({})\r", c as u8, c);
                }
            }
            Key::Ctrl('q') => self.should_quit = true,
            _ => (),
        }
        Ok(())
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        Terminal::clear_screen();
        Terminal::cursor_position(0, 0);

        if self.should_quit {
            println!("Goodbye!\r");
        } else {
            self.draw_rows();
            Terminal::cursor_position(0, 0);
        }

        Terminal::flush_stdout()
    }

    fn draw_rows(&self) {
        for _ in 0..self.terminal.size().height {
            println!("~\r");
        }
    }
}

fn die(e: &std::io::Error) {
    Terminal::clear_screen();
    panic!("{}", e);
}
