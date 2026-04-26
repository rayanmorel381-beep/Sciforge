#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MdErrorKind {
    Eof,
    InvalidHeading,
    UnterminatedCodeBlock,
    UnterminatedCodeSpan,
    MaxDepthExceeded,
    MaxLineLengthExceeded,
    MaxNodeCountExceeded,
    MaxListLengthExceeded,
    InvalidUtf8,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MdErrorPosition {
    pub line: usize,
    pub column: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MdError {
    pub kind: MdErrorKind,
    pub offset: usize,
}

impl MdError {
    pub(crate) const fn new(kind: MdErrorKind, offset: usize) -> Self {
        Self { kind, offset }
    }

    pub fn line_column(&self, input: &[u8]) -> MdErrorPosition {
        let end = core::cmp::min(self.offset, input.len());
        let mut line = 1usize;
        let mut col = 1usize;
        let mut idx = 0usize;

        while idx < end {
            if input[idx] == b'\n' {
                line += 1;
                col = 1;
            } else {
                col += 1;
            }
            idx += 1;
        }

        MdErrorPosition { line, column: col }
    }
}
