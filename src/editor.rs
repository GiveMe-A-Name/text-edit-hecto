use crate::Args;
use crate::Document;
use crate::Result;
use crate::Row;
use crate::Terminal;

use clap::StructOpt;
use std::env;
use std::time::Duration;
use std::time::Instant;
use termion::color;
use termion::event::Key;

const STATUS_FG_COLOR: color::Rgb = color::Rgb(63, 63, 63);
const STATUS_BG_COLOR: color::Rgb = color::Rgb(239, 239, 239);
const VERSION: &str = env!("CARGO_PKG_VERSION");

struct StatusMessage {
    text: String,
    time: Instant,
}

impl From<String> for StatusMessage {
    fn from(message: String) -> Self {
        Self {
            time: Instant::now(),
            text: message,
        }
    }
}

#[derive(Debug, Default)]
pub struct Position {
    pub x: u16,
    pub y: u16,
}

trait Draw {
    fn refresh_screen(&self) -> Result<()>;
    fn draw_rows(&self);
    fn draw_row(&self, row: &Row);
    fn draw_welcome(&self);
    fn draw_status_bar(&self);
    fn draw_message_bar(&self);
}

trait Handle {
    fn process_keypress(&mut self) -> Result<()>;
    fn move_cursor(&mut self, key: &Key);
    fn scroll(&mut self);
}

pub struct Editor {
    should_quit: bool,
    terminal: Terminal,
    cursor_position: Position,
    document: Document,
    offset: Position,
    status_message: StatusMessage,
}

impl Draw for Editor {
    fn refresh_screen(&self) -> Result<()> {
        Terminal::cursor_hide();
        Terminal::cursor_position(&Position::default());

        if self.should_quit {
            Terminal::flush_stdout()?;
            Terminal::clear_screen();
            println!("Goodbye!\r");
        } else {
            self.draw_rows();
            self.draw_status_bar();
            self.draw_message_bar();
            Terminal::cursor_position(&Position {
                x: self.cursor_position.x.saturating_sub(self.offset.x),
                y: self.cursor_position.y.saturating_sub(self.offset.y),
            });
        }

        Terminal::cursor_show();
        Terminal::flush_stdout()
    }

    fn draw_rows(&self) {
        let height = self.terminal.size().height;
        for terminal_row in 0..height {
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

    // start            end
    // | ............... |
    //     row's width
    fn draw_row(&self, row: &Row) {
        let start = self.offset.x as usize;
        let end = self.terminal.size().width as usize + start;
        let row = row.render(start, end);
        println!("{}\r", row);
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

    fn draw_status_bar(&self) {
        let mut status = self.document.status_bar_text();
        let width = self.terminal.size().width as usize;
        let line_indicator = format!(
            "{}/{}",
            self.cursor_position.y.saturating_add(1),
            self.document.len(),
        );
        let len = status.len() + line_indicator.len();
        if width > len {
            status.push_str(&" ".repeat(width - len));
        }
        status = format!("{}{}", status, line_indicator);
        status.truncate(width);

        Terminal::set_bg_color(STATUS_BG_COLOR);
        Terminal::set_fg_color(STATUS_FG_COLOR);
        println!("{}\r", status);
        Terminal::reset_bg_color();
        Terminal::reset_fg_color();
    }

    fn draw_message_bar(&self) {
        Terminal::clear_current_line();
        let message = &self.status_message;
        if Instant::now() - message.time < Duration::new(5, 0) {
            let mut text = message.text.clone();
            text.truncate(self.terminal.size().width as usize);
            print!("{}", text);
        }
    }
}

impl Handle for Editor {
    fn process_keypress(&mut self) -> Result<()> {
        let pressed_key = Terminal::read_key()?;
        match pressed_key {
            Key::Ctrl('q') => self.should_quit = true,
            Key::Char(c) => {
                self.document.insert(&self.cursor_position, c);
                self.move_cursor(&Key::Right);
            }
            Key::Delete => self.document.delete(&self.cursor_position),
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

    fn move_cursor(&mut self, key: &Key) {
        let Position { mut x, mut y } = self.cursor_position;
        let height = self.document.len() as u16;
        let width = if let Some(row) = self.document.row(y.into()) {
            row.len()
        } else {
            0
        } as u16;
        let terminal_height = self.terminal.size().height;
        match key {
            Key::Left => {
                if x > 0 {
                    x -= 1;
                } else {
                    if y > 0 {
                        y -= 1;
                        x = if let Some(row) = self.document.row(y.into()) {
                            row.len() as u16
                        } else {
                            0
                        };
                    }
                }
            }
            Key::Right => {
                if x >= width {
                    if y < height {
                        y = y + 1;
                    }
                    x = 0;
                } else {
                    x = x.saturating_add(1);
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
            Key::PageUp => {
                y = if y > terminal_height {
                    y - terminal_height
                } else {
                    0
                };
            }
            Key::PageDown => {
                y = if y + terminal_height < height {
                    y + terminal_height
                } else {
                    height
                };
            }
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

impl Editor {
    pub fn default() -> Self {
        let args = Args::parse();
        let mut initial_status = String::from("HELP: Ctrl-Q = quit");
        let document = if let Some(filename) = args.file {
            let doc = Document::open(&filename);
            if doc.is_ok() {
                doc.unwrap()
            } else {
                initial_status = format!("ERR: Could not open file: {}", filename);
                Document::default()
            }
        } else {
            Document::default()
        };

        Self {
            should_quit: false,
            terminal: Terminal::default().expect("Failed to initialize terminal"),
            cursor_position: Position::default(),
            document,
            offset: Position::default(),
            status_message: StatusMessage::from(initial_status),
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
}

fn die(e: &std::io::Error) {
    Terminal::clear_screen();
    panic!("{}", e);
}
