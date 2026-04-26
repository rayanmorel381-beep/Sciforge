use sciforge::parser::markdown::{MdValue, parse_md, validate_md};

#[test]
fn md_heading() {
    assert!(parse_md(b"# Title\n").unwrap().is_document());
}

#[test]
fn md_validate_empty() {
    assert!(validate_md(b"").is_ok());
}

#[test]
fn md_code_block() {
    assert!(validate_md(b"```rust\nfn main() {}\n```\n").is_ok());
}

#[test]
fn md_thematic_break() {
    assert!(validate_md(b"---\n").is_ok());
}

#[test]
fn md_blockquote() {
    assert!(validate_md(b"> quoted\n").is_ok());
}

#[test]
fn md_list() {
    assert!(validate_md(b"- a\n- b\n- c\n").is_ok());
}

#[test]
fn md_table() {
    assert!(validate_md(b"| A | B |\n|---|---|\n| 1 | 2 |\n").is_ok());
}

#[test]
fn md_value_heading_level() {
    assert_eq!(MdValue::Heading(3).heading_level(), Some(3));
}

#[test]
fn md_value_text() {
    assert_eq!(MdValue::Text("x").as_text(), Some("x"));
}

#[test]
fn md_value_types() {
    assert!(MdValue::CodeBlock.is_code_block());
    assert!(MdValue::List.is_list());
    assert!(MdValue::Table.is_table());
    assert!(MdValue::Document.is_document());
}
