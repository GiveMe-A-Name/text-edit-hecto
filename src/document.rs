use crate::Position;
use crate::Result;
use crate::Row;
use crate::SearchDirection;
use std::fs;
use std::io::Write;

#[derive(Default)]
pub struct Document {
    rows: Vec<Row>,
    pub filename: Option<String>,
    dirty: bool,
}

impl Document {
    /// read document from file
    pub fn open(filename: &str) -> Result<Self> {
        let contents = fs::read_to_string(filename)?;
        let rows: Vec<Row> = contents.lines().map(Row::from).collect();
        Ok(Self {
            rows,
            dirty: false,
            filename: Some(filename.to_string()),
        })
    }

    /// get document's row from index
    pub fn row(&self, index: usize) -> Option<&Row> {
        self.rows.get(index)
    }

    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }

    pub fn len(&self) -> usize {
        self.rows.len()
    }

    pub fn status_bar_text(&self) -> String {
        let modified_indicator = if self.dirty { " (modified)" } else { "" };
        let file_name = if let Some(ref name) = self.filename {
            let mut name = name.clone();
            name.truncate(20);
            name
        } else {
            "[No Name]".to_string()
        };
        format!("{} - {} lines{}", file_name, self.len(), modified_indicator)
    }

    pub fn insert_newline(&mut self, position: &Position) {
        let (x, y) = (position.x as usize, position.y as usize);
        if let Some(row) = self.rows.get_mut(y) {
            let new_row = row.split(x);
            self.rows.insert(y + 1, new_row);
        } else {
            let new_row = Row::default();
            self.rows.push(new_row);
        }
    }

    pub fn insert(&mut self, position: &Position, ch: char) {
        let (x, y) = (position.x as usize, position.y as usize);
        if y > self.len() {
            return;
        }
        self.dirty = true;
        if ch == '\n' {
            self.insert_newline(position);
            return;
        }
        if let Some(row) = self.rows.get_mut(y) {
            row.insert(x, ch);
        } else {
            let mut row = Row::default();
            row.insert(x, ch);
            self.rows.push(row);
        }
    }

    pub fn delete_line(&mut self, position: &Position) {
        let y = position.y as usize;
        let current_row = self.rows.remove(y);
        if let Some(row) = self.rows.get_mut(y - 1) {
            row.extend(&current_row);
        }
    }

    pub fn delete(&mut self, position: &Position) {
        let (x, y) = (position.x as usize, position.y as usize);
        if x == 0 {
            self.delete_line(position);
            return;
        }
        if let Some(row) = self.rows.get_mut(y) {
            row.delete(x);
        }
    }

    /// save doc into disk
    pub fn save(&mut self) -> Result<()> {
        if let Some(ref filename) = self.filename {
            let mut file = fs::File::create(filename)?;
            for row in &self.rows {
                file.write_all(row.as_bytes())?;
                file.write_all(b"\n")?;
            }
            self.dirty = false;
        }
        Ok(())
    }

    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    pub fn find(
        &self,
        query: &str,
        after: &Position,
        direaction: SearchDirection,
    ) -> Option<Position> {
        if after.y as usize >= self.rows.len() {
            return None;
        }
        let (mut position_x, mut position_y) = (after.x as usize, after.y as usize);

        let start = if direaction == SearchDirection::Forward {
            after.y as usize
        } else {
            0
        };
        let end = if direaction == SearchDirection::Forward {
            self.rows.len()
        } else {
            after.y.saturating_add(1) as usize
        };

        for _ in start..end {
            if let Some(row) = self.rows.get(position_y) {
                if let Some(x) = row.find(query, position_x, direaction) {
                    position_x = x;
                    return Some(Position {
                        x: position_x as u16,
                        y: position_y as u16,
                    });
                }
                if direaction == SearchDirection::Forward {
                    position_y = after.y.saturating_add(1) as usize;
                    position_x = 0;
                } else {
                    position_y = after.y.saturating_sub(1) as usize;
                    position_x = self.rows[position_y].len();
                }
            } else {
                return None;
            }
        }
        // let mut x = after.x as usize;
        // for (j, row) in self.rows.iter().enumerate().skip(after.y as usize) {
        //     if let Some(i) = row.find(query, x) {
        //         return Some(Position {
        //             x: i.try_into().unwrap(),
        //             y: j.try_into().unwrap(),
        //         });
        //     }
        //     x = 0
        // }
        None
    }
}
