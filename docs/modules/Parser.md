# Parser Module

The parser module provides **6 complete format parsers**, each built from scratch with **zero dependencies**. Every parser follows a consistent three-stage architecture: **lexer → parser → typed value tree**. This module enables SciForge to read and write structured data in all common scientific exchange formats.

---

## Chapter 1 - Module Scope

### Scientific Purpose
This section defines the scope: which problem this module solves, which abstraction level it targets, and which outputs are expected.

### Modeling Assumptions
- The equations are numerical models, not analytical proofs.
- Inputs must respect the documented units.
- Validity domains (linear regime, local approximation, and so on) should be verified by the reader.

### Reading Strategy
1. Read the module structure.
2. Identify equations and their conditions.
3. Map these equations to Rust functions in `docs/code/`.


## Reading Guide

### Goal
This page explains the module from a scientific perspective: assumptions, models, equations, and usage limits.

### How To Read This Document
- Start with the domain section relevant to your case.
- Identify key equations and their usage conditions.
- Then verify the corresponding Rust signatures in `docs/code/`.

### Conventions
- Units: SI unless explicitly stated.
- Variables: standard mathematical notation for the domain.
- Equations are presented as reference models, not formal proofs.


## Architecture

Each format implementation follows an identical layered pattern across 4–5 source files:

| Layer | File | Role |
|---|---|---|
| **Lexer** | `lexer.rs` | Byte-level cursor over `&[u8]` input. Advances through raw bytes, tracks position, handles whitespace skipping and token boundary detection. |
| **Parser** | `parser.rs` | Recursive-descent parser consuming tokens from the lexer to build a typed value tree. |
| **Value** | `value.rs` | Enum representing parsed data nodes (strings, numbers, booleans, containers). All values borrow from the input (`&'a str`) — **zero-copy** where possible. |
| **Error** | `error.rs` | Format-specific error type with byte position and error kind for precise diagnostics. |

**Shared design principles:**

- **Zero-allocation parsing** — values borrow slices from the input buffer via lifetime `'a`. String values are `&'a str` referencing the original bytes, avoiding heap copies.
- **Cursor-based lexing** — every lexer wraps a `Cursor<'a>` (or `LineCursor<'a>` for line-oriented formats) with `peek()`, `advance(n)`, `is_eof()`, and `position()` methods.
- **`const fn` accessors** — type-checking methods (`is_object()`, `as_str()`, `as_number()`) are `const fn` for compile-time usability.

---

## 1. CSV

**Comma-separated values** for tabular scientific data.

**Value types:**
```rust
enum CsvValue<'a> {
    Table,      // Root document node
    Record,     // One row
    Field(&'a str),  // One cell (zero-copy reference)
}
```

The lexer operates on raw bytes with a simple `Cursor` tracking position. Fields are delimited by commas; records by newlines. Quoted fields (RFC 4180) handle embedded commas, newlines, and escaped double-quotes.

**Writer.** The CSV writer module generates properly escaped output from structured data — quoting fields that contain delimiters, quotes, or line breaks.

---

## 2. JSON

**JavaScript Object Notation** — full RFC 8259 compliance.

**Value types:**
```rust
enum JsonValue<'a> {
    Null,
    Bool(bool),
    Number(f64),
    String(&'a str),
    Array,       // Container node
    Object,      // Container node
}
```

The parser handles:
- **Number parsing** (`number.rs`): integers, floats, and scientific notation (`1.23e-4`), parsed via a dedicated module that handles all edge cases.
- **String parsing** (`string.rs`): full Unicode escape sequences (`\uXXXX`, including surrogate pairs for supplementary plane characters), plus standard escapes (`\n`, `\t`, `\\`, `\"`, etc.).
- **Nested structures**: recursive descent with arbitrary nesting depth for objects and arrays.

Error reporting includes byte offset for precise Location of syntax errors.

---

## 3. YAML

**YAML Ain't Markup Language** — indentation-based human-readable format.

**Value types:**
```rust
enum YamlValue<'a> {
    Null,
    Bool(bool),
    Number(f64),
    String(&'a str),
    Sequence,    // Array/list container
    Mapping,     // Key-value container
}
```

The YAML lexer uses a **line-oriented cursor** (`LineCursor`) that tracks both byte position and indentation level. Each `YamlLine` carries `indent`, `content`, and `offset`.

