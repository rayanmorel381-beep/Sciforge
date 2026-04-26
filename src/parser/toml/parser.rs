use super::error::{TomlError, TomlErrorKind};
use super::lexer::{Cursor, Token};
use super::value::TomlValue;

pub const DEFAULT_MAX_TOML_DEPTH: usize = 64;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TomlLimits {
    pub max_depth: usize,
    pub max_string_len: usize,
    pub max_array_len: usize,
    pub max_table_len: usize,
    pub max_node_count: usize,
}

pub const DEFAULT_TOML_LIMITS: TomlLimits = TomlLimits {
    max_depth: DEFAULT_MAX_TOML_DEPTH,
    max_string_len: 64 * 1024,
    max_array_len: 16 * 1024,
    max_table_len: 16 * 1024,
    max_node_count: 128 * 1024,
};

pub struct TomlParser<'a> {
    cursor: Cursor<'a>,
    limits: TomlLimits,
    nodes_seen: usize,
}

impl<'a> TomlParser<'a> {
    pub const fn new(bytes: &'a [u8]) -> Self {
        Self {
            cursor: Cursor::new(bytes),
            limits: DEFAULT_TOML_LIMITS,
            nodes_seen: 0,
        }
    }

    pub const fn with_max_depth(mut self, max_depth: usize) -> Self {
        self.limits.max_depth = max_depth;
        self
    }

    pub const fn with_limits(mut self, limits: TomlLimits) -> Self {
        self.limits = limits;
        self
    }

