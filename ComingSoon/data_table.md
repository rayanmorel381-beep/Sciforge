## 1️⃣ Current State

| Building Block | Location | Description |
|---|---|---|
| CsvValue<'a> | `src/parser/csv/` | Event-based (Table, Record, Field) — no tabular storage |
| write_csv | `src/parser/csv/` | `write_csv(header, rows) → String` — direct writing |
| Entry<'a> | `src/benchmark/export.rs` | BTreeMap<String, String> to accumulate columns per entry |
| BenchmarkMetrics | `src/benchmark/` | 25+ fields, serialized to CSV/JSON/YAML/TOML via string concatenation |
| CSV_HEADER | `src/benchmark/` | Hardcoded string constant |

No intermediate structure between parsing and export. Data in `Vec<String>` or `BTreeMap`, no per-column typing.

## 2️⃣ Data Structure — `src/hub/tools/table/types.rs`

| Type | Description |
|---|---|
| `CellValue::Null` | Null value |
| `CellValue::Bool(bool)` | Boolean |
| `CellValue::Integer(i64)` | 64-bit integer |
| `CellValue::Float(f64)` | 64-bit float |
| `CellValue::Text(String)` | Text |
| `Column` | `{ name: String, values: Vec<CellValue> }` |
| `DataTable` | `{ columns: Vec<Column>, row_count: usize }` |

Invariant: all columns have the same length (row_count).

## 3️⃣ Building — `src/hub/tools/table/builder.rs`

| Function | Signature | Description |
|---|---|---|
| new | `DataTableBuilder::new() → Self` | Empty builder |
| add_column | `add_column(name, values) → &mut Self` | Add a column |
| add_row | `add_row(values) → &mut Self` | Add a value to each column |
| build | `build() → Result<DataTable, TableError>` | Build (checks lengths) |
| from_rows | `from_rows(headers, rows) → Result<DataTable, TableError>` | Row-based constructor |

## 4️⃣ Operations — `src/hub/tools/table/ops.rs`

| Function | Signature | Description |
|---|---|---|
| column | `column(name) → Option<&Column>` | Access column by name |
| column_index | `column_index(name) → Option<usize>` | Column index |
| row | `row(idx) → Option<Vec<&CellValue>>` | Access row by index |
| select | `select(columns) → DataTable` | Column subset |
| filter | `filter(column, predicate) → DataTable` | Filter by predicate |
| sort_by | `sort_by(column, ascending) → DataTable` | Sort by column |
| head | `head(n) → DataTable` | First N rows |
| tail | `tail(n) → DataTable` | Last N rows |
| shape | `shape() → (usize, usize)` | (rows, cols) |
| rename_column | `rename_column(old, new) → Result<(), TableError>` | Rename column |
| add_computed_column | `add_computed_column(name, f) → ()` | Computed column |

## 5️⃣ Statistics — `src/hub/tools/table/stats.rs`

| Function | Signature | Description |
|---|---|---|
| column_sum | `column_sum(col) → Option<f64>` | Sum (if numeric) |
| column_mean | `column_mean(col) → Option<f64>` | Mean |
| column_min | `column_min(col) → Option<f64>` | Minimum |
| column_max | `column_max(col) → Option<f64>` | Maximum |
| column_stddev | `column_stddev(col) → Option<f64>` | Standard deviation |
| column_count_nulls | `column_count_nulls(col) → usize` | Null count |
| column_unique_count | `column_unique_count(col) → usize` | Unique values |
| describe | `DataTable::describe() → DataTable` | Statistical summary (count, mean, std, min, max per numeric column) |

## 6️⃣ Conversion — `src/hub/tools/table/convert.rs`

| Function | Signature | Description |
|---|---|---|
| from_csv | `from_csv(bytes) → Result<DataTable, TableError>` | Via CsvParser, type inference (Integer → Float → Text) |
| from_json_array | `from_json_array(bytes) → Result<DataTable, TableError>` | Via JsonParser, array of objects → columns |
| to_csv | `to_csv(&self) → String` | Via existing write_csv |
| to_json | `to_json(&self) → String` | Array of objects |
| to_yaml | `to_yaml(&self) → String` | YAML |
| to_toml | `to_toml(&self) → String` | TOML (when the parser exists) |
| from_benchmark_metrics | `from_benchmark_metrics(metrics) → DataTable` | 25+ fields → typed columns |

## 7️⃣ Error Handling — `src/hub/tools/table/error.rs`

| Variant | Description |
|---|---|
| `ColumnNotFound(String)` | Column not found |
| `RowOutOfBounds(usize)` | Row index out of bounds |
| `ColumnLengthMismatch { expected, found }` | Inconsistent column lengths |
| `TypeMismatch { column, expected }` | Wrong type for operation |
| `ParseError(String)` | Parsing error |

## 8️⃣ Display — `src/hub/tools/table/display.rs`

| Component | Description |
|---|---|
| `impl Display for DataTable` | Aligned ASCII table rendering |
| Format | `\| name \| value \| unit \|` with `---` separators |
| Truncation | Overly wide columns truncated |
| Footer | `[N rows × M columns]` |

## 9️⃣ Module Structure

| File | Description |
|---|---|
| `src/hub/tools/table/mod.rs` | mod types, builder, ops, stats, convert, error, display |
| Re-exports | DataTable, DataTableBuilder, CellValue, Column, TableError |
| `src/hub/tools/mod.rs` | Add `pub mod table` |

## 🔟 Refactoring Export

| Task | Description |
|---|---|
| Pipeline | compute → DataTable → filter/sort → export multi-format |
| Refactor export.rs | Go through `from_benchmark_metrics() → DataTable → to_csv()`, etc. |
| Removal | Remove the ad hoc `BTreeMap<String, String>` |

## 1️⃣1️⃣ Tests

| Test | Description |
|---|---|
| Building | from_rows, builder, verify shape() |
| Operations | select, filter, sort, head/tail |
| Statistics | mean, stddev on known data |
| Round-trip CSV | parse → DataTable → to_csv → parse → compare |
| Round-trip JSON | same for JSON |
| from_benchmark_metrics | Verify all fields become columns |
| Display | Verify formatted ASCII rendering |
| Errors | column not found, row out of bounds, length mismatch |
