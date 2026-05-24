#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TomlErrorKind {
    Eof,
    UnexpectedToken,
    InvalidKey,
    InvalidString,
    InvalidNumber,
    InvalidBool,
    InvalidEscape,
    DuplicateKey,
    UnterminatedString,
    MaxDepthExceeded,
    MaxStringLengthExceeded,
    MaxArrayLengthExceeded,
    MaxTableLengthExceeded,
    MaxNodeCountExceeded,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TomlErrorPosition {
    pub line: usize,
    pub column: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TomlError {
    pub kind: TomlErrorKind,
    pub offset: usize,
}

impl TomlError {
    pub(crate) const fn new(kind: TomlErrorKind, offset: usize) -> Self {
        Self { kind, offset }
    }

    pub fn line_column(&self, input: &[u8]) -> TomlErrorPosition {
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

        TomlErrorPosition { line, column: col }
    }
}
