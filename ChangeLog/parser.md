# Parser — ChangeLog

---

## v0.0.3

### 1️⃣ TOML — `parser::toml` — 10 pub fn + 5 pub struct + 3 pub enum + 3 pub const

| Item | Declaration | Description | Module |
|---|---|---|---|
| `TomlErrorKind` | `pub enum` | `Eof`, `UnexpectedToken`, `InvalidKey`, `InvalidString`, `InvalidNumber`, `InvalidBool`, `InvalidEscape`, `DuplicateKey`, `UnterminatedString`, `MaxDepthExceeded`, `MaxStringLengthExceeded`, `MaxArrayLengthExceeded`, `MaxTableLengthExceeded`, `MaxNodeCountExceeded` | `parser::toml::error` |
| `TomlErrorPosition` | `pub struct` | `line: usize`, `column: usize` | `parser::toml::error` |
| `TomlError` | `pub struct` | `kind: TomlErrorKind`, `offset: usize` | `parser::toml::error` |
| `Token<'a>` | `pub enum` | `BareKey(&'a str)`, `BasicString(&'a str)`, `LiteralString(&'a str)`, `Integer(i64)`, `Float(f64)`, `Bool(bool)`, `Equals`, `Dot`, `Comma`, `OpenBracket`, `CloseBracket`, `OpenDoubleBracket`, `CloseDoubleBracket`, `Newline` | `parser::toml::lexer` |
| `Cursor<'a>` | `pub struct` | Byte-level lexer cursor | `parser::toml::lexer` |
| `DEFAULT_MAX_TOML_DEPTH` | `pub const DEFAULT_MAX_TOML_DEPTH: usize = 64` | Maximum nesting depth | `parser::toml::parser` |
| `TomlLimits` | `pub struct` | `max_depth`, `max_string_len`, `max_array_len`, `max_table_len`, `max_node_count` | `parser::toml::parser` |
| `DEFAULT_TOML_LIMITS` | `pub const DEFAULT_TOML_LIMITS: TomlLimits` | Default parsing limits | `parser::toml::parser` |
| `TomlParser<'a>` | `pub struct` | Streaming TOML parser | `parser::toml::parser` |
| `TomlValue<'a>` | `pub enum` | `String(&'a str)`, `Integer(i64)`, `Float(f64)`, `Bool(bool)`, `Table`, `Array`, `ArrayOfTables` | `parser::toml::value` |

