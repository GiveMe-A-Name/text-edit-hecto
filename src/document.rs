use unicode_segmentation::UnicodeSegmentation;

use crate::Result;
use std::cmp;
use std::fs;
pub struct Row {
    content: String,
    len: usize,
}

impl From<&str> for Row {
    fn from(slice: &str) -> Self {
        let content = slice.to_string();
        let len = content[..].graphemes(true).count();
        Row { content, len }
    }
}

impl Row {
    /// render a document's row into terminal
    pub fn render(&self, start: usize, end: usize) -> String {
        let end = cmp::min(end, self.content.len());
        let start = cmp::min(start, end);
        self.content[..]
            .graphemes(true)
            .skip(start)
            .take(end - start)
            .map(|grapheme| if grapheme == "\t" { " " } else { grapheme })
            .collect::<String>()
    }

    pub fn len(&self) -> usize {
        self.len
    }

    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.content.is_empty()
    }
}

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
}
