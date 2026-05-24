use sciforge_hub::parser::toml::{TomlValue, parse_toml, parse_toml_with_max_depth, validate_toml};

#[test]
fn toml_string() {
    let input = b"key = \"hello\"";
    let val = parse_toml(input).unwrap();
    assert!(val.is_table());
}

#[test]
fn toml_integer() {
    let input = b"key = 42";
    let val = parse_toml(input).unwrap();
    assert!(val.is_table());
}

#[test]
fn toml_float() {
    let input = b"key = 3.14";
    let val = parse_toml(input).unwrap();
    assert!(val.is_table());
}

#[test]
fn toml_bool() {
    let input = b"key = true";
    let val = parse_toml(input).unwrap();
    assert!(val.is_table());
}

#[test]
fn toml_table() {
    let input = b"[section]\nkey = \"val\"";
    let val = parse_toml(input).unwrap();
    assert!(val.is_table());
}

#[test]
fn toml_array() {
    let input = b"key = [1, 2, 3]";
    let val = parse_toml(input).unwrap();
    assert!(val.is_array() || val.is_table());
}

#[test]
fn toml_validate() {
    assert!(validate_toml(b"key = 42").is_ok());
}

#[test]
fn toml_max_depth() {
    let input = b"[section]\nkey = 1";
    let val = parse_toml_with_max_depth(input, 10).unwrap();
    assert!(val.is_table());
}

#[test]
fn toml_empty_is_valid() {
    assert!(validate_toml(b"").is_ok());
}

#[test]
fn toml_max_depth_exceeded() {
    use sciforge_hub::parser::toml::TomlErrorKind;
    assert_eq!(
        parse_toml_with_max_depth(b"[a]\nk = [1]", 2)
            .unwrap_err()
            .kind,
        TomlErrorKind::MaxDepthExceeded,
    );
}

#[test]
fn toml_value_methods() {
    assert!(TomlValue::Table.is_table());
    assert!(TomlValue::Array.is_array());
    assert_eq!(TomlValue::Integer(5).as_integer(), Some(5));
    assert_eq!(TomlValue::Bool(true).as_bool(), Some(true));
    assert_eq!(TomlValue::Float(1.0).as_float(), Some(1.0));
    assert_eq!(TomlValue::String("hi").as_str(), Some("hi"));
}
