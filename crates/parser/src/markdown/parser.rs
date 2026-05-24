use super::error::{MdError, MdErrorKind};
use super::inline::validate_inline;
use super::lexer::{LineCursor, MdLine};
use super::value::MdValue;

pub const DEFAULT_MAX_MD_DEPTH: usize = 64;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MdLimits {
    pub max_depth: usize,
    pub max_line_len: usize,
    pub max_list_len: usize,
    pub max_node_count: usize,
}

pub const DEFAULT_MD_LIMITS: MdLimits = MdLimits {
    max_depth: DEFAULT_MAX_MD_DEPTH,
    max_line_len: 64 * 1024,
    max_list_len: 16 * 1024,
    max_node_count: 128 * 1024,
};

pub struct MdParser<'a> {
    cursor: LineCursor<'a>,
    limits: MdLimits,
    nodes_seen: usize,
}

impl<'a> MdParser<'a> {
    pub const fn new(bytes: &'a [u8]) -> Self {
        Self {
            cursor: LineCursor::new(bytes),
            limits: DEFAULT_MD_LIMITS,
            nodes_seen: 0,
        }
    }

    pub const fn with_limits(mut self, limits: MdLimits) -> Self {
        self.limits = limits;
        self
    }

    pub const fn with_max_depth(mut self, max_depth: usize) -> Self {
        self.limits.max_depth = max_depth;
        self
    }

