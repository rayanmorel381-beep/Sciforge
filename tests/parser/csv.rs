use sciforge_hub::parser::csv::{
    CsvErrorKind, CsvLimits, CsvValue, DEFAULT_CSV_LIMITS, parse_csv, parse_csv_with_limits,
    validate_csv, write_csv,
};

#[test]
fn csv_parse_simple() {
    assert!(parse_csv(b"a,b,c\n1,2,3\n").unwrap().is_table());
}

#[test]
fn csv_validate() {
    assert!(validate_csv(b"a,b\n1,2\n").is_ok());
}

#[test]
fn csv_empty() {
    assert!(parse_csv(b"").unwrap().is_table());
}

#[test]
fn csv_quoted() {
    assert!(validate_csv(b"\"hello, world\"\n").is_ok());
}

#[test]
fn csv_escaped_quote() {
    assert!(validate_csv(b"\"he said \"\"hi\"\"\"\n").is_ok());
}

#[test]
fn csv_unterminated_quote() {
    assert_eq!(
        parse_csv(b"\"hello\n").unwrap_err().kind,
        CsvErrorKind::UnterminatedQuotedField
    );
}

#[test]
fn csv_write_roundtrip() {
    let out = write_csv(
        &["name", "age"],
        &[
            vec!["Alice".into(), "30".into()],
            vec!["Bob".into(), "25".into()],
        ],
    );
    assert!(out.starts_with("name,age"));
    assert!(validate_csv(out.as_bytes()).is_ok());
}

#[test]
fn csv_write_escapes_commas() {
    let out = write_csv(&["val"], &[vec!["a,b".into()]]);
    assert!(out.contains("\"a,b\""));
}

#[test]
fn csv_limits_max_rows() {
    let lim = CsvLimits {
        max_rows: 1,
        ..DEFAULT_CSV_LIMITS
    };
    assert_eq!(
        parse_csv_with_limits(b"a\n1\n2\n", lim).unwrap_err().kind,
        CsvErrorKind::MaxRowsExceeded
    );
}

#[test]
fn csv_value_field() {
    let v = CsvValue::Field("data");
    assert_eq!(v.as_field(), Some("data"));
    assert!(!v.is_table());
}

#[test]
fn csv_value_table() {
    assert!(CsvValue::Table.is_table());
    assert_eq!(CsvValue::Table.as_field(), None);
}

#[test]
fn csv_error_position() {
    let input = b"ok\n\"bad";
    let pos = parse_csv(input).unwrap_err().line_column(input);
    assert!(pos.line >= 2);
}
