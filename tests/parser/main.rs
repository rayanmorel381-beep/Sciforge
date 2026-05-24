use sciforge_hub::parser::csv::{validate_csv, write_csv};
use sciforge_hub::parser::html::validate_html;
use sciforge_hub::parser::json::validate_json;
use sciforge_hub::parser::markdown::validate_md;
use sciforge_hub::parser::yaml::validate_yaml;
use std::fs;
use std::path::Path;

mod csv;
mod html;
mod json;
mod markdown;
mod toml;
mod yaml;

#[test]
fn generate_csv_file() {
    let dir = Path::new("output/tests/parser");
    fs::create_dir_all(dir).unwrap();

    let header = &[
        "symbol",
        "name",
        "number",
        "mass",
        "group",
        "period",
        "category",
        "density",
        "melting_point",
        "boiling_point",
    ];
    let rows = vec![
        vec![
            "H".into(),
            "Hydrogen".into(),
            "1".into(),
            "1.008".into(),
            "1".into(),
            "1".into(),
            "nonmetal".into(),
            "0.00008988".into(),
            "-259.16".into(),
            "-252.87".into(),
        ],
        vec![
            "He".into(),
            "Helium".into(),
            "2".into(),
            "4.0026".into(),
            "18".into(),
            "1".into(),
            "noble gas".into(),
            "0.0001785".into(),
            "-272.2".into(),
            "-268.93".into(),
        ],
        vec![
            "Li".into(),
            "Lithium".into(),
            "3".into(),
            "6.941".into(),
            "1".into(),
            "2".into(),
            "alkali metal".into(),
            "0.534".into(),
            "180.54".into(),
            "1342".into(),
        ],
        vec![
            "Be".into(),
            "Beryllium".into(),
            "4".into(),
            "9.0122".into(),
            "2".into(),
            "2".into(),
            "alkaline earth".into(),
            "1.85".into(),
            "1287".into(),
            "2470".into(),
        ],
        vec![
            "B".into(),
            "Boron".into(),
            "5".into(),
            "10.81".into(),
            "13".into(),
            "2".into(),
            "metalloid".into(),
            "2.34".into(),
            "2075".into(),
            "4000".into(),
        ],
        vec![
            "C".into(),
            "Carbon".into(),
            "6".into(),
            "12.011".into(),
            "14".into(),
            "2".into(),
            "nonmetal".into(),
            "2.267".into(),
            "3550".into(),
            "4027".into(),
        ],
        vec![
            "N".into(),
            "Nitrogen".into(),
            "7".into(),
            "14.007".into(),
            "15".into(),
            "2".into(),
            "nonmetal".into(),
            "0.0012506".into(),
            "-210.0".into(),
            "-195.79".into(),
        ],
        vec![
            "O".into(),
            "Oxygen".into(),
            "8".into(),
            "15.999".into(),
            "16".into(),
            "2".into(),
            "nonmetal".into(),
            "0.001429".into(),
            "-218.79".into(),
            "-182.96".into(),
        ],
        vec![
            "F".into(),
            "Fluorine".into(),
            "9".into(),
            "18.998".into(),
            "17".into(),
            "2".into(),
            "halogen".into(),
            "0.001696".into(),
            "-219.67".into(),
            "-188.11".into(),
        ],
        vec![
            "Ne".into(),
            "Neon".into(),
            "10".into(),
            "20.180".into(),
            "18".into(),
            "2".into(),
            "noble gas".into(),
            "0.0008999".into(),
            "-248.59".into(),
            "-246.08".into(),
        ],
        vec![
            "Na".into(),
            "Sodium".into(),
            "11".into(),
            "22.990".into(),
            "1".into(),
            "3".into(),
            "alkali metal".into(),
            "0.971".into(),
            "97.794".into(),
            "882.94".into(),
        ],
        vec![
            "Mg".into(),
            "Magnesium".into(),
            "12".into(),
            "24.305".into(),
            "2".into(),
            "3".into(),
            "alkaline earth".into(),
            "1.738".into(),
            "650".into(),
            "1091".into(),
        ],
        vec![
            "Al".into(),
            "Aluminium".into(),
            "13".into(),
            "26.982".into(),
            "13".into(),
            "3".into(),
            "post-transition".into(),
            "2.698".into(),
            "660.32".into(),
            "2519".into(),
        ],
        vec![
            "Si".into(),
            "Silicon".into(),
            "14".into(),
            "28.085".into(),
            "14".into(),
            "3".into(),
            "metalloid".into(),
            "2.3296".into(),
            "1414".into(),
            "3265".into(),
        ],
        vec![
            "P".into(),
            "Phosphorus".into(),
            "15".into(),
            "30.974".into(),
            "15".into(),
            "3".into(),
            "nonmetal".into(),
            "1.82".into(),
            "44.15".into(),
            "280.5".into(),
        ],
        vec![
            "S".into(),
            "Sulfur".into(),
            "16".into(),
            "32.06".into(),
            "16".into(),
            "3".into(),
            "nonmetal".into(),
            "2.067".into(),
            "115.21".into(),
            "444.67".into(),
        ],
        vec![
            "Cl".into(),
            "Chlorine".into(),
            "17".into(),
            "35.45".into(),
            "17".into(),
            "3".into(),
            "halogen".into(),
            "0.003214".into(),
            "-101.5".into(),
            "-34.04".into(),
        ],
        vec![
            "Ar".into(),
            "Argon".into(),
            "18".into(),
            "39.948".into(),
            "18".into(),
            "3".into(),
            "noble gas".into(),
            "0.0017837".into(),
            "-189.34".into(),
            "-185.85".into(),
        ],
        vec![
            "K".into(),
            "Potassium".into(),
            "19".into(),
            "39.098".into(),
            "1".into(),
            "4".into(),
            "alkali metal".into(),
            "0.862".into(),
            "63.5".into(),
            "759".into(),
        ],
        vec![
            "Ca".into(),
            "Calcium".into(),
            "20".into(),
            "40.078".into(),
            "2".into(),
            "4".into(),
            "alkaline earth".into(),
            "1.54".into(),
            "842".into(),
            "1484".into(),
        ],
    ];

    let csv = write_csv(header, &rows);
    fs::write(dir.join("parser.csv"), &csv).unwrap();
    let read = fs::read(dir.join("parser.csv")).unwrap();
    assert!(validate_csv(&read).is_ok());
    assert!(csv.lines().count() >= 21);
}

