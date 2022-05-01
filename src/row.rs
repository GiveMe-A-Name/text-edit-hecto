use std::cmp;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Default)]
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

    pub fn update_len(&mut self) {
        self.len = self.content[..].graphemes(true).count();
    }

    pub fn insert(&mut self, at: usize, ch: char) {
        if at >= self.len() {
            self.content.push(ch);
        } else {
            let mut updated_content: String = self.content[..].graphemes(true).take(at).collect();
            let remainer: String = self.content[..].graphemes(true).skip(at).collect();
            updated_content.push(ch);
            updated_content.push_str(&remainer);
            self.content = updated_content;
        }
        self.update_len();
    }

    pub fn delete(&mut self, at: usize) {
        if at >= self.len() {
            return;
        } else {
            let mut update_content: String = self.content[..].graphemes(true).take(at).collect();
            let remainer: String = self.content[..].graphemes(true).skip(at + 1).collect();
            update_content.push_str(&remainer);
            self.content = update_content;
        }
        self.update_len();
    }

    pub fn split(&mut self, at: usize) -> Self {
        let benning: String = self.content[..].graphemes(true).take(at).collect();
        let remainer: String = self.content[..].graphemes(true).skip(at).collect();
        self.content = benning;
        self.update_len();
        Row::from(&remainer[..])
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.content.as_bytes()
    }
}