| Function | Signature | Description | Module |
|---|---|---|---|
| Line column | `TomlError::line_column(&self, input: &[u8]) → TomlErrorPosition` | Error location | `parser::toml::error` |
| New cursor | `Cursor::new(bytes: &'a [u8]) → Self` | Initialize lexer | `parser::toml::lexer` |
| Position | `Cursor::position(&self) → usize` | Current offset | `parser::toml::lexer` |
| Bytes | `Cursor::bytes(&self) → &'a [u8]` | Source bytes | `parser::toml::lexer` |
| Set position | `Cursor::set_position(&mut self, pos: usize)` | Seek offset | `parser::toml::lexer` |
| Is EOF | `Cursor::is_eof(&self) → bool` | End of input | `parser::toml::lexer` |
| Remaining | `Cursor::remaining(&self) → &'a [u8]` | Unread bytes | `parser::toml::lexer` |
| Next token | `Cursor::next_token(&mut self) → Result<Option<Token<'a>>, TomlError>` | Consume next token | `parser::toml::lexer` |
| New parser | `TomlParser::new(bytes: &'a [u8]) → Self` | Initialize parser | `parser::toml::parser` |
| With max depth | `TomlParser::with_max_depth(self, max_depth: usize) → Self` | Set depth limit | `parser::toml::parser` |
| With limits | `TomlParser::with_limits(self, limits: TomlLimits) → Self` | Set all limits | `parser::toml::parser` |
| Parse | `TomlParser::parse(self) → Result<TomlValue<'a>, TomlError>` | Parse to AST | `parser::toml::parser` |
| Validate | `TomlParser::validate(self) → Result<(), TomlError>` | Validate syntax | `parser::toml::parser` |
| Parse TOML | `parse_toml(bytes: &[u8]) → Result<TomlValue<'_>, TomlError>` | One-shot parse | `parser::toml::parser` |
| Parse with depth | `parse_toml_with_max_depth(bytes: &[u8], max_depth: usize) → Result<TomlValue<'_>, TomlError>` | Parse with depth limit | `parser::toml::parser` |
| Parse with limits | `parse_toml_with_limits(bytes: &[u8], limits: TomlLimits) → Result<TomlValue<'_>, TomlError>` | Parse with all limits | `parser::toml::parser` |
| Validate TOML | `validate_toml(bytes: &[u8]) → Result<(), TomlError>` | One-shot validate | `parser::toml::parser` |
| Is table | `TomlValue::is_table(&self) → bool` | Type check | `parser::toml::value` |
| Is array | `TomlValue::is_array(&self) → bool` | Type check | `parser::toml::value` |
| As string | `TomlValue::as_str(&self) → Option<&str>` | Extract string | `parser::toml::value` |
| As bool | `TomlValue::as_bool(&self) → Option<bool>` | Extract boolean | `parser::toml::value` |
| As integer | `TomlValue::as_integer(&self) → Option<i64>` | Extract integer | `parser::toml::value` |
| As float | `TomlValue::as_float(&self) → Option<f64>` | Extract float | `parser::toml::value` |
| Escape TOML | `push_toml_escaped(out: &mut String, s: &str)` | Escape special chars | `parser::toml::writer` |
| Push string | `push_toml_str(out: &mut String, key: &str, val: &str)` | Write key = "val" | `parser::toml::writer` |
| Push number | `push_toml_num(out: &mut String, key: &str, val: &str)` | Write key = num | `parser::toml::writer` |
| Push section | `push_toml_section(out: &mut String, name: &str)` | Write [section] | `parser::toml::writer` |
| Push array section | `push_toml_array_section(out: &mut String, name: &str)` | Write [[section]] | `parser::toml::writer` |

### 2️⃣ HTML Writer — `parser::html::writer` — 3 pub fn

| Function | Signature | Description | Module |
|---|---|---|---|
| Escape HTML | `push_html_escaped(out: &mut String, text: &str)` | Escape `<>&"` | `parser::html::writer` |
| Push row | `push_html_row(out: &mut String, label: &str, value: &str)` | HTML table row | `parser::html::writer` |
| Escape JS | `push_js_escaped(out: &mut String, s: &str)` | JavaScript string escape | `parser::html::writer` |

### 3️⃣ JSON Writer — `parser::json::writer` — 3 pub fn + 1 pub enum

| Item | Declaration | Description | Module |
|---|---|---|---|
| `JsonVal<'a>` | `pub enum` | `Str(&'a str)`, `Num(f64)`, `Int(i64)`, `Bool(bool)`, `Null`, `Raw(&'a str)` | `parser::json::writer` |

| Function | Signature | Description | Module |
|---|---|---|---|
| Write object | `write_json_object(fields: &[(&str, JsonVal<'_>)]) → String` | JSON object | `parser::json::writer` |
| Write array | `write_json_array(items: &[String]) → String` | JSON array | `parser::json::writer` |
| Escape JSON | `push_json_escaped(out: &mut String, s: &str)` | JSON string escape | `parser::json::writer` |

### 4️⃣ Markdown Writer — `parser::markdown::writer` — 7 pub fn

| Function | Signature | Description | Module |
|---|---|---|---|
| Escape | `push_escaped(out: &mut String, text: &str)` | Escape Markdown chars | `parser::markdown::writer` |
| Render inline | `render_inline(out: &mut String, text: &str)` | Inline formatting | `parser::markdown::writer` |
| Render HTML | `render_md_html(src: &str) → String` | Markdown → HTML | `parser::markdown::writer` |
| Write heading | `write_md_heading(level: u8, text: &str) → String` | ATX heading | `parser::markdown::writer` |
| Write table | `write_md_table(headers: &[&str], rows: &[Vec<String>]) → String` | Pipe table | `parser::markdown::writer` |
| Write code block | `write_md_code_block(content: &str) → String` | Fenced code | `parser::markdown::writer` |
| Write row | `write_md_row(out: &mut String, label: &str, value: &str)` | Label–value row | `parser::markdown::writer` |