    pub fn parse(mut self) -> Result<MdValue<'a>, MdError> {
        self.parse_blocks(0)?;
        Ok(MdValue::Document)
    }

    pub fn validate(mut self) -> Result<(), MdError> {
        self.parse_blocks(0)?;
        Ok(())
    }

    fn tick_node(&mut self) -> Result<(), MdError> {
        self.nodes_seen = self.nodes_seen.saturating_add(1);
        if self.nodes_seen > self.limits.max_node_count {
            return Err(MdError::new(
                MdErrorKind::MaxNodeCountExceeded,
                self.cursor.position(),
            ));
        }
        Ok(())
    }

    fn check_line_len(&self, line: &MdLine<'_>) -> Result<(), MdError> {
        if line.content.len() > self.limits.max_line_len {
            return Err(MdError::new(
                MdErrorKind::MaxLineLengthExceeded,
                line.offset,
            ));
        }
        Ok(())
    }

    fn parse_blocks(&mut self, depth: usize) -> Result<(), MdError> {
        if depth > self.limits.max_depth {
            return Err(MdError::new(
                MdErrorKind::MaxDepthExceeded,
                self.cursor.position(),
            ));
        }

        while let Some(line) = self.cursor.peek_line()? {
            self.check_line_len(&line)?;

            let trimmed = line.content.trim();

            if trimmed.is_empty() {
                self.cursor.advance_line();
                continue;
            }

            if is_thematic_break(trimmed) {
                self.tick_node()?;
                self.cursor.advance_line();
                continue;
            }

            if is_atx_heading(trimmed) {
                self.parse_heading(line)?;
                continue;
            }

            if is_fenced_code_start(trimmed) {
                self.parse_fenced_code(line)?;
                continue;
            }

            if trimmed.starts_with('>') {
                self.parse_block_quote(depth)?;
                continue;
            }

            if is_list_item(trimmed) {
                self.parse_list(depth)?;
                continue;
            }

            if is_table_row(trimmed) {
                self.parse_table()?;
                continue;
            }

            self.parse_paragraph()?;
        }

        Ok(())
    }

    fn parse_heading(&mut self, line: MdLine<'a>) -> Result<(), MdError> {
        self.tick_node()?;
        let trimmed = line.content.trim();
        let bytes = trimmed.as_bytes();
        let mut level = 0usize;
        while level < bytes.len() && bytes[level] == b'#' {
            level += 1;
        }

        self.cursor.advance_line();

        if level >= bytes.len() {
            return Ok(());
        }

        let content = trimmed[level..].trim();
        let content = content.trim_end_matches(['#', ' ']);
        if !content.is_empty() {
            validate_inline(content, line.offset)?;
        }

        Ok(())
    }

    fn parse_fenced_code(&mut self, line: MdLine<'_>) -> Result<(), MdError> {
        self.tick_node()?;
        let trimmed = line.content.trim();
        let fence_char = trimmed.as_bytes()[0];
        let mut fence_len = 0usize;
        while fence_len < trimmed.len() && trimmed.as_bytes()[fence_len] == fence_char {
            fence_len += 1;
        }

        self.cursor.advance_line();

        loop {
            let Some(inner) = self.cursor.peek_line()? else {
                return Err(MdError::new(
                    MdErrorKind::UnterminatedCodeBlock,
                    line.offset,
                ));
            };
            self.check_line_len(&inner)?;
            self.cursor.advance_line();

            let inner_trimmed = inner.content.trim();
            if is_closing_fence(inner_trimmed, fence_char, fence_len) {
                return Ok(());
            }
        }
    }

    fn parse_block_quote(&mut self, depth: usize) -> Result<(), MdError> {
        self.tick_node()?;
        if depth + 1 > self.limits.max_depth {
            return Err(MdError::new(
                MdErrorKind::MaxDepthExceeded,
                self.cursor.position(),
            ));
        }

        while let Some(line) = self.cursor.peek_line()? {
            let trimmed = line.content.trim();
            if !trimmed.starts_with('>') {
                break;
            }
            self.check_line_len(&line)?;
            self.cursor.advance_line();
            self.tick_node()?;

            let inner = if trimmed.len() > 1 {
                if trimmed.as_bytes()[1] == b' ' {
                    &trimmed[2..]
                } else {
                    &trimmed[1..]
                }
            } else {
                ""
            };

            let inner = inner.trim();
            if !inner.is_empty() {
                validate_inline(inner, line.offset)?;
            }
        }

        Ok(())
    }

    fn parse_list(&mut self, depth: usize) -> Result<(), MdError> {
        self.tick_node()?;
        if depth + 1 > self.limits.max_depth {
            return Err(MdError::new(
                MdErrorKind::MaxDepthExceeded,
                self.cursor.position(),
            ));
        }

        let mut count = 0usize;

        while let Some(line) = self.cursor.peek_line()? {
            let trimmed = line.content.trim();
            if trimmed.is_empty() {
                break;
            }
            if !is_list_item(trimmed) && line.indent < 2 {
                break;
            }
            self.check_line_len(&line)?;
            self.tick_node()?;
            self.cursor.advance_line();

            let item_text = strip_list_marker(trimmed);
            if !item_text.is_empty() {
                validate_inline(item_text, line.offset)?;
            }

            count = count.saturating_add(1);
            if count > self.limits.max_list_len {
                return Err(MdError::new(
                    MdErrorKind::MaxListLengthExceeded,
                    line.offset,
                ));
            }
        }

        Ok(())
    }

    fn parse_table(&mut self) -> Result<(), MdError> {
        self.tick_node()?;

        while let Some(line) = self.cursor.peek_line()? {
            let trimmed = line.content.trim();
            if !is_table_row(trimmed) {
                break;
            }
            self.check_line_len(&line)?;
            self.tick_node()?;
            self.cursor.advance_line();
        }

        Ok(())
    }

    fn parse_paragraph(&mut self) -> Result<(), MdError> {
        self.tick_node()?;

        while let Some(line) = self.cursor.peek_line()? {
            let trimmed = line.content.trim();
            if trimmed.is_empty()
                || is_atx_heading(trimmed)
                || is_fenced_code_start(trimmed)
                || is_thematic_break(trimmed)
                || trimmed.starts_with('>')
                || is_list_item(trimmed)
                || is_table_row(trimmed)
            {
                break;
            }
            self.check_line_len(&line)?;
            validate_inline(trimmed, line.offset)?;
            self.cursor.advance_line();
        }

        Ok(())
    }
}