#[test]
fn generate_json_file() {
    let dir = Path::new("output/tests/parser");
    fs::create_dir_all(dir).unwrap();
    let json = r#"{
  "periodic_table": {
    "title": "First 10 Elements",
    "elements": [
      {"symbol": "H",  "name": "Hydrogen",  "number": 1,  "mass": 1.008,   "category": "nonmetal",       "electron_config": "1s1",             "electronegativity": 2.20, "discovered": 1766},
      {"symbol": "He", "name": "Helium",    "number": 2,  "mass": 4.0026,  "category": "noble gas",      "electron_config": "1s2",             "electronegativity": null, "discovered": 1868},
      {"symbol": "Li", "name": "Lithium",   "number": 3,  "mass": 6.941,   "category": "alkali metal",   "electron_config": "[He] 2s1",        "electronegativity": 0.98, "discovered": 1817},
      {"symbol": "Be", "name": "Beryllium", "number": 4,  "mass": 9.0122,  "category": "alkaline earth", "electron_config": "[He] 2s2",        "electronegativity": 1.57, "discovered": 1798},
      {"symbol": "B",  "name": "Boron",     "number": 5,  "mass": 10.81,   "category": "metalloid",      "electron_config": "[He] 2s2 2p1",   "electronegativity": 2.04, "discovered": 1808},
      {"symbol": "C",  "name": "Carbon",    "number": 6,  "mass": 12.011,  "category": "nonmetal",       "electron_config": "[He] 2s2 2p2",   "electronegativity": 2.55, "discovered": -3750},
      {"symbol": "N",  "name": "Nitrogen",  "number": 7,  "mass": 14.007,  "category": "nonmetal",       "electron_config": "[He] 2s2 2p3",   "electronegativity": 3.04, "discovered": 1772},
      {"symbol": "O",  "name": "Oxygen",    "number": 8,  "mass": 15.999,  "category": "nonmetal",       "electron_config": "[He] 2s2 2p4",   "electronegativity": 3.44, "discovered": 1774},
      {"symbol": "F",  "name": "Fluorine",  "number": 9,  "mass": 18.998,  "category": "halogen",        "electron_config": "[He] 2s2 2p5",   "electronegativity": 3.98, "discovered": 1886},
      {"symbol": "Ne", "name": "Neon",      "number": 10, "mass": 20.180,  "category": "noble gas",      "electron_config": "[He] 2s2 2p6",   "electronegativity": null, "discovered": 1898}
    ],
    "metadata": {
      "source": "IUPAC 2024",
      "units": {"mass": "u", "electronegativity": "Pauling"},
      "total_known_elements": 118,
      "naturally_occurring": 94
    }
  }
}"#;
    fs::write(dir.join("parser.json"), json).unwrap();
    let read = fs::read(dir.join("parser.json")).unwrap();
    assert!(validate_json(&read).is_ok());
    assert!(json.len() > 1500);
}