### 5️⃣ YAML Writer — `parser::yaml::writer` — 3 pub fn

| Function | Signature | Description | Module |
|---|---|---|---|
| Write map | `write_yaml_map(fields: &[(&str, &str)]) → String` | YAML mapping | `parser::yaml::writer` |
| Write document | `write_yaml_document(header: &[(&str, &str)], list_key: &str, items: &[Vec<(&str, &str)>]) → String` | Full YAML document | `parser::yaml::writer` |
| Push value | `push_yaml_value(out: &mut String, val: &str)` | Append YAML value | `parser::yaml::writer` |

No new functions in csv, html, json, markdown, yaml core — see `testing.md` for test details.

---

## v0.0.2

### Changes — `parser` — 10 files modified

| Change | Detail |
|---|---|
| Submodules | 5 (csv, html, json, markdown, yaml) — unchanged |
| Files modified | 10 (mods, lexers, parsers) |

---

## v0.0.1

### 1️⃣ CSV — `parser::csv` — 12 pub fn + 4 pub struct + 2 pub enum + 1 pub const

| Item | Declaration | Description | Module |
|---|---|---|---|
| `CsvErrorKind` | `pub enum` | `UnexpectedQuote`, `UnterminatedQuotedField`, `InvalidUtf8`, `TrailingCharactersAfterQuote`, `MaxRowsExceeded`, `MaxColumnsExceeded`, `MaxFieldLengthExceeded`, `MaxNodeCountExceeded` | `parser::csv::error` |
| `CsvErrorPosition` | `pub struct` | `line: usize`, `column: usize` | `parser::csv::error` |
| `CsvError` | `pub struct` | `kind: CsvErrorKind`, `offset: usize` | `parser::csv::error` |
| `Cursor<'a>` | `pub struct` | Byte-level lexer cursor | `parser::csv::lexer` |
| `CsvLimits` | `pub struct` | `max_rows`, `max_columns`, `max_field_len`, `max_node_count` | `parser::csv::parser` |
| `DEFAULT_CSV_LIMITS` | `pub const DEFAULT_CSV_LIMITS: CsvLimits` | Default parsing limits | `parser::csv::parser` |
| `CsvParser<'a>` | `pub struct` (hidden) | Streaming CSV parser | `parser::csv::parser` |
| `CsvValue<'a>` | `pub enum` | `Table`, `Record`, `Field(&'a str)` | `parser::csv::value` |

| Function | Signature | Description | Module |
|---|---|---|---|
| Line column | `CsvError::line_column(&self, input: &[u8]) → CsvErrorPosition` | Error location | `parser::csv::error` |
| New cursor | `Cursor::new(bytes: &'a [u8]) → Self` | Initialize lexer | `parser::csv::lexer` |
| Position | `Cursor::position(&self) → usize` | Current offset | `parser::csv::lexer` |
| Bytes | `Cursor::bytes(&self) → &'a [u8]` | Source bytes | `parser::csv::lexer` |
| Is EOF | `Cursor::is_eof(&self) → bool` | End of input | `parser::csv::lexer` |
| Peek | `Cursor::peek(&self) → Option<u8>` | Peek next byte | `parser::csv::lexer` |
| Advance | `Cursor::advance(&mut self, n: usize)` | Skip n bytes | `parser::csv::lexer` |
| Next | `Cursor::next(&mut self) → Option<u8>` | Consume next byte | `parser::csv::lexer` |
| New parser | `CsvParser::new(bytes: &'a [u8]) → Self` | Initialize parser | `parser::csv::parser` |
| With limits | `CsvParser::with_limits(self, limits: CsvLimits) → Self` | Set all limits | `parser::csv::parser` |
| Parse | `CsvParser::parse(self) → Result<CsvValue<'a>, CsvError>` | Parse to AST | `parser::csv::parser` |
| Validate | `CsvParser::validate(self) → Result<(), CsvError>` | Validate syntax | `parser::csv::parser` |
| Parse CSV | `parse_csv(bytes: &[u8]) → Result<CsvValue<'_>, CsvError>` | One-shot parse | `parser::csv::parser` |
| Parse with limits | `parse_csv_with_limits(bytes: &[u8], limits: CsvLimits) → Result<CsvValue<'_>, CsvError>` | Parse with limits | `parser::csv::parser` |
| Validate CSV | `validate_csv(bytes: &[u8]) → Result<(), CsvError>` | One-shot validate | `parser::csv::parser` |
| Is table | `CsvValue::is_table(&self) → bool` | Type check | `parser::csv::value` |
| Is record | `CsvValue::is_record(&self) → bool` | Type check | `parser::csv::value` |
| As field | `CsvValue::as_field(&self) → Option<&str>` | Extract field text | `parser::csv::value` |
| Write CSV | `write_csv(header: &[&str], rows: &[Vec<String>]) → String` | Generate CSV output | `parser::csv::writer` |

