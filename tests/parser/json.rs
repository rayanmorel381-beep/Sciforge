use sciforge::parser::json::{
    JsonErrorKind, JsonValue, parse_json, parse_json_with_max_depth, validate_json,
};

#[test]
fn json_null() {
    assert_eq!(parse_json(b"null").unwrap(), JsonValue::Null);
}

#[test]
fn json_bool() {
    assert_eq!(parse_json(b"true").unwrap().as_bool(), Some(true));
    assert_eq!(parse_json(b"false").unwrap().as_bool(), Some(false));
}

#[test]
fn json_number() {
    assert_eq!(parse_json(b"42").unwrap().as_number(), Some(42.0));
}

#[test]
fn json_negative_float() {
    let n = parse_json(b"-3.14").unwrap().as_number().unwrap();
    assert!((n - (-std::f64::consts::PI)).abs() < 1e-1);
}

#[test]
fn json_string() {
    assert_eq!(parse_json(b"\"hello\"").unwrap().as_str(), Some("hello"));
}

#[test]
fn json_object() {
    assert!(parse_json(b"{}").unwrap().is_object());
}

#[test]
fn json_array() {
    assert!(parse_json(b"[]").unwrap().is_array());
}

#[test]
fn json_validate_nested() {
    assert!(validate_json(b"{\"a\":{\"b\":1}}").is_ok());
}

#[test]
fn json_invalid() {
    assert!(validate_json(b"{invalid}").is_err());
}

#[test]
fn json_eof() {
    assert_eq!(parse_json(b"").unwrap_err().kind, JsonErrorKind::Eof);
}

#[test]
fn json_trailing() {
    assert_eq!(
        parse_json(b"truefalse").unwrap_err().kind,
        JsonErrorKind::TrailingCharacters
    );
}

#[test]
fn json_max_depth() {
    assert_eq!(
        parse_json_with_max_depth(b"[[[[[[[[[[1]]]]]]]]]]", 3)
            .unwrap_err()
            .kind,
        JsonErrorKind::MaxDepthExceeded,
    );
}

#[test]
fn json_escape() {
    assert_eq!(
        parse_json(b"\"hello\\nworld\"").unwrap().as_str(),
        Some("hello\\nworld")
    );
}

#[test]
fn json_value_methods() {
    assert!(JsonValue::Object.is_object());
    assert!(JsonValue::Array.is_array());
    assert_eq!(JsonValue::Null.as_str(), None);
}
