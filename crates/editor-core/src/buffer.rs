use std::ops::Range;

use ropey::Rope;
use thiserror::Error;

/// Errors returned by text-buffer operations.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum BufferError {
    #[error("character range is out of bounds")]
    OutOfBounds,
    #[error("range start must not be greater than range end")]
    InvalidRange,
}

/// UTF-8 rope-backed document buffer. All public offsets are Unicode scalar-value indices.
#[derive(Debug, Clone)]
pub struct TextBuffer {
    rope: Rope,
    revision: u64,
}

impl Default for TextBuffer {
    fn default() -> Self {
        Self::new("")
    }
}

impl TextBuffer {
    /// Create a buffer from initial text.
    pub fn new(text: &str) -> Self {
        Self {
            rope: Rope::from_str(text),
            revision: 0,
        }
    }

    /// Return the current document revision, incremented after every successful mutation.
    pub fn revision(&self) -> u64 {
        self.revision
    }

    /// Return the number of Unicode scalar values in the document.
    pub fn len_chars(&self) -> usize {
        self.rope.len_chars()
    }

    /// Return the number of logical lines in the document.
    pub fn len_lines(&self) -> usize {
        self.rope.len_lines()
    }

    /// Return true when the document has no content.
    pub fn is_empty(&self) -> bool {
        self.rope.len_chars() == 0
    }

    /// Return the whole document as an owned string.
    pub fn to_text(&self) -> String {
        self.rope.to_string()
    }

    /// Insert text at a Unicode scalar-value offset.
    pub fn insert(&mut self, char_idx: usize, text: &str) -> Result<(), BufferError> {
        if char_idx > self.rope.len_chars() {
            return Err(BufferError::OutOfBounds);
        }
        if text.is_empty() {
            return Ok(());
        }
        self.rope.insert(char_idx, text);
        self.revision = self.revision.wrapping_add(1);
        Ok(())
    }

    /// Remove a half-open Unicode scalar-value range and return the removed text.
    pub fn remove(&mut self, range: Range<usize>) -> Result<String, BufferError> {
        self.validate_range(&range)?;
        if range.is_empty() {
            return Ok(String::new());
        }
        let removed = self.rope.slice(range.clone()).to_string();
        self.rope.remove(range);
        self.revision = self.revision.wrapping_add(1);
        Ok(removed)
    }

    /// Replace a half-open Unicode scalar-value range with new text.
    pub fn replace(&mut self, range: Range<usize>, text: &str) -> Result<(), BufferError> {
        self.validate_range(&range)?;
        if range.is_empty() && text.is_empty() {
            return Ok(());
        }
        self.rope.remove(range.start..range.end);
        self.rope.insert(range.start, text);
        self.revision = self.revision.wrapping_add(1);
        Ok(())
    }

    /// Convert a logical line and Unicode scalar-value column into a character offset.
    pub fn line_column_to_char(&self, line: usize, column: usize) -> Option<usize> {
        if line >= self.rope.len_lines() {
            return None;
        }
        let line_start = self.rope.line_to_char(line);
        let line_len = self.rope.line(line).len_chars();
        if column > line_len {
            return None;
        }
        Some(line_start + column)
    }

    fn validate_range(&self, range: &Range<usize>) -> Result<(), BufferError> {
        if range.start > range.end {
            return Err(BufferError::InvalidRange);
        }
        if range.end > self.rope.len_chars() {
            return Err(BufferError::OutOfBounds);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{BufferError, TextBuffer};

    #[test]
    fn inserts_and_removes_unicode_text() {
        let mut buffer = TextBuffer::new("ab");
        buffer.insert(1, "🍀").expect("valid insert");
        assert_eq!(buffer.to_text(), "a🍀b");
        let removed = buffer.remove(1..2).expect("valid remove");
        assert_eq!(removed, "🍀");
        assert_eq!(buffer.to_text(), "ab");
    }

    #[test]
    fn rejects_out_of_bounds_ranges() {
        let mut buffer = TextBuffer::new("abc");
        assert_eq!(buffer.remove(0..4), Err(BufferError::OutOfBounds));
    }
}
