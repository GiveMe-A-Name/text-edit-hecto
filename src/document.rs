use crate::Result;
use std::cmp;
use std::fs;
pub struct Row {
    content: String,
}

impl From<&str> for Row {
    fn from(slice: &str) -> Self {
        Row {
            content: slice.to_string(),
        }
    }
}

impl Row {
    pub fn render(&self, start: usize, end: usize) -> String {
        let end = cmp::min(end, self.content.len());
        let start = cmp::min(start, end);
        self.content.get(start..end).unwrap_or_default().to_string()
    }
    pub fn len(&self) -> usize {
        self.content.len()
    }
    pub fn is_empty(&self) -> bool {
        self.content.is_empty()
    }
}

#[derive(Default)]
pub struct Document {
    rows: Vec<Row>,
}

impl Document {
    /// read document from file
    pub fn open(filename: &str) -> Result<Self> {
        let contents = fs::read_to_string(filename)?;
        let rows: Vec<Row> = contents
            .lines()
            .map(|line| Row {
                content: line.to_string(),
            })
            .collect();
        Ok(Self { rows })
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
}
