use super::error::{MdError, MdErrorKind};

pub fn validate_inline(text: &str, base_offset: usize) -> Result<(), MdError> {
    let bytes = text.as_bytes();
    let mut idx = 0usize;

    while idx < bytes.len() {
        match bytes[idx] {
            b'`' => {
                idx = validate_code_span(bytes, idx, base_offset)?;
            }
            b'\\' if idx + 1 < bytes.len() => {
                idx += 2;
            }
            b'<' => {
                idx = skip_autolink(bytes, idx);
            }
            b'!' if idx + 1 < bytes.len() && bytes[idx + 1] == b'[' => {
                idx = skip_bracket_span(bytes, idx + 1).unwrap_or(idx + 1);
            }
            b'[' => {
                idx = skip_bracket_span(bytes, idx).unwrap_or(idx + 1);
            }
            _ => {
                idx += 1;
            }
        }
    }

    Ok(())
}

fn validate_code_span(bytes: &[u8], start: usize, base_offset: usize) -> Result<usize, MdError> {
    let mut open_len = 0usize;
    let mut idx = start;
    while idx < bytes.len() && bytes[idx] == b'`' {
        open_len += 1;
        idx += 1;
    }

    while idx < bytes.len() {
        if bytes[idx] == b'`' {
            let run_start = idx;
            while idx < bytes.len() && bytes[idx] == b'`' {
                idx += 1;
            }
            if idx - run_start == open_len {
                return Ok(idx);
            }
        } else {
            idx += 1;
        }
    }

    Err(MdError::new(
        MdErrorKind::UnterminatedCodeSpan,
        base_offset + start,
    ))
}

fn skip_autolink(bytes: &[u8], start: usize) -> usize {
    let mut idx = start + 1;
    while idx < bytes.len() {
        match bytes[idx] {
            b'>' => return idx + 1,
            b' ' | b'\n' | b'\r' => return start + 1,
            _ => idx += 1,
        }
    }
    start + 1
}

fn skip_bracket_span(bytes: &[u8], start: usize) -> Option<usize> {
    let mut idx = start + 1;
    let mut depth = 1usize;

    while idx < bytes.len() && depth > 0 {
        match bytes[idx] {
            b'[' => depth += 1,
            b']' => depth -= 1,
            b'\\' if idx + 1 < bytes.len() => {
                idx += 1;
            }
            _ => {}
        }
        idx += 1;
    }

    if depth > 0 {
        return None;
    }

    if idx < bytes.len() && bytes[idx] == b'(' {
        idx += 1;
        let mut paren_depth = 1usize;
        while idx < bytes.len() && paren_depth > 0 {
            match bytes[idx] {
                b'(' => paren_depth += 1,
                b')' => paren_depth -= 1,
                b'\\' if idx + 1 < bytes.len() => {
                    idx += 1;
                }
                _ => {}
            }
            idx += 1;
        }
        if paren_depth > 0 {
            return None;
        }
        return Some(idx);
    }

    if idx < bytes.len() && bytes[idx] == b'[' {
        idx += 1;
        while idx < bytes.len() && bytes[idx] != b']' {
            idx += 1;
        }
        if idx < bytes.len() {
            return Some(idx + 1);
        }
        return None;
    }

    Some(idx)
}
