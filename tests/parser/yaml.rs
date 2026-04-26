use sciforge::parser::yaml::{
    YamlErrorKind, YamlValue, parse_yaml, parse_yaml_with_max_depth, validate_yaml,
};

#[test]
fn yaml_null() {
    assert_eq!(parse_yaml(b"null").unwrap(), YamlValue::Null);
}

#[test]
fn yaml_tilde_null() {
    assert_eq!(parse_yaml(b"~").unwrap(), YamlValue::Null);
}

#[test]
fn yaml_bool() {
    assert_eq!(parse_yaml(b"true").unwrap().as_bool(), Some(true));
}

#[test]
fn yaml_number() {
    assert_eq!(parse_yaml(b"42").unwrap().as_number(), Some(42.0));
}

#[test]
fn yaml_string() {
    assert_eq!(parse_yaml(b"hello").unwrap().as_str(), Some("hello"));
}

#[test]
fn yaml_quoted_string() {
    assert_eq!(
        parse_yaml(b"\"hello world\"").unwrap().as_str(),
        Some("hello world")
    );
}

#[test]
fn yaml_mapping() {
    assert!(parse_yaml(b"key: value").unwrap().is_mapping());
}

#[test]
fn yaml_sequence() {
    assert!(parse_yaml(b"- a\n- b\n- c").unwrap().is_sequence());
}

#[test]
fn yaml_validate() {
    assert!(validate_yaml(b"key: val").is_ok());
}

#[test]
fn yaml_empty_is_error() {
    assert!(validate_yaml(b"").is_err());
}

#[test]
fn yaml_max_depth() {
    assert_eq!(
        parse_yaml_with_max_depth(b"a:\n  b:\n    c:\n      d: 1", 2)
            .unwrap_err()
            .kind,
        YamlErrorKind::MaxDepthExceeded,
    );
}

#[test]
fn yaml_value_methods() {
    assert!(YamlValue::Mapping(Vec::new()).is_mapping());
    assert!(YamlValue::Sequence(Vec::new()).is_sequence());
    assert_eq!(YamlValue::Null.as_str(), None);
}
