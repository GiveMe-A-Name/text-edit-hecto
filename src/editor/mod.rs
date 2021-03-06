mod draw_editor;
mod process_editor;

use crate::Args;
use crate::Document;
use crate::Terminal;
use clap::StructOpt;
use std::time::Instant;

const QUIT_TIMES: u8 = 3;

#[derive(Clone, Copy, PartialEq)]
pub enum SearchDirection {
    Forward,
    Backward,
}

#[derive(Debug, Default, Clone)]
pub struct Position {
    pub x: u16,
    pub y: u16,
}

pub struct StatusMessage {
    pub text: String,
    pub time: Instant,
}

impl From<String> for StatusMessage {
    fn from(message: String) -> Self {
        Self {
            time: Instant::now(),
            text: message,
        }
    }
}

pub struct Editor {
    should_quit: bool,
    terminal: Terminal,
    cursor_position: Position,
    document: Document,
    offset: Position,
    status_message: StatusMessage,
    quit_times: u8,
}

impl Editor {
    pub fn default() -> Self {
        let args = Args::parse();
        let mut initial_status =
            String::from("HELP: Ctrl-Q = quit | Ctrl-S = save | Ctrl-F = find");
        let document = if let Some(filename) = args.file {
            let doc = Document::open(&filename);
            if let Ok(doc) = doc {
                doc
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
            quit_times: QUIT_TIMES,
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
