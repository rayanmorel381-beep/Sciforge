#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HtmlErrorKind {
    Eof,
    UnexpectedToken,
    InvalidTagName,
    UnterminatedTag,
    UnterminatedComment,
    UnterminatedAttribute,
    UnterminatedDoctype,
    UnterminatedEntity,
    DuplicateAttribute,
    MismatchedClosingTag,
    InvalidUtf8,
    MaxDepthExceeded,
    MaxNodeCountExceeded,
    MaxAttributeCountExceeded,
    MaxAttributeValueLengthExceeded,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HtmlErrorPosition {
    pub line: usize,
    pub column: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HtmlError {
    pub kind: HtmlErrorKind,
    pub offset: usize,
}

impl HtmlError {
    pub(crate) const fn new(kind: HtmlErrorKind, offset: usize) -> Self {
        Self { kind, offset }
    }

    pub fn line_column(&self, input: &[u8]) -> HtmlErrorPosition {
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

        HtmlErrorPosition { line, column: col }
    }
}
