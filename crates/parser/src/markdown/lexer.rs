use super::error::{MdError, MdErrorKind};

#[derive(Clone, Copy)]
pub struct MdLine<'a> {
    pub content: &'a str,
    pub offset: usize,
    pub indent: usize,
}

pub struct LineCursor<'a> {
    bytes: &'a [u8],
    pos: usize,
}

impl<'a> LineCursor<'a> {
    pub const fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, pos: 0 }
    }

    pub const fn position(&self) -> usize {
        self.pos
    }

    pub fn peek_line(&self) -> Result<Option<MdLine<'a>>, MdError> {
        if self.pos >= self.bytes.len() {
            return Ok(None);
        }
        let line_end = find_line_end(self.bytes, self.pos);
        let line_bytes = &self.bytes[self.pos..line_end];

        let mut indent = 0usize;
        while indent < line_bytes.len() && line_bytes[indent] == b' ' {
            indent += 1;
        }

        let content = core::str::from_utf8(line_bytes)
            .map_err(|_| MdError::new(MdErrorKind::InvalidUtf8, self.pos))?;

        Ok(Some(MdLine {
            content,
            offset: self.pos,
            indent,
        }))
    }

    pub fn advance_line(&mut self) {
        let line_end = find_line_end(self.bytes, self.pos);
        self.pos = if line_end < self.bytes.len() {
            line_end + 1
        } else {
            line_end
        };
    }
}

fn find_line_end(bytes: &[u8], mut start: usize) -> usize {
    while start < bytes.len() {
        if bytes[start] == b'\n' {
            return start;
        }
        start += 1;
    }
    bytes.len()
}
