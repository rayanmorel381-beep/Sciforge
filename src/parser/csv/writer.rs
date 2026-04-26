pub fn write_csv(header: &[&str], rows: &[Vec<String>]) -> String {
    let mut out = String::new();
    for (i, h) in header.iter().enumerate() {
        if i > 0 {
            out.push(',');
        }
        write_field(&mut out, h);
    }
    for row in rows {
        out.push('\n');
        for (i, field) in row.iter().enumerate() {
            if i > 0 {
                out.push(',');
            }
            write_field(&mut out, field);
        }
    }
    out
}

fn write_field(out: &mut String, field: &str) {
    if field.contains(',') || field.contains('"') || field.contains('\n') {
        out.push('"');
        for c in field.chars() {
            if c == '"' {
                out.push('"');
            }
            out.push(c);
        }
        out.push('"');
    } else {
        out.push_str(field);
    }
}