### 2️⃣ HTML — `parser::html` — 21 pub fn + 5 pub struct + 2 pub enum + 2 pub const

| Item | Declaration | Description | Module |
|---|---|---|---|
| `HtmlErrorKind` | `pub enum` | `Eof`, `UnexpectedToken`, `InvalidTagName`, `UnterminatedTag`, `UnterminatedComment`, `UnterminatedAttribute`, `UnterminatedDoctype`, `UnterminatedEntity`, `DuplicateAttribute`, `MismatchedClosingTag`, `InvalidUtf8`, `MaxDepthExceeded`, `MaxNodeCountExceeded`, `MaxAttributeCountExceeded`, `MaxAttributeValueLengthExceeded` | `parser::html::error` |
| `HtmlErrorPosition` | `pub struct` | `line: usize`, `column: usize` | `parser::html::error` |
| `HtmlError` | `pub struct` | `kind: HtmlErrorKind`, `offset: usize` | `parser::html::error` |
| `Cursor<'a>` | `pub struct` | Byte-level lexer cursor | `parser::html::lexer` |
| `DEFAULT_MAX_HTML_DEPTH` | `pub const DEFAULT_MAX_HTML_DEPTH: usize = 128` | Maximum nesting depth | `parser::html::parser` |
| `HtmlLimits` | `pub struct` | `max_depth`, `max_node_count`, `max_attribute_count`, `max_attribute_value_len` | `parser::html::parser` |
| `DEFAULT_HTML_LIMITS` | `pub const DEFAULT_HTML_LIMITS: HtmlLimits` | Default parsing limits | `parser::html::parser` |
| `HtmlParser<'a>` | `pub struct` (hidden) | Streaming HTML parser | `parser::html::parser` |
| `HtmlValue<'a>` | `pub enum` | `Document`, `Element`, `Text(&'a str)`, `Comment`, `Doctype` | `parser::html::value` |

