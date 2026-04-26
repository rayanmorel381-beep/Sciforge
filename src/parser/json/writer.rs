pub enum JsonVal<'a> {
    Str(&'a str),
    Num(f64),
    Int(i64),
    Bool(bool),
    Null,
    Raw(&'a str),
}

pub fn write_json_object(fields: &[(&str, JsonVal<'_>)]) -> String {
    let mut out = String::with_capacity(512);
    out.push_str("{\n");
    for (i, (key, val)) in fields.iter().enumerate() {
        out.push_str("  \"");
        push_json_escaped(&mut out, key);
        out.push_str("\": ");
        push_json_val(&mut out, val);
        if i + 1 < fields.len() {
            out.push(',');
        }
        out.push('\n');
    }
    out.push('}');
    out
}

pub fn write_json_array(items: &[String]) -> String {
    let mut out = String::with_capacity(items.iter().map(|s| s.len() + 2).sum::<usize>() + 4);
    out.push_str("[\n");
    for (i, item) in items.iter().enumerate() {
        out.push_str(item);
        if i + 1 < items.len() {
            out.push_str(",\n");
        }
        out.push('\n');
    }
    out.push(']');
    out
}

pub fn push_json_escaped(out: &mut String, s: &str) {
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => {
                out.push_str(&format!("\\u{:04x}", c as u32));
            }
            _ => out.push(c),
        }
    }
}

fn push_json_val(out: &mut String, val: &JsonVal<'_>) {
    match val {
        JsonVal::Str(s) => {
            out.push('"');
            push_json_escaped(out, s);
            out.push('"');
        }
        JsonVal::Num(n) => {
            if n.fract() == 0.0 && n.abs() < 1e15 {
                out.push_str(&format!("{}", *n as i64));
            } else {
                out.push_str(&format!("{n}"));
            }
        }
        JsonVal::Int(n) => out.push_str(&format!("{n}")),
        JsonVal::Bool(b) => out.push_str(if *b { "true" } else { "false" }),
        JsonVal::Null => out.push_str("null"),
        JsonVal::Raw(s) => out.push_str(s),
    }
}
