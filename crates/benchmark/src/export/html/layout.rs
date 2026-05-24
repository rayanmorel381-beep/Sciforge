use super::charts::build_charts;
use super::{Entry, PTABLE_CATS, css_cat, group_colors};
use sciforge_parser::html::{push_html_escaped, push_js_escaped};
use sciforge_parser::markdown::render_md_html;
use std::collections::BTreeMap;

pub fn build_html(
    title: &str,
    entries: &[Entry<'_>],
    _tag_keys: &[&str],
    csv_files: &BTreeMap<String, String>,
    json_files: &BTreeMap<String, String>,
    yaml_files: &BTreeMap<String, String>,
    toml_files: &BTreeMap<String, String>,
    md_content: &str,
    bmk_files: &BTreeMap<String, String>,
) -> String {
    let mut h = String::with_capacity(131072);
    let has_grid = entries.iter().any(|e| e.grid_row.is_some());

    let is_ptable = has_grid
        && entries.iter().any(|e| {
            e.tags
                .iter()
                .any(|(k, v)| *k == "category" && PTABLE_CATS.contains(v))
        });

    let mut dyn_cats: Vec<&str> = Vec::new();
    if has_grid && !is_ptable {
        for entry in entries {
            if let Some((_, v)) = entry.tags.iter().find(|(k, _)| *k == "category")
                && !dyn_cats.contains(v)
            {
                dyn_cats.push(v);
            }
        }
    }
    let max_col = if has_grid {
        entries
            .iter()
            .filter_map(|e| e.grid_col)
            .max()
            .unwrap_or(18)
    } else {
        18
    };

    h.push_str("<!DOCTYPE html>\n<html lang=\"en\">\n<head>\n<meta charset=\"utf-8\">\n<title>");
    push_html_escaped(&mut h, title);
    h.push_str("</title>\n<style>\n");

    h.push_str(concat!(
        "*{margin:0;padding:0;box-sizing:border-box}\n",
        "body{font-family:system-ui,sans-serif;background:#0d1117;color:#c9d1d9;padding:2em;max-width:1800px;margin:0 auto}\n",
        "h1{color:#58a6ff;margin-bottom:.5em;font-size:2em}\n",
        "h2{color:#79c0ff;margin:1.5em 0 .5em;border-bottom:1px solid #21262d;padding-bottom:.3em}\n",
        "p{margin:.5em 0;line-height:1.6}\n",
        "table{border-collapse:collapse;width:100%;margin:1em 0}\n",
        "th,td{border:1px solid #30363d;padding:6px 10px;text-align:left;font-size:.85em}\n",
        "th{background:#161b22;color:#58a6ff;position:sticky;top:0}\n",
        "tr:nth-child(even){background:#161b22}\n",
        "pre{background:#161b22;padding:1em;border-radius:6px;overflow-x:auto;max-height:600px;font-size:.8em}\n",
        "code{font-family:ui-monospace,monospace}\n",
        ".stats{display:grid;grid-template-columns:repeat(auto-fill,minmax(180px,1fr));gap:1em;margin:1em 0}\n",
        ".stat-card{background:#161b22;border:1px solid #30363d;border-radius:8px;padding:1em;text-align:center}\n",
        ".stat-card .num{font-size:2em;color:#58a6ff;font-weight:bold}\n",
        ".stat-card .lbl{font-size:.8em;color:#8b949e}\n",
        ".tabs{display:flex;gap:0;margin-top:1.5em;flex-wrap:wrap}\n",
        ".tab{padding:.6em 1.2em;cursor:pointer;background:#161b22;border:1px solid #30363d;border-bottom:none;border-radius:6px 6px 0 0;color:#8b949e;font-weight:600}\n",
        ".tab:hover{color:#c9d1d9}\n",
        ".tab.active{background:#0d1117;color:#58a6ff;border-bottom:1px solid #0d1117}\n",
        ".panel{display:none;border:1px solid #30363d;padding:1em;border-radius:0 6px 6px 6px}\n",
        ".panel.active{display:block}\n",
        ".fsel{width:100%;padding:.5em;margin-bottom:.8em;background:#161b22;color:#c9d1d9;border:1px solid #30363d;border-radius:4px;font-family:ui-monospace,monospace;font-size:.85em}\n",
        ".charts{display:grid;grid-template-columns:repeat(auto-fill,minmax(500px,1fr));gap:1.5em;margin:1em 0}\n",
        ".chart-box{background:#161b22;border:1px solid #30363d;border-radius:8px;padding:1.2em}\n",
        ".chart-box h3{color:#79c0ff;font-size:.95em;margin-bottom:.8em}\n",
    ));

    if has_grid {
        h.push_str(&format!(
            ".ptable{{display:grid;grid-template-columns:repeat({max_col},1fr);gap:3px;margin:1.5em 0}}\n"
        ));
        h.push_str(concat!(
            ".el{padding:6px 3px;text-align:center;border-radius:6px;min-height:68px;display:flex;flex-direction:column;justify-content:center;cursor:pointer;transition:transform .15s,box-shadow .15s;border:1px solid rgba(255,255,255,.06)}\n",
            ".el:hover{transform:scale(1.4);box-shadow:0 0 16px rgba(88,166,255,.6);z-index:10;position:relative}\n",
            ".el .z{font-size:.55em;opacity:.65;color:#8b949e}\n",
            ".el .sym{font-size:1.5em;font-weight:700;line-height:1.1}\n",
            ".el .mass{font-size:.5em;opacity:.55;color:#8b949e}\n",
            ".el .ns{font-size:.45em;opacity:.45;color:#58a6ff}\n",
            ".el .nm{font-size:.5em;opacity:.6;color:#8b949e;white-space:nowrap;overflow:hidden;text-overflow:ellipsis;max-width:100%}\n",
            ".empty{background:transparent}\n",
            ".marker{display:flex;align-items:center;justify-content:center;font-weight:700;color:#58a6ff;font-size:.85em}\n",
            ".legend{display:flex;flex-wrap:wrap;gap:.8em;margin:.8em 0;align-items:center}\n",
            ".legend-item{display:flex;align-items:center;gap:6px;font-size:.75em;color:#8b949e}\n",
            ".legend-swatch{width:18px;height:18px;border-radius:4px;border:1px solid rgba(255,255,255,.1)}\n",
            "@keyframes hl{0%{box-shadow:0 0 20px #58a6ff}100%{box-shadow:none}}\n",
            ".highlight{animation:hl 2s ease-out}\n",
            ".detail{display:none;margin:1.5em 0;background:#161b22;border:1px solid #30363d;border-radius:10px;padding:1.5em;position:relative}\n",
            ".detail.open{display:grid;grid-template-columns:auto 1fr;gap:.6em 1.5em;align-items:baseline}\n",
            ".detail .close{position:absolute;top:10px;right:14px;cursor:pointer;color:#8b949e;font-size:1.4em;background:none;border:none}\n",
            ".detail .close:hover{color:#f85149}\n",
            ".detail .dk{color:#8b949e;font-size:.85em;text-align:right}\n",
            ".detail .dv{color:#c9d1d9;font-size:.95em;font-weight:600}\n",
            ".detail .dtitle{grid-column:1/-1;font-size:1.3em;font-weight:700;margin-bottom:.3em}\n",
            ".detail .files{grid-column:1/-1;display:flex;gap:.5em;margin-top:.5em}\n",
            ".detail .fbtn{padding:.3em .8em;background:#21262d;border:1px solid #30363d;border-radius:4px;color:#58a6ff;cursor:pointer;font-size:.8em;text-decoration:none}\n",
            ".detail .fbtn:hover{background:#30363d}\n",
        ));
        if is_ptable {
            h.push_str(concat!(
                ".nonmetal{background:linear-gradient(135deg,#1a3a1a,#0d2610);color:#7ee787}\n",
                ".noble-gas{background:linear-gradient(135deg,#3a1a2e,#2a0d1e);color:#d2a8ff}\n",
                ".alkali{background:linear-gradient(135deg,#3a2a0a,#2a1d05);color:#ffa657}\n",
                ".alkaline{background:linear-gradient(135deg,#3a2a1a,#2a1d0d);color:#f0b27a}\n",
                ".transition{background:linear-gradient(135deg,#0a2a3a,#051d2a);color:#79c0ff}\n",
                ".post-trans{background:linear-gradient(135deg,#2a1a2a,#1d0d1d);color:#d2a8ff}\n",
                ".metalloid{background:linear-gradient(135deg,#1a3a2a,#0d2a1a);color:#7ee7b0}\n",
                ".halogen{background:linear-gradient(135deg,#2a1a3a,#1d0d2a);color:#bc8cff}\n",
                ".lanthanide{background:linear-gradient(135deg,#3a1a2a,#2a0d1a);color:#ff7eb6}\n",
                ".actinide{background:linear-gradient(135deg,#2e1a3a,#1e0d2a);color:#e2b0ff}\n",
                ".unknown{background:linear-gradient(135deg,#1a1a1a,#0d0d0d);color:#8b949e}\n",
            ));
        } else {
            for (i, _cat) in dyn_cats.iter().enumerate() {
                let (fg, bg) = group_colors(i);
                h.push_str(&format!(
                    ".cat-{i}{{background:linear-gradient(135deg,{bg},#0d0d0d);color:{fg}}}\n"
                ));
            }
        }
    }

    h.push_str("</style>\n</head>\n<body>\n\n");

    h.push_str("<h1>");
    push_html_escaped(&mut h, title);
    h.push_str("</h1>\n");
    h.push_str(&format!(
        "<p>{} benchmark entries &mdash; 6 formats: CSV, JSON, YAML, TOML, BMK, Markdown.</p>\n\n",
        entries.len()
    ));

    h.push_str("<div class=\"stats\">\n");
    h.push_str(&format!(
        "<div class=\"stat-card\"><div class=\"num\">{}</div><div class=\"lbl\">Elements</div></div>\n",
        entries.len()
    ));
    h.push_str("<div class=\"stat-card\"><div class=\"num\">6</div><div class=\"lbl\">Formats</div></div>\n");
    let total_files = csv_files.len()
        + json_files.len()
        + yaml_files.len()
        + toml_files.len()
        + bmk_files.len()
        + entries.len();
    h.push_str(&format!(
        "<div class=\"stat-card\"><div class=\"num\">{total_files}</div><div class=\"lbl\">Files</div></div>\n"
    ));
    h.push_str("</div>\n\n");

    if has_grid {
        if is_ptable {
            h.push_str("<h2>Periodic Table</h2>\n");
            h.push_str("<div class=\"legend\">\n");
            for (cls, lbl) in [
                ("nonmetal", "Nonmetal"),
                ("noble-gas", "Noble Gas"),
                ("alkali", "Alkali Metal"),
                ("alkaline", "Alkaline Earth"),
                ("transition", "Transition Metal"),
                ("post-trans", "Post-Transition"),
                ("metalloid", "Metalloid"),
                ("halogen", "Halogen"),
                ("lanthanide", "Lanthanide"),
                ("actinide", "Actinide"),
            ] {
                h.push_str(&format!("<div class=\"legend-item\"><div class=\"legend-swatch {cls}\"></div>{lbl}</div>\n"));
            }
            h.push_str("</div>\n<div class=\"ptable\">\n");

            for entry in entries {
                if let (Some(row), Some(col)) = (entry.grid_row, entry.grid_col) {
                    let sym = entry
                        .tags
                        .iter()
                        .find(|(k, _)| *k == "symbol")
                        .map(|(_, v)| *v)
                        .unwrap_or("?");
                    let cat = entry
                        .tags
                        .iter()
                        .find(|(k, _)| *k == "category")
                        .map(|(_, v)| *v)
                        .unwrap_or("");
                    let name = entry
                        .tags
                        .iter()
                        .find(|(k, _)| *k == "name")
                        .map(|(_, v)| *v)
                        .unwrap_or("");
                    let mass = entry
                        .tags
                        .iter()
                        .find(|(k, _)| *k == "mass")
                        .map(|(_, v)| *v)
                        .unwrap_or("");
                    let z_str = entry.label.split('_').next().unwrap_or("0");
                    let z_disp = z_str.trim_start_matches('0');
                    let z_disp = if z_disp.is_empty() { "0" } else { z_disp };
                    let cls = css_cat(cat);
                    h.push_str(&format!(
                        "<div id=\"el-{}\" class=\"el {cls}\" style=\"grid-row:{row};grid-column:{col}\" title=\"{name} ({z_disp}) - {mass} u - {:.1}ns\" onclick=\"go('{}')\">",
                        entry.label, entry.metrics.avg_time_ns, entry.label
                    ));
                    h.push_str(&format!("<div class=\"z\">{z_disp}</div>"));
                    h.push_str(&format!("<div class=\"sym\">{sym}</div>"));
                    h.push_str(&format!("<div class=\"mass\">{mass}</div>"));
                    h.push_str(&format!(
                        "<div class=\"ns\">{:.0}ns</div>",
                        entry.metrics.avg_time_ns
                    ));
                    h.push_str("</div>\n");
                }
            }

            h.push_str("<div class=\"marker\" style=\"grid-row:9;grid-column:3\">*</div>\n");
            h.push_str("<div class=\"marker\" style=\"grid-row:10;grid-column:3\">**</div>\n");
        } else {
            h.push_str("<h2>Interactive Grid</h2>\n");
            h.push_str("<div class=\"legend\">\n");
            for (i, cat) in dyn_cats.iter().enumerate() {
                h.push_str(&format!(
                    "<div class=\"legend-item\"><div class=\"legend-swatch cat-{i}\"></div>{cat}</div>\n"
                ));
            }
            h.push_str("</div>\n<div class=\"ptable\">\n");

            for entry in entries {
                if let (Some(row), Some(col)) = (entry.grid_row, entry.grid_col) {
                    let tag_sym = entry
                        .tags
                        .iter()
                        .find(|(k, _)| *k == "symbol")
                        .map(|(_, v)| *v);
                    let cat = entry
                        .tags
                        .iter()
                        .find(|(k, _)| *k == "category")
                        .map(|(_, v)| *v)
                        .unwrap_or("");
                    let tag_name = entry
                        .tags
                        .iter()
                        .find(|(k, _)| *k == "name")
                        .map(|(_, v)| *v);

                    let parts: Vec<&str> = entry.label.split('_').collect();
                    let base_parts = if parts.len() > 1 {
                        &parts[1..]
                    } else {
                        &parts[..]
                    };
                    let derived_sym: String = base_parts
                        .iter()
                        .filter_map(|w| w.chars().next())
                        .map(|c| c.to_uppercase().next().unwrap_or(c))
                        .collect();
                    let derived_name: String = base_parts
                        .iter()
                        .enumerate()
                        .map(|(i, w)| {
                            if i > 0 {
                                format!(" {}", w)
                            } else {
                                w.to_string()
                            }
                        })
                        .collect::<String>()
                        .split(' ')
                        .map(|w| {
                            let mut c = w.chars();
                            match c.next() {
                                Some(f) => format!(
                                    "{}{}",
                                    f.to_uppercase().next().unwrap_or(f),
                                    c.as_str()
                                ),
                                None => String::new(),
                            }
                        })
                        .collect::<Vec<_>>()
                        .join(" ");

                    let sym = tag_sym.unwrap_or(&derived_sym);
                    let name_display = tag_name.map(|s| s.to_string()).unwrap_or(derived_name);

                    let ci = dyn_cats.iter().position(|c| *c == cat).unwrap_or(0);
                    h.push_str(&format!(
                        "<div id=\"el-{}\" class=\"el cat-{ci}\" style=\"grid-row:{row};grid-column:{col}\" title=\"{name_display} - {:.1}ns\" onclick=\"go('{}')\">",
                        entry.label, entry.metrics.avg_time_ns, entry.label
                    ));
                    h.push_str(&format!("<div class=\"sym\">{sym}</div>"));
                    h.push_str(&format!("<div class=\"nm\">{name_display}</div>"));
                    h.push_str(&format!(
                        "<div class=\"ns\">{:.0}ns</div>",
                        entry.metrics.avg_time_ns
                    ));
                    h.push_str("</div>\n");
                }
            }
        }
        h.push_str("</div>\n\n");

        h.push_str("<div id=\"detail\" class=\"detail\">\n");
        h.push_str("<button class=\"close\" onclick=\"closeDetail()\">&times;</button>\n");
        h.push_str("<div class=\"dtitle\" id=\"d-title\"></div>\n");
        h.push_str("<div id=\"d-tags\"></div>\n");
        h.push_str("<div class=\"dk\">Precision</div><div class=\"dv\" id=\"d-prec\"></div>\n");
        h.push_str("<div class=\"dk\">Iterations</div><div class=\"dv\" id=\"d-iter\"></div>\n");
        h.push_str("<div class=\"dk\">Avg Time</div><div class=\"dv\" id=\"d-avg\"></div>\n");
        h.push_str("<div class=\"dk\">Min Time</div><div class=\"dv\" id=\"d-min\"></div>\n");
        h.push_str("<div class=\"dk\">Max Time</div><div class=\"dv\" id=\"d-max\"></div>\n");
        h.push_str("<div class=\"dk\">Std Dev</div><div class=\"dv\" id=\"d-std\"></div>\n");
        h.push_str("<div class=\"dk\">Iters/sec</div><div class=\"dv\" id=\"d-ips\"></div>\n");
        h.push_str("<div class=\"files\" id=\"d-files\"></div>\n");
        h.push_str("</div>\n\n");
    }

    h.push_str("<h2>Data Formats</h2>\n<div class=\"tabs\">\n");
    h.push_str("<div class=\"tab active\" onclick=\"switchTab(event,'csv')\">CSV</div>\n");
    h.push_str("<div class=\"tab\" onclick=\"switchTab(event,'json')\">JSON</div>\n");
    h.push_str("<div class=\"tab\" onclick=\"switchTab(event,'yaml')\">YAML</div>\n");
    h.push_str("<div class=\"tab\" onclick=\"switchTab(event,'toml')\">TOML</div>\n");
    h.push_str("<div class=\"tab\" onclick=\"switchTab(event,'bmk')\">BMK</div>\n");
    h.push_str("<div class=\"tab\" onclick=\"switchTab(event,'mdprev')\">Markdown</div>\n");
    h.push_str("</div>\n");

    h.push_str("<div id=\"csv\" class=\"panel active\">\n");
    h.push_str("<select class=\"fsel\" onchange=\"showFile('csv',this.value)\">\n");
    for fname in csv_files.keys() {
        h.push_str("<option>");
        push_html_escaped(&mut h, fname);
        h.push_str("</option>\n");
    }
    h.push_str("</select>\n");
    h.push_str(
        "<div id=\"csv-code\" style=\"overflow-x:auto;max-height:600px;overflow-y:auto\"></div>\n",
    );
    h.push_str("</div>\n");

    for (id, files) in [
        ("json", json_files),
        ("yaml", yaml_files),
        ("toml", toml_files),
        ("bmk", bmk_files),
    ] {
        h.push_str(&format!("<div id=\"{id}\" class=\"panel\">\n"));
        h.push_str(&format!(
            "<select class=\"fsel\" onchange=\"showFile('{id}',this.value)\">\n"
        ));
        for fname in files.keys() {
            h.push_str("<option>");
            push_html_escaped(&mut h, fname);
            h.push_str("</option>\n");
        }
        h.push_str("</select>\n");
        h.push_str(&format!("<pre><code id=\"{id}-code\"></code></pre>\n"));
        h.push_str("</div>\n");
    }

    h.push_str("<div id=\"mdprev\" class=\"panel\" style=\"max-height:700px;overflow-y:auto\">\n");
    h.push_str(&render_md_html(md_content));
    h.push_str("</div>\n\n");

    build_charts(&mut h, entries);

    h.push_str("<script>\nvar F={\n");
    let format_list: [(&str, &BTreeMap<String, String>); 5] = [
        ("csv", csv_files),
        ("json", json_files),
        ("yaml", yaml_files),
        ("toml", toml_files),
        ("bmk", bmk_files),
    ];
    for (fi, (id, files)) in format_list.iter().enumerate() {
        if fi > 0 {
            h.push_str(",\n");
        }
        h.push_str(id);
        h.push_str(":{\n");
        for (j, (fname, content)) in files.iter().enumerate() {
            if j > 0 {
                h.push_str(",\n");
            }
            h.push('"');
            push_js_escaped(&mut h, fname);
            h.push_str("\":\"");
            push_js_escaped(&mut h, content);
            h.push('"');
        }
        h.push('}');
    }
    h.push_str("\n};\n");

    h.push_str("var D={\n");
    for (i, entry) in entries.iter().enumerate() {
        if i > 0 {
            h.push_str(",\n");
        }
        let m = entry.metrics;
        h.push('"');
        push_js_escaped(&mut h, entry.label);
        h.push_str("\":{tags:{");
        for (ti, (k, v)) in entry.tags.iter().enumerate() {
            if ti > 0 {
                h.push(',');
            }
            h.push('"');
            push_js_escaped(&mut h, k);
            h.push_str("\":\"");
            push_js_escaped(&mut h, v);
            h.push('"');
        }
        h.push_str(&format!(
            "}},p:\"{}\",it:{},avg:{:.2},mn:{:.2},mx:{:.2},sd:{:.4},ips:{:.2}",
            m.precision,
            m.iterations,
            m.avg_time_ns,
            m.min_time_ns,
            m.max_time_ns,
            m.time_stddev,
            m.iterations_per_sec
        ));
        h.push('}');
    }
    h.push_str("\n};\n");

    h.push_str(concat!(
        "function switchTab(e,id){document.querySelectorAll('.tab').forEach(function(t){t.classList.remove('active')});document.querySelectorAll('.panel').forEach(function(p){p.classList.remove('active')});e.target.classList.add('active');document.getElementById(id).classList.add('active')}\n",
        "function showFile(fmt,name){var c=F[fmt];if(!c)return;if(fmt==='csv'){var el=document.getElementById('csv-code');if(!el)return;var raw=c[name]||'';var lines=raw.split('\\n').filter(function(l){return l.length>0});if(lines.length===0){el.innerHTML='';return;}var t='<table>';var hdr=lines[0].split(',');t+='<thead><tr>';for(var i=0;i<hdr.length;i++)t+='<th>'+hdr[i].replace(/</g,'&lt;')+'</th>';t+='</tr></thead><tbody>';for(var r=1;r<lines.length;r++){var cols=lines[r].split(',');t+='<tr>';for(var j=0;j<cols.length;j++)t+='<td>'+cols[j].replace(/</g,'&lt;')+'</td>';t+='</tr>';}t+='</tbody></table>';el.innerHTML=t}else{var el=document.getElementById(fmt+'-code');if(el)el.textContent=c[name]||''}}\n",
        "function go(id){var d=D[id];if(!d)return;var p=document.getElementById('detail');if(p){p.className='detail open';",
        "document.getElementById('d-title').textContent=id;",
        "var tg=document.getElementById('d-tags');tg.innerHTML='';var tkeys=Object.keys(d.tags);for(var i=0;i<tkeys.length;i++){var r=document.createElement('div');r.className='dk';r.textContent=tkeys[i];tg.appendChild(r);var v=document.createElement('div');v.className='dv';v.textContent=d.tags[tkeys[i]];tg.appendChild(v);}",
        "document.getElementById('d-prec').textContent=d.p;",
        "document.getElementById('d-iter').textContent=d.it;",
        "document.getElementById('d-avg').textContent=d.avg.toFixed(2)+' ns';",
        "document.getElementById('d-min').textContent=d.mn.toFixed(2)+' ns';",
        "document.getElementById('d-max').textContent=d.mx.toFixed(2)+' ns';",
        "document.getElementById('d-std').textContent=d.sd.toFixed(4);",
        "document.getElementById('d-ips').textContent=d.ips.toFixed(2);",
        "var fb=document.getElementById('d-files');if(fb){fb.innerHTML='';['json','yaml','toml'].forEach(function(fmt){var fk=id+'.'+fmt;if(F[fmt]&&F[fmt][fk]){var b=document.createElement('button');b.className='fbtn';b.textContent=fmt.toUpperCase();(function(f,k){b.onclick=function(){var ts=document.querySelectorAll('.tab');for(var i=0;i<ts.length;i++){if(ts[i].textContent.toLowerCase()===f){switchTab({target:ts[i]},f);break}}var s=document.querySelector('#'+f+' select');if(s){s.value=k;showFile(f,k)}}})(fmt,fk);fb.appendChild(b)}})}",
        "p.scrollIntoView({behavior:'smooth',block:'center'})}",
        "var el=document.getElementById('el-'+id);if(el){el.classList.add('highlight');setTimeout(function(){el.classList.remove('highlight')},2000)}}\n",
        "function closeDetail(){var p=document.getElementById('detail');if(p)p.className='detail'}\n",
    ));
    h.push_str("['csv','json','yaml','toml','bmk'].forEach(function(fmt){var s=document.querySelector('#'+fmt+' select');if(s&&s.options.length>0)showFile(fmt,s.options[0].text)});\n");
    h.push_str("</script>\n</body>\n</html>");

    h
}