| Function | Signature | Description | Module |
|---|---|---|---|
| Line column | `HtmlError::line_column(&self, input: &[u8]) → HtmlErrorPosition` | Error location | `parser::html::error` |
| New cursor | `Cursor::new(bytes: &'a [u8]) → Self` | Initialize lexer | `parser::html::lexer` |
| Position | `Cursor::position(&self) → usize` | Current offset | `parser::html::lexer` |
| Is EOF | `Cursor::is_eof(&self) → bool` | End of input | `parser::html::lexer` |
| Peek | `Cursor::peek(&self) → Option<u8>` | Peek next byte | `parser::html::lexer` |
| Advance | `Cursor::advance(&mut self, n: usize)` | Skip n bytes | `parser::html::lexer` |
| Remaining | `Cursor::remaining(&self) → &'a [u8]` | Unread bytes | `parser::html::lexer` |
| Starts with | `Cursor::starts_with(&self, needle: &[u8]) → bool` | Prefix check | `parser::html::lexer` |
| Skip whitespace | `Cursor::skip_ws(&mut self)` | Skip spaces | `parser::html::lexer` |
| Read while | `Cursor::read_while<F: Fn(u8) → bool>(&mut self, pred: F) → &'a [u8]` | Consume matching bytes | `parser::html::lexer` |
| Read tag name | `Cursor::read_tag_name(&mut self) → Result<&'a str, HtmlError>` | Extract tag name | `parser::html::lexer` |
| Validate entity | `validate_entity(cursor: &mut Cursor<'_>) → Result<(), HtmlError>` | HTML entity check | `parser::html::entity` |
| New parser | `HtmlParser::new(bytes: &'a [u8]) → Self` | Initialize parser | `parser::html::parser` |
| With limits | `HtmlParser::with_limits(self, limits: HtmlLimits) → Self` | Set all limits | `parser::html::parser` |
| With max depth | `HtmlParser::with_max_depth(self, max_depth: usize) → Self` | Set depth limit | `parser::html::parser` |
| Parse | `HtmlParser::parse(self) → Result<HtmlValue<'a>, HtmlError>` | Parse to AST | `parser::html::parser` |
| Validate | `HtmlParser::validate(self) → Result<(), HtmlError>` | Validate syntax | `parser::html::parser` |
| Parse HTML | `parse_html(bytes: &[u8]) → Result<HtmlValue<'_>, HtmlError>` | One-shot parse | `parser::html::parser` |
| Parse with depth | `parse_html_with_max_depth(bytes: &[u8], max_depth: usize) → Result<HtmlValue<'_>, HtmlError>` | Parse with depth limit | `parser::html::parser` |
| Parse with limits | `parse_html_with_limits(bytes: &[u8], limits: HtmlLimits) → Result<HtmlValue<'_>, HtmlError>` | Parse with all limits | `parser::html::parser` |
| Validate HTML | `validate_html(bytes: &[u8]) → Result<(), HtmlError>` | One-shot validate | `parser::html::parser` |
| Is document | `HtmlValue::is_document(&self) → bool` | Type check | `parser::html::value` |
| Is element | `HtmlValue::is_element(&self) → bool` | Type check | `parser::html::value` |
| Is comment | `HtmlValue::is_comment(&self) → bool` | Type check | `parser::html::value` |
| Is doctype | `HtmlValue::is_doctype(&self) → bool` | Type check | `parser::html::value` |
| As text | `HtmlValue::as_text(&self) → Option<&str>` | Extract text | `parser::html::value` |

### 3️⃣ JSON — `parser::json` — 15 pub fn + 3 pub struct + 3 pub enum + 2 pub const

| Item | Declaration | Description | Module |
|---|---|---|---|
| `JsonErrorKind` | `pub enum` | `Eof`, `TrailingCharacters`, `UnexpectedToken`, `InvalidLiteral`, `InvalidNumber`, `InvalidEscape`, `InvalidUnicodeEscape`, `InvalidUnicodeSurrogate`, `ControlCharacterInString`, `InvalidUtf8`, `MaxDepthExceeded`, `MaxStringLengthExceeded`, `MaxArrayLengthExceeded`, `MaxObjectLengthExceeded`, `MaxNodeCountExceeded`, `DuplicateObjectKey` | `parser::json::error` |
| `JsonErrorPosition` | `pub struct` | `line: usize`, `column: usize` | `parser::json::error` |
| `JsonError` | `pub struct` | `kind: JsonErrorKind`, `offset: usize` | `parser::json::error` |
| `DEFAULT_MAX_DEPTH` | `pub const DEFAULT_MAX_DEPTH: usize = 64` | Maximum nesting depth | `parser::json::parser` |
| `DuplicateKeyPolicy` | `pub enum` | `Allow`, `Reject` | `parser::json::parser` |
| `JsonLimits` | `pub struct` | `max_depth`, `max_string_len`, `max_array_len`, `max_object_len`, `max_node_count`, `duplicate_key_policy` | `parser::json::parser` |
| `DEFAULT_LIMITS` | `pub const DEFAULT_LIMITS: JsonLimits` | Default parsing limits | `parser::json::parser` |
| `JsonParser<'a>` | `pub struct` (hidden) | Streaming JSON parser | `parser::json::parser` |
| `JsonValue<'a>` | `pub enum` | `Null`, `Bool(bool)`, `Number(f64)`, `String(&'a str)`, `Array`, `Object` | `parser::json::value` |

