pub fn push_toml_escaped(out: &mut String, s: &str) {
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => {
                out.push_str(&format!("\\u{:04X}", c as u32));
            }
            _ => out.push(c),
        }
    }
}

pub fn push_toml_str(out: &mut String, key: &str, val: &str) {
    out.push_str(key);
    out.push_str(" = \"");
    push_toml_escaped(out, val);
    out.push_str("\"\n");
}

pub fn push_toml_num(out: &mut String, key: &str, val: &str) {
    out.push_str(key);
    out.push_str(" = ");
    out.push_str(val);
    out.push('\n');
}

pub fn push_toml_section(out: &mut String, name: &str) {
    out.push('[');
    out.push_str(name);
    out.push_str("]\n");
}

pub fn push_toml_array_section(out: &mut String, name: &str) {
    out.push_str("[[");
    out.push_str(name);
    out.push_str("]]\n");
}