#[test]
fn generate_yaml_file() {
    let dir = Path::new("output/tests/parser");
    fs::create_dir_all(dir).unwrap();
    let yaml = "\
title: Periodic Table - First 10 Elements\n\
source: IUPAC 2024\n\
elements:\n\
  - symbol: H\n\
    name: Hydrogen\n\
    number: 1\n\
    mass: 1.008\n\
    category: nonmetal\n\
    electron_config: 1s1\n\
  - symbol: He\n\
    name: Helium\n\
    number: 2\n\
    mass: 4.0026\n\
    category: noble gas\n\
    electron_config: 1s2\n\
  - symbol: Li\n\
    name: Lithium\n\
    number: 3\n\
    mass: 6.941\n\
    category: alkali metal\n\
    electron_config: 2s1\n\
  - symbol: Be\n\
    name: Beryllium\n\
    number: 4\n\
    mass: 9.0122\n\
    category: alkaline earth\n\
    electron_config: 2s2\n\
  - symbol: B\n\
    name: Boron\n\
    number: 5\n\
    mass: 10.81\n\
    category: metalloid\n\
    electron_config: 2s2 2p1\n\
  - symbol: C\n\
    name: Carbon\n\
    number: 6\n\
    mass: 12.011\n\
    category: nonmetal\n\
    electron_config: 2s2 2p2\n\
  - symbol: N\n\
    name: Nitrogen\n\
    number: 7\n\
    mass: 14.007\n\
    category: nonmetal\n\
    electron_config: 2s2 2p3\n\
  - symbol: O\n\
    name: Oxygen\n\
    number: 8\n\
    mass: 15.999\n\
    category: nonmetal\n\
    electron_config: 2s2 2p4\n\
  - symbol: F\n\
    name: Fluorine\n\
    number: 9\n\
    mass: 18.998\n\
    category: halogen\n\
    electron_config: 2s2 2p5\n\
  - symbol: Ne\n\
    name: Neon\n\
    number: 10\n\
    mass: 20.180\n\
    category: noble gas\n\
    electron_config: 2s2 2p6\n";
    fs::write(dir.join("parser.yaml"), yaml).unwrap();
    let read = fs::read(dir.join("parser.yaml")).unwrap();
    assert!(validate_yaml(&read).is_ok());
    assert!(yaml.lines().count() >= 50);
}