| Function | Signature | Description | Module |
|---|---|---|---|
| Line column | `JsonError::line_column(&self, input: &[u8]) → JsonErrorPosition` | Error location | `parser::json::error` |
| New parser | `JsonParser::new(bytes: &'a [u8]) → Self` | Initialize parser | `parser::json::parser` |
| With max depth | `JsonParser::with_max_depth(self, max_depth: usize) → Self` | Set depth limit | `parser::json::parser` |
| With limits | `JsonParser::with_limits(self, limits: JsonLimits) → Self` | Set all limits | `parser::json::parser` |
| Parse | `JsonParser::parse(self) → Result<JsonValue<'a>, JsonError>` | Parse to AST | `parser::json::parser` |
| Validate | `JsonParser::validate(self) → Result<(), JsonError>` | Validate syntax | `parser::json::parser` |
| Parse JSON | `parse_json(bytes: &[u8]) → Result<JsonValue<'_>, JsonError>` | One-shot parse | `parser::json::parser` |
| Parse with depth | `parse_json_with_max_depth(bytes: &[u8], max_depth: usize) → Result<JsonValue<'_>, JsonError>` | Parse with depth limit | `parser::json::parser` |
| Parse with limits | `parse_json_with_limits(bytes: &[u8], limits: JsonLimits) → Result<JsonValue<'_>, JsonError>` | Parse with all limits | `parser::json::parser` |
| Validate JSON | `validate_json(bytes: &[u8]) → Result<(), JsonError>` | One-shot validate | `parser::json::parser` |
| Is object | `JsonValue::is_object(&self) → bool` | Type check | `parser::json::value` |
| Is array | `JsonValue::is_array(&self) → bool` | Type check | `parser::json::value` |
| As string | `JsonValue::as_str(&self) → Option<&str>` | Extract string | `parser::json::value` |
| As bool | `JsonValue::as_bool(&self) → Option<bool>` | Extract boolean | `parser::json::value` |
| As number | `JsonValue::as_number(&self) → Option<f64>` | Extract number | `parser::json::value` |

### 4️⃣ Markdown — `parser::markdown` — 18 pub fn + 5 pub struct + 2 pub enum + 2 pub const

| Item | Declaration | Description | Module |
|---|---|---|---|
| `MdErrorKind` | `pub enum` | `Eof`, `InvalidHeading`, `UnterminatedCodeBlock`, `UnterminatedCodeSpan`, `MaxDepthExceeded`, `MaxLineLengthExceeded`, `MaxNodeCountExceeded`, `MaxListLengthExceeded`, `InvalidUtf8` | `parser::markdown::error` |
| `MdErrorPosition` | `pub struct` | `line: usize`, `column: usize` | `parser::markdown::error` |
| `MdError` | `pub struct` | `kind: MdErrorKind`, `offset: usize` | `parser::markdown::error` |
| `MdLine<'a>` | `pub struct` | `content: &'a str`, `offset: usize`, `indent: usize` | `parser::markdown::lexer` |
| `LineCursor<'a>` | `pub struct` | Line-level lexer cursor | `parser::markdown::lexer` |
| `DEFAULT_MAX_MD_DEPTH` | `pub const DEFAULT_MAX_MD_DEPTH: usize = 64` | Maximum nesting depth | `parser::markdown::parser` |
| `MdLimits` | `pub struct` | `max_depth`, `max_line_len`, `max_list_len`, `max_node_count` | `parser::markdown::parser` |
| `DEFAULT_MD_LIMITS` | `pub const DEFAULT_MD_LIMITS: MdLimits` | Default parsing limits | `parser::markdown::parser` |
| `MdParser<'a>` | `pub struct` (hidden) | Streaming Markdown parser | `parser::markdown::parser` |
| `MdValue<'a>` | `pub enum` | `Document`, `Heading(u8)`, `Paragraph`, `CodeBlock`, `ThematicBreak`, `BlockQuote`, `List`, `Table`, `Text(&'a str)` | `parser::markdown::value` |

