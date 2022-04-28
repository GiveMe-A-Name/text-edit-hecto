use crate::Args;
use crate::Document;
use crate::Result;
use crate::Row;
use crate::Terminal;
use clap::StructOpt;
use std::env;
use termion::event::Key;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug, Default)]
pub struct Position {
    pub x: u16,
    pub y: u16,
}

pub struct Editor {
    should_quit: bool,
    terminal: Terminal,
    cursor_position: Position,
    document: Document,
    offset: Position,
}

impl Editor {
    pub fn default() -> Self {
        let args = Args::parse();
        let document = if let Some(filename) = args.file {
            Document::open(&filename).unwrap_or_default()
        } else {
            Document::default()
        };

        Self {
            should_quit: false,
            terminal: Terminal::default().expect("Failed to initialize terminal"),
            cursor_position: Position::default(),
            document,
            offset: Position::default(),
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
            Key::Up
            | Key::Down
            | Key::Left
            | Key::Right
            | Key::Home
            | Key::End
            | Key::PageDown
            | Key::PageUp => self.move_cursor(&pressed_key),
            _ => (),
        }
        Ok(())
    }

    fn refresh_screen(&self) -> Result<()> {
        Terminal::cursor_hide();
        Terminal::cursor_position(&Position::default());

        if self.should_quit {
            Terminal::clear_screen();
            println!("Goodbye!\r");
        } else {
            self.draw_rows();
            Terminal::cursor_position(&Position {
                x: self.cursor_position.x.saturating_sub(self.offset.x),
                y: self.cursor_position.y.saturating_sub(self.offset.y),
            });
        }

        Terminal::cursor_show();
        Terminal::flush_stdout()
    }

    // start            end
    // | ............... |
    //     row's width
    fn draw_row(&self, row: &Row) {
        let mut end = self.terminal.size().width as usize;
        let start = self.offset.x as usize;
        end += start;
        let row = row.render(start, end);
        println!("{}\r", row);
    }

    fn draw_rows(&self) {
        let height = self.terminal.size().height;
        for terminal_row in 0..height - 1 {
            Terminal::clear_current_line();
            if let Some(row) = self.document.row((terminal_row + self.offset.y) as usize) {
                self.draw_row(row);
            } else if terminal_row == height / 3 && self.document.is_empty() {
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
        let height = self.document.len() as u16;
        let width = if let Some(row) = self.document.row(y.into()) {
            row.len()
        } else {
            0
        } as u16;

        match key {
            Key::Left => x = x.saturating_sub(1),
            Key::Right => {
                if x < width {
                    x = x.saturating_add(1)
                }
            }
            Key::Up => y = y.saturating_sub(1),
            Key::Down => {
                if y < height {
                    y = y.saturating_add(1)
                }
            }
            Key::Home => x = 0,
            Key::End => x = width,
            Key::PageUp => y = 0,
            Key::PageDown => y = height,
            _ => (),
        };
        self.cursor_position = Position { x, y };
        self.scroll();
    }

    fn scroll(&mut self) {
        let Position { x, y } = self.cursor_position;
        let (width, height) = (self.terminal.size().width, self.terminal.size().height);
        let offset = &mut self.offset;

        if y < offset.y {
            offset.y = y;
        } else if y >= offset.y.saturating_add(height) {
            offset.y = y.saturating_sub(height).saturating_add(1);
        }

        if x < offset.x {
            offset.x = x;
        } else if x >= offset.x.saturating_add(width) {
            offset.x = x.saturating_sub(width).saturating_add(1);
        }
    }
}

fn die(e: &std::io::Error) {
    Terminal::clear_screen();
    panic!("{}", e);
}
