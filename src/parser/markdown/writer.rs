pub fn push_escaped(out: &mut String, text: &str) {
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

pub fn render_inline(out: &mut String, text: &str) {
    let mut rest = text;
    loop {
        match rest.find("**") {
            None => {
                push_escaped(out, rest);
                break;
            }
            Some(pos) => {
                push_escaped(out, &rest[..pos]);
                rest = &rest[pos + 2..];
                match rest.find("**") {
                    None => {
                        out.push_str("**");
                        push_escaped(out, rest);
                        break;
                    }
                    Some(end) => {
                        out.push_str("<strong>");
                        push_escaped(out, &rest[..end]);
                        out.push_str("</strong>");
                        rest = &rest[end + 2..];
                    }
                }
            }
        }
    }
}

pub fn render_md_html(src: &str) -> String {
    let mut out = String::with_capacity(src.len() * 2);
    let mut in_code = false;
    let mut in_table = false;
    let mut past_sep = false;

    for line in src.lines() {
        if in_code {
            if line.starts_with("```") {
                out.push_str("</code></pre>\n");
                in_code = false;
            } else {
                push_escaped(&mut out, line);
                out.push('\n');
            }
            continue;
        }

        if line.starts_with("```") {
            if in_table {
                out.push_str("</tbody></table>\n");
                in_table = false;
            }
            out.push_str("<pre><code>");
            in_code = true;
            continue;
        }

        if line.starts_with('|') {
            let inner = line.trim_start_matches('|').trim_end_matches('|');
            let is_sep = !inner.is_empty()
                && inner
                    .chars()
                    .all(|c| c == '-' || c == '|' || c == ' ' || c == ':');
            if is_sep {
                if in_table {
                    out.push_str("</thead><tbody>\n");
                    past_sep = true;
                }
                continue;
            }
            if !in_table {
                out.push_str("<table>\n<thead>\n");
                in_table = true;
                past_sep = false;
            }
            let tag = if past_sep { "td" } else { "th" };
            out.push_str("<tr>");
            for cell in line.split('|') {
                let cell = cell.trim();
                if cell.is_empty() {
                    continue;
                }
                out.push('<');
                out.push_str(tag);
                out.push('>');
                render_inline(&mut out, cell);
                out.push_str("</");
                out.push_str(tag);
                out.push('>');
            }
            out.push_str("</tr>\n");
            continue;
        }

        if in_table {
            out.push_str("</tbody></table>\n");
            in_table = false;
            past_sep = false;
        }

        if let Some(rest) = line.strip_prefix("## ") {
            out.push_str("<h3 style=\"color:#79c0ff;margin:1em 0 .4em\">");
            push_escaped(&mut out, rest);
            out.push_str("</h3>\n");
        } else if let Some(rest) = line.strip_prefix("# ") {
            out.push_str("<h2 style=\"color:#58a6ff;border-bottom:1px solid #21262d;padding-bottom:.3em;margin-bottom:.5em\">");
            push_escaped(&mut out, rest);
            out.push_str("</h2>\n");
        } else if line == "---" {
            out.push_str("<hr style=\"border:none;border-top:1px solid #30363d;margin:1em 0\">\n");
        } else if line.is_empty() {
            continue;
        } else {
            out.push_str("<p style=\"line-height:1.6;margin:.4em 0\">");
            render_inline(&mut out, line);
            out.push_str("</p>\n");
        }
    }

    if in_table {
        out.push_str("</tbody></table>\n");
    }
    if in_code {
        out.push_str("</code></pre>\n");
    }

    out
}

pub fn write_md_heading(level: u8, text: &str) -> String {
    let mut out = String::with_capacity(text.len() + 6);
    for _ in 0..level {
        out.push('#');
    }
    out.push(' ');
    out.push_str(text);
    out.push('\n');
    out
}

pub fn write_md_table(headers: &[&str], rows: &[Vec<String>]) -> String {
    let mut out = String::with_capacity(headers.len() * 20 + rows.len() * headers.len() * 15);
    out.push('|');
    for h in headers {
        out.push(' ');
        out.push_str(h);
        out.push_str(" |");
    }
    out.push('\n');
    out.push('|');
    for _ in headers {
        out.push_str("---|");
    }
    out.push('\n');
    for row in rows {
        out.push('|');
        for cell in row {
            out.push(' ');
            out.push_str(cell);
            out.push_str(" |");
        }
        out.push('\n');
    }
    out
}

pub fn write_md_code_block(content: &str) -> String {
    let mut out = String::with_capacity(content.len() + 10);
    out.push_str("```\n");
    out.push_str(content);
    if !content.ends_with('\n') {
        out.push('\n');
    }
    out.push_str("```\n");
    out
}

pub fn write_md_row(out: &mut String, label: &str, value: &str) {
    out.push_str("| ");
    out.push_str(label);
    out.push_str(" | ");
    out.push_str(value);
    out.push_str(" |\n");
}