| Function | Signature | Description | Module |
|---|---|---|---|
| Line column | `MdError::line_column(&self, input: &[u8]) → MdErrorPosition` | Error location | `parser::markdown::error` |
| Validate inline | `validate_inline(text: &str, base_offset: usize) → Result<(), MdError>` | Inline syntax check | `parser::markdown::inline` |
| New cursor | `LineCursor::new(bytes: &'a [u8]) → Self` | Initialize lexer | `parser::markdown::lexer` |
| Position | `LineCursor::position(&self) → usize` | Current offset | `parser::markdown::lexer` |
| Peek line | `LineCursor::peek_line(&self) → Result<Option<MdLine<'a>>, MdError>` | Peek next line | `parser::markdown::lexer` |
| Advance line | `LineCursor::advance_line(&mut self)` | Consume next line | `parser::markdown::lexer` |
| New parser | `MdParser::new(bytes: &'a [u8]) → Self` | Initialize parser | `parser::markdown::parser` |
| With limits | `MdParser::with_limits(self, limits: MdLimits) → Self` | Set all limits | `parser::markdown::parser` |
| With max depth | `MdParser::with_max_depth(self, max_depth: usize) → Self` | Set depth limit | `parser::markdown::parser` |
| Parse | `MdParser::parse(self) → Result<MdValue<'a>, MdError>` | Parse to AST | `parser::markdown::parser` |
| Validate | `MdParser::validate(self) → Result<(), MdError>` | Validate syntax | `parser::markdown::parser` |
| Parse Markdown | `parse_md(bytes: &[u8]) → Result<MdValue<'_>, MdError>` | One-shot parse | `parser::markdown::parser` |
| Parse with depth | `parse_md_with_max_depth(bytes: &[u8], max_depth: usize) → Result<MdValue<'_>, MdError>` | Parse with depth limit | `parser::markdown::parser` |
| Parse with limits | `parse_md_with_limits(bytes: &[u8], limits: MdLimits) → Result<MdValue<'_>, MdError>` | Parse with all limits | `parser::markdown::parser` |
| Validate Markdown | `validate_md(bytes: &[u8]) → Result<(), MdError>` | One-shot validate | `parser::markdown::parser` |
| Is document | `MdValue::is_document(&self) → bool` | Type check | `parser::markdown::value` |
| Is heading | `MdValue::is_heading(&self) → bool` | Type check | `parser::markdown::value` |
| Heading level | `MdValue::heading_level(&self) → Option<u8>` | Extract level | `parser::markdown::value` |
| Is code block | `MdValue::is_code_block(&self) → bool` | Type check | `parser::markdown::value` |
| Is list | `MdValue::is_list(&self) → bool` | Type check | `parser::markdown::value` |
| Is table | `MdValue::is_table(&self) → bool` | Type check | `parser::markdown::value` |
| As text | `MdValue::as_text(&self) → Option<&str>` | Extract text | `parser::markdown::value` |

### 5️⃣ YAML — `parser::yaml` — 16 pub fn + 5 pub struct + 2 pub enum + 2 pub const

