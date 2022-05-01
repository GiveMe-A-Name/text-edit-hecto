use crate::Position;
use crate::Result;
use crate::Row;
use std::fs;
use std::io::Write;

#[derive(Default)]
pub struct Document {
    rows: Vec<Row>,
    pub filename: Option<String>,
}

impl Document {
    /// read document from file
    pub fn open(filename: &str) -> Result<Self> {
        let contents = fs::read_to_string(filename)?;
        let rows: Vec<Row> = contents.lines().map(|line| Row::from(line)).collect();
        Ok(Self {
            rows,
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
        let file_name = if let Some(ref name) = self.filename {
            let mut name = name.clone();
            name.truncate(20);
            name
        } else {
            "[No Name]".to_string()
        };
        format!("{} - {} lines", file_name, self.len())
    }

    pub fn insert_newline(&mut self, position: &Position) {
        let (x, y) = (position.x as usize, position.y as usize);
        if y > self.len() {
            return;
        }
        // if y >= self.len() {
        //     self.rows.push(new_row);
        // } else {

        //     self.rows.insert(y + 1, new_row);
        // }
        if let Some(row) = self.rows.get_mut(y) {
            let new_row = row.split(x);
            self.rows.insert(y + 1, new_row);
        } else {
            let new_row = Row::default();
            self.rows.push(new_row);
        }
    }

    pub fn insert(&mut self, position: &Position, ch: char) {
        if ch == '\n' {
            self.insert_newline(position);
            return;
        }
        let (x, y) = (position.x as usize, position.y as usize);
        if let Some(row) = self.rows.get_mut(y) {
            row.insert(x, ch);
        } else {
            let mut row = Row::default();
            row.insert(x, ch);
            self.rows.push(row);
        }
    }

    pub fn delete(&mut self, position: &Position) {
        let (x, y) = (position.x as usize, position.y as usize);
        if let Some(row) = self.rows.get_mut(y) {
            row.delete(x);
        }
    }

    /// save doc into disk
    pub fn save(&self) -> Result<()> {
        if let Some(ref filename) = self.filename {
            let mut file = fs::File::create(filename)?;
            for row in self.rows.iter() {
                file.write_all(row.as_bytes())?;
                file.write_all(b"\n")?;
            }
        }
        Ok(())
    }
}
