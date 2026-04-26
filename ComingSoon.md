# Coming Soon

SciForge — pure Rust (edition 2024, zero dependencies) scientific computing library.

## Current State (According to 0.0.1 Version)

| Area | Item | Status | Code/Docs Evidence | Remaining Coverage |
|---|---|---|---|---|
| Documentation | Full API reference with input/output examples for every public function | Partial | `docs/code/*.md`, `docs/modules/*.md` | Generate an exhaustive reference for all public functions (`benchmark`, `hub`, `parser`) with runnable examples |
| Documentation | Tutorials and guided workflows (simulation, data analysis, cross-domain) | Partial | `README.md`, `docs/` | Add end-to-end scenario tutorials with expected inputs/outputs |
| Documentation | Module-level scientific model guides | Implemented | `docs/modules/Astronomy.md`, `docs/modules/Physics.md`, `docs/modules/Biology.md`, and others | Ongoing maintenance and enrichment |
| Extended Testing | Unit and integration tests for all scientific modules | Partial | `tests/constants/*`, `tests/maths/*`, `tests/physics/*`, `tests/chemistry/*`, `tests/biology/*`, `tests/geology/*`, `tests/astronomy/*`, `tests/meteorology/*` | Close remaining coverage gaps in less-tested submodules |
| Extended Testing | Property-style testing and edge-case validation | Implemented | `tests/scientific_properties.rs`, `tests/scientific_validation.rs` | Expand invariant coverage per domain |
| Extended Testing | Cross-module integration tests | Implemented | `tests/hub/cross_domain.rs`, `tests/hub/main.rs` | Add more multi-step cross-domain scenarios |
| Hub Expansion | Unified dispatch API through Hub | Implemented | `src/hub/engine/dispatch/*.rs` | Continue signature normalization |
| Hub Expansion | Ergonomic prelude re-exports | Implemented | `src/hub/prelude.rs` | Stabilize naming conventions |
| Hub Expansion | Domain query engine and tool pipelines | Implemented | `src/hub/engine/query.rs`, `src/hub/engine/pipeline/flow.rs`, `src/hub/engine/worker/scheduler.rs` | Add more predefined pipeline templates |
| Computational Enhancements | Performance benchmarking across scientific modules | Partial | `src/benchmark/*`, `tests/benchmark.rs`, `output/*/bmk/` | Complete a benchmark matrix for all major subdomains |
| Computational Enhancements | Memory-optimized structures for large-scale workloads | Partial | `src/hub/tools/arena.rs`, `src/hub/tools/profiler.rs` | Add systematic memory/performance evidence |
| Computational Enhancements | Deterministic and reproducible execution paths | Implemented | `src/hub/tools/deterministic.rs` | Extend reproducibility assertions in tests |
| Future Features | Interactive visualization utilities | Implemented | `src/hub/tools/visualization.rs`, `src/benchmark/export.rs` | Add more visualization templates |
| Future Features | Extended cross-domain analysis tools (astrochem, geophysics, bioinformatics, etc.) | Partial | `src/biology/bioinformatics/*`, cross-domain dispatch paths in `src/hub/engine/dispatch/*` | Expand explicit coupled-domain coverage (astrochem/geophysics) |
| Future Features | Additional export formats and reporting | Implemented | HTML/MD/CSV/JSON/YAML/TOML export via `src/benchmark/export.rs` and `output/` | Add cross-format validation schemas |
| Future Features | Automated scientific validation pipeline | Partial | `tests/scientific_validation.rs` + `output/validation/` artifacts | Formalize a single pipeline command with gating thresholds |

## Quantitative Summary

| Status | Count |
|---|---|
| Implemented | 9 |
| Partial | 8 |
| Remaining | 0 |

## 0.0.3 — Backlog

1. Refactor files > 600 lines (split `src/benchmark/*/`, `tests/parser.rs`, `tests/benchmark.rs`)
2. Complete API reference for all public functions
3. Complete test coverage in less-covered scientific submodules
4. Scientific validation pipeline with blocking thresholds
5. Cross-domain scenarios (astrochemistry, geophysics, …) with numeric validations
6. Readable `.bmk` in HTML benchmark reports
7. Delete duplicate `SPEED_OF_LIGHT` (`C`) across the codebase
8. [TOML parser](ComingSoon/toml_parser.md) — full TOML v1.0 spec parser (keys, tables, arrays, multi-line strings, dates)

## 0.0.4 — Next Priorities

1. Uncertainty quantification (Monte Carlo, bootstrap, error propagation)
2. Heuristic optimization (simulated annealing, genetic algorithms, PSO)
3. Wavelet and spectral analysis (astronomy, seismology, meteorology)
4. Lightweight Bayesian inference (data exploration, inverse problems)
5. PCA and clustering (dimensionality reduction, structure discovery)
6. New constants and functions — details:
   - [Constants](ComingSoon/constants.md) — fundamental physics, astronomy (planets, satellites, dark matter, galactic, stellar), chemistry, units
   - [Astronomy functions](ComingSoon/astronomy_functions.md) — tidal physics, rotational dynamics, general relativity corrections, impact/cratering
   - [Meteorology functions](ComingSoon/meteorology_functions.md) — atmospheric physics, winds & storms, oceanography
   - [Geology functions](ComingSoon/geology_functions.md) — hydrology, glaciology, erosion & weathering
7. Convert every f32 to f64 precision

## 0.0.5 — Future Features

- [Markdown interpreter](ComingSoon/md_interpreter.md) — full AST-based `.md` parsing (MdNode, MdInline, metadata extraction)
- [3D/5D rendering engine](ComingSoon/3&5d.md) — software rasterizer, camera, Phong lighting, 5D projection

## 0.0.6 — Demo & Tutorials

1. Reproducible guided tutorials (Hub + Parser + Benchmark)
2. Small demo exercising the full crate

## Planned Modules

- [Symbolic expressions](ComingSoon/symbolic_expressions.md) — expression tree, eval, simplify, differentiate, LaTeX conversion
- [Dimensional analysis](ComingSoon/dimensional_analysis.md) — type-safe `Quantity` with SI dimensions, compile-time checks
- [Data table](ComingSoon/data_table.md) — lightweight `DataTable` structure with column stats, filter, sort, CSV/JSON bridges
- [Parallel execution](ComingSoon/parallel_execution.md) — native thread pool, scoped `parallel_for`, parallel scheduler
- [APK creation](ComingSoon/apk_creation.md) — Android packaging via JNI bridge, touch-enabled 3D viewer
- Performance: pure Rust CPU rasterizer, optional SIMD for scanline fill and matrix ops, benchmark via `BenchmarkMetrics` (ns/frame, pixels/s), target resolution 800×600+