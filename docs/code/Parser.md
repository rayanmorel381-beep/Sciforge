# Parser Source Code Guide

This page documents the source implementation behind `sciforge::parser`, including lexer/parser/value/error layering per format.

## Source Coverage

### What is explained
- File-level implementation layout in `src/parser/`.
- Main parsing algorithms and where they are implemented.
- Runtime parse flow from byte cursor to typed value tree.

### Visibility and external access
- `parser` is public from `src/lib.rs` (`pub mod parser;`).
- It can be consumed directly as `sciforge::parser::{csv,json,yaml,markdown,html,toml}`.
- Limit-based parse variants are recommended for untrusted inputs.


## Source Code Explanation

### Relevant file structure
- Main implementation: `src/parser/`
- Module entry point: `src/parser/mod.rs`
- Hub routing (when applicable): `(n/a - parser is direct)`

### Internal execution flow
1. The module exposes format subdomains through `mod.rs` and targeted `pub use` exports.
2. Each format combines lexer/parser/value/error files into a coherent parsing pipeline.
3. Callers invoke parse or validate functions directly; no Hub dispatcher is required.

### What to check while reading code
- Exact signature: input/output types and argument order.
- Lifetime and borrowing rules for zero-copy values.
- Depth/size limit enforcement for untrusted inputs.

## Modules

- `csv` — RFC-4180 CSV parser
- `json` — RFC-8259 JSON parser
- `yaml` — YAML 1.2 subset parser
- `markdown` — CommonMark Markdown parser
- `html` — HTML5-like parser
- `toml` — TOML v1.0 parser

All parsers share the same design philosophy:
- Parse from `&[u8]` bytes without copying
- Return a `Result<*Value<'_>, *Error>` with lifetime tied to input
- Provide `validate_*` for syntax checking without full allocation
- Provide `*_with_limits` variants for resource-bounded parsing

---

## csv API (27 items)

### Structs

| Struct |
|--------|
| `CsvErrorPosition` |
| `CsvError` |
| `Cursor` |
| `CsvLimits` |
| `CsvParser` |

### Enums

| Enum |
|------|
| `CsvErrorKind` |
| `CsvValue` |

### Functions

| Function | Signature |
|----------|-----------|
| `line_column` | `(&self, input: &[u8]) -> CsvErrorPosition` |
| `peek` | `(&self) -> Option<u8>` |
| `advance` | `(&mut self, n: usize)` |
| `next` | `(&mut self) -> Option<u8>` |
| `parse` | `(mut self) -> Result<CsvValue<'a>, CsvError>` |
| `validate` | `(mut self) -> Result<(), CsvError>` |
| `parse_csv` | `(bytes: &[u8]) -> Result<CsvValue<'_>, CsvError>` |
| `parse_csv_with_limits` | `(bytes: &[u8], limits: CsvLimits) -> Result<CsvValue<'_>, CsvError>` |
| `validate_csv` | `(bytes: &[u8]) -> Result<(), CsvError>` |
| `write_csv` | `(header: &[&str], rows: &[Vec<String>]) -> String` |

---

## html API (38 items)

### Structs

| Struct |
|--------|
| `HtmlErrorPosition` |
| `HtmlError` |
| `Cursor` |
| `HtmlLimits` |
| `HtmlParser` |

### Enums

| Enum |
|------|
| `HtmlErrorKind` |
| `HtmlValue` |

### Functions

