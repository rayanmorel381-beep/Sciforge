# SciForge — Documentation

SciForge is a scientific computing library written in pure Rust, with no external dependencies. It covers a wide range of domains — from classical physics to molecular biology — organized into independent modules, each exposing both a domain specification and a Rust implementation.

---

## Chapter 1 - Documentation Scope

### Purpose
This page explains how the documentation is organized, what each section is meant to provide, and how to move between scientific explanations and Rust APIs.

### Reading Strategy
1. Start with the module page in `docs/modules/` to understand the scientific scope and models.
2. Continue with the matching page in `docs/code/` to inspect the implementation layout and public-facing API.
3. Use the repository structure overview at the end of this page to locate the relevant files quickly.


## Reading Guide

### Goal
This page acts as the entry point for the documentation set and maps each scientific area to its conceptual guide and code guide.

### How To Read This Document
- Use the module summaries below to identify the relevant scientific area.
- Open the paired `modules/` and `code/` pages depending on whether you need theory or implementation details.
- Use the repository tree at the end as a navigation shortcut.

### Conventions
- `modules/` pages focus on scientific models, assumptions, and equations.
- `code/` pages focus on source layout, exported APIs, and execution flow.
- Paths are written relative to the repository root.

Each module has two complementary pages:

- **Modules/** — describes the scientific concepts, formulas, and phenomena the module covers. Start here to understand *what* a module models.
- **Code/** — describes the Rust API: structs, functions, inputs/outputs. Go here when you want to *use* a module.

---

## Chapter 2 - Modules at a Glance

### [Astronomy](modules/Astronomy.md)
Celestial mechanics, astrophysics, and cosmology (4 submodules). Covers orbital dynamics (Kepler's laws, vis-viva, Hohmann transfers, orbital perturbations), stellar physics (HR diagram, luminosity-mass-radius relations, main-sequence lifetime, stellar nucleosynthesis), cosmology (direct `E(z)` parameterizations, `H(z)` evaluation, comoving/luminosity/angular-diameter distances in LCDM/general/wCDM/CPL, dark-energy equation-of-state models, analytical LCDM shortcuts), and celestial mechanics (ecliptic/equatorial/galactic coordinate transforms, Julian date, sidereal time, nutation).
→ [Module spec](modules/Astronomy.md) · [Rust API](code/Astronomy.md)

---

### [Benchmark](modules/Benchmark.md)
Performance measurement infrastructure for SciForge (6 submodules). Covers a 30+ field `BenchmarkMetrics` engine with statistical aggregation and CSV export, a compact binary encoding format (magic header, LE layout, 168+ byte records), zero-copy decoding via `BenchmarkMetricsView`, a simulation framework with `SimState`/`SimConfig`/`StepFn` traits, multi-format report generation (CSV, Markdown, HTML, JSON, YAML, TOML), and a file-based export orchestrator with periodic-table-themed CSS.
→ [Module spec](modules/Benchmark.md) · [Rust API](code/Benchmark.md)

---

### [Biology](modules/Biology.md)
The largest scientific module (44 submodules, 176 source files). Covers aging (Gompertz/Weibull mortality, telomere dynamics), bioelectricity (Nernst, Goldman–Hodgkin–Katz, Hodgkin–Huxley), bioenergetics (ATP yield, Kleiber's law), biomechanics (Poiseuille flow, Hill muscle model), biophysics (Helfrich membrane, WLC, FRET), biostatistics (Kaplan–Meier, meta-analysis), ecology (Lotka–Volterra, Rosenzweig–MacArthur), enzyme kinetics (Michaelis–Menten, Hill, inhibition), evolution (molecular clock, dN/dS, coalescent), genetics (Hardy–Weinberg, $F_{ST}$), immunology (affinity maturation), neuroscience (integrate-and-fire), pharmacology (PK/PD, hepatic clearance), population dynamics (Leslie matrix, SIR/SEIR), synthetic biology (toggle switch, repressilator, FBA), virology (quasispecies, error catastrophe), and 28 more submodules.
→ [Module spec](modules/Biology.md) · [Rust API](code/Biology.md)

---

### [Chemistry](modules/Chemistry.md)
Physical, analytical, inorganic, and computational chemistry (26 submodules). Covers acid–base equilibria (Henderson–Hasselbalch, polyprotic titrations), kinetics (Arrhenius, Eyring, Lindemann), electrochemistry (Nernst, Butler–Volmer, Tafel), thermochemistry (Hess's law, Kirchhoff), gas laws (van der Waals, virial, compressibility), quantum chemistry (Hartree–Fock, DFT concepts, Slater determinants), crystallography (Bragg's law, Miller indices, 14 Bravais lattices), molecular modeling (Lennard-Jones, Morse potential, VSEPR), colloids (DLVO theory, Stokes–Einstein), green chemistry (atom economy, E-factor), and environmental chemistry (Henry's law, BOD/COD).
→ [Module spec](modules/Chemistry.md) · [Rust API](code/Chemistry.md)

---

### [Constants](modules/Constants.md)
Curated registry of physical constants, astronomical data, atomic properties, and unit conversions (5 submodules). Includes 16 CODATA fundamental constants ($c$, $G$, $h$, $\hbar$, $k_B$, $N_A$, $e$, $\varepsilon_0$, $\mu_0$, $\sigma_{SB}$, $\alpha$, Planck units), 10 astronomical constants (AU, parsec, solar/Earth parameters, Hubble constant), 13 atomic constants (particle masses, Bohr radius, Rydberg energy, magnetons), the full periodic table (118 elements loaded from YAML with `OnceLock`), and 12 unit conversion factors (eV↔J, cal↔J, atm↔Pa, deg↔rad, barn, ångström, fermi).
→ [Module spec](modules/Constants.md) · [Rust API](code/Constants.md)

---

### [Geology](modules/Geology.md)
Earth sciences and geophysics (4 submodules). Covers radiometric dating (decay law $N(t) = N_0 e^{-\lambda t}$, half-life conversion, isochron method, $^{14}$C calibration), petrology (CIPW norm, magma viscosity, Stokes settling, crystallization sequences), seismology (P/S/surface wave propagation, Gutenberg–Richter, Omori aftershock law, magnitude scales $M_L$/$M_W$/$m_b$, focal mechanisms), and tectonics (plate velocity on a sphere, Euler poles, Byerlee's friction law, thermal subsidence).
→ [Module spec](modules/Geology.md) · [Rust API](code/Geology.md)

---

### [Hub](modules/Hub.md)
Central orchestration layer (5 areas: API, Domain, Engine, Tools, Prelude — 35 source files). Provides a zero-dependency HTTP server (`TcpListener`-based), structured DTOs (`ComputeRequest`/`ComputeResponse` with 8 result types), domain dispatchers for all 7 scientific modules, direct astronomy-cosmology routing (`E(z)`, `H(z)`, and distance functions) through the Hub, an `Experiment` builder with 15 `ParameterValue` variants (Scalar through Tensor), a `DynamicalSystem` trait for ODE simulations (with built-in HarmonicOscillator/LotkaVolterra), priority-based task scheduling with dependency-aware topological sort, campaign batch execution, and tooling (Config, Logger, Metrics).
→ [Module spec](modules/Hub.md) · [Rust API](code/Hub.md)

---

### [Mathematics](modules/Mathematics.md)
Numerical methods and pure mathematics (17 submodules). Covers complex arithmetic, linear algebra (Gauss elimination, LU/Cholesky decomposition, eigenvalues), calculus (symbolic differentiation, Simpson/Gauss quadrature, automatic differentiation), ODE solvers (Euler, RK4, adaptive RK45), Fourier and Laplace transforms, statistics (descriptive, distributions, hypothesis testing), probability, number theory (primes, GCD, modular arithmetic), combinatorics, set theory, graph theory (BFS, DFS, Dijkstra, Kruskal), geometry (2D/3D transforms, convex hull), trigonometry, polynomial operations, vector calculus (gradient, divergence, curl, Stokes' theorem), and special functions (Gamma, Beta, Bessel, error function).
→ [Module spec](modules/Mathematics.md) · [Rust API](code/Mathematics.md)

---

### [Meteorology](modules/Meteorology.md)
Atmospheric science and weather physics (4 submodules). Covers atmosphere (barometric formula, ISA standard layers, pressure altitude, scale height, lapse rates), dynamics (geostrophic/gradient wind, Rossby number, vorticity, thermal wind, Ekman spiral, CAPE/CIN, Richardson number), precipitation (Clausius–Clapeyron, Köhler theory, Marshall–Palmer DSD, $Z$–$R$ relations, terminal velocity, Bergeron process), and radiation (Planck blackbody, Wien's law, Stefan–Boltzmann, solar zenith angle, Beer–Lambert atmospheric extinction, albedo, greenhouse radiative forcing $\Delta F = 5.35 \ln(C/C_0)$).
→ [Module spec](modules/Meteorology.md) · [Rust API](code/Meteorology.md)

---

### [Parser](modules/Parser.md)
Multi-format data parsing library with zero dependencies (5 formats, 26 source files). Covers CSV (RFC 4180, `CsvValue` enum with Table/Record/Field, quoted field handling, writer), JSON (RFC 8259, `JsonValue` with 6 variants, number/string submodules), YAML (`YamlValue` with indent-based `LineCursor`, scalar type auto-detection for booleans/numbers/null — used by the Constants module for periodic table data), HTML (`HtmlValue` with Document/Element/Text/Comment, named entity decoding — used by Benchmark export), and Markdown (`MdValue` with 9 block-level variants, inline emphasis/links/code processing). All formats share a layered `Cursor<'a>` architecture with zero-copy `&'a str` borrowing and `const fn` constructors.
→ [Module spec](modules/Parser.md) · [Rust API](code/Parser.md)

---

### [Physics](modules/Physics.md)
Classical and modern physics (11 submodules). Covers acoustics (wave equation, Doppler, decibel scale, resonance), electrodynamics (Maxwell's equations, Lorentz force, electromagnetic waves, Poynting vector), electronics (Ohm's law, RC/RL/RLC circuits, transistor models, op-amp configurations), fluid mechanics (Navier–Stokes, Bernoulli, Reynolds number, drag), materials science (stress–strain, Young's modulus, Hooke's law, creep), nucleosynthesis (binding energy, decay chains, cross-sections, pp-chain, CNO cycle), optics (Snell's law, thin lens, diffraction, interference, polarization), quantum mechanics (Schrödinger equation, particle-in-a-box, harmonic oscillator, hydrogen atom, tunneling), relativity (Lorentz transforms, $E = mc^2$, time dilation, gravitational redshift, Schwarzschild metric), solid mechanics (beam bending, torsion, Mohr's circle, buckling), and thermodynamics (laws of thermodynamics, Carnot cycle, entropy, heat transfer, blackbody radiation).
→ [Module spec](modules/Physics.md) · [Rust API](code/Physics.md)

---

## Chapter 3 - Repository Structure

```
docs/
├── Summary.md          # This file
├── modules/            # Scientific domain documentation (concepts & formulas)
│   ├── Astronomy.md
│   ├── Benchmark.md
│   ├── Biology.md
│   ├── Chemistry.md
│   ├── Constants.md
│   ├── Geology.md
│   ├── Hub.md
│   ├── Mathematics.md
│   ├── Meteorology.md
│   ├── Parser.md
│   └── Physics.md
└── code/               # Rust implementation documentation (API reference)
    ├── Astronomy.md
    ├── Benchmark.md
    ├── Biology.md
    ├── Chemistry.md
    ├── Constants.md
    ├── Geology.md
    ├── Hub.md
    ├── Mathematics.md
    ├── Meteorology.md
    ├── Parser.md
    └── Physics.md

```

