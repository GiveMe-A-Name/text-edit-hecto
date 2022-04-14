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
        self.content.get(start..end).unwrap_or_default().to_string()
    }
}

impl Row {}

#[derive(Default)]
pub struct Document {
    rows: Vec<Row>,
}

impl Document {
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
    pub fn row(&self, index: usize) -> Option<&Row> {
        self.rows.get(index)
    }
    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }
}
