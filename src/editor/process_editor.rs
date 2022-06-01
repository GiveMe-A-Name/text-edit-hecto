use super::Position;
use super::SearchDirection;
use super::StatusMessage;
use super::QUIT_TIMES;
use crate::Editor;
use crate::Result;
use crate::Terminal;
use termion::event::Key;

impl Editor {
    pub fn process_keypress(&mut self) -> Result<()> {
        let pressed_key = Terminal::read_key()?;
        match pressed_key {
            Key::Ctrl('q') => {
                return {
                    self.quit();
                    Ok(())
                }
            }
            Key::Ctrl('s') => {
                return {
                    self.save();
                    Ok(())
                }
            }
            Key::Ctrl('f') => self.search(),
            Key::Char(c) => {
                self.document.insert(&self.cursor_position, c);
                self.move_cursor(Key::Right);
            }
            Key::Delete => self.document.delete(&self.cursor_position),
            Key::Backspace => {
                if self.cursor_position.x > 0 && self.cursor_position.y > 0 {
                    self.move_cursor(Key::Left);
                    self.document.delete(&self.cursor_position);
                } else if self.cursor_position.x == 0 && self.cursor_position.y > 0 {
                    let previous_row = self
                        .document
                        .row(self.cursor_position.y as usize - 1)
                        .unwrap();
                    let new_position = Position {
                        x: previous_row.len() as u16,
                        y: self.cursor_position.y - 1,
                    };
                    self.document.delete(&self.cursor_position);
                    self.cursor_position = new_position;
                }
            }
            Key::Up
            | Key::Down
            | Key::Left
            | Key::Right
            | Key::Home
            | Key::End
            | Key::PageDown
            | Key::PageUp => self.move_cursor(pressed_key),
            _ => (),
        }
        if self.quit_times < QUIT_TIMES {
            self.quit_times = QUIT_TIMES;
            self.status_message = StatusMessage::from(String::new());
        }
        Ok(())
    }

    fn search(&mut self) {
        let old_position = self.cursor_position.clone();
        let mut direaction = SearchDirection::Forward;
        if let Some(query) = self
            .prompt(
                "Search (ESC to cancel, Arrows to navigate): ",
                |editor, key, query| {
                    let mut moved = false;
                    match key {
                        Key::Right | Key::Down => {
                            editor.move_cursor(Key::Right);
                            direaction = SearchDirection::Forward;
                            moved = true;
                        }
                        Key::Left | Key::Up => {
                            direaction = SearchDirection::Backward;
                            moved = true
                        }
                        _ => (),
                    }
                    if let Some(position) = editor.document.find(query, &editor.cursor_position) {
                        editor.cursor_position = position;
                        editor.scroll();
                    } else if moved {
                        editor.move_cursor(Key::Left);
                    }
                },
            )
            .unwrap_or(None)
        {
            if let Some(find_position) = self.document.find(query.as_str(), &old_position) {
                self.cursor_position = find_position;
                self.status_message = StatusMessage::from(String::new())
            } else {
                self.cursor_position = old_position;
                self.status_message = StatusMessage::from(format!("Not found :{}.", query));
            }
        } else {
            self.status_message = StatusMessage::from(String::new());
        }
    }

    fn move_cursor(&mut self, key: Key) {
        let Position { mut x, mut y } = self.cursor_position;
        let height = self.document.len() as u16;
        let width = if let Some(row) = self.document.row(y.into()) {
            row.len() as u16
        } else {
            0
        };
        let terminal_height = self.terminal.size().height;
        match key {
            Key::Left => {
                if x > 0 {
                    x -= 1;
                } else if y > 0 {
                    y -= 1;
                    x = if let Some(row) = self.document.row(y.into()) {
                        row.len() as u16
                    } else {
                        0
                    };
                }
            }
            Key::Right => {
                if x >= width {
                    if y < height {
                        y += 1;
                    }
                    x = 0;
                } else {
                    x = x.saturating_add(1);
                }
            }
            Key::Up => y = y.saturating_sub(1),
            Key::Down => {
                if y < height {
                    y = y.saturating_add(1);
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

    fn prompt<F>(&mut self, prompt: &str, callback: F) -> Result<Option<String>>
    where
        F: Fn(&mut Self, Key, &str),
    {
        let mut result = String::new();
        loop {
            self.status_message = StatusMessage::from(format!("{}{}", prompt, result));
            self.refresh_screen()?;
            let key = Terminal::read_key()?;
            match key {
                Key::Char('\n') => break,
                Key::Char(c) => {
                    if !c.is_control() {
                        result.push(c);
                    }
                }
                Key::Backspace => {
                    if !result.is_empty() {
                        result.truncate(result.len() - 1);
                    }
                }
                Key::Esc => {
                    result.truncate(0);
                    break;
                }
                _ => (),
            }
            callback(self, key, &result);
        }
        if result.is_empty() {
            Ok(None)
        } else {
            Ok(Some(result))
        }
    }

    fn save(&mut self) {
        if self.document.filename.is_none() {
            let new_filename = self.prompt("Save as: ", |_, _, _| {}).unwrap_or(None);
            match new_filename {
                Some(filename) => self.document.filename = Some(filename),
                None => {
                    self.status_message = StatusMessage::from("Save aborted.".to_string());
                    return;
                }
            }
        }
        if self.document.save().is_ok() {
            self.status_message = StatusMessage::from("File saved successfully.".to_string());
        } else {
            self.status_message = StatusMessage::from("Error writing file!".to_string());
        }
    }

    fn quit(&mut self) {
        if self.document.is_dirty() && self.quit_times > 0 {
            self.status_message = StatusMessage::from(format!(
                "WARNING! File has unsaved changes. Press Ctrl-Q {} more times to quit.",
                self.quit_times
            ));
            self.quit_times -= 1;
        } else {
            self.should_quit = true;
        }
    }
}