#[test]
fn generate_markdown_file() {
    let dir = Path::new("output/tests/parser");
    fs::create_dir_all(dir).unwrap();
    let md = "\
# Periodic Table of Elements\n\n\
## Overview\n\n\
The periodic table organizes chemical elements by atomic number, electron configuration, and recurring chemical properties.\n\n\
> The modern periodic table was first proposed by Dmitri Mendeleev in 1869.\n\n\
## First 10 Elements\n\n\
| Z | Symbol | Name | Mass (u) | Category | Config |\n\
|---|--------|------|----------|----------|--------|\n\
| 1 | H | Hydrogen | 1.008 | Nonmetal | 1s1 |\n\
| 2 | He | Helium | 4.003 | Noble gas | 1s2 |\n\
| 3 | Li | Lithium | 6.941 | Alkali metal | [He] 2s1 |\n\
| 4 | Be | Beryllium | 9.012 | Alkaline earth | [He] 2s2 |\n\
| 5 | B | Boron | 10.81 | Metalloid | [He] 2s2 2p1 |\n\
| 6 | C | Carbon | 12.01 | Nonmetal | [He] 2s2 2p2 |\n\
| 7 | N | Nitrogen | 14.01 | Nonmetal | [He] 2s2 2p3 |\n\
| 8 | O | Oxygen | 16.00 | Nonmetal | [He] 2s2 2p4 |\n\
| 9 | F | Fluorine | 19.00 | Halogen | [He] 2s2 2p5 |\n\
| 10 | Ne | Neon | 20.18 | Noble gas | [He] 2s2 2p6 |\n\n\
## Categories\n\n\
1. Nonmetals\n\
2. Noble gases\n\
3. Alkali metals\n\
4. Alkaline earth metals\n\
5. Metalloids\n\
6. Halogens\n\
7. Transition metals\n\
8. Post-transition metals\n\
9. Lanthanides\n\
10. Actinides\n\n\
---\n\n\
## Electron Configuration\n\n\
```\n\
Period 1: 1s\n\
Period 2: 2s 2p\n\
Period 3: 3s 3p\n\
Period 4: 4s 3d 4p\n\
Period 5: 5s 4d 5p\n\
Period 6: 6s 4f 5d 6p\n\
Period 7: 7s 5f 6d 7p\n\
```\n\n\
## Notes\n\n\
- Hydrogen is unique: it can behave as both a metal and a nonmetal\n\
- Helium has the highest ionization energy of all elements\n\
- Carbon forms more compounds than any other element\n";
    fs::write(dir.join("parser.md"), md).unwrap();
    let read = fs::read(dir.join("parser.md")).unwrap();
    assert!(validate_md(&read).is_ok());
    assert!(md.lines().count() >= 40);
}

