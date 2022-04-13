use crate::Result;
use crate::Terminal;
use termion::event::Key;

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct Position {
    pub x: u16,
    pub y: u16,
}

pub struct Editor {
    should_quit: bool,
    terminal: Terminal,
    cursor_position: Position,
}

impl Editor {
    pub fn default() -> Self {
        Self {
            should_quit: false,
            terminal: Terminal::default().expect("Failed to initialize terminal"),
            cursor_position: Position { x: 0, y: 0 },
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

    fn process_keypress(&mut self) -> Result<()> {
        let pressed_key = Terminal::read_key()?;
        match pressed_key {
            Key::Ctrl('q') => self.should_quit = true,
            Key::Up | Key::Down | Key::Left | Key::Right => self.move_cursor(&pressed_key),
            _ => (),
        }
        Ok(())
    }

    fn refresh_screen(&self) -> Result<()> {
        Terminal::cursor_hide();
        Terminal::cursor_position(&Position { x: 0, y: 0 });

        if self.should_quit {
            Terminal::clear_screen();
            println!("Goodbye!\r");
        } else {
            self.draw_rows();
            Terminal::cursor_position(&self.cursor_position);
        }

        Terminal::cursor_show();
        Terminal::flush_stdout()
    }

    fn draw_rows(&self) {
        let height = self.terminal.size().height;
        for row in 0..height - 1 {
            Terminal::clear_current_line();
            if row == height / 3 {
                self.draw_welcome();
            } else {
                println!("~\r");
            }
        }
    }

    fn draw_welcome(&self) {
        let mut welcome_message = format!("Hecto edit -- version {}", VERSION);
        let len = welcome_message.len();
        let width = self.terminal.size().width as usize;
        let padding = width.saturating_sub(len) / 2;
        let whites = " ".repeat(padding.saturating_add(1));
        welcome_message = format!("~{}{}", whites, welcome_message);
        welcome_message.truncate(width);
        println!("{}\r", welcome_message);
    }

    fn move_cursor(&mut self, key: &Key) {
        let Position { mut x, mut y } = self.cursor_position;
        match key {
            Key::Left => x = x.saturating_sub(1),
            Key::Right => x = x.saturating_add(1),
            Key::Up => y = y.saturating_sub(1),
            Key::Down => y = y.saturating_add(1),
            _ => (),
        };
        self.cursor_position = Position { x, y };
    }
}

fn die(e: &std::io::Error) {
    Terminal::clear_screen();
    panic!("{}", e);
}
