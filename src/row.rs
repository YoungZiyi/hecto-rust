use std::cmp;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Default)]
pub struct Row {
    string: String,
    len: usize,
}

impl From<&str> for Row {
    fn from(slice: &str) -> Self {
        let mut row = Self {
            string: String::from(slice),
            len: 0,
        };
        row.update_len();
        row
    }
}

impl Row {
    pub fn render(&self, start: usize, end: usize) -> String {
        let end = cmp::min(end, self.string.len());
        let start = cmp::min(start, end);
        // self.string.get(start..end).unwrap_or_default().to_string();
        let mut result = String::new();
        for grapheme in self.string[..]
            .graphemes(true)
            .skip(start)
            .take(end - start) {
            
            if grapheme == "\t" {
                result.push_str(" ")
            } else {
                result.push_str(grapheme);
            }
        }
        result
    }
    pub fn len(&self) -> usize {
        self.len
    }
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
    fn update_len(&mut self) {
        self.len = self.string[..].graphemes(true).count();
    }
    pub fn insert(&mut self, at: usize, c: char) {
        // Insert a character into a single row
        if at == self.len() {
            // if insert position equals row length, we push the char into the row directly.
            self.string.push(c);
        } else {
            let mut result: String = self.string[..].graphemes(true).take(at).collect();
            let remainder: String = self.string[..].graphemes(true).skip(at).collect();
            result.push(c);
            result.push_str(&remainder);
            self.string = result;
        }
        self.update_len();
    }
}