| Function | Signature |
|----------|-----------|
| `validate_entity` | `(cursor: &mut Cursor<'_>) -> Result<(), HtmlError>` |
| `line_column` | `(&self, input: &[u8]) -> HtmlErrorPosition` |
| `is_eof` | `(&self) -> bool` |
| `peek` | `(&self) -> Option<u8>` |
| `advance` | `(&mut self, n: usize)` |
| `remaining` | `(&self) -> &'a [u8]` |
| `starts_with` | `(&self, needle: &[u8]) -> bool` |
| `skip_ws` | `(&mut self)` |
| `read_while<F: Fn` | `(u8) -> bool>(&mut self, pred: F) -> &'a [u8]` |
| `read_tag_name` | `(&mut self) -> Result<&'a str, HtmlError>` |
| `parse` | `(mut self) -> Result<HtmlValue<'a>, HtmlError>` |
| `validate` | `(mut self) -> Result<(), HtmlError>` |
| `parse_html` | `(bytes: &[u8]) -> Result<HtmlValue<'_>, HtmlError>` |
| `parse_html_with_max_depth` | `( bytes: &[u8], max_depth: usize, ) -> Result<HtmlValue<'_>, HtmlError>` |
| `parse_html_with_limits` | `( bytes: &[u8], limits: HtmlLimits, ) -> Result<HtmlValue<'_>, HtmlError>` |
| `validate_html` | `(bytes: &[u8]) -> Result<(), HtmlError>` |
| `push_html_escaped` | `(out: &mut String, text: &str)` |
| `push_html_row` | `(out: &mut String, label: &str, value: &str)` |
| `push_js_escaped` | `(out: &mut String, s: &str)` |

---

## json API (28 items)

### Structs

| Struct |
|--------|
| `JsonErrorPosition` |
| `JsonError` |
| `JsonLimits` |
| `JsonParser` |

### Enums

| Enum |
|------|
| `JsonErrorKind` |
| `DuplicateKeyPolicy` |
| `JsonValue` |
| `JsonVal` |

### Functions

| Function | Signature |
|----------|-----------|
| `line_column` | `(&self, input: &[u8]) -> JsonErrorPosition` |
| `parse` | `(mut self) -> Result<JsonValue<'a>, JsonError>` |
| `validate` | `(mut self) -> Result<(), JsonError>` |
| `parse_json` | `(bytes: &[u8]) -> Result<JsonValue<'_>, JsonError>` |
| `parse_json_with_max_depth` | `( bytes: &[u8], max_depth: usize, ) -> Result<JsonValue<'_>, JsonError>` |
| `parse_json_with_limits` | `( bytes: &[u8], limits: JsonLimits, ) -> Result<JsonValue<'_>, JsonError>` |
| `validate_json` | `(bytes: &[u8]) -> Result<(), JsonError>` |
| `write_json_object` | `(fields: &[(&str, JsonVal<'_>)]) -> String` |
| `write_json_array` | `(items: &[String]) -> String` |
| `push_json_escaped` | `(out: &mut String, s: &str)` |

---

## markdown API (39 items)

### Structs

| Struct |
|--------|
| `MdErrorPosition` |
| `MdError` |
| `MdLine` |
| `LineCursor` |
| `MdLimits` |
| `MdParser` |

### Enums

| Enum |
|------|
| `MdErrorKind` |
| `MdValue` |

### Functions

| Function | Signature |
|----------|-----------|
| `line_column` | `(&self, input: &[u8]) -> MdErrorPosition` |
| `validate_inline` | `(text: &str, base_offset: usize) -> Result<(), MdError>` |
| `peek_line` | `(&self) -> Result<Option<MdLine<'a>>, MdError>` |
| `advance_line` | `(&mut self)` |
| `parse` | `(mut self) -> Result<MdValue<'a>, MdError>` |
| `validate` | `(mut self) -> Result<(), MdError>` |
| `parse_md` | `(bytes: &[u8]) -> Result<MdValue<'_>, MdError>` |
| `parse_md_with_max_depth` | `(bytes: &[u8], max_depth: usize) -> Result<MdValue<'_>, MdError>` |
| `parse_md_with_limits` | `(bytes: &[u8], limits: MdLimits) -> Result<MdValue<'_>, MdError>` |
| `validate_md` | `(bytes: &[u8]) -> Result<(), MdError>` |
| `push_escaped` | `(out: &mut String, text: &str)` |
| `render_inline` | `(out: &mut String, text: &str)` |
| `render_md_html` | `(src: &str) -> String` |
| `write_md_heading` | `(level: u8, text: &str) -> String` |
| `write_md_table` | `(headers: &[&str], rows: &[Vec<String>]) -> String` |
| `write_md_code_block` | `(content: &str) -> String` |
| `write_md_row` | `(out: &mut String, label: &str, value: &str)` |