| Item | Declaration | Description | Module |
|---|---|---|---|
| `YamlErrorKind` | `pub enum` | `Eof`, `InvalidIndentation`, `UnexpectedToken`, `InvalidMappingKey`, `InvalidScalar`, `UnsupportedFeature`, `MaxDepthExceeded`, `MaxScalarLengthExceeded`, `MaxSequenceLengthExceeded`, `MaxMappingLengthExceeded`, `MaxNodeCountExceeded` | `parser::yaml::error` |
| `YamlErrorPosition` | `pub struct` | `line: usize`, `column: usize` | `parser::yaml::error` |
| `YamlError` | `pub struct` | `kind: YamlErrorKind`, `offset: usize` | `parser::yaml::error` |
| `YamlLine<'a>` | `pub struct` | `indent: usize`, `content: &'a str`, `offset: usize` | `parser::yaml::lexer` |
| `LineCursor<'a>` | `pub struct` | Line-level lexer cursor | `parser::yaml::lexer` |
| `DEFAULT_MAX_YAML_DEPTH` | `pub const DEFAULT_MAX_YAML_DEPTH: usize = 64` | Maximum nesting depth | `parser::yaml::parser` |
| `YamlLimits` | `pub struct` | `max_depth`, `max_scalar_len`, `max_sequence_len`, `max_mapping_len`, `max_node_count` | `parser::yaml::parser` |
| `DEFAULT_YAML_LIMITS` | `pub const DEFAULT_YAML_LIMITS: YamlLimits` | Default parsing limits | `parser::yaml::parser` |
| `YamlParser<'a>` | `pub struct` (hidden) | Streaming YAML parser | `parser::yaml::parser` |
| `YamlValue<'a>` | `pub enum` | `Null`, `Bool(bool)`, `Number(f64)`, `String(&'a str)`, `Sequence`, `Mapping` | `parser::yaml::value` |

| Function | Signature | Description | Module |
|---|---|---|---|
| Line column | `YamlError::line_column(&self, input: &[u8]) → YamlErrorPosition` | Error location | `parser::yaml::error` |
| New cursor | `LineCursor::new(bytes: &'a [u8]) → Self` | Initialize lexer | `parser::yaml::lexer` |
| Position | `LineCursor::position(&self) → usize` | Current offset | `parser::yaml::lexer` |
| Peek | `LineCursor::peek(&self) → Result<Option<YamlLine<'a>>, YamlError>` | Peek next line | `parser::yaml::lexer` |
| Next | `LineCursor::next(&mut self) → Result<Option<YamlLine<'a>>, YamlError>` | Consume next line | `parser::yaml::lexer` |
| New parser | `YamlParser::new(bytes: &'a [u8]) → Self` | Initialize parser | `parser::yaml::parser` |
| With max depth | `YamlParser::with_max_depth(self, max_depth: usize) → Self` | Set depth limit | `parser::yaml::parser` |
| With limits | `YamlParser::with_limits(self, limits: YamlLimits) → Self` | Set all limits | `parser::yaml::parser` |
| Parse | `YamlParser::parse(self) → Result<YamlValue<'a>, YamlError>` | Parse to AST | `parser::yaml::parser` |
| Validate | `YamlParser::validate(self) → Result<(), YamlError>` | Validate syntax | `parser::yaml::parser` |
| Parse YAML | `parse_yaml(bytes: &[u8]) → Result<YamlValue<'_>, YamlError>` | One-shot parse | `parser::yaml::parser` |
| Parse with depth | `parse_yaml_with_max_depth(bytes: &[u8], max_depth: usize) → Result<YamlValue<'_>, YamlError>` | Parse with depth limit | `parser::yaml::parser` |
| Parse with limits | `parse_yaml_with_limits(bytes: &[u8], limits: YamlLimits) → Result<YamlValue<'_>, YamlError>` | Parse with all limits | `parser::yaml::parser` |
| Validate YAML | `validate_yaml(bytes: &[u8]) → Result<(), YamlError>` | One-shot validate | `parser::yaml::parser` |
| Parse scalar | `parse_scalar<'a>(raw: &'a str, offset: usize) → Result<YamlValue<'a>, YamlError>` | Scalar type inference | `parser::yaml::scalar` |
| Is mapping | `YamlValue::is_mapping(&self) → bool` | Type check | `parser::yaml::value` |
| Is sequence | `YamlValue::is_sequence(&self) → bool` | Type check | `parser::yaml::value` |
| As string | `YamlValue::as_str(&self) → Option<&str>` | Extract string | `parser::yaml::value` |
| As bool | `YamlValue::as_bool(&self) → Option<bool>` | Extract boolean | `parser::yaml::value` |
| As number | `YamlValue::as_number(&self) → Option<f64>` | Extract number | `parser::yaml::value` |
