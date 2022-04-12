use std::io;
use std::io::stdout;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

pub struct Editor {
    should_quit: bool,
}

impl Editor {
    pub fn default() -> Self {
        Self { should_quit: false }
    }
    pub fn run(&mut self) {
        let _stdout = stdout().into_raw_mode().unwrap();
        loop {
            if let Err(ref err) = self.process_keypress() {
                die(err);
            }
            if self.should_quit {
                break;
            }
        }
    }

    fn process_keypress(&mut self) -> Result<(), std::io::Error> {
        let pressed_key = read_key()?;
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
}

fn read_key() -> Result<Key, std::io::Error> {
    loop {
        if let Some(key) = io::stdin().lock().keys().next() {
            return key;
        }
    }
}

fn die(e: &std::io::Error) {
    panic!("{}", e);
}
