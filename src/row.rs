use std::cmp;
use unicode_segmentation::UnicodeSegmentation;

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
