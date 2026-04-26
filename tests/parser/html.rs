use sciforge::parser::html::{
    HtmlErrorKind, HtmlValue, parse_html, parse_html_with_max_depth, validate_html,
};

#[test]
fn html_simple() {
    assert!(parse_html(b"<p>hello</p>").unwrap().is_document());
}

#[test]
fn html_validate() {
    assert!(validate_html(b"<div></div>").is_ok());
}

#[test]
fn html_void() {
    assert!(validate_html(b"<br>").is_ok());
}

#[test]
fn html_self_closing() {
    assert!(validate_html(b"<img />").is_ok());
}

#[test]
fn html_doctype() {
    assert!(validate_html(b"<!DOCTYPE html><html></html>").is_ok());
}

#[test]
fn html_comment() {
    assert!(validate_html(b"<!-- comment -->").is_ok());
}

#[test]
fn html_attributes() {
    assert!(validate_html(b"<a href=\"url\" class=\"c\">txt</a>").is_ok());
}

#[test]
fn html_nested() {
    assert!(validate_html(b"<div><p><span>x</span></p></div>").is_ok());
}

#[test]
fn html_script_raw() {
    assert!(validate_html(b"<script>var x = 1 < 2;</script>").is_ok());
}

#[test]
fn html_empty() {
    assert!(parse_html(b"").unwrap().is_document());
}

#[test]
fn html_plain_text() {
    assert!(parse_html(b"just text").unwrap().is_document());
}

#[test]
fn html_value_types() {
    assert!(HtmlValue::Element.is_element());
    assert!(HtmlValue::Comment.is_comment());
    assert!(HtmlValue::Doctype.is_doctype());
    assert!(HtmlValue::Document.is_document());
    assert_eq!(HtmlValue::Text("hi").as_text(), Some("hi"));
}

#[test]
fn html_max_depth() {
    assert_eq!(
        parse_html_with_max_depth(b"<a><b><c><d><e>x</e></d></c></b></a>", 2)
            .unwrap_err()
            .kind,
        HtmlErrorKind::MaxDepthExceeded,
    );
}
