use super::entity::validate_entity;
use super::error::{HtmlError, HtmlErrorKind};
use super::lexer::Cursor;
use super::value::HtmlValue;

const VOID_ELEMENTS: &[&str] = &[
    "area", "base", "br", "col", "embed", "hr", "img", "input", "link", "meta", "param", "source",
    "track", "wbr",
];

const RAW_TEXT_ELEMENTS: &[&str] = &["script", "style"];

pub const DEFAULT_MAX_HTML_DEPTH: usize = 128;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HtmlLimits {
    pub max_depth: usize,
    pub max_node_count: usize,
    pub max_attribute_count: usize,
    pub max_attribute_value_len: usize,
}

pub const DEFAULT_HTML_LIMITS: HtmlLimits = HtmlLimits {
    max_depth: DEFAULT_MAX_HTML_DEPTH,
    max_node_count: 256 * 1024,
    max_attribute_count: 256,
    max_attribute_value_len: 64 * 1024,
};

pub struct HtmlParser<'a> {
    cursor: Cursor<'a>,
    limits: HtmlLimits,
    nodes_seen: usize,
}

impl<'a> HtmlParser<'a> {
    pub const fn new(bytes: &'a [u8]) -> Self {
        Self {
            cursor: Cursor::new(bytes),
            limits: DEFAULT_HTML_LIMITS,
            nodes_seen: 0,
        }
    }

    pub const fn with_limits(mut self, limits: HtmlLimits) -> Self {
        self.limits = limits;
        self
    }

    pub const fn with_max_depth(mut self, max_depth: usize) -> Self {
        self.limits.max_depth = max_depth;
        self
    }