---

## toml API (38 items)

### Structs

| Struct |
|--------|
| `TomlErrorPosition` |
| `TomlError` |
| `Cursor` |
| `TomlLimits` |
| `TomlParser` |

### Enums

| Enum |
|------|
| `TomlErrorKind` |
| `Token` |
| `TomlValue` |

### Functions

| Function | Signature |
|----------|-----------|
| `line_column` | `(&self, input: &[u8]) -> TomlErrorPosition` |
| `set_position` | `(&mut self, pos: usize)` |
| `remaining` | `(&self) -> &'a [u8]` |
| `next_token` | `(&mut self) -> Result<Option<Token<'a>>, TomlError>` |
| `parse` | `(mut self) -> Result<TomlValue<'a>, TomlError>` |
| `validate` | `(mut self) -> Result<(), TomlError>` |
| `parse_toml` | `(bytes: &[u8]) -> Result<TomlValue<'_>, TomlError>` |
| `parse_toml_with_max_depth` | `( bytes: &[u8], max_depth: usize, ) -> Result<TomlValue<'_>, TomlError>` |
| `parse_toml_with_limits` | `( bytes: &[u8], limits: TomlLimits, ) -> Result<TomlValue<'_>, TomlError>` |
| `validate_toml` | `(bytes: &[u8]) -> Result<(), TomlError>` |
| `push_toml_escaped` | `(out: &mut String, s: &str)` |
| `push_toml_str` | `(out: &mut String, key: &str, val: &str)` |
| `push_toml_num` | `(out: &mut String, key: &str, val: &str)` |
| `push_toml_section` | `(out: &mut String, name: &str)` |
| `push_toml_array_section` | `(out: &mut String, name: &str)` |

---

## yaml API (33 items)

### Structs

| Struct |
|--------|
| `YamlErrorPosition` |
| `YamlError` |
| `YamlLine` |
| `LineCursor` |
| `YamlLimits` |
| `YamlParser` |

### Enums

| Enum |
|------|
| `YamlErrorKind` |
| `YamlValue` |

### Functions

| Function | Signature |
|----------|-----------|
| `line_column` | `(&self, input: &[u8]) -> YamlErrorPosition` |
| `peek` | `(&self) -> Result<Option<YamlLine<'a>>, YamlError>` |
| `next` | `(&mut self) -> Result<Option<YamlLine<'a>>, YamlError>` |
| `parse` | `(mut self) -> Result<YamlValue<'a>, YamlError>` |
| `validate` | `(mut self) -> Result<(), YamlError>` |
| `parse_yaml` | `(bytes: &[u8]) -> Result<YamlValue<'_>, YamlError>` |
| `parse_yaml_with_max_depth` | `( bytes: &[u8], max_depth: usize, ) -> Result<YamlValue<'_>, YamlError>` |
| `parse_yaml_with_limits` | `( bytes: &[u8], limits: YamlLimits, ) -> Result<YamlValue<'_>, YamlError>` |
| `validate_yaml` | `(bytes: &[u8]) -> Result<(), YamlError>` |
| `parse_scalar<'a>` | `(raw: &'a str, offset: usize) -> Result<YamlValue<'a>, YamlError>` |
| `write_yaml_map` | `(fields: &[(&str, &str)]) -> String` |
| `write_yaml_document` | `( header: &[(&str, &str)], list_key: &str, items: &[Vec<(&str, &str)>], ) -> String` |
| `push_yaml_value` | `(out: &mut String, val: &str)` |

---