#[test]
fn generate_html_file() {
    let dir = Path::new("output/tests/parser");
    fs::create_dir_all(dir).unwrap();
    let html = "\
<!DOCTYPE html>\n\
<html lang=\"en\">\n\
<head>\n\
<meta charset=\"utf-8\">\n\
<title>Periodic Table - First 20 Elements (IUPAC 2024)</title>\n\
<style>\n\
*{margin:0;padding:0;box-sizing:border-box}\n\
body{font-family:system-ui,sans-serif;background:#0d1117;color:#c9d1d9;padding:2em;max-width:1200px;margin:0 auto}\n\
h1{color:#58a6ff;margin-bottom:.5em;font-size:2em}\n\
h2{color:#79c0ff;margin:1.5em 0 .5em;border-bottom:1px solid #21262d;padding-bottom:.3em}\n\
p{margin:.5em 0;line-height:1.6}\n\
table{border-collapse:collapse;width:100%;margin:1em 0}\n\
th,td{border:1px solid #30363d;padding:6px 10px;text-align:left;font-size:.85em}\n\
th{background:#161b22;color:#58a6ff;position:sticky;top:0}\n\
tr:nth-child(even){background:#161b22}\n\
pre{background:#161b22;padding:1em;border-radius:6px;overflow-x:auto;font-size:.8em}\n\
code{font-family:ui-monospace,monospace}\n\
.legend{display:flex;flex-wrap:wrap;gap:.8em;margin:.8em 0;align-items:center}\n\
.legend-item{display:flex;align-items:center;gap:6px;font-size:.75em;color:#8b949e}\n\
.legend-swatch{width:18px;height:18px;border-radius:4px;border:1px solid rgba(255,255,255,.1)}\n\
.nonmetal{background:linear-gradient(135deg,#1a3a1a,#0d2610);color:#7ee787}\n\
.noble-gas{background:linear-gradient(135deg,#3a1a2e,#2a0d1e);color:#d2a8ff}\n\
.alkali{background:linear-gradient(135deg,#3a2a0a,#2a1d05);color:#ffa657}\n\
.alkaline{background:linear-gradient(135deg,#3a2a1a,#2a1d0d);color:#f0b27a}\n\
.metalloid{background:linear-gradient(135deg,#1a3a2a,#0d2a1a);color:#7ee7b0}\n\
.halogen{background:linear-gradient(135deg,#2a1a3a,#1d0d2a);color:#bc8cff}\n\
.cat-cell{padding:4px 10px;border-radius:4px;font-weight:600;font-size:.8em}\n\
.note{font-style:italic;color:#8b949e;margin-top:2em;font-size:.85em}\n\
ul{list-style:none;padding:0}\n\
li{padding:.35em 0;color:#c9d1d9}\n\
li span{display:inline-block;padding:2px 10px;border-radius:4px;font-weight:600;font-size:.85em;margin-right:.5em}\n\
</style>\n\
</head>\n\
<body>\n\n\
<h1>Periodic Table of Elements</h1>\n\
<p>The first 20 elements organized by atomic number &mdash; IUPAC 2024.</p>\n\n\
<h2>Legend</h2>\n\
<div class=\"legend\">\n\
<div class=\"legend-item\"><div class=\"legend-swatch nonmetal\"></div>Nonmetal</div>\n\
<div class=\"legend-item\"><div class=\"legend-swatch noble-gas\"></div>Noble Gas</div>\n\
<div class=\"legend-item\"><div class=\"legend-swatch alkali\"></div>Alkali Metal</div>\n\
<div class=\"legend-item\"><div class=\"legend-swatch alkaline\"></div>Alkaline Earth</div>\n\
<div class=\"legend-item\"><div class=\"legend-swatch metalloid\"></div>Metalloid</div>\n\
<div class=\"legend-item\"><div class=\"legend-swatch halogen\"></div>Halogen</div>\n\
</div>\n\n\
<h2>Elements</h2>\n\
<table>\n\
<thead>\n\
<tr><th>Z</th><th>Symbol</th><th>Name</th><th>Mass (u)</th><th>Category</th><th>Config</th></tr>\n\
</thead>\n\
<tbody>\n\
<tr><td>1</td><td>H</td><td>Hydrogen</td><td>1.008</td><td><span class=\"cat-cell nonmetal\">Nonmetal</span></td><td><code>1s<sup>1</sup></code></td></tr>\n\
<tr><td>2</td><td>He</td><td>Helium</td><td>4.003</td><td><span class=\"cat-cell noble-gas\">Noble gas</span></td><td><code>1s<sup>2</sup></code></td></tr>\n\
<tr><td>3</td><td>Li</td><td>Lithium</td><td>6.941</td><td><span class=\"cat-cell alkali\">Alkali metal</span></td><td><code>[He] 2s<sup>1</sup></code></td></tr>\n\
<tr><td>4</td><td>Be</td><td>Beryllium</td><td>9.012</td><td><span class=\"cat-cell alkaline\">Alkaline earth</span></td><td><code>[He] 2s<sup>2</sup></code></td></tr>\n\
<tr><td>5</td><td>B</td><td>Boron</td><td>10.81</td><td><span class=\"cat-cell metalloid\">Metalloid</span></td><td><code>[He] 2s<sup>2</sup> 2p<sup>1</sup></code></td></tr>\n\
<tr><td>6</td><td>C</td><td>Carbon</td><td>12.01</td><td><span class=\"cat-cell nonmetal\">Nonmetal</span></td><td><code>[He] 2s<sup>2</sup> 2p<sup>2</sup></code></td></tr>\n\
<tr><td>7</td><td>N</td><td>Nitrogen</td><td>14.01</td><td><span class=\"cat-cell nonmetal\">Nonmetal</span></td><td><code>[He] 2s<sup>2</sup> 2p<sup>3</sup></code></td></tr>\n\
<tr><td>8</td><td>O</td><td>Oxygen</td><td>16.00</td><td><span class=\"cat-cell nonmetal\">Nonmetal</span></td><td><code>[He] 2s<sup>2</sup> 2p<sup>4</sup></code></td></tr>\n\
<tr><td>9</td><td>F</td><td>Fluorine</td><td>19.00</td><td><span class=\"cat-cell halogen\">Halogen</span></td><td><code>[He] 2s<sup>2</sup> 2p<sup>5</sup></code></td></tr>\n\
<tr><td>10</td><td>Ne</td><td>Neon</td><td>20.18</td><td><span class=\"cat-cell noble-gas\">Noble gas</span></td><td><code>[He] 2s<sup>2</sup> 2p<sup>6</sup></code></td></tr>\n\
<tr><td>11</td><td>Na</td><td>Sodium</td><td>22.99</td><td><span class=\"cat-cell alkali\">Alkali metal</span></td><td><code>[Ne] 3s<sup>1</sup></code></td></tr>\n\
<tr><td>12</td><td>Mg</td><td>Magnesium</td><td>24.31</td><td><span class=\"cat-cell alkaline\">Alkaline earth</span></td><td><code>[Ne] 3s<sup>2</sup></code></td></tr>\n\
<tr><td>13</td><td>Al</td><td>Aluminium</td><td>26.98</td><td><span class=\"cat-cell alkaline\">Post-transition</span></td><td><code>[Ne] 3s<sup>2</sup> 3p<sup>1</sup></code></td></tr>\n\
<tr><td>14</td><td>Si</td><td>Silicon</td><td>28.09</td><td><span class=\"cat-cell metalloid\">Metalloid</span></td><td><code>[Ne] 3s<sup>2</sup> 3p<sup>2</sup></code></td></tr>\n\
<tr><td>15</td><td>P</td><td>Phosphorus</td><td>30.97</td><td><span class=\"cat-cell nonmetal\">Nonmetal</span></td><td><code>[Ne] 3s<sup>2</sup> 3p<sup>3</sup></code></td></tr>\n\
<tr><td>16</td><td>S</td><td>Sulfur</td><td>32.06</td><td><span class=\"cat-cell nonmetal\">Nonmetal</span></td><td><code>[Ne] 3s<sup>2</sup> 3p<sup>4</sup></code></td></tr>\n\
<tr><td>17</td><td>Cl</td><td>Chlorine</td><td>35.45</td><td><span class=\"cat-cell halogen\">Halogen</span></td><td><code>[Ne] 3s<sup>2</sup> 3p<sup>5</sup></code></td></tr>\n\
<tr><td>18</td><td>Ar</td><td>Argon</td><td>39.95</td><td><span class=\"cat-cell noble-gas\">Noble gas</span></td><td><code>[Ne] 3s<sup>2</sup> 3p<sup>6</sup></code></td></tr>\n\
<tr><td>19</td><td>K</td><td>Potassium</td><td>39.10</td><td><span class=\"cat-cell alkali\">Alkali metal</span></td><td><code>[Ar] 4s<sup>1</sup></code></td></tr>\n\
<tr><td>20</td><td>Ca</td><td>Calcium</td><td>40.08</td><td><span class=\"cat-cell alkaline\">Alkaline earth</span></td><td><code>[Ar] 4s<sup>2</sup></code></td></tr>\n\
</tbody>\n\
</table>\n\n\
<h2>Categories</h2>\n\
<ul>\n\
<li><span class=\"nonmetal\">Nonmetal</span> H, C, N, O, P, S, Se</li>\n\
<li><span class=\"noble-gas\">Noble gas</span> He, Ne, Ar, Kr, Xe, Rn, Og</li>\n\
<li><span class=\"alkali\">Alkali metal</span> Li, Na, K, Rb, Cs, Fr</li>\n\
<li><span class=\"alkaline\">Alkaline earth</span> Be, Mg, Ca, Sr, Ba, Ra</li>\n\
<li><span class=\"metalloid\">Metalloid</span> B, Si, Ge, As, Sb, Te</li>\n\
<li><span class=\"halogen\">Halogen</span> F, Cl, Br, I, At, Ts</li>\n\
</ul>\n\n\
<h2>Electron Configuration</h2>\n\
<pre><code>\
Period 1: 1s\n\
Period 2: 2s 2p\n\
Period 3: 3s 3p\n\
Period 4: 4s 3d 4p\n\
Period 5: 5s 4d 5p\n\
Period 6: 6s 4f 5d 6p\n\
Period 7: 7s 5f 6d 7p\
</code></pre>\n\n\
<p class=\"note\">Data from IUPAC Periodic Table of the Elements, 2024 edition.</p>\n\
</body>\n\
</html>";
    fs::write(dir.join("parser.html"), html).unwrap();
    let read = fs::read(dir.join("parser.html")).unwrap();
    assert!(validate_html(&read).is_ok());
    assert!(html.len() > 2000);
}
