# Changelog

All notable changes to SciForge are documented in this file.
Each module has a detailed ChangeLog with full function tables.

## Detailed ChangeLogs

| Module | ChangeLog |
|---|---|
| [Astronomy](ChangeLog/astronomy.md) | [Benchmark](ChangeLog/benchmark.md) | [Biology](ChangeLog/biology.md) | [Chemistry](ChangeLog/chemistry.md) |
| [Constants](ChangeLog/constants.md) | [Cosmology](ChangeLog/cosmology.md) | [Documentation](ChangeLog/documentation.md) | [Geology](ChangeLog/geology.md) |
| [Hub](ChangeLog/hub.md) | [Mathematics](ChangeLog/mathematics.md) | [Meteorology](ChangeLog/meteorology.md) | [Parser](ChangeLog/parser.md) |
| [Periodic Table](ChangeLog/periodic_table.md) | [Physics](ChangeLog/physics.md) | [Testing](ChangeLog/testing.md) | |

---

## [0.0.3] - 2026-04-11

| Module | Changes |
|---|---|
| [Astronomy](ChangeLog/astronomy.md) | New `galactic` submodule (21 pub fn) |
| [Benchmark](ChangeLog/benchmark.md) | Refactored `export.rs` → `export/` (8 files) |
| [Constants](ChangeLog/constants.md) | Restructured 5 flat files → 6 domain submodules (20 files, ~527 pub const) |
| [Cosmology](ChangeLog/cosmology.md) | Expanded 91 → 124 pub fn (+33), replaced hardcoded constants |
| [Documentation](ChangeLog/documentation.md) | Updated 14/23 docs files, added 15 ChangeLog files |
| [Geology](ChangeLog/geology.md) | New `erosion` (5 fn) and `geomorphology` (20 fn) submodules |
| [Hub](ChangeLog/hub.md) | Refactored dispatch → per-domain (~190 files), new `astrobiology` cross-domain |
| [Mathematics](ChangeLog/mathematics.md) | 10 new functions (`integration/quadrature` +8, `probability/distributions` +2) |
| [Meteorology](ChangeLog/meteorology.md) | New `clouds` submodule (21 fn) + new functions across submodules |
| [Parser](ChangeLog/parser.md) | New TOML parser (10 fn, 5 structs, 3 enums, 3 consts) |
| [Physics](ChangeLog/physics.md) | New `particle` submodule (35 pub fn) |
| [Testing](ChangeLog/testing.md) | 369 → 814 tests (+445), 46 → 172 test files (+126) |
| Examples | 8 runnable examples — periodic_table, solar_system, chemical_reactions, star_catalog, weather_station, radioactive_decay, unit_converter, geological_ages |
| Refactoring | Split files > 600 lines (`src/benchmark/*/`, `tests/parser.rs`, `tests/benchmark.rs`) |
| Documentation | Complete API reference for all public functions |
| Testing | Complete test coverage in less-covered scientific submodules |
| Validation | Scientific validation pipeline with blocking thresholds |
| Hub | Cross-domain scenarios (astrochemistry, geophysics, …) with numeric validations |
| Benchmark | Readable `.bmk` in HTML benchmark reports |
| Constants | Deleted duplicate `SPEED_OF_LIGHT` (`C`) across the codebase |

---

## [0.0.2] - 2026-03-24

- [Documentation](ChangeLog/documentation.md) — consolidated under `docs/` (paired `modules/` + `code/` guides)
- [Testing](ChangeLog/testing.md) — scientific validation and property-based suites, expanded integration coverage
- [Astronomy](ChangeLog/astronomy.md) / [Hub](ChangeLog/hub.md) — extended cosmology `E(z)`, `H(z)` helpers and Hub dispatch integration
- Bumped crate version to `0.0.2`

---

## [0.0.1] - 2026-03-23

Initial release — Rust edition 2024, zero external dependencies, 575 source files, 48 600+ lines.

| Module | Scope |
|---|---|
| [Constants](ChangeLog/constants.md) | CODATA physical constants, astrophysical/atomic constants, unit conversions, 118 elements |
| [Mathematics](ChangeLog/mathematics.md) | 17 submodules — complex, tensor, vector, polynomial, linalg, sparse, integration, interpolation, ODE, PDE, FFT, optimization, statistics, probability, signal, graph, non-euclidean |
| [Physics](ChangeLog/physics.md) | 11 submodules — relativity, quantum, thermo, electrodynamics, fluids, solids, optics, acoustics, nucleosynthesis, electronics, materials |
| [Chemistry](ChangeLog/chemistry.md) | 26 submodules — kinetics, equilibrium, electro/thermo-chemistry, molecular, organic, inorganic, analytical, quantum, solutions, gas laws, nuclear, polymers, spectroscopy, crystallography, etc. |
| [Biology](ChangeLog/biology.md) | 44 submodules — population dynamics, genetics, genomics, enzyme kinetics, neuroscience, ecology, evolution, bioinformatics, microbiology, etc. |
| [Geology](ChangeLog/geology.md) | 4 submodules — seismology, radiometric dating, petrology, plate tectonics |
| [Astronomy](ChangeLog/astronomy.md) | 4 submodules — orbital mechanics, stellar astrophysics, cosmology, celestial mechanics |
| [Meteorology](ChangeLog/meteorology.md) | 4 submodules — atmospheric modeling, radiation transfer, dynamics, precipitation |
| [Hub](ChangeLog/hub.md) | Central dispatch API (`api`, `domain`, `engine`, `tools`) + prelude |
| [Benchmark](ChangeLog/benchmark.md) | 6 submodules — engine, encode/decode, simulation, report, export (HTML dashboard, multi-format) |
| [Parser](ChangeLog/parser.md) | 5 submodules — CSV, JSON, YAML, Markdown, HTML |
| [Periodic Table](ChangeLog/periodic_table.md) | 118 element data files by category |
| [Testing](ChangeLog/testing.md) | 94 tests (28 benchmark + 66 parser), zero warnings |