    pub fn parse(mut self) -> Result<HtmlValue<'a>, HtmlError> {
        self.parse_nodes(0)?;
        Ok(HtmlValue::Document)
    }

    pub fn validate(mut self) -> Result<(), HtmlError> {
        self.parse_nodes(0)?;
        Ok(())
    }

    fn tick_node(&mut self) -> Result<(), HtmlError> {
        self.nodes_seen = self.nodes_seen.saturating_add(1);
        if self.nodes_seen > self.limits.max_node_count {
            return Err(HtmlError::new(
                HtmlErrorKind::MaxNodeCountExceeded,
                self.cursor.position(),
            ));
        }
        Ok(())
    }

    fn parse_nodes(&mut self, depth: usize) -> Result<(), HtmlError> {
        if depth > self.limits.max_depth {
            return Err(HtmlError::new(
                HtmlErrorKind::MaxDepthExceeded,
                self.cursor.position(),
            ));
        }

        while !self.cursor.is_eof() {
            if self.cursor.peek() == Some(b'<') {
                if self.cursor.starts_with(b"</") {
                    return Ok(());
                }
                if self.cursor.starts_with(b"<!--") {
                    self.parse_comment()?;
                } else if self.cursor.starts_with(b"<!") {
                    self.parse_doctype()?;
                } else {
                    self.parse_element(depth)?;
                }
            } else {
                self.parse_text()?;
            }
        }

        Ok(())
    }

    fn parse_comment(&mut self) -> Result<(), HtmlError> {
        self.tick_node()?;
        let start = self.cursor.position();
        self.cursor.advance(4);

        loop {
            if self.cursor.is_eof() {
                return Err(HtmlError::new(HtmlErrorKind::UnterminatedComment, start));
            }
            if self.cursor.starts_with(b"-->") {
                self.cursor.advance(3);
                return Ok(());
            }
            self.cursor.advance(1);
        }
    }

    fn parse_doctype(&mut self) -> Result<(), HtmlError> {
        self.tick_node()?;
        let start = self.cursor.position();
        self.cursor.advance(2);

        loop {
            if self.cursor.is_eof() {
                return Err(HtmlError::new(HtmlErrorKind::UnterminatedDoctype, start));
            }
            if self.cursor.peek() == Some(b'>') {
                self.cursor.advance(1);
                return Ok(());
            }
            self.cursor.advance(1);
        }
    }

    fn parse_text(&mut self) -> Result<(), HtmlError> {
        self.tick_node()?;
        while let Some(b) = self.cursor.peek() {
            match b {
                b'<' => break,
                b'&' => validate_entity(&mut self.cursor)?,
                _ => self.cursor.advance(1),
            }
        }
        Ok(())
    }

    fn parse_element(&mut self, depth: usize) -> Result<(), HtmlError> {
        self.tick_node()?;
        let tag_start = self.cursor.position();
        self.cursor.advance(1);

        let tag_name = self.cursor.read_tag_name()?;

        self.parse_attributes()?;
        self.cursor.skip_ws();

        let self_closing = self.cursor.peek() == Some(b'/');
        if self_closing {
            self.cursor.advance(1);
        }

        if self.cursor.peek() != Some(b'>') {
            return Err(HtmlError::new(HtmlErrorKind::UnterminatedTag, tag_start));
        }
        self.cursor.advance(1);

        if self_closing || is_void_element(tag_name) {
            return Ok(());
        }

        if is_raw_text_element(tag_name) {
            return self.skip_raw_text(tag_name, tag_start);
        }

        self.parse_nodes(depth + 1)?;
        self.parse_closing_tag(tag_name, tag_start)
    }

    fn parse_attributes(&mut self) -> Result<(), HtmlError> {
        let mut count = 0usize;

        loop {
            self.cursor.skip_ws();
            match self.cursor.peek() {
                Some(b'>') | Some(b'/') | None => return Ok(()),
                _ => {}
            }

            self.parse_attribute()?;
            count = count.saturating_add(1);
            if count > self.limits.max_attribute_count {
                return Err(HtmlError::new(
                    HtmlErrorKind::MaxAttributeCountExceeded,
                    self.cursor.position(),
                ));
            }
        }
    }

    fn parse_attribute(&mut self) -> Result<(), HtmlError> {
        self.cursor.read_while(|b| {
            b.is_ascii_alphanumeric() || b == b'-' || b == b'_' || b == b':' || b == b'.'
        });

        self.cursor.skip_ws();

        if self.cursor.peek() != Some(b'=') {
            return Ok(());
        }
        self.cursor.advance(1);
        self.cursor.skip_ws();

        match self.cursor.peek() {
            Some(b'"') => self.parse_quoted_value(b'"'),
            Some(b'\'') => self.parse_quoted_value(b'\''),
            _ => {
                self.cursor.read_while(|b| {
                    !matches!(
                        b,
                        b' ' | b'\t' | b'\n' | b'\r' | b'>' | b'/' | b'"' | b'\'' | b'='
                    )
                });
                Ok(())
            }
        }
    }

    fn parse_quoted_value(&mut self, quote: u8) -> Result<(), HtmlError> {
        let start = self.cursor.position();
        self.cursor.advance(1);
        let content_start = self.cursor.position();

        loop {
            match self.cursor.peek() {
                None => return Err(HtmlError::new(HtmlErrorKind::UnterminatedAttribute, start)),
                Some(b) if b == quote => {
                    let len = self.cursor.position() - content_start;
                    if len > self.limits.max_attribute_value_len {
                        return Err(HtmlError::new(
                            HtmlErrorKind::MaxAttributeValueLengthExceeded,
                            content_start,
                        ));
                    }
                    self.cursor.advance(1);
                    return Ok(());
                }
                Some(b'&') => validate_entity(&mut self.cursor)?,
                _ => self.cursor.advance(1),
            }
        }
    }

    fn parse_closing_tag(&mut self, expected: &str, open_offset: usize) -> Result<(), HtmlError> {
        if !self.cursor.starts_with(b"</") {
            return Err(HtmlError::new(
                HtmlErrorKind::MismatchedClosingTag,
                self.cursor.position(),
            ));
        }
        self.cursor.advance(2);

        let close_name = self.cursor.read_tag_name()?;

        if !eq_ignore_ascii_case(close_name, expected) {
            return Err(HtmlError::new(
                HtmlErrorKind::MismatchedClosingTag,
                open_offset,
            ));
        }

        self.cursor.skip_ws();

        if self.cursor.peek() != Some(b'>') {
            return Err(HtmlError::new(
                HtmlErrorKind::UnterminatedTag,
                self.cursor.position(),
            ));
        }
        self.cursor.advance(1);
        Ok(())
    }

    fn skip_raw_text(&mut self, tag_name: &str, open_offset: usize) -> Result<(), HtmlError> {
        loop {
            if self.cursor.is_eof() {
                return Err(HtmlError::new(HtmlErrorKind::UnterminatedTag, open_offset));
            }
            if self.cursor.starts_with(b"</") {
                let saved = self.cursor.position();
                self.cursor.advance(2);
                if let Ok(name) = self.cursor.read_tag_name()
                    && eq_ignore_ascii_case(name, tag_name)
                {
                    self.cursor.skip_ws();
                    if self.cursor.peek() == Some(b'>') {
                        self.cursor.advance(1);
                        return Ok(());
                    }
                }
                self.cursor.advance(0);
                if self.cursor.position() == saved + 2 {
                    self.cursor.advance(1);
                }
                continue;
            }
            self.cursor.advance(1);
        }
    }
}

fn is_void_element(name: &str) -> bool {
    VOID_ELEMENTS.iter().any(|&v| eq_ignore_ascii_case(v, name))
}

fn is_raw_text_element(name: &str) -> bool {
    RAW_TEXT_ELEMENTS
        .iter()
        .any(|&v| eq_ignore_ascii_case(v, name))
}

fn eq_ignore_ascii_case(a: &str, b: &str) -> bool {
    if a.len() != b.len() {
        return false;
    }
    a.bytes()
        .zip(b.bytes())
        .all(|(x, y)| x.eq_ignore_ascii_case(&y))
}

pub fn parse_html(bytes: &[u8]) -> Result<HtmlValue<'_>, HtmlError> {
    HtmlParser::new(bytes).parse()
}

pub fn parse_html_with_max_depth(
    bytes: &[u8],
    max_depth: usize,
) -> Result<HtmlValue<'_>, HtmlError> {
    HtmlParser::new(bytes).with_max_depth(max_depth).parse()
}

pub fn parse_html_with_limits(
    bytes: &[u8],
    limits: HtmlLimits,
) -> Result<HtmlValue<'_>, HtmlError> {
    HtmlParser::new(bytes).with_limits(limits).parse()
}

pub fn validate_html(bytes: &[u8]) -> Result<(), HtmlError> {
    HtmlParser::new(bytes).validate()
}
