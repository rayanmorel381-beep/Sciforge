use super::error::{HtmlError, HtmlErrorKind};

pub struct Cursor<'a> {
    bytes: &'a [u8],
    pos: usize,
}

impl<'a> Cursor<'a> {
    pub const fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, pos: 0 }
    }

    pub const fn position(&self) -> usize {
        self.pos
    }

    pub fn is_eof(&self) -> bool {
        self.pos >= self.bytes.len()
    }

    pub fn peek(&self) -> Option<u8> {
        self.bytes.get(self.pos).copied()
    }

    pub fn advance(&mut self, n: usize) {
        self.pos = core::cmp::min(self.pos.saturating_add(n), self.bytes.len());
    }

    pub fn remaining(&self) -> &'a [u8] {
        &self.bytes[self.pos..]
    }

    pub fn starts_with(&self, needle: &[u8]) -> bool {
        self.remaining().starts_with(needle)
    }

    pub fn skip_ws(&mut self) {
        while let Some(b) = self.peek() {
            match b {
                b' ' | b'\t' | b'\n' | b'\r' => self.pos += 1,
                _ => break,
            }
        }
    }

    pub fn read_while<F: Fn(u8) -> bool>(&mut self, pred: F) -> &'a [u8] {
        let start = self.pos;
        while let Some(b) = self.peek() {
            if pred(b) {
                self.pos += 1;
            } else {
                break;
            }
        }
        &self.bytes[start..self.pos]
    }

    pub fn read_tag_name(&mut self) -> Result<&'a str, HtmlError> {
        let start = self.pos;
        let first = self
            .peek()
            .ok_or(HtmlError::new(HtmlErrorKind::Eof, self.pos))?;
        if !first.is_ascii_alphabetic() {
            return Err(HtmlError::new(HtmlErrorKind::InvalidTagName, self.pos));
        }
        self.pos += 1;

        while let Some(b) = self.peek() {
            if b.is_ascii_alphanumeric() || b == b'-' || b == b'_' || b == b':' {
                self.pos += 1;
            } else {
                break;
            }
        }

        core::str::from_utf8(&self.bytes[start..self.pos])
            .map_err(|_| HtmlError::new(HtmlErrorKind::InvalidUtf8, start))
    }
}
