use super::Position;
use super::StatusMessage;
use crate::Editor;
use crate::Result;
use crate::Terminal;
use termion::event::Key;

impl Editor {
    pub fn process_keypress(&mut self) -> Result<()> {
        let pressed_key = Terminal::read_key()?;
        match pressed_key {
            Key::Ctrl('q') => self.should_quit = true,
            Key::Ctrl('s') => {
                if self.document.save().is_ok() {
                    self.status_message =
                        StatusMessage::from("File saved successfully.".to_string());
                } else {
                    self.status_message = StatusMessage::from("Error writing file!".to_string());
                }
            }
            Key::Char(c) => {
                self.document.insert(&self.cursor_position, c);
                self.move_cursor(&Key::Right);
            }
            Key::Delete => self.document.delete(&self.cursor_position),
            Key::Backspace => {
                if self.cursor_position.x > 0 || self.cursor_position.y > 0 {
                    self.move_cursor(&Key::Left);
                    self.document.delete(&self.cursor_position);
                }
            }
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
