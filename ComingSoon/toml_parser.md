## 1️⃣ Current State

| Building Block | Location | Description |
|---|---|---|
| CSV parser | `src/parser/csv/` | CsvValue<'a> event-based, zero-copy |
| HTML parser | `src/parser/html/` | Full HTML parser |
| JSON parser | `src/parser/json/` | Full JSON parser |
| Markdown parser | `src/parser/markdown/` | Full Markdown parser |
| YAML parser | `src/parser/yaml/` | Full YAML parser |
| TOML export (ad hoc) | `src/benchmark/export.rs`, `report.rs` | push_toml_str(), push_toml_num() — basic escaping |
| TOML output files | `output/*/toml/` | ~388 .toml files generated |

Convention : chaque parser suit le pattern `error.rs`, `lexer.rs`, `value.rs`, `parser.rs`, `mod.rs` avec `XxxValue<'a>` zero-copy, `XxxParser<'a>`, `XxxLimits`, `XxxError`.

## 2️⃣ Error Module — `src/parser/toml/error.rs`

| Component | Description |
|---|---|
| `TomlErrorKind` | Eof, InvalidKey, InvalidValue, UnterminatedString, InvalidEscape, InvalidNumber, InvalidDate, DuplicateKey, MaxDepthExceeded, MaxLineLengthExceeded, MaxNodeCountExceeded, InvalidUtf8 |
| `TomlErrorPosition` | line, column |
| `TomlError` | kind + offset, line_column() |

## 3️⃣ Value Types — `src/parser/toml/value.rs`

| Variant | Rust Type | Description |
|---|---|---|
| `String` | `&'a str` | Borrowed string |
| `Integer` | `i64` | Integer |
| `Float` | `f64` | Float |
| `Boolean` | `bool` | true / false |
| `DateTime` | `&'a str` | ISO 8601 |
| `Array` | `Vec<TomlValue<'a>>` | Inline arrays |
| `Table` | `Vec<(&'a str, TomlValue<'a>)>` | Key-value tables |

| Method | Description |
|---|---|
| `as_str()` | Extract string |
| `as_integer()` | Extract i64 |
| `as_float()` | Extract f64 |
| `as_bool()` | Extract bool |
| `as_array()` | Extract array |
| `as_table()` | Extract table |
| `get(key)` | Lookup by key |

## 4️⃣ Lexer — `src/parser/toml/lexer.rs`

| Token Category | Tokens |
|---|---|
| Keys | BareKey, QuotedKey |
| Structure | Equals, Dot, Comma, OpenBracket, CloseBracket, OpenDoubleBracket, CloseDoubleBracket |
| Strings | BasicString, LiteralString, MultiLineBasicString, MultiLineLiteralString |
| Values | Integer, Float, Bool, DateTime |
| Whitespace | Newline, Comment (#), Whitespace |

Escaping complet : `\n`, `\t`, `\r`, `\\`, `\"`, `\uXXXX`, `\UXXXXXXXX`

## 5️⃣ Parser — `src/parser/toml/parser.rs`

| Component | Description |
|---|---|
| `TomlParser<'a>` | bytes, cursor, limits, nodes_seen |
| `TomlLimits` | max_depth, max_line_len, max_key_len, max_node_count |
| `DEFAULT_TOML_LIMITS` | Preset safe limits |
| `parse()` | → `Result<TomlValue<'a>, TomlError>` |
| `validate()` | → `Result<(), TomlError>` |

| TOML v1.0 Feature | Description |
|---|---|
| Key-value pairs | Bare keys, quoted keys, dotted keys |
| `[table]` | Standard table headers |
| `[[array-of-tables]]` | Array of tables |
| Inline tables | `{ k = v, ... }` |
| Inline arrays | `[ v, v, ... ]` |
| Multi-line strings | Basic `"""` and literal `'''` |
| Integers | Decimal, hex `0x`, octal `0o`, binary `0b`, underscores |
| Floats | Frac, exp, inf, nan |
| Booleans | true, false |
| Dates | Offset datetime, local datetime, local date, local time |

## 6️⃣ Public API — `src/parser/toml/mod.rs`

| Export | Description |
|---|---|
| `TomlError`, `TomlErrorKind`, `TomlErrorPosition` | Error types |
| `TomlParser`, `TomlLimits`, `DEFAULT_TOML_LIMITS` | Parser + config |
| `TomlValue` | Parsed value tree |
| `parse_toml()` | Parse bytes → TomlValue |
| `parse_toml_with_limits()` | Parse with custom limits |
| `validate_toml()` | Validate without building tree |

## 7️⃣ Round-Trip with Export

| Task | Description |
|---|---|
| `write_toml(value: &TomlValue) → String` | Clean serialization replacing string-pushing |
| Refactor `report.rs` | Use TomlValue instead of push_toml_str / push_toml_num |
| Validate `output/*/toml/*.toml` | Must be parsable by the new parser |

## 8️⃣ Tests

| Test | Description |
|---|---|
| Existing round-trip | Parse output/*.toml → serialize → parse → compare |
| TOML v1.0 spec | Each value type, dotted keys, array of tables, inline tables |
| Limits | max_depth, max_node_count, lines too long |
| Errors | Duplicate keys, unterminated strings, invalid numbers |
