use crate::Position;
use crate::Result;
use crate::Row;
use std::fs;

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

    pub fn insert(&mut self, position: &Position, ch: char) {
        let (x, y) = (position.x as usize, position.y as usize);
        if let Some(row) = self.rows.get_mut(y) {
            row.insert(x, ch);
        } else {
            let mut row = Row::default();
            row.insert(x, ch);
            self.rows.push(row);
        }
    }
}