**Scalar parsing** (`scalar.rs`): determines the type of an unquoted scalar:
- `"null"`, `"~"` → `Null`
- `"true"`, `"false"` → `Bool`
- Numeric patterns → `Number(f64)`
- Everything else → `String`

The YAML parser is used internally by the constants module to load all 118 periodic table element files at compile time.

---

## 4. HTML

**HyperText Markup Language** parsing for document generation and analysis.

**Value types:**
```rust
enum HtmlValue<'a> {
    Document,       // Root node
    Element,        // HTML tag
    Text(&'a str),  // Text content
    Comment,        // <!-- ... -->
    Doctype,        // <!DOCTYPE ...>
}
```

**Entity decoding** (`entity.rs`): handles both named entities (`&amp;` → `&`, `&lt;` → `<`, `&gt;` → `>`, `&quot;` → `"`, etc.) and numeric character references (`&#123;`, `&#x7B;`).

The lexer tracks open/close tags, self-closing tags, and attributes. Used by the benchmark export module to generate self-contained HTML dashboards.

---

## 5. Markdown

**Lightweight markup** parsing for documentation and report generation.

**Value types:**
```rust
enum MdValue<'a> {
    Document,        // Root node
    Heading(u8),     // h1–h6, level stored as u8
    Paragraph,
    CodeBlock,       // Fenced (```) or indented
    ThematicBreak,   // --- or ***
    BlockQuote,      // > prefixed blocks
    List,            // Ordered or unordered
    Table,           // GFM-style tables
    Text(&'a str),   // Inline text content
}
```

**Inline processing** (`inline.rs`): parses bold (`**`), italic (`*`), inline code (`` ` ``), links (`[text](url)`), and other inline formatting.

The Markdown lexer uses a `LineCursor` similar to YAML, processing input line-by-line with indent tracking. Headings are detected by leading `#` characters (1–6 levels). Fenced code blocks delimit by triple backticks.

Used by the benchmark report module to generate Markdown summaries, and can parse SciForge's own documentation files for programmatic analysis.

---

## 6. TOML

**Tom's Obvious, Minimal Language** — configuration format widely used in Rust ecosystems (`Cargo.toml`, `rustfmt.toml`, etc.).

**Value types:**
```rust
enum TomlValue<'a> {
    String(&'a str),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    DateTime(&'a str),
    Array,        // Container node
    Table,        // Container node (key-value section)
}
```

The TOML parser follows the same layered architecture as the other formats: **lexer → parser → typed value tree**, with a dedicated error module for precise diagnostics.

**Lexer** (`lexer.rs`): a byte-level `Cursor<'a>` that tokenizes into `Token` variants — bare keys, quoted strings (basic and literal), integers, floats, booleans, datetimes, brackets, dots, equals signs, and newlines. Handles multi-line basic strings (`"""`) and literal strings (`'''`).

**Parser** (`parser.rs`): recursive-descent parser that handles standard tables (`[section]`), arrays of tables (`[[array]]`), dotted keys (`a.b.c = val`), and inline tables/arrays. Provides configurable depth and size limits via `TomlLimits` to prevent denial-of-service from malicious input. Three entry points:
- `parse_toml(bytes)` — full parse to `TomlValue` tree
- `parse_toml_with_limits(bytes, limits)` — parse with custom nesting/size bounds
- `validate_toml(bytes)` — syntax check without building the value tree

**Error** (`error.rs`): `TomlError` carries a `TomlErrorKind` enum and byte offset. The `line_column()` method converts byte position to human-readable `(line, column)` for diagnostic messages.

**Writer** (`writer.rs`): serialization helpers that emit properly quoted and escaped TOML output — `push_toml_str` for string values, `push_toml_num` for numbers, `push_toml_section` for `[table]` headers, and `push_toml_array_section` for `[[array-of-tables]]` headers.

---

## Chapter 3 - Limits, Precision, and Validation

### Numerical Limits
- `f64` rounding errors can accumulate in long simulations.
- Extreme regimes (very large or very small scales) require explicit numerical stability checks.

### Recommended Verification
- Compare against a simple analytical case when available.
- Check the order of magnitude of results.
- Run sensitivity analysis on dominant parameters.

### Transition to Implementation
For concrete function calls, Rust signatures and module paths are documented in `docs/code/`.