    pub fn parse(mut self) -> Result<TomlValue<'a>, TomlError> {
        self.parse_document()
    }

    pub fn validate(mut self) -> Result<(), TomlError> {
        self.parse_document()?;
        Ok(())
    }

    fn parse_document(&mut self) -> Result<TomlValue<'a>, TomlError> {
        self.skip_newlines()?;
        self.parse_key_values(0)?;

        loop {
            self.skip_newlines()?;
            let Some(tok) = self.cursor.next_token()? else {
                break;
            };
            match tok {
                Token::OpenDoubleBracket => {
                    self.parse_array_of_tables_header()?;
                    self.skip_newlines()?;
                    self.parse_key_values(1)?;
                }
                Token::OpenBracket => {
                    self.parse_table_header()?;
                    self.skip_newlines()?;
                    self.parse_key_values(1)?;
                }
                Token::Newline => continue,
                _ => {
                    return Err(TomlError::new(
                        TomlErrorKind::UnexpectedToken,
                        self.cursor.position(),
                    ));
                }
            }
        }

        Ok(TomlValue::Table)
    }

    fn skip_newlines(&mut self) -> Result<(), TomlError> {
        loop {
            let pos = self.cursor.position();
            match self.cursor.next_token()? {
                Some(Token::Newline) => continue,
                None => break,
                Some(_) => {
                    self.cursor.set_position(pos);
                    break;
                }
            }
        }
        Ok(())
    }

    fn parse_table_header(&mut self) -> Result<(), TomlError> {
        self.bump_node()?;
        self.parse_key_path()?;
        self.expect_close_bracket()?;
        Ok(())
    }

    fn parse_array_of_tables_header(&mut self) -> Result<(), TomlError> {
        self.bump_node()?;
        self.parse_key_path()?;
        self.expect_close_double_bracket()?;
        Ok(())
    }

    fn parse_key_path(&mut self) -> Result<(), TomlError> {
        self.parse_simple_key()?;
        loop {
            let pos = self.cursor.position();
            match self.cursor.next_token()? {
                Some(Token::Dot) => {
                    self.parse_simple_key()?;
                }
                Some(_) => {
                    self.cursor.set_position(pos);
                    break;
                }
                None => break,
            }
        }
        Ok(())
    }

    fn parse_simple_key(&mut self) -> Result<(), TomlError> {
        let tok = self
            .cursor
            .next_token()?
            .ok_or(TomlError::new(TomlErrorKind::Eof, self.cursor.position()))?;
        match tok {
            Token::BareKey(k) => {
                if k.len() > self.limits.max_string_len {
                    return Err(TomlError::new(
                        TomlErrorKind::MaxStringLengthExceeded,
                        self.cursor.position(),
                    ));
                }
            }
            Token::BasicString(s) | Token::LiteralString(s) => {
                if s.len() > self.limits.max_string_len {
                    return Err(TomlError::new(
                        TomlErrorKind::MaxStringLengthExceeded,
                        self.cursor.position(),
                    ));
                }
            }
            _ => {
                return Err(TomlError::new(
                    TomlErrorKind::InvalidKey,
                    self.cursor.position(),
                ));
            }
        }
        Ok(())
    }

    fn parse_key_values(&mut self, depth: usize) -> Result<(), TomlError> {
        if depth > self.limits.max_depth {
            return Err(TomlError::new(
                TomlErrorKind::MaxDepthExceeded,
                self.cursor.position(),
            ));
        }

        let mut count = 0usize;
        loop {
            let pos = self.cursor.position();
            let Some(tok) = self.cursor.next_token()? else {
                break;
            };
            match tok {
                Token::BareKey(_) | Token::BasicString(_) | Token::LiteralString(_) => {
                    self.cursor.set_position(pos);
                    self.parse_key_path()?;
                    self.expect_equals()?;
                    self.parse_value(depth + 1)?;
                    self.bump_node()?;
                    count = count.saturating_add(1);
                    if count > self.limits.max_table_len {
                        return Err(TomlError::new(
                            TomlErrorKind::MaxTableLengthExceeded,
                            self.cursor.position(),
                        ));
                    }
                    self.skip_newlines()?;
                }
                Token::Newline => continue,
                _ => {
                    self.cursor.set_position(pos);
                    break;
                }
            }
        }
        Ok(())
    }

    fn parse_value(&mut self, depth: usize) -> Result<TomlValue<'a>, TomlError> {
        if depth > self.limits.max_depth {
            return Err(TomlError::new(
                TomlErrorKind::MaxDepthExceeded,
                self.cursor.position(),
            ));
        }

        let tok = self
            .cursor
            .next_token()?
            .ok_or(TomlError::new(TomlErrorKind::Eof, self.cursor.position()))?;

        match tok {
            Token::BasicString(s) => {
                if s.len() > self.limits.max_string_len {
                    return Err(TomlError::new(
                        TomlErrorKind::MaxStringLengthExceeded,
                        self.cursor.position(),
                    ));
                }
                Ok(TomlValue::String(s))
            }
            Token::LiteralString(s) => {
                if s.len() > self.limits.max_string_len {
                    return Err(TomlError::new(
                        TomlErrorKind::MaxStringLengthExceeded,
                        self.cursor.position(),
                    ));
                }
                Ok(TomlValue::String(s))
            }
            Token::Integer(v) => Ok(TomlValue::Integer(v)),
            Token::Float(v) => Ok(TomlValue::Float(v)),
            Token::Bool(v) => Ok(TomlValue::Bool(v)),
            Token::OpenBracket => self.parse_inline_array(depth),
            _ => Err(TomlError::new(
                TomlErrorKind::UnexpectedToken,
                self.cursor.position(),
            )),
        }
    }

    fn parse_inline_array(&mut self, depth: usize) -> Result<TomlValue<'a>, TomlError> {
        let mut count = 0usize;
        loop {
            let pos = self.cursor.position();
            let Some(tok) = self.cursor.next_token()? else {
                return Err(TomlError::new(TomlErrorKind::Eof, pos));
            };
            match tok {
                Token::CloseBracket => return Ok(TomlValue::Array),
                Token::Comma | Token::Newline => continue,
                _ => {
                    self.cursor.set_position(pos);
                    self.parse_value(depth + 1)?;
                    self.bump_node()?;
                    count = count.saturating_add(1);
                    if count > self.limits.max_array_len {
                        return Err(TomlError::new(
                            TomlErrorKind::MaxArrayLengthExceeded,
                            self.cursor.position(),
                        ));
                    }
                }
            }
        }
    }

    fn bump_node(&mut self) -> Result<(), TomlError> {
        self.nodes_seen = self.nodes_seen.saturating_add(1);
        if self.nodes_seen > self.limits.max_node_count {
            return Err(TomlError::new(
                TomlErrorKind::MaxNodeCountExceeded,
                self.cursor.position(),
            ));
        }
        Ok(())
    }

    fn expect_equals(&mut self) -> Result<(), TomlError> {
        let tok = self
            .cursor
            .next_token()?
            .ok_or(TomlError::new(TomlErrorKind::Eof, self.cursor.position()))?;
        if !matches!(tok, Token::Equals) {
            return Err(TomlError::new(
                TomlErrorKind::UnexpectedToken,
                self.cursor.position(),
            ));
        }
        Ok(())
    }

    fn expect_close_bracket(&mut self) -> Result<(), TomlError> {
        let tok = self
            .cursor
            .next_token()?
            .ok_or(TomlError::new(TomlErrorKind::Eof, self.cursor.position()))?;
        if !matches!(tok, Token::CloseBracket) {
            return Err(TomlError::new(
                TomlErrorKind::UnexpectedToken,
                self.cursor.position(),
            ));
        }
        Ok(())
    }

    fn expect_close_double_bracket(&mut self) -> Result<(), TomlError> {
        let tok = self
            .cursor
            .next_token()?
            .ok_or(TomlError::new(TomlErrorKind::Eof, self.cursor.position()))?;
        if !matches!(tok, Token::CloseDoubleBracket) {
            return Err(TomlError::new(
                TomlErrorKind::UnexpectedToken,
                self.cursor.position(),
            ));
        }
        Ok(())
    }
}

pub fn parse_toml(bytes: &[u8]) -> Result<TomlValue<'_>, TomlError> {
    TomlParser::new(bytes).parse()
}

pub fn parse_toml_with_max_depth(
    bytes: &[u8],
    max_depth: usize,
) -> Result<TomlValue<'_>, TomlError> {
    TomlParser::new(bytes).with_max_depth(max_depth).parse()
}

pub fn parse_toml_with_limits(
    bytes: &[u8],
    limits: TomlLimits,
) -> Result<TomlValue<'_>, TomlError> {
    TomlParser::new(bytes).with_limits(limits).parse()
}

pub fn validate_toml(bytes: &[u8]) -> Result<(), TomlError> {
    TomlParser::new(bytes).validate()
}
