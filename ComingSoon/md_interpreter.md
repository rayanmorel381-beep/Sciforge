## 1️⃣ Current State

| Building Block | Location | Description |
|---|---|---|
| MdParser | `src/parser/markdown/` | Validates syntax but does not build a tree |
| parse() | `src/parser/markdown/` | Returns empty MdValue::Document, without children |
| parse_* | `src/parser/markdown/` | heading, paragraph, list, table, code, blockquote — advance the cursor and validate without collecting |
| MdValue | `src/parser/markdown/` | Copy (no allocation) → incompatible with a tree |

Structures found in the target .md files (hypothese/) :
heading `#` with numbered emoji, metadata `**Key:** Value`, separator `---`, sections `##`, paragraphs, ordered/unordered lists with sublists, inline (`**bold**`, `*italic*`, `` `code` ``, `[text](url)`), inline LaTeX `\(...\)`.

## 2️⃣ MdNode — Recursive AST

| Variant | Fields | Description |
|---|---|---|
| `Document` | `children: Vec<MdNode>` | Document root |
| `Heading` | `level: u8, text: String` | Heading `#` to `######` |
| `Paragraph` | `spans: Vec<MdInline>` | Paragraph with parsed inline content |
| `List` | `ordered: bool, items: Vec<MdNode>` | Ordered or unordered list |
| `ListItem` | `children: Vec<MdNode>` | List item with sub-elements |
| `CodeBlock` | `lang: Option<String>, content: String` | Fenced code block |
| `ThematicBreak` | — | Separator `---` |
| `BlockQuote` | `children: Vec<MdNode>` | Blockquote with recursive children |
| `Table` | `headers: Vec<String>, rows: Vec<Vec<String>>` | Markdown table |

## 3️⃣ MdInline — Inline Content

| Variant | Fields | Description |
|---|---|---|
| `Text(String)` | plain text | Unformatted text |
| `Bold(String)` | `**text**` | Bold |
| `Italic(String)` | `*text*` | Italic |
| `Code(String)` | `` `code` `` | Inline code |
| `Link` | `text: String, url: String` | Link `[text](url)` |
| `Image` | `alt: String, url: String` | Image `![alt](url)` |
| `Math(String)` | `\(formula\)` | Inline LaTeX |

Transform `validate_inline` into `parse_inline → Vec<MdInline>`.

## 4️⃣ Parser — Tree Construction

| Function | Return | Description |
|---|---|---|
| `parse()` | `MdNode::Document { children }` | Main entry point |
| `parse_heading` | `MdNode::Heading` | Level + text extraction |
| `parse_paragraph` | `MdNode::Paragraph` | Spans via parse_inline |
| `parse_list` | `MdNode::List` | Collected items, ordered/unordered |
| `parse_fenced_code` | `MdNode::CodeBlock` | Language + collected content |
| `parse_block_quote` | `MdNode::BlockQuote` | Recursive children |
| `parse_table` | `MdNode::Table` | Headers + extracted rows |

## 5️⃣ File Reading

| Function | Description |
|---|---|
| `parse_md_file(path: &Path) → Result<MdNode, MdError>` | Read the file with std::fs::read then pass to MdParser::new().parse() |
| `MdErrorKind::IoError` | New variant for filesystem errors |

## 6️⃣ Metadata Extraction

| Component | Description |
|---|---|
| Pattern `**Key:** Value` | Recognized in paragraphs under headings |
| `MdMeta` | `{ domains, elements, variables, ... }` |
| Grouping by `##` | Group child content by level 2 section |

## 7️⃣ Public API

| Function | Description |
|---|---|
| `parse_md(bytes) → MdNode` | Enriched existing parser |
| `parse_md_file(path) → MdNode` | New: reading + parsing |
| `MdNode::sections()` | Iterator over `##` sections with their content |
| `MdNode::metadata() → Option<MdMeta>` | Extraction of `**Key:** Value` |
| `MdNode::text_content() → String` | All plain text concatenated, without markup |

## 8️⃣ Tests

| Test | Description |
|---|---|
| Heading | Parse a hypothese/.md file, verify heading extracted correctly |
| Metadata | Domains/Elements/Variables parsed from `**Key:** Value` |
| Sections | Correct number of `##` sections |
| Lists | List content (Protocol Steps) verified |
| Links | Links extracted from References |
