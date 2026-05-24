pub fn write_yaml_map(fields: &[(&str, &str)]) -> String {
    let mut out = String::with_capacity(fields.len() * 40);
    for (key, val) in fields {
        out.push_str(key);
        out.push_str(": ");
        push_yaml_value(&mut out, val);
        out.push('\n');
    }
    out
}

pub fn write_yaml_document(
    header: &[(&str, &str)],
    list_key: &str,
    items: &[Vec<(&str, &str)>],
) -> String {
    let mut out = String::with_capacity(256 + items.len() * 80);
    for (key, val) in header {
        out.push_str(key);
        out.push_str(": ");
        push_yaml_value(&mut out, val);
        out.push('\n');
    }
    out.push_str(list_key);
    out.push_str(":\n");
    for item in items {
        for (i, (key, val)) in item.iter().enumerate() {
            if i == 0 {
                out.push_str("  - ");
            } else {
                out.push_str("    ");
            }
            out.push_str(key);
            out.push_str(": ");
            push_yaml_value(&mut out, val);
            out.push('\n');
        }
    }
    out
}

pub fn push_yaml_value(out: &mut String, val: &str) {
    if val.contains(':')
        || val.contains('#')
        || val.contains('\n')
        || val.contains('"')
        || val.starts_with(' ')
        || val.ends_with(' ')
    {
        out.push('"');
        for c in val.chars() {
            match c {
                '"' => out.push_str("\\\""),
                '\\' => out.push_str("\\\\"),
                '\n' => out.push_str("\\n"),
                _ => out.push(c),
            }
        }
        out.push('"');
    } else {
        out.push_str(val);
    }
}
