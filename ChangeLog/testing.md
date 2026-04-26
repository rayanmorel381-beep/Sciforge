# Testing — ChangeLog

---

## v0.0.4

### Expansion

| Metric | v0.0.3 | v0.0.4 | Delta |
|---|---|---|---|
| Total tests | 814 | 823 | +9 |

### Changes

| Change | Detail |
|---|---|
| New tests | Bayesian inference, PCA, k-means, DBSCAN, silhouette, `error_propagation`, `monte_carlo_uncertainty`, `rayleigh_phase`, new astronomy submodules |
| Stress test | `tests/stress/main.rs` — `metric()` signature migrated `f32` → `f64` |
| Engine test | `tests/benchmark/engine.rs` — assertions updated: `BENCHMARK_VERSION = 5`, `BENCHMARK_HEADER_SIZE = 216` |

---

## v0.0.3

### Expansion

| Metric | v0.0.2 | v0.0.3 | Delta |
|---|---|---|---|
| Total tests | 369 | 814 | +445 |
| Test files | 46 | 172 | +126 |

### Per-domain breakdown

| Domain | Tests v0.0.2 | Tests v0.0.3 | Delta | Files v0.0.3 |
|---|---|---|---|---|
| Astronomy | 42 | 60 | +18 | 10 |
| Benchmark | 28 | 28 | — | 5 |
| Biology | 23 | 184 | +161 | 49 |
| Chemistry | 29 | 107 | +78 | 28 |
| Constants | 33 | 33 | — | 5 |
| Geology | 22 | 45 | +23 | 8 |
| Hub | 33 | 83 | +50 | 16 |
| Mathematics | 26 | 71 | +45 | 22 |
| Meteorology | 23 | 46 | +23 | 8 |
| Parser | 66 | 77 | +11 | 7 |
| Physics | 38 | 74 | +36 | 17 |
| Scientific properties | 3 | 3 | — | 1 |
| Scientific validation | 3 | 3 | — | 1 |

### Test files — Astronomy (10 files, 60 tests)

| File | Tests |
|---|---|
| `astronomy/main.rs` | 1 |
| `astronomy/celestial.rs` | — |
| `astronomy/cosmology.rs` | — |
| `astronomy/galactic.rs` | — |
| `astronomy/impacts.rs` | — |
| `astronomy/orbits.rs` | — |
| `astronomy/planetary.rs` | — |
| `astronomy/rotation.rs` | — |
| `astronomy/stellar.rs` | — |
| `astronomy/tides.rs` | — |

### Test files — Benchmark (5 files, 28 tests)

| File | Tests |
|---|---|
| `benchmark/main.rs` | — |
| `benchmark/encode.rs` | — |
| `benchmark/engine.rs` | — |
| `benchmark/report.rs` | — |
| `benchmark/simulation.rs` | — |

### Test files — Biology (49 files, 184 tests)

1 test file per submodule covering all 44 biology subdirectories + `dispatch.rs`, `main.rs`, additional integration files.

### Test files — Chemistry (28 files, 107 tests)

1 test file per submodule covering all 26 chemistry subdirectories + `dispatch.rs`, `main.rs`.

### Test files — Constants (5 files, 33 tests)

| File | Tests |
|---|---|
| `constants/fundamental.rs` | 10 |
| `constants/astro.rs` | 8 |
| `constants/atomic.rs` | 7 |
| `constants/units.rs` | 7 |
| `constants/main.rs` | 1 |

### Test files — Geology (8 files, 45 tests)

| File | Tests |
|---|---|
| `geology/main.rs` | — |
| `geology/seismology.rs` | — |
| `geology/dating.rs` | — |
| `geology/tectonics.rs` | — |
| `geology/erosion.rs` | — |
| `geology/glaciology.rs` | — |
| `geology/hydrology.rs` | — |
| `geology/petrology.rs` | — |

### Test files — Hub (16 files, 83 tests)

