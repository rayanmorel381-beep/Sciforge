use super::error::{HtmlError, HtmlErrorKind};
use super::lexer::Cursor;

pub fn validate_entity(cursor: &mut Cursor<'_>) -> Result<(), HtmlError> {
    let start = cursor.position();
    cursor.advance(1);

    if cursor.peek() == Some(b'#') {
        cursor.advance(1);
        if cursor.peek() == Some(b'x') || cursor.peek() == Some(b'X') {
            cursor.advance(1);
            let digits = cursor.read_while(|b| b.is_ascii_hexdigit());
            if digits.is_empty() {
                return Err(HtmlError::new(HtmlErrorKind::UnterminatedEntity, start));
            }
        } else {
            let digits = cursor.read_while(|b| b.is_ascii_digit());
            if digits.is_empty() {
                return Err(HtmlError::new(HtmlErrorKind::UnterminatedEntity, start));
            }
        }
    } else {
        let name = cursor.read_while(|b| b.is_ascii_alphanumeric());
        if name.is_empty() {
            return Err(HtmlError::new(HtmlErrorKind::UnterminatedEntity, start));
        }
    }

    if cursor.peek() == Some(b';') {
        cursor.advance(1);
    }

    Ok(())
}