fn is_atx_heading(trimmed: &str) -> bool {
    let bytes = trimmed.as_bytes();
    if bytes.is_empty() || bytes[0] != b'#' {
        return false;
    }
    let mut level = 0usize;
    while level < bytes.len() && bytes[level] == b'#' {
        level += 1;
    }
    level <= 6 && (level == bytes.len() || bytes[level] == b' ')
}

fn is_thematic_break(trimmed: &str) -> bool {
    let bytes = trimmed.as_bytes();
    if bytes.len() < 3 {
        return false;
    }
    let ch = bytes[0];
    if ch != b'-' && ch != b'*' && ch != b'_' {
        return false;
    }
    let mut count = 0usize;
    for &b in bytes {
        if b == ch {
            count += 1;
        } else if b != b' ' {
            return false;
        }
    }
    count >= 3
}

fn is_fenced_code_start(trimmed: &str) -> bool {
    let bytes = trimmed.as_bytes();
    if bytes.len() < 3 {
        return false;
    }
    let ch = bytes[0];
    if ch != b'`' && ch != b'~' {
        return false;
    }
    let mut count = 0usize;
    for &b in bytes {
        if b == ch {
            count += 1;
        } else {
            break;
        }
    }
    count >= 3
}

fn is_closing_fence(trimmed: &str, fence_char: u8, min_len: usize) -> bool {
    let bytes = trimmed.as_bytes();
    if bytes.is_empty() {
        return false;
    }
    for &b in bytes {
        if b != fence_char {
            return false;
        }
    }
    bytes.len() >= min_len
}

fn is_list_item(trimmed: &str) -> bool {
    let bytes = trimmed.as_bytes();
    if bytes.is_empty() {
        return false;
    }
    if (bytes[0] == b'-' || bytes[0] == b'*' || bytes[0] == b'+')
        && (bytes.len() == 1 || bytes[1] == b' ')
    {
        return true;
    }
    let mut idx = 0usize;
    while idx < bytes.len() && bytes[idx].is_ascii_digit() {
        idx += 1;
    }
    if idx > 0 && idx < bytes.len() && (bytes[idx] == b'.' || bytes[idx] == b')') {
        return idx + 1 == bytes.len() || bytes[idx + 1] == b' ';
    }
    false
}

fn strip_list_marker(trimmed: &str) -> &str {
    let bytes = trimmed.as_bytes();
    if bytes.is_empty() {
        return "";
    }
    if bytes[0] == b'-' || bytes[0] == b'*' || bytes[0] == b'+' {
        return if bytes.len() > 2 {
            trimmed[2..].trim_start()
        } else {
            ""
        };
    }
    let mut idx = 0usize;
    while idx < bytes.len() && bytes[idx].is_ascii_digit() {
        idx += 1;
    }
    if idx < bytes.len() && (bytes[idx] == b'.' || bytes[idx] == b')') {
        idx += 1;
        return if idx < bytes.len() {
            trimmed[idx..].trim_start()
        } else {
            ""
        };
    }
    trimmed
}

fn is_table_row(trimmed: &str) -> bool {
    trimmed.starts_with('|')
}

pub fn parse_md(bytes: &[u8]) -> Result<MdValue<'_>, MdError> {
    MdParser::new(bytes).parse()
}

pub fn parse_md_with_max_depth(bytes: &[u8], max_depth: usize) -> Result<MdValue<'_>, MdError> {
    MdParser::new(bytes).with_max_depth(max_depth).parse()
}

pub fn parse_md_with_limits(bytes: &[u8], limits: MdLimits) -> Result<MdValue<'_>, MdError> {
    MdParser::new(bytes).with_limits(limits).parse()
}

pub fn validate_md(bytes: &[u8]) -> Result<(), MdError> {
    MdParser::new(bytes).validate()
}