| File | Tests |
|---|---|
| `hub/main.rs` | 1 |
| `hub/tools.rs` | — |
| `hub/pipeline.rs` | — |
| `hub/cross_domain/astrobiology.rs` | — |
| `hub/cross_domain/astrochemistry.rs` | — |
| `hub/cross_domain/astrophysics.rs` | — |
| `hub/cross_domain/atmospheric_chemistry.rs` | — |
| `hub/cross_domain/atmospheric_physics.rs` | — |
| `hub/cross_domain/biochemistry.rs` | — |
| `hub/cross_domain/biomathematics.rs` | — |
| `hub/cross_domain/biophysics.rs` | — |
| `hub/cross_domain/geochemistry.rs` | — |
| `hub/cross_domain/geophysics.rs` | — |
| `hub/cross_domain/mathematical_physics.rs` | — |
| `hub/cross_domain/planetary_geology.rs` | — |

### Test files — Mathematics (22 files, 71 tests)

1 test file per submodule (complex, fft, graph, integration, interpolation, linalg, non_euclidean, ode, optimization, pde, polynomial, probability, signal, sparse, statistics, tensor, vector) + `misc.rs`, `main.rs`, `dispatch.rs`, `calculus.rs`.

### Test files — Meteorology (8 files, 46 tests)

| File | Tests |
|---|---|
| `meteorology/main.rs` | — |
| `meteorology/atmosphere.rs` | — |
| `meteorology/dynamics.rs` | — |
| `meteorology/radiation.rs` | — |
| `meteorology/ocean.rs` | — |
| `meteorology/precipitation.rs` | — |
| `meteorology/storms.rs` | — |
| `meteorology/winds.rs` | — |

### Test files — Parser (7 files, 77 tests)

| File | Tests |
|---|---|
| `parser/main.rs` | — |
| `parser/csv.rs` | — |
| `parser/html.rs` | — |
| `parser/json.rs` | — |
| `parser/markdown.rs` | — |
| `parser/toml.rs` | — |
| `parser/yaml.rs` | — |

### Test files — Physics (17 files, 74 tests)

| File | Tests |
|---|---|
| `physics/main.rs` | — |
| `physics/acoustics.rs` | — |
| `physics/dispatch.rs` | — |
| `physics/electrodynamics.rs` | — |
| `physics/electronics.rs` | — |
| `physics/fluid_mechanics.rs` | — |
| `physics/materials.rs` | — |
| `physics/mechanics.rs` | — |
| `physics/nucleosynthesis.rs` | — |
| `physics/optics.rs` | — |
| `physics/particle.rs` | — |
| `physics/quantum.rs` | — |
| `physics/relativity.rs` | — |
| `physics/solid_mechanics.rs` | — |
| `physics/thermodynamics.rs` | — |

### Test files — Standalone (2 files, 6 tests)

| File | Tests |
|---|---|
| `scientific_properties.rs` | 3 |
| `scientific_validation.rs` | 3 |

---

## v0.0.2

### Expansion

| Metric | v0.0.1 | v0.0.2 | Delta |
|---|---|---|---|
| Total tests | 94 | 369 | +275 |
| Test files | 2 | 46 | +44 |

### Per-domain breakdown

| Domain | Tests | Files |
|---|---|---|
| Astronomy | 42 | 4 |
| Benchmark | 28 | 1 |
| Biology | 23 | 5 |
| Chemistry | 29 | 5 |
| Constants | 33 | 5 |
| Geology | 22 | 4 |
| Hub | 33 | 4 |
| Mathematics | 26 | 5 |
| Meteorology | 23 | 4 |
| Parser | 66 | 1 |
| Physics | 38 | 6 |
| Scientific properties | 3 | 1 |
| Scientific validation | 3 | 1 |

---

## v0.0.1

### Initial test suite

| File | Tests | Coverage |
|---|---|---|
| `tests/benchmark.rs` | 28 | Engine metrics, encode/decode roundtrip, simulation determinism (118 elements), report format, multi-format export |
| `tests/parser.rs` | 66 | CSV, JSON, YAML, Markdown, HTML parsing correctness, validation, edge cases, empty/malformed input, nested structures, escape sequences |

| Metric | Value |
|---|---|
| Total tests | 94 |
| Test files | 2 |
| Clippy warnings | 0 |
| `#[allow]` directives | 0 |
