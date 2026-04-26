use super::error::{TomlError, TomlErrorKind};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Token<'a> {
    BareKey(&'a str),
    BasicString(&'a str),
    LiteralString(&'a str),
    Integer(i64),
    Float(f64),
    Bool(bool),
    Equals,
    Dot,
    Comma,
    OpenBracket,
    CloseBracket,
    OpenDoubleBracket,
    CloseDoubleBracket,
    Newline,
}

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

    pub const fn bytes(&self) -> &'a [u8] {
        self.bytes
    }

    pub fn set_position(&mut self, pos: usize) {
        self.pos = pos;
    }

    pub const fn is_eof(&self) -> bool {
        self.pos >= self.bytes.len()
    }

    pub fn remaining(&self) -> &'a [u8] {
        &self.bytes()[self.pos..]
    }

    fn skip_whitespace_and_comments(&mut self) {
        while !self.is_eof() {
            let b = self.bytes[self.pos];
            if b == b' ' || b == b'\t' || b == b'\r' {
                self.pos += 1;
            } else if b == b'#' {
                while self.pos < self.bytes.len() && self.bytes[self.pos] != b'\n' {
                    self.pos += 1;
                }
            } else {
                break;
            }
        }
    }

    fn skip_inline_whitespace(&mut self) {
        while !self.is_eof() {
            let b = self.bytes[self.pos];
            if b == b' ' || b == b'\t' {
                self.pos += 1;
            } else {
                break;
            }
        }
    }

    pub fn next_token(&mut self) -> Result<Option<Token<'a>>, TomlError> {
        self.skip_whitespace_and_comments();

        if self.is_eof() {
            return Ok(None);
        }

        let b = self.bytes[self.pos];

        if b == b'\n' {
            self.pos += 1;
            return Ok(Some(Token::Newline));
        }

        if b == b'=' {
            self.pos += 1;
            self.skip_inline_whitespace();
            return Ok(Some(Token::Equals));
        }

        if b == b',' {
            self.pos += 1;
            return Ok(Some(Token::Comma));
        }

        if b == b'.' {
            self.pos += 1;
            return Ok(Some(Token::Dot));
        }

        if b == b'[' {
            if self.pos + 1 < self.bytes.len() && self.bytes[self.pos + 1] == b'[' {
                self.pos += 2;
                return Ok(Some(Token::OpenDoubleBracket));
            }
            self.pos += 1;
            return Ok(Some(Token::OpenBracket));
        }

        if b == b']' {
            if self.pos + 1 < self.bytes.len() && self.bytes[self.pos + 1] == b']' {
                self.pos += 2;
                return Ok(Some(Token::CloseDoubleBracket));
            }
            self.pos += 1;
            return Ok(Some(Token::CloseBracket));
        }

        if b == b'"' {
            return self.read_basic_string().map(Some);
        }

        if b == b'\'' {
            return self.read_literal_string().map(Some);
        }

        if b == b't' || b == b'f' {
            return self.try_read_bool().map(Some);
        }

        if b.is_ascii_digit() || b == b'+' || b == b'-' {
            return self.read_number().map(Some);
        }

        if is_bare_key_char(b) {
            return self.read_bare_key().map(Some);
        }

        Err(TomlError::new(TomlErrorKind::UnexpectedToken, self.pos))
    }

    fn read_basic_string(&mut self) -> Result<Token<'a>, TomlError> {
        let start = self.pos;
        self.pos += 1;
        let content_start = self.pos;

        while self.pos < self.bytes.len() {
            let b = self.bytes[self.pos];
            if b == b'"' {
                let content = core::str::from_utf8(&self.bytes[content_start..self.pos])
                    .map_err(|_| TomlError::new(TomlErrorKind::InvalidString, start))?;
                self.pos += 1;
                return Ok(Token::BasicString(content));
            }
            if b == b'\\' {
                self.pos += 1;
                if self.pos >= self.bytes.len() {
                    return Err(TomlError::new(TomlErrorKind::InvalidEscape, self.pos));
                }
                let esc = self.bytes[self.pos];
                match esc {
                    b'"' | b'\\' | b'b' | b't' | b'n' | b'f' | b'r' => {
                        self.pos += 1;
                    }
                    b'u' => {
                        self.pos += 1;
                        for _ in 0..4 {
                            if self.pos >= self.bytes.len()
                                || !self.bytes[self.pos].is_ascii_hexdigit()
                            {
                                return Err(TomlError::new(TomlErrorKind::InvalidEscape, self.pos));
                            }
                            self.pos += 1;
                        }
                    }
                    b'U' => {
                        self.pos += 1;
                        for _ in 0..8 {
                            if self.pos >= self.bytes.len()
                                || !self.bytes[self.pos].is_ascii_hexdigit()
                            {
                                return Err(TomlError::new(TomlErrorKind::InvalidEscape, self.pos));
                            }
                            self.pos += 1;
                        }
                    }
                    _ => {
                        return Err(TomlError::new(TomlErrorKind::InvalidEscape, self.pos));
                    }
                }
                continue;
            }
            if b == b'\n' {
                return Err(TomlError::new(TomlErrorKind::UnterminatedString, start));
            }
            self.pos += 1;
        }

        Err(TomlError::new(TomlErrorKind::UnterminatedString, start))
    }

    fn read_literal_string(&mut self) -> Result<Token<'a>, TomlError> {
        let start = self.pos;
        self.pos += 1;
        let content_start = self.pos;

        while self.pos < self.bytes.len() {
            let b = self.bytes[self.pos];
            if b == b'\'' {
                let content = core::str::from_utf8(&self.bytes[content_start..self.pos])
                    .map_err(|_| TomlError::new(TomlErrorKind::InvalidString, start))?;
                self.pos += 1;
                return Ok(Token::LiteralString(content));
            }
            if b == b'\n' {
                return Err(TomlError::new(TomlErrorKind::UnterminatedString, start));
            }
            self.pos += 1;
        }

        Err(TomlError::new(TomlErrorKind::UnterminatedString, start))
    }

    fn try_read_bool(&mut self) -> Result<Token<'a>, TomlError> {
        let rem = self.remaining();
        if rem.starts_with(b"true") && (rem.len() <= 4 || !is_bare_key_char(rem[4])) {
            self.pos += 4;
            return Ok(Token::Bool(true));
        }
        if rem.starts_with(b"false") && (rem.len() <= 5 || !is_bare_key_char(rem[5])) {
            self.pos += 5;
            return Ok(Token::Bool(false));
        }
        self.read_bare_key()
    }

    fn read_number(&mut self) -> Result<Token<'a>, TomlError> {
        let start = self.pos;

        if self.bytes[self.pos] == b'+' || self.bytes[self.pos] == b'-' {
            self.pos += 1;
        }

        if self.is_eof() || !self.bytes[self.pos].is_ascii_digit() {
            return Err(TomlError::new(TomlErrorKind::InvalidNumber, start));
        }

        let mut is_float = false;
        let mut has_underscore = false;

        while !self.is_eof() {
            let b = self.bytes[self.pos];
            if b.is_ascii_digit() {
                self.pos += 1;
            } else if b == b'_' {
                has_underscore = true;
                self.pos += 1;
            } else if b == b'.' {
                is_float = true;
                self.pos += 1;
            } else if b == b'e' || b == b'E' {
                is_float = true;
                self.pos += 1;
                if self.pos < self.bytes.len()
                    && (self.bytes[self.pos] == b'+' || self.bytes[self.pos] == b'-')
                {
                    self.pos += 1;
                }
            } else {
                break;
            }
        }

        let raw = core::str::from_utf8(&self.bytes[start..self.pos])
            .map_err(|_| TomlError::new(TomlErrorKind::InvalidNumber, start))?;

        let cleaned: String;
        let num_str = if has_underscore {
            cleaned = raw.replace('_', "");
            &cleaned
        } else {
            raw
        };

        if is_float {
            let val = num_str
                .parse::<f64>()
                .map_err(|_| TomlError::new(TomlErrorKind::InvalidNumber, start))?;
            Ok(Token::Float(val))
        } else {
            let val = num_str
                .parse::<i64>()
                .map_err(|_| TomlError::new(TomlErrorKind::InvalidNumber, start))?;
            Ok(Token::Integer(val))
        }
    }

    fn read_bare_key(&mut self) -> Result<Token<'a>, TomlError> {
        let start = self.pos;
        while self.pos < self.bytes.len() && is_bare_key_char(self.bytes[self.pos]) {
            self.pos += 1;
        }
        if self.pos == start {
            return Err(TomlError::new(TomlErrorKind::InvalidKey, start));
        }
        let key = core::str::from_utf8(&self.bytes[start..self.pos])
            .map_err(|_| TomlError::new(TomlErrorKind::InvalidKey, start))?;
        Ok(Token::BareKey(key))
    }
}

fn is_bare_key_char(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'-' || b == b'_'
}
