pub fn push_html_escaped(out: &mut String, text: &str) {
    for c in text.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&#39;"),
            _ => out.push(c),
        }
    }
}

pub fn push_html_row(out: &mut String, label: &str, value: &str) {
    out.push_str("<tr><td>");
    push_html_escaped(out, label);
    out.push_str("</td><td>");
    push_html_escaped(out, value);
    out.push_str("</td></tr>\n");
}

pub fn push_js_escaped(out: &mut String, s: &str) {
    for ch in s.chars() {
        match ch {
            '\\' => out.push_str("\\\\"),
            '"' => out.push_str("\\\""),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '<' => out.push_str("\\x3c"),
            '>' => out.push_str("\\x3e"),
            _ => out.push(ch),
        }
    }
}
